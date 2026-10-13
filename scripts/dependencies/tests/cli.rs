use rusqlite::Connection;
use serde_json::{Value, json};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Barrier};
use tempfile::TempDir;

const BIN: &str = env!("CARGO_BIN_EXE_dependencies");

fn coverage_file(data: &Value, path: &str) -> Value {
    data["files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["path"] == path)
        .unwrap()
        .clone()
}

fn analyzed_operation(f: &Fixture, path: &str) -> Value {
    let file = coverage_file(&f.ok(&["coverage"]), path);
    json!({"op": "analyzed", "file": path, "fingerprint": file["fingerprint"],
        "expected_revision": file["analysis_revision"], "note": "已阅读当前内容"})
}

fn raw(root: &Path, args: &[&str]) -> (i32, String) {
    let result = Command::new(BIN)
        .arg("--root")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(result.stderr.is_empty());
    (
        result.status.code().unwrap(),
        String::from_utf8(result.stdout).unwrap(),
    )
}

#[test]
fn coverage_manifest_is_git_based_read_only_and_includes_unregistered_files() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q"]);
    fs::create_dir_all(dir.path().join("docs")).unwrap();
    fs::write(dir.path().join("docs/中文 空格.md"), "hello").unwrap();
    fs::write(dir.path().join("untracked"), "local").unwrap();
    git(dir.path(), &["add", "docs"]);
    let (status, value) = invoke(dir.path(), &["coverage"], None);
    assert_eq!(status, 0);
    assert_eq!(value["data"]["summary"]["tracked"], 1);
    assert_eq!(value["data"]["summary"]["unanalyzed"], 1);
    assert_eq!(value["data"]["summary"]["unlinked"], 1);
    assert_eq!(value["data"]["initialized"], false);
    assert_eq!(value["data"]["files"][0]["path"], "docs/中文 空格.md");
    assert_eq!(
        value["data"]["files"][0]["fingerprint"],
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
    assert!(!dir.path().join(".dependencies").exists());
    let (status, text) = raw(dir.path(), &["tree"]);
    assert_eq!(status, 0);
    assert!(text.contains("└── docs/\n    └── 中文 空格.md  无关联 · 未分析"));
    assert!(!text.contains("untracked"));
    assert!(!dir.path().join(".dependencies").exists());
}

#[test]
fn analysis_is_independent_of_relations_and_survives_new_processes() {
    let f = Fixture::new();
    f.ok(&["add", "a.md", "b.md"]);
    let before = f.ok(&["coverage"]);
    assert_eq!(before["summary"]["unanalyzed"], 5);
    assert_eq!(
        coverage_file(&before, "b.md")["analysis_status"],
        "unanalyzed"
    );
    let operation = analyzed_operation(&f, "isolated.md");
    let (status, result) = invoke(f.dir.path(), &["batch"], Some(&json!([operation])));
    assert_eq!(status, 0, "{result}");
    let after = f.ok(&["coverage"]);
    assert_eq!(after["summary"]["analyzed"], 1);
    assert_eq!(after["summary"]["unlinked"], 3);
    let isolated = coverage_file(&after, "isolated.md");
    assert_eq!(isolated["analysis_status"], "analyzed");
    assert_eq!(isolated["dependencies"], 0);
    assert_eq!(isolated["dependents"], 0);
    assert_eq!(isolated["note"], "已阅读当前内容");
    assert_eq!(f.ok(&["coverage", "--only", "remaining"])["shown"], 4);
    assert_eq!(f.ok(&["tree", "--json", "--only", "unlinked"])["shown"], 3);
}

#[test]
fn analysis_receipts_reject_changed_content_and_roll_back_relations() {
    let f = Fixture::new();
    let operation = analyzed_operation(&f, "a.md");
    fs::write(f.dir.path().join("a.md"), "changed after reading").unwrap();
    let (status, result) = invoke(
        f.dir.path(),
        &["batch"],
        Some(&json!([
            {"op":"add","dependent":"a.md","dependency":"b.md"}, operation
        ])),
    );
    assert_eq!(status, 2);
    assert_eq!(result["error"]["code"], "CONTENT_CHANGED");
    assert_eq!(result["error"]["details"]["rolled_back"], true);
    assert_eq!(result["error"]["details"]["operation_index"], 1);
    assert_eq!(f.count("objects"), 0);
    assert_eq!(f.count("dependencies"), 0);
    assert_eq!(f.count("analyses"), 0);
    let operation = analyzed_operation(&f, "a.md");
    let (status, result) = invoke(
        f.dir.path(),
        &["batch"],
        Some(&json!([
            operation, {"op":"add","dependent":"a.md","dependency":"missing"}
        ])),
    );
    assert_eq!(status, 2, "{result}");
    assert_eq!(f.count("analyses"), 0);
}

#[test]
fn coverage_detects_changes_missing_files_and_new_git_members() {
    let f = Fixture::new();
    let report = f.ok(&["coverage"]);
    let operations: Vec<_> = report["files"].as_array().unwrap().iter().map(|file|
        json!({"op":"analyzed","file":file["path"],"fingerprint":file["fingerprint"],"expected_revision":0})
    ).collect();
    assert_eq!(
        invoke(f.dir.path(), &["batch"], Some(&json!(operations))).0,
        0
    );
    assert_eq!(f.ok(&["coverage"])["summary"]["complete"], true);
    // Contents, not Git staging status, determine whether the previous analysis still matches.
    fs::write(f.dir.path().join("a.md"), "unstaged changed").unwrap();
    fs::remove_file(f.dir.path().join("b.md")).unwrap();
    f.file("new/file.md");
    let report = f.fail(&["coverage"], "SCOPE_VIOLATION")["data"].clone();
    assert_eq!(report["summary"]["tracked"], 6);
    assert_eq!(report["summary"]["analyzed"], 3);
    assert_eq!(report["summary"]["stale"], 1);
    assert_eq!(report["summary"]["unavailable"], 1);
    assert_eq!(report["summary"]["unanalyzed"], 1);
    assert_eq!(report["summary"]["remaining"], 3);
    assert_eq!(report["summary"]["complete"], false);
    assert_eq!(
        coverage_file(&report, "b.md")["issue"]["code"],
        "FILE_NOT_FOUND"
    );
    assert_eq!(
        f.fail(&["coverage", "--only", "remaining"], "SCOPE_VIOLATION")["data"]["shown"],
        3
    );
    let (status, text) = raw(f.dir.path(), &["tree"]);
    assert_eq!(status, 2);
    assert!(text.contains("b.md  无关联 · 无法读取 (FILE_NOT_FOUND)"));
    assert!(text.contains("错误 SCOPE_VIOLATION"));
}

#[test]
fn schema_one_upgrade_is_explicit_and_preserves_existing_graph() {
    let f = Fixture::new();
    f.ok(&["add", "a.md", "b.md", "--reason", "keep"]);
    let original = f.ok(&["dependencies", "a.md"]);
    let db = f.database();
    db.execute_batch("DROP TABLE analyses; PRAGMA user_version=1;")
        .unwrap();
    assert_eq!(f.ok(&["dependencies", "a.md"]), original);
    let coverage = f.ok(&["coverage"]);
    assert_eq!(coverage["schema_version"], 1);
    assert_eq!(coverage["summary"]["unanalyzed"], 5);
    let hash = coverage_file(&coverage, "a.md")["fingerprint"]
        .as_str()
        .unwrap()
        .to_string();
    f.fail(
        &[
            "analyzed",
            "a.md",
            "--fingerprint",
            &hash,
            "--expected-revision",
            "0",
        ],
        "MIGRATION_REQUIRED",
    );
    assert_eq!(
        db.pragma_query_value(None, "user_version", |r| r.get::<_, i32>(0))
            .unwrap(),
        1
    );
    assert_eq!(f.ok(&["init"])["upgraded"], true);
    assert_eq!(f.ok(&["init"])["upgraded"], false);
    assert_eq!(f.ok(&["dependencies", "a.md"]), original);
    assert_eq!(f.count("analyses"), 0);
    f.ok(&[
        "analyzed",
        "a.md",
        "--fingerprint",
        &hash,
        "--expected-revision",
        "0",
    ]);
    assert_eq!(
        coverage_file(&f.ok(&["coverage"]), "a.md")["analysis_status"],
        "analyzed"
    );
}

#[test]
fn concurrent_analysis_updates_conflict_instead_of_overwriting() {
    let f = Fixture::new();
    let hash = coverage_file(&f.ok(&["coverage"]), "a.md")["fingerprint"]
        .as_str()
        .unwrap()
        .to_string();
    let mut children = Vec::new();
    for _ in 0..4 {
        children.push(
            Command::new(BIN)
                .arg("--root")
                .arg(f.dir.path())
                .args([
                    "analyzed",
                    "a.md",
                    "--fingerprint",
                    &hash,
                    "--expected-revision",
                    "0",
                ])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap(),
        );
    }
    let mut successes = 0;
    for child in children {
        let output = child.wait_with_output().unwrap();
        let result: Value = serde_json::from_slice(&output.stdout).unwrap();
        if output.status.success() {
            successes += 1;
        } else {
            assert_eq!(result["error"]["code"], "ANALYSIS_CHANGED", "{result}");
        }
    }
    assert_eq!(successes, 1);
    assert_eq!(
        coverage_file(&f.ok(&["coverage"]), "a.md")["analysis_revision"],
        1
    );
    let (status, result) = invoke(
        f.dir.path(),
        &["batch"],
        Some(&json!([
            {"op":"add","dependent":"a.md","dependency":"b.md"},
            {"op":"analyzed","file":"a.md","fingerprint":hash,"expected_revision":0}
        ])),
    );
    assert_eq!(status, 2);
    assert_eq!(result["error"]["code"], "ANALYSIS_CHANGED");
    assert_eq!(f.count("dependencies"), 0);
}

#[test]
fn tree_filters_keep_ancestors_and_escape_control_characters() {
    let f = Fixture::new();
    f.file("docs/中文/a\nb.md");
    f.file("docs/中文/linked.md");
    f.ok(&[
        "add",
        "docs/中文/linked.md",
        "a.md",
        "--reason",
        "line\nbreak\u{1b}[31m",
    ]);
    let (status, tree) = raw(f.dir.path(), &["tree", "--only", "unlinked"]);
    assert_eq!(status, 0);
    assert!(
        tree.contains("├── docs/\n│   └── 中文/\n│       └── a\\nb.md  无关联 · 未分析"),
        "{tree}"
    );
    assert!(!tree.contains("linked.md"));
    let json = f.ok(&["tree", "--json", "--only", "unlinked"]);
    assert_eq!(json["summary"]["tracked"], 7);
    assert_eq!(json["shown"], 5);
    let (status, text) = raw(
        f.dir.path(),
        &["dependencies", "docs/中文/linked.md", "--text"],
    );
    assert_eq!(status, 0);
    assert!(text.contains("它依赖谁"));
    assert!(text.contains("line\\nbreak\\u{1b}[31m"), "{text}");
    assert!(!text.contains('\u{1b}'));
    let (status, tree) = raw(f.dir.path(), &["tree", "--only", "stale"]);
    assert_eq!(status, 0);
    assert!(tree.contains("没有符合筛选条件的文件"));
}

#[test]
fn coverage_subdirectories_and_outside_records_are_not_silently_hidden() {
    let f = Fixture::new();
    f.file("sub/local.md");
    let sub = f.dir.path().join("sub");
    let (status, report) = invoke(&sub, &["coverage"], None);
    assert_eq!(status, 0);
    assert_eq!(report["data"]["summary"]["tracked"], 1);
    assert_eq!(report["data"]["files"][0]["path"], "local.md");
    f.ok(&["add", "a.md", "b.md"]);
    let operation = analyzed_operation(&f, "b.md");
    assert_eq!(
        invoke(f.dir.path(), &["batch"], Some(&json!([operation]))).0,
        0
    );
    git(f.dir.path(), &["rm", "--cached", "b.md"]);
    let report = f.fail(&["coverage"], "SCOPE_VIOLATION")["data"].clone();
    assert_eq!(report["outside_objects"].as_array().unwrap().len(), 1);
    assert_eq!(report["outside_relations"].as_array().unwrap().len(), 1);
    assert_eq!(coverage_file(&report, "a.md")["dependencies"], 0);
    assert_eq!(f.count("analyses"), 1);
    f.ok(&["remove", "a.md", "b.md"]);
    f.ok(&["forget", "b.md"]);
    assert_eq!(f.count("analyses"), 0);
    assert_eq!(f.ok(&["coverage"])["outside_objects"], json!([]));
}

fn git(root: &Path, args: &[&str]) {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn invoke(root: &Path, args: &[&str], input: Option<&Value>) -> (i32, Value) {
    let mut child = Command::new(BIN)
        .arg("--root")
        .arg(root)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(value) = input {
        child
            .stdin
            .take()
            .unwrap()
            .write_all(value.to_string().as_bytes())
            .unwrap();
    }
    drop(child.stdin.take());
    let output = child.wait_with_output().unwrap();
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|e| {
        panic!(
            "invalid JSON: {e}; {}",
            String::from_utf8_lossy(&output.stdout)
        )
    });
    (output.status.code().unwrap(), value)
}

struct Fixture {
    dir: TempDir,
}
impl Fixture {
    fn new() -> Self {
        let result = Self {
            dir: tempfile::tempdir().unwrap(),
        };
        git(result.dir.path(), &["init", "-q"]);
        for name in ["a.md", "b.md", "c.py", "d.md", "isolated.md"] {
            result.file(name);
        }
        result.ok(&["init"]);
        result
    }
    fn file(&self, name: &str) {
        let path = self.dir.path().join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, "fixture\n").unwrap();
        git(self.dir.path(), &["add", "--", name]);
    }
    fn ok(&self, args: &[&str]) -> Value {
        let (status, value) = invoke(self.dir.path(), args, None);
        assert_eq!(status, 0, "{value}");
        assert_eq!(value["ok"], true);
        assert!(value["error"].is_null());
        value["data"].clone()
    }
    fn fail(&self, args: &[&str], code: &str) -> Value {
        let (status, value) = invoke(self.dir.path(), args, None);
        assert_eq!(status, 2, "{value}");
        assert_eq!(value["error"]["code"], code, "{value}");
        assert_eq!(value["ok"], false);
        value
    }
    fn database(&self) -> Connection {
        Connection::open(self.dir.path().join(".dependencies/graph.sqlite3")).unwrap()
    }
    fn count(&self, table: &str) -> i64 {
        self.database()
            .query_row(&format!("SELECT count(*) FROM {table}"), [], |r| r.get(0))
            .unwrap()
    }
}

