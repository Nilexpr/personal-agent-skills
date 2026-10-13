use super::*;
use sha2::{Digest, Sha256};
use std::io::Read;

#[derive(Clone, Copy, Default, clap::ValueEnum)]
pub enum Filter {
    #[default]
    All,
    Unlinked,
    Unanalyzed,
    Stale,
    Remaining,
}

impl Workspace {
    pub(super) fn fingerprint(&self, locator: &str) -> Result<String> {
        self.require_file(locator)?;
        let path = self.root.join(locator);
        let mut file = fs::File::open(&path).map_err(|e| io_error(e, &path))?;
        let mut hash = Sha256::new();
        let mut buffer = [0; 65536];
        loop {
            let count = file.read(&mut buffer).map_err(|e| io_error(e, &path))?;
            if count == 0 {
                break;
            }
            hash.update(&buffer[..count]);
        }
        self.require_file(locator)?;
        Ok(format!("{:x}", hash.finalize()))
    }

    pub(super) fn verify_fingerprint(&self, locator: &str, expected: &str) -> Result<()> {
        let actual = self.fingerprint(locator)?;
        if actual != expected {
            return Err(Failure::new(
                "CONTENT_CHANGED",
                "文件内容与提交的分析指纹不同，请重新读取并分析",
            )
            .detail(json!({"locator": locator, "expected": expected, "actual": actual})));
        }
        Ok(())
    }

    pub(super) fn mark_analyzed(
        &self,
        connection: &Connection,
        scope: &GitScope,
        file: &str,
        fingerprint: &str,
        expected_revision: i64,
        note: &Option<String>,
    ) -> Result<Value> {
        let version: i32 = connection.pragma_query_value(None, "user_version", |r| r.get(0))?;
        if version < SCHEMA_VERSION {
            return Err(Failure::new(
                "MIGRATION_REQUIRED",
                "请执行 init 升级数据库后再登记分析结果",
            ));
        }
        if fingerprint.len() != 64
            || !fingerprint
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            || expected_revision < 0
            || expected_revision == i64::MAX
        {
            return Err(Failure::new(
                "INVALID_ARGUMENT",
                "fingerprint 必须为小写 SHA-256，expected_revision 必须为有效非负版本号",
            ));
        }
        let locator = self.locator(file)?;
        scope.require(&locator)?;
        self.verify_fingerprint(&locator, fingerprint)?;
        let object = ensure_object(connection, &locator)?;
        let revision: i64 = connection
            .query_row(
                "SELECT revision FROM analyses WHERE object_id=?1",
                [object.id],
                |r| r.get(0),
            )
            .optional()?
            .unwrap_or(0);
        if revision != expected_revision {
            return Err(Failure::new("ANALYSIS_CHANGED", "分析记录已被其他调用更新，请重新取得 coverage 后核对")
                .detail(json!({"locator": locator, "expected_revision": expected_revision, "actual_revision": revision})));
        }
        connection.execute(
            "INSERT INTO analyses (object_id,fingerprint,revision,note) VALUES (?1,?2,?3,?4)
             ON CONFLICT(object_id) DO UPDATE SET fingerprint=excluded.fingerprint,revision=excluded.revision,note=excluded.note",
            params![object.id, fingerprint, revision + 1, note],
        )?;
        Ok(
            json!({"op": "analyzed", "changed": true, "locator": locator, "fingerprint": fingerprint, "revision": revision + 1}),
        )
    }