#[test]
fn init_is_idempotent_and_queries_do_not_initialize() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q"]);
    fs::write(dir.path().join("a.md"), "").unwrap();
    git(dir.path(), &["add", "a.md"]);
    let (status, value) = invoke(dir.path(), &["dependents", "a.md"], None);
    assert_eq!(status, 2);
    assert_eq!(value["error"]["code"], "NOT_INITIALIZED");
    assert!(!dir.path().join(".dependencies").exists());
    let f = Fixture::new();
    assert_eq!(f.ok(&["init"])["created"], false);
    assert_eq!(
        f.database()
            .pragma_query_value(None, "user_version", |r| r.get::<_, i32>(0))
            .unwrap(),
        2
    );
}

#[test]
fn direction_direct_transitive_diamond_cycles_and_reasons() {
    let f = Fixture::new();
    for (a, b) in [
        ("a.md", "b.md"),
        ("b.md", "c.py"),
        ("a.md", "d.md"),
        ("d.md", "c.py"),
        ("c.py", "a.md"),
    ] {
        f.ok(&["add", a, b, "--reason", &format!("{a} follows {b}")]);
    }
    let direct = f.ok(&["dependents", "c.py"]);
    assert_eq!(direct["matches"].as_array().unwrap().len(), 2);
    assert_eq!(direct["matches"][0]["object"]["locator"], "b.md");
    let reverse = f.ok(&["dependents", "c.py", "--transitive"]);
    assert_eq!(reverse["matches"].as_array().unwrap().len(), 3);
    assert_eq!(
        reverse["matches"][2]["path"],
        json!(["a.md", "b.md", "c.py"])
    );
    assert_eq!(reverse["relations"].as_array().unwrap().len(), 5);
    assert_eq!(reverse["relations"][0]["reason"], "a.md follows b.md");
    let forward = f.ok(&["dependencies", "a.md", "--transitive"]);
    assert_eq!(
        forward["matches"][2]["path"],
        json!(["a.md", "b.md", "c.py"])
    );
    for result in [&reverse, &forward] {
        for matched in result["matches"].as_array().unwrap() {
            let path = matched["path"].as_array().unwrap();
            for pair in path.windows(2) {
                assert!(
                    result["relations"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|e| e["dependent"]["locator"] == pair[0]
                            && e["dependency"]["locator"] == pair[1])
                );
            }
        }
    }
    assert_eq!(reverse, f.ok(&["dependents", "c.py", "--transitive"]));
}