    /// The Git manifest is authoritative, including files that have never become graph objects.
    /// A read transaction keeps graph counts and analysis receipts in the same DB snapshot.
    pub fn coverage(&self) -> Result<Value> {
        let scope = GitScope::load(&self.root)?;
        let initialized = self.database_path().exists();
        let mut version = None;
        let mut analyses = BTreeMap::<String, (String, i64, Option<String>)>::new();
        let mut counts = BTreeMap::<String, (usize, usize)>::new();
        let mut outside_objects = Vec::new();
        let mut outside_relations = Vec::new();
        let mut relation_count = 0;
        if initialized {
            let mut connection = self.open(false, false)?;
            let transaction = connection.transaction()?;
            check_schema(&transaction)?;
            let schema: i32 = transaction.pragma_query_value(None, "user_version", |r| r.get(0))?;
            version = Some(schema);
            {
                let mut statement =
                    transaction.prepare("SELECT id,kind,locator FROM objects ORDER BY locator")?;
                let objects = statement.query_map([], |r| {
                    Ok(Object {
                        id: r.get(0)?,
                        kind: r.get(1)?,
                        locator: r.get(2)?,
                    })
                })?;
                for object in objects {
                    let object = object?;
                    if !scope.files.contains(&object.locator) {
                        outside_objects.push(object);
                    }
                }
                let mut statement = transaction.prepare(
                    "SELECT a.locator,b.locator FROM dependencies d
                     JOIN objects a ON a.id=d.dependent_id JOIN objects b ON b.id=d.dependency_id
                     ORDER BY a.locator,b.locator",
                )?;
                let relations = statement
                    .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?;
                for relation in relations {
                    let (from, to) = relation?;
                    if scope.files.contains(&from) && scope.files.contains(&to) {
                        counts.entry(from).or_default().0 += 1;
                        counts.entry(to).or_default().1 += 1;
                        relation_count += 1;
                    } else {
                        outside_relations.push(json!({"dependent": from, "dependency": to}));
                    }
                }
                if schema >= 2 {
                    let mut statement = transaction.prepare(
                        "SELECT o.locator,a.fingerprint,a.revision,a.note FROM analyses a JOIN objects o ON o.id=a.object_id"
                    )?;
                    let rows = statement.query_map([], |r| {
                        Ok((r.get::<_, String>(0)?, (r.get(1)?, r.get(2)?, r.get(3)?)))
                    })?;
                    for row in rows {
                        let (locator, receipt) = row?;
                        analyses.insert(locator, receipt);
                    }
                }
            }
            transaction.commit()?;
        }
        let mut files = Vec::new();
        let (mut analyzed, mut unanalyzed, mut stale, mut unavailable, mut unlinked) =
            (0, 0, 0, 0, 0);
        for path in &scope.files {
            let receipt = analyses.get(path);
            let (fingerprint, issue) = match self.fingerprint(path) {
                Ok(hash) => (Some(hash), None),
                Err(error) => (None, Some(error)),
            };
            let status = if issue.is_some() {
                unavailable += 1;
                "unavailable"
            } else if let Some((previous, _, _)) = receipt {
                if Some(previous) == fingerprint.as_ref() {
                    analyzed += 1;
                    "analyzed"
                } else {
                    stale += 1;
                    "stale"
                }
            } else {
                unanalyzed += 1;
                "unanalyzed"
            };
            let (outgoing, incoming) = counts.get(path).copied().unwrap_or_default();
            if outgoing + incoming == 0 {
                unlinked += 1;
            }
            files.push(json!({"path": path, "fingerprint": fingerprint, "analysis_status": status,
                "analysis_revision": receipt.map_or(0, |r| r.1), "analyzed_fingerprint": receipt.map(|r| &r.0),
                "note": receipt.and_then(|r| r.2.as_ref()), "dependencies": outgoing, "dependents": incoming, "issue": issue}));
        }
        scope.verify(&self.root)?;
        let invalid =
            unavailable > 0 || !outside_objects.is_empty() || !outside_relations.is_empty();
        let data = json!({"root": self.root, "repository": scope.repository, "initialized": initialized, "schema_version": version,
            "summary": {"tracked": scope.files.len(), "analyzed": analyzed, "unanalyzed": unanalyzed,
                "stale": stale, "unavailable": unavailable, "remaining": unanalyzed + stale + unavailable,
                "unlinked": unlinked, "relations": relation_count,
                "complete": analyzed == scope.files.len() && !invalid},
            "files": files, "outside_objects": outside_objects, "outside_relations": outside_relations});
        if invalid {
            let mut error = Failure::new(
                "SCOPE_VIOLATION",
                "清单中有不可读取的文件或范围外旧记录，详见 data；未修改数据库",
            );
            error.data = Some(data);
            return Err(error);
        }
        Ok(data)
    }
}

pub fn filter_report(data: &mut Value, filter: Filter) {
    if let Some(files) = data["files"].as_array_mut() {
        files.retain(|file| match filter {
            Filter::All => true,
            Filter::Unlinked => file["dependencies"] == 0 && file["dependents"] == 0,
            Filter::Unanalyzed => file["analysis_status"] == "unanalyzed",
            Filter::Stale => file["analysis_status"] == "stale",
            Filter::Remaining => file["analysis_status"] != "analyzed",
        });
        data["shown"] = json!(files.len());
    }
}