#[test]
fn duplicates_do_not_overwrite_and_ids_survive_removal() {
    let f = Fixture::new();
    let first = f.ok(&["add", "a.md", "b.md", "--reason", "first"]);
    assert_eq!(
        f.ok(&["add", "./a.md", "b.md", "--reason", "first"])["results"][0]["changed"],
        false
    );
    f.fail(
        &["add", "a.md", "b.md", "--reason", "different"],
        "RELATION_EXISTS",
    );
    assert_eq!(f.count("dependencies"), 1);
    f.ok(&["remove", "a.md", "b.md"]);
    assert_eq!(
        f.ok(&["remove", "a.md", "b.md"])["results"][0]["changed"],
        false
    );
    let again = f.ok(&["add", "a.md", "b.md"]);
    assert_eq!(
        first["results"][0]["relation"]["dependent"]["id"],
        again["results"][0]["relation"]["dependent"]["id"]
    );
}

#[test]
fn update_reason_clear_and_endpoints_are_atomic() {
    let f = Fixture::new();
    f.ok(&["add", "a.md", "b.md", "--reason", "first"]);
    let update = f.ok(&["update", "a.md", "b.md", "--reason", "quote ' \" 中文"]);
    assert_eq!(
        update["results"][0]["relation"]["reason"],
        "quote ' \" 中文"
    );
    f.ok(&[
        "update",
        "a.md",
        "b.md",
        "--new-dependent",
        "d.md",
        "--new-dependency",
        "c.py",
        "--clear-reason",
    ]);
    assert_eq!(f.ok(&["dependents", "b.md"])["matches"], json!([]));
    assert!(f.ok(&["dependents", "c.py"])["relations"][0]["reason"].is_null());
    f.ok(&["add", "a.md", "b.md"]);
    f.fail(
        &[
            "update",
            "a.md",
            "b.md",
            "--new-dependent",
            "d.md",
            "--new-dependency",
            "c.py",
        ],
        "RELATION_EXISTS",
    );
    assert_eq!(f.count("dependencies"), 2);
    f.fail(&["update", "a.md", "b.md"], "INVALID_OPERATION");
    f.fail(
        &["update", "a.md", "c.py", "--clear-reason"],
        "RELATION_NOT_FOUND",
    );
}