// Escape terminal controls and newlines without losing ordinary Unicode filenames.
pub fn display_text(text: &str) -> String {
    text.chars()
        .flat_map(|c| {
            if c.is_control() {
                c.escape_default().collect::<Vec<_>>()
            } else {
                vec![c]
            }
        })
        .collect()
}

#[derive(Default)]
struct Node<'a> {
    children: BTreeMap<&'a str, Node<'a>>,
    file: Option<&'a Value>,
}

impl Node<'_> {
    fn render(&self, prefix: &str, output: &mut String) {
        // Directories first, then files, both sorted by their actual names.
        let mut children: Vec<_> = self.children.iter().collect();
        children.sort_by_key(|(name, node)| (node.file.is_some(), **name));
        for (index, (name, node)) in children.iter().enumerate() {
            let last = index + 1 == children.len();
            output.push_str(&format!(
                "{prefix}{}{}",
                if last { "└── " } else { "├── " },
                display_text(name)
            ));
            if let Some(file) = node.file {
                let state = match file["analysis_status"].as_str().unwrap_or_default() {
                    "analyzed" => "已分析",
                    "stale" => "内容变化 · 需复核",
                    "unavailable" => "无法读取",
                    _ => "未分析",
                };
                if file["dependencies"] == 0 && file["dependents"] == 0 {
                    output.push_str(&format!("  无关联 · {state}"));
                } else {
                    output.push_str(&format!(
                        "  依赖 {} / 被依赖 {} · {state}",
                        file["dependencies"], file["dependents"]
                    ));
                }
                if let Some(code) = file["issue"]["code"].as_str() {
                    output.push_str(&format!(" ({code})"));
                }
            } else {
                output.push('/');
            }
            output.push('\n');
            node.render(
                &format!("{prefix}{}", if last { "    " } else { "│   " }),
                output,
            );
        }
    }
}

pub fn render_tree(data: &Value) -> String {
    let mut tree = Node::default();
    if let Some(files) = data["files"].as_array() {
        for file in files {
            let mut node = &mut tree;
            for part in file["path"].as_str().unwrap_or_default().split('/') {
                node = node.children.entry(part).or_default();
            }
            node.file = Some(file);
        }
    }
    let mut text = String::from(".\n");
    tree.render("", &mut text);
    if tree.children.is_empty() {
        text.push_str("（没有符合筛选条件的文件）\n");
    }
    let summary = &data["summary"];
    text.push_str(&format!("\n显示 {} / Git 跟踪 {} 个文件 · {} 条范围内关联\n已分析 {} · 未分析 {} · 需复核 {} · 无法读取 {} · 无关联 {}\n",
        data["shown"], summary["tracked"], summary["relations"], summary["analyzed"], summary["unanalyzed"],
        summary["stale"], summary["unavailable"], summary["unlinked"]));
    text.push_str("统计基于全部 Git 跟踪文件；无关联与分析状态独立，已分析不代表发现全部依赖。\n");
    text
}

pub fn render_query(data: &Value) -> String {
    let direction = if data["direction"] == "dependents" {
        "谁依赖它"
    } else {
        "它依赖谁"
    };
    let locator = display_text(data["query"]["locator"].as_str().unwrap_or_default());
    let mut text = format!("{locator} · {direction}\n");
    if let Some(matches) = data["matches"].as_array() {
        for entry in matches {
            let path: Vec<_> = entry["path"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(display_text)
                .collect();
            text.push_str(&format!("  {}\n", path.join(" → ")));
        }
        if matches.is_empty() {
            text.push_str("  无已登记关联\n");
        }
    }
    if let Some(edges) = data["relations"].as_array() {
        for edge in edges {
            text.push_str(&format!(
                "  {} → {}：{}\n",
                display_text(edge["dependent"]["locator"].as_str().unwrap_or_default()),
                display_text(edge["dependency"]["locator"].as_str().unwrap_or_default()),
                display_text(edge["reason"].as_str().unwrap_or("未填写原因"))
            ));
        }
    }
    text
}