#[test]
fn batch_failure_rolls_back_objects_and_relations() {
    let f = Fixture::new();
    f.file("missing.md");
    fs::remove_file(f.dir.path().join("missing.md")).unwrap();
    let input = json!([
        {"op":"add", "dependent":"a.md", "dependency":"b.md"},
        {"op":"add", "dependent":"a.md", "dependency":"missing.md"}
    ]);
    let (status, result) = invoke(f.dir.path(), &["batch"], Some(&input));
    assert_eq!(status, 2);
    assert_eq!(result["error"]["code"], "FILE_NOT_FOUND");
    assert_eq!(result["error"]["details"]["operation_index"], 1);
    assert_eq!(result["error"]["details"]["rolled_back"], true);
    assert!(result["data"].is_null());
    assert_eq!(f.count("objects"), 0);
    assert_eq!(f.count("dependencies"), 0);
}

#[test]
fn batch_mixed_operations_and_strict_input() {
    let f = Fixture::new();
    let input = json!([
        {"op":"add", "dependent":"a.md", "dependency":"b.md"},
        {"op":"update", "dependent":"a.md", "dependency":"b.md", "reason":"review"},
        {"op":"add", "dependent":"b.md", "dependency":"c.py"},
        {"op":"remove", "dependent":"b.md", "dependency":"c.py"}
    ]);
    let (status, _) = invoke(f.dir.path(), &["batch", "-"], Some(&input));
    assert_eq!(status, 0);
    assert_eq!(f.count("dependencies"), 1);
    assert_eq!(
        f.ok(&["dependents", "b.md"])["relations"][0]["reason"],
        "review"
    );
    for invalid in [
        json!({"op":"add"}),
        json!([{"op":"add","dependent":"a.md","dependency":"b.md","typo":1}]),
        json!([{"op":"unknown"}]),
    ] {
        let (status, value) = invoke(f.dir.path(), &["batch"], Some(&invalid));
        assert_eq!(status, 2);
        assert_eq!(value["error"]["code"], "INVALID_BATCH");
    }
    assert_eq!(f.count("dependencies"), 1);
}

#[test]
fn missing_query_and_result_files_return_relations_with_error() {
    let f = Fixture::new();
    f.ok(&["add", "a.md", "b.md"]);
    fs::remove_file(f.dir.path().join("b.md")).unwrap();
    let result = f.fail(&["dependents", "b.md"], "FILE_NOT_FOUND");
    assert_eq!(result["data"]["matches"][0]["object"]["locator"], "a.md");
    assert_eq!(
        result["error"]["details"]["issues"][0]["details"]["locator"],
        "b.md"
    );
    f.fail(&["dependencies", "a.md"], "FILE_NOT_FOUND");
    assert_eq!(f.count("dependencies"), 1);
    f.ok(&["update", "a.md", "b.md", "--new-dependency", "c.py"]);
    fs::remove_file(f.dir.path().join("a.md")).unwrap();
    f.ok(&["remove", "a.md", "c.py"]);
    assert_eq!(f.count("dependencies"), 0);
}

#[test]
fn failed_batch_restores_updates_and_deletions() {
    let f = Fixture::new();
    f.file("missing.md");
    fs::remove_file(f.dir.path().join("missing.md")).unwrap();
    f.ok(&["add", "a.md", "b.md", "--reason", "original"]);
    f.ok(&["add", "b.md", "c.py"]);
    let before = f.ok(&["dependencies", "a.md", "--transitive"]);
    let input = json!([
        {"op":"update", "dependent":"a.md", "dependency":"b.md", "reason":"changed"},
        {"op":"remove", "dependent":"b.md", "dependency":"c.py"},
        {"op":"add", "dependent":"a.md", "dependency":"missing.md"}
    ]);
    let (status, value) = invoke(f.dir.path(), &["batch"], Some(&input));
    assert_eq!(status, 2);
    assert_eq!(value["error"]["details"]["operation_index"], 2);
    assert_eq!(before, f.ok(&["dependencies", "a.md", "--transitive"]));
}

#[test]
fn batch_file_and_invalid_json_use_the_same_contract() {
    let f = Fixture::new();
    let input = f.dir.path().join("operations.json");
    fs::write(
        &input,
        r#"[{"op":"add","dependent":"a.md","dependency":"b.md"}]"#,
    )
    .unwrap();
    f.ok(&["batch", input.to_str().unwrap()]);
    fs::write(&input, "not JSON").unwrap();
    f.fail(&["batch", input.to_str().unwrap()], "INVALID_BATCH");
    fs::remove_file(&input).unwrap();
    f.fail(&["batch", input.to_str().unwrap()], "IO_ERROR");
    assert_eq!(f.count("dependencies"), 1);
}

#[test]
fn paths_are_normalized_and_workspaces_are_independent() {
    let f = Fixture::new();
    f.file("文档/有 空格.md");
    let absolute = f.dir.path().join("b.md").to_string_lossy().into_owned();
    f.ok(&["add", "文档/../a.md", &absolute]);
    assert_eq!(
        f.ok(&["dependents", "./b.md"])["matches"][0]["object"]["locator"],
        "a.md"
    );
    f.ok(&["add", "文档/有 空格.md", "b.md"]);
    f.fail(&["add", "a.md", "文档"], "FILE_NOT_TRACKED");
    f.fail(&["add", "a.md", "../escape.md"], "OUTSIDE_ROOT");
    f.fail(&["add", "a.md", ""], "INVALID_PATH");
    let other = Fixture::new();
    assert_eq!(other.ok(&["dependents", "b.md"])["matches"], json!([]));
}

#[test]
fn isolated_unregistered_and_self_cycles() {
    let f = Fixture::new();
    f.file("missing.md");
    fs::remove_file(f.dir.path().join("missing.md")).unwrap();
    assert!(f.ok(&["dependencies", "isolated.md"])["query"]["id"].is_null());
    assert_eq!(
        f.fail(&["dependents", "missing.md"], "FILE_NOT_FOUND")["data"]["matches"],
        json!([])
    );
    f.ok(&["add", "a.md", "a.md"]);
    let result = f.ok(&["dependents", "a.md", "--transitive"]);
    assert_eq!(result["matches"], json!([]));
    assert_eq!(result["relations"].as_array().unwrap().len(), 1);
}

fn parallel(root: &Path, commands: Vec<Vec<String>>) -> Vec<(i32, Value)> {
    let barrier = Arc::new(Barrier::new(commands.len()));
    let handles: Vec<_> = commands
        .into_iter()
        .map(|args| {
            let root = root.to_path_buf();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                invoke(
                    &root,
                    &args.iter().map(String::as_str).collect::<Vec<_>>(),
                    None,
                )
            })
        })
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

#[test]
fn concurrent_distinct_writes_are_not_lost() {
    let f = Fixture::new();
    let commands = (0..12)
        .map(|i| {
            let name = format!("{i}.md");
            f.file(&name);
            vec!["add".into(), name, "b.md".into()]
        })
        .collect();
    for (status, value) in parallel(f.dir.path(), commands) {
        assert_eq!(status, 0, "{value}");
    }
    assert_eq!(f.count("dependencies"), 12);
    assert_eq!(
        f.ok(&["dependents", "b.md"])["matches"]
            .as_array()
            .unwrap()
            .len(),
        12
    );
}

#[test]
fn concurrent_duplicates_and_initialization() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q"]);
    for (status, value) in parallel(dir.path(), vec![vec!["init".into()]; 8]) {
        assert_eq!(status, 0, "{value}");
    }
    let f = Fixture::new();
    let commands = vec![vec!["add".into(), "a.md".into(), "b.md".into()]; 8];
    for (status, value) in parallel(f.dir.path(), commands) {
        assert_eq!(status, 0, "{value}");
    }
    assert_eq!(f.count("dependencies"), 1);
    assert_eq!(f.count("objects"), 2);
}

#[test]
fn readers_do_not_see_uncommitted_changes_and_lock_failure_is_stable() {
    let f = Fixture::new();
    f.ok(&["add", "a.md", "b.md", "--reason", "committed"]);
    let db = f.database();
    db.execute_batch("BEGIN IMMEDIATE; UPDATE dependencies SET reason='uncommitted';")
        .unwrap();
    assert_eq!(
        f.ok(&["dependents", "b.md"])["relations"][0]["reason"],
        "committed"
    );
    f.fail(&["add", "c.py", "b.md"], "DATABASE_BUSY");
    db.execute_batch("ROLLBACK").unwrap();
    assert_eq!(f.count("dependencies"), 1);
}

#[test]
fn unsupported_schema_and_unrelated_database_are_not_overwritten() {
    let f = Fixture::new();
    f.database()
        .pragma_update(None, "user_version", 99)
        .unwrap();
    f.fail(&["dependents", "a.md"], "SCHEMA_MISMATCH");
    f.fail(&["init"], "SCHEMA_MISMATCH");
    assert_eq!(
        f.database()
            .pragma_query_value(None, "user_version", |r| r.get::<_, i32>(0))
            .unwrap(),
        99
    );
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q"]);
    fs::create_dir(dir.path().join(".dependencies")).unwrap();
    let db = Connection::open(dir.path().join(".dependencies/graph.sqlite3")).unwrap();
    db.execute_batch("CREATE TABLE unrelated(value TEXT); INSERT INTO unrelated VALUES ('keep');")
        .unwrap();
    let (status, value) = invoke(dir.path(), &["init"], None);
    assert_eq!(status, 2);
    assert_eq!(value["error"]["code"], "SCHEMA_MISMATCH");
    assert_eq!(
        db.query_row("SELECT value FROM unrelated", [], |r| r.get::<_, String>(0))
            .unwrap(),
        "keep"
    );
}

#[test]
fn malformed_cli_and_database_report_json_errors() {
    let f = Fixture::new();
    f.fail(&["add"], "INVALID_ARGUMENT");
    f.fail(&["unknown"], "INVALID_ARGUMENT");
    f.fail(
        &["update", "a.md", "b.md", "--reason", "x", "--clear-reason"],
        "INVALID_ARGUMENT",
    );
    fs::write(
        f.dir.path().join(".dependencies/graph.sqlite3"),
        b"not a sqlite database",
    )
    .unwrap();
    f.fail(&["dependents", "a.md"], "DATABASE_CORRUPT");
}

#[cfg(unix)]
#[test]
fn symlink_aliases_are_distinct_but_escapes_are_rejected() {
    use std::os::unix::fs::symlink;
    let f = Fixture::new();
    symlink("a.md", f.dir.path().join("alias.md")).unwrap();
    git(f.dir.path(), &["add", "alias.md"]);
    f.ok(&["add", "alias.md", "b.md"]);
    assert_eq!(
        f.ok(&["dependents", "b.md"])["matches"][0]["object"]["locator"],
        "alias.md"
    );
    let outside = tempfile::NamedTempFile::new().unwrap();
    symlink(outside.path(), f.dir.path().join("outside.md")).unwrap();
    git(f.dir.path(), &["add", "outside.md"]);
    f.fail(&["add", "outside.md", "b.md"], "OUTSIDE_ROOT");
    fs::remove_file(f.dir.path().join("alias.md")).unwrap();
    symlink(outside.path(), f.dir.path().join("alias.md")).unwrap();
    assert_eq!(
        f.fail(&["dependents", "b.md"], "INVALID_OBJECTS")["data"]["matches"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn tracked_modified_staged_new_and_ignore_rules() {
    let f = Fixture::new();
    fs::write(f.dir.path().join("a.md"), "uncommitted change").unwrap();
    fs::write(f.dir.path().join(".gitignore"), "a.md\nignored.md\n").unwrap();
    fs::write(f.dir.path().join("ignored.md"), "local reference").unwrap();
    fs::write(f.dir.path().join("new.md"), "new").unwrap();
    f.ok(&["add", "a.md", "b.md"]); // Ignored but already indexed remains managed.
    f.fail(&["add", "ignored.md", "b.md"], "FILE_NOT_TRACKED");
    f.fail(&["add", "a.md", "new.md"], "FILE_NOT_TRACKED");
    git(f.dir.path(), &["add", "new.md"]);
    f.ok(&["add", "a.md", "new.md"]);
    let scope = f.ok(&["scope"]);
    assert!(
        scope["tracked_files"]
            .as_array()
            .unwrap()
            .contains(&json!("new.md"))
    );
    assert!(
        !scope["tracked_files"]
            .as_array()
            .unwrap()
            .contains(&json!("ignored.md"))
    );
}

#[test]
fn scope_is_read_only_and_non_git_roots_are_rejected() {
    let directory = tempfile::tempdir().unwrap();
    let (status, value) = invoke(directory.path(), &["init"], None);
    assert_eq!(status, 2);
    assert_eq!(value["error"]["code"], "NOT_GIT_WORKTREE");
    assert!(!directory.path().join(".dependencies").exists());
    git(directory.path(), &["init", "-q"]);
    fs::write(directory.path().join("a.md"), "content").unwrap();
    git(directory.path(), &["add", "a.md"]);
    let (status, value) = invoke(directory.path(), &["scope"], None);
    assert_eq!(status, 0);
    assert_eq!(value["data"]["initialized"], false);
    assert_eq!(value["data"]["tracked_files"], json!(["a.md"]));
    assert!(!directory.path().join(".dependencies").exists());
}

#[test]
fn subdirectory_scope_uses_local_paths_and_database() {
    let f = Fixture::new();
    f.file("sub/a.md");
    f.file("sub/b.md");
    let sub = f.dir.path().join("sub");
    assert_eq!(invoke(&sub, &["init"], None).0, 0);
    assert_eq!(invoke(&sub, &["add", "a.md", "b.md"], None).0, 0);
    let (_, data) = invoke(&sub, &["scope"], None);
    assert_eq!(data["data"]["tracked_files"], json!(["a.md", "b.md"]));
    assert_eq!(f.count("dependencies"), 0);
    assert!(sub.join(".dependencies/graph.sqlite3").exists());
    assert_eq!(
        invoke(&sub, &["add", "a.md", "../b.md"], None).1["error"]["code"],
        "OUTSIDE_ROOT"
    );
}

#[test]
fn outside_nodes_do_not_bridge_transitive_queries_and_cleanup_is_explicit() {
    let f = Fixture::new();
    f.ok(&["add", "a.md", "b.md"]);
    f.ok(&["add", "b.md", "c.py"]);
    git(f.dir.path(), &["rm", "--cached", "b.md"]);
    let query = f.fail(&["dependencies", "a.md", "--transitive"], "SCOPE_VIOLATION");
    assert_eq!(query["data"]["matches"], json!([]));
    assert_eq!(query["data"]["relations"], json!([]));
    assert_eq!(
        query["error"]["details"]["excluded_relations"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    f.fail(&["dependents", "b.md"], "FILE_NOT_TRACKED");
    let scope = f.fail(&["scope"], "SCOPE_VIOLATION");
    assert_eq!(
        scope["data"]["outside_relations"].as_array().unwrap().len(),
        2
    );
    assert_eq!(f.count("dependencies"), 2);
    f.fail(&["forget", "b.md"], "OBJECT_IN_USE");
    f.fail(&["forget", "a.md"], "FILE_STILL_TRACKED");
    let batch = json!([
        {"op":"remove", "dependent":"a.md", "dependency":"b.md"},
        {"op":"remove", "dependent":"b.md", "dependency":"c.py"},
        {"op":"forget", "file":"b.md"}
    ]);
    assert_eq!(invoke(f.dir.path(), &["batch"], Some(&batch)).0, 0);
    assert_eq!(f.count("objects"), 2);
    assert_eq!(f.count("dependencies"), 0);
    assert_eq!(f.ok(&["scope"])["outside_objects"], json!([]));
}

#[test]
fn update_checks_final_endpoints_and_can_repair_old_scope() {
    let f = Fixture::new();
    f.ok(&["add", "a.md", "b.md"]);
    git(f.dir.path(), &["rm", "--cached", "b.md"]);
    f.fail(
        &["update", "a.md", "b.md", "--reason", "new"],
        "FILE_NOT_TRACKED",
    );
    f.ok(&["update", "a.md", "b.md", "--new-dependency", "c.py"]);
    f.ok(&["forget", "b.md"]);
    assert_eq!(
        f.ok(&["dependencies", "a.md"])["matches"][0]["object"]["locator"],
        "c.py"
    );
}

#[test]
fn scope_failure_rolls_back_batch_and_remove_checks_preview_reason() {
    let f = Fixture::new();
    fs::write(f.dir.path().join("untracked.md"), "new").unwrap();
    let batch = json!([
        {"op":"add", "dependent":"a.md", "dependency":"b.md"},
        {"op":"add", "dependent":"a.md", "dependency":"untracked.md"}
    ]);
    let (status, value) = invoke(f.dir.path(), &["batch"], Some(&batch));
    assert_eq!(status, 2);
    assert_eq!(value["error"]["code"], "FILE_NOT_TRACKED");
    assert_eq!(value["error"]["details"]["rolled_back"], true);
    assert_eq!(f.count("objects"), 0);
    f.ok(&["add", "a.md", "b.md", "--reason", "current"]);
    f.fail(
        &["remove", "a.md", "b.md", "--expected-reason", "old"],
        "RELATION_CHANGED",
    );
    assert_eq!(f.count("dependencies"), 1);
    f.ok(&["remove", "a.md", "b.md", "--expected-reason", "current"]);
}

#[test]
fn missing_git_is_an_error_and_remove_can_clean_non_git_legacy_data() {
    let f = Fixture::new();
    f.ok(&["add", "a.md", "b.md"]);
    let output = Command::new(BIN)
        .arg("--root")
        .arg(f.dir.path())
        .arg("scope")
        .env("PATH", "")
        .output()
        .unwrap();
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["error"]["code"], "GIT_UNAVAILABLE");
    fs::remove_dir_all(f.dir.path().join(".git")).unwrap();
    f.fail(&["dependencies", "a.md"], "NOT_GIT_WORKTREE");
    f.ok(&["remove", "a.md", "b.md"]);
    assert_eq!(f.count("dependencies"), 0);
}

#[test]
fn index_paths_preserve_spaces_unicode_and_newlines() {
    let f = Fixture::new();
    let name = "目录/带 空格\n换行.md";
    f.file(name);
    f.ok(&["add", name, "a.md"]);
    assert_eq!(
        f.ok(&["dependents", "a.md"])["matches"][0]["object"]["locator"],
        name
    );
}

#[test]
fn linked_worktrees_are_supported_and_bare_repositories_rejected() {
    let f = Fixture::new();
    git(
        f.dir.path(),
        &[
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
            "commit",
            "-qm",
            "fixture",
        ],
    );
    let linked = tempfile::tempdir().unwrap();
    git(
        f.dir.path(),
        &[
            "worktree",
            "add",
            "--detach",
            linked.path().to_str().unwrap(),
        ],
    );
    assert!(linked.path().join(".git").is_file());
    assert_eq!(invoke(linked.path(), &["init"], None).0, 0);
    assert_eq!(
        invoke(linked.path(), &["scope"], None).1["data"]["tracked_files"]
            .as_array()
            .unwrap()
            .len(),
        5
    );
    let bare = tempfile::tempdir().unwrap();
    git(bare.path(), &["init", "--bare", "-q"]);
    assert_eq!(
        invoke(bare.path(), &["init"], None).1["error"]["code"],
        "NOT_GIT_WORKTREE"
    );
}
