use rusqlite::{Connection, OpenFlags, OptionalExtension, TransactionBehavior, params};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::time::Duration;

pub mod coverage;

const APPLICATION_ID: i32 = 0x44455053;
pub const SCHEMA_VERSION: i32 = 2;

#[derive(Debug, Serialize)]
pub struct Failure {
    pub code: &'static str,
    pub message: String,
    pub details: Value,
    #[serde(skip)]
    pub data: Option<Value>,
}

impl Failure {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: json!({}),
            data: None,
        }
    }

    fn detail(mut self, details: Value) -> Self {
        self.details = details;
        self
    }
}

impl From<rusqlite::Error> for Failure {
    fn from(error: rusqlite::Error) -> Self {
        use rusqlite::ErrorCode;
        let code = match error.sqlite_error_code() {
            Some(ErrorCode::DatabaseBusy | ErrorCode::DatabaseLocked) => "DATABASE_BUSY",
            Some(ErrorCode::DatabaseCorrupt | ErrorCode::NotADatabase) => "DATABASE_CORRUPT",
            _ => "DATABASE_ERROR",
        };
        Self::new(code, error.to_string())
    }
}

type Result<T> = std::result::Result<T, Failure>;

#[derive(Clone, Debug, Serialize)]
pub struct Object {
    pub id: i64,
    pub kind: String,
    pub locator: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Relation {
    pub dependent: Object,
    pub dependency: Object,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Operation {
    Analyzed {
        file: String,
        fingerprint: String,
        expected_revision: i64,
        note: Option<String>,
    },
    Add {
        dependent: String,
        dependency: String,
        reason: Option<String>,
    },
    Remove {
        dependent: String,
        dependency: String,
        expected_reason: Option<String>,
    },
    Forget {
        file: String,
    },
    Update {
        dependent: String,
        dependency: String,
        new_dependent: Option<String>,
        new_dependency: Option<String>,
        reason: Option<String>,
        #[serde(default)]
        clear_reason: bool,
    },
}

#[derive(Clone, Copy)]
pub enum Direction {
    Dependencies,
    Dependents,
}

pub struct Workspace {
    pub root: PathBuf,
    input_root: PathBuf,
}

#[derive(Debug, PartialEq, Eq)]
struct GitScope {
    repository: PathBuf,
    files: BTreeSet<String>,
}

impl GitScope {
    fn git(root: &Path, args: &[&str]) -> Result<Vec<u8>> {
        let output = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .env("GIT_OPTIONAL_LOCKS", "0")
            .env("LC_ALL", "C")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env_remove("GIT_INDEX_FILE")
            .output()
            .map_err(|e| Failure::new("GIT_UNAVAILABLE", e.to_string()))?;
        if !output.status.success() {
            let message = String::from_utf8_lossy(&output.stderr).into_owned();
            let code = if message.contains("not a git repository") {
                "NOT_GIT_WORKTREE"
            } else {
                "GIT_ERROR"
            };
            return Err(Failure::new(code, message));
        }
        Ok(output.stdout)
    }

    fn load(root: &Path) -> Result<Self> {
        if Self::git(root, &["rev-parse", "--is-inside-work-tree"])? != b"true\n" {
            return Err(Failure::new(
                "NOT_GIT_WORKTREE",
                "工作目录必须位于 Git 工作树内",
            ));
        }
        let bytes = Self::git(root, &["rev-parse", "--show-toplevel"])?;
        let text = std::str::from_utf8(&bytes)
            .map_err(|_| Failure::new("INVALID_PATH", "Git 根路径必须为 UTF-8"))?;
        let repository = fs::canonicalize(text.strip_suffix('\n').unwrap_or(text))
            .map_err(|e| io_error(e, root))?;
        let output = Self::git(
            &repository,
            &["ls-files", "--cached", "--stage", "--full-name", "-z"],
        )?;
        let mut files = BTreeSet::new();
        for entry in output.split(|b| *b == 0).filter(|e| !e.is_empty()) {
            let split = entry
                .iter()
                .position(|b| *b == b'\t')
                .ok_or_else(|| Failure::new("GIT_ERROR", "无法解析 Git 索引条目"))?;
            // gitlinks are submodules, not files managed by the parent workspace.
            if entry.starts_with(b"160000 ") {
                continue;
            }
            let path = std::str::from_utf8(&entry[split + 1..])
                .map_err(|_| Failure::new("INVALID_PATH", "Git 文件路径必须为 UTF-8"))?;
            if let Ok(relative) = repository.join(path).strip_prefix(root)
                && !relative.as_os_str().is_empty()
            {
                files.insert(
                    relative
                        .to_string_lossy()
                        .replace(std::path::MAIN_SEPARATOR, "/"),
                );
            }
        }
        Ok(Self { repository, files })
    }

    fn require(&self, locator: &str) -> Result<()> {
        if !self.files.contains(locator) {
            return Err(
                Failure::new("FILE_NOT_TRACKED", "文件不在当前工作目录的 Git 跟踪范围内")
                    .detail(json!({"locator": locator})),
            );
        }
        Ok(())
    }

    fn verify(&self, root: &Path) -> Result<()> {
        if Self::load(root)? != *self {
            return Err(Failure::new(
                "INDEX_CHANGED",
                "执行期间 Git 跟踪范围发生变化，请重新查询后重试",
            ));
        }
        Ok(())
    }
}

fn io_error(error: std::io::Error, path: &Path) -> Failure {
    Failure::new("IO_ERROR", error.to_string()).detail(json!({"path": path}))
}

fn lexical(path: &Path) -> Result<PathBuf> {
    let mut output = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !output.pop() {
                    return Err(Failure::new("OUTSIDE_ROOT", "路径超出了工作目录边界"));
                }
            }
            other => output.push(other.as_os_str()),
        }
    }
    Ok(output)
}

impl Workspace {
    pub fn new(root: &Path) -> Result<Self> {
        let cwd = std::env::current_dir().map_err(|e| io_error(e, root))?;
        let input_root = lexical(&cwd.join(root))?;
        let canonical = fs::canonicalize(&input_root).map_err(|e| io_error(e, root))?;
        if !canonical.is_dir() {
            return Err(Failure::new("INVALID_ROOT", "工作目录必须是已存在的目录"));
        }
        Ok(Self {
            root: canonical,
            input_root,
        })
    }

    /// Keep logical paths stable after deletion; symlink aliases remain distinct objects.
    pub fn locator(&self, input: &str) -> Result<String> {
        if input.is_empty() || input.contains('\0') {
            return Err(Failure::new("INVALID_PATH", "文件路径不能为空或包含 NUL"));
        }
        let path = Path::new(input);
        let absolute = if path.is_absolute() {
            lexical(path)?
        } else {
            lexical(&self.root.join(path))?
        };
        let relative = absolute
            .strip_prefix(&self.root)
            .or_else(|_| absolute.strip_prefix(&self.input_root))
            .map_err(|_| Failure::new("OUTSIDE_ROOT", "文件必须位于当前工作目录内"))?;
        if relative.as_os_str().is_empty() {
            return Err(Failure::new("NOT_A_FILE", "工作目录本身不是文件对象"));
        }
        relative
            .to_str()
            .map(|s| s.replace(std::path::MAIN_SEPARATOR, "/"))
            .ok_or_else(|| Failure::new("INVALID_PATH", "路径必须可表示为 UTF-8"))
    }

    fn require_file(&self, locator: &str) -> Result<()> {
        let path = self.root.join(locator);
        let resolved = fs::canonicalize(&path).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                Failure::new("FILE_NOT_FOUND", "文件不存在").detail(json!({"locator": locator}))
            } else {
                io_error(error, &path)
            }
        })?;
        if !resolved.starts_with(&self.root) {
            return Err(Failure::new("OUTSIDE_ROOT", "符号链接目标位于工作目录外")
                .detail(json!({"locator": locator})));
        }
        let metadata = fs::metadata(&resolved).map_err(|e| io_error(e, &resolved))?;
        if !metadata.is_file() {
            return Err(Failure::new("NOT_A_FILE", "首版只接受普通文件对象")
                .detail(json!({"locator": locator})));
        }
        Ok(())
    }

    pub fn database_path(&self) -> PathBuf {
        self.root.join(".dependencies/graph.sqlite3")
    }

    fn open(&self, create: bool, writable: bool) -> Result<Connection> {
        let database = self.database_path();
        let directory = database.parent().expect("database has parent");
        for path in [directory, database.as_path()] {
            match fs::symlink_metadata(path) {
                Ok(m) if m.file_type().is_symlink() => {
                    return Err(Failure::new(
                        "INVALID_DATABASE_PATH",
                        "数据库目录和文件不能是符号链接",
                    ));
                }
                Err(e) if e.kind() != std::io::ErrorKind::NotFound => {
                    return Err(io_error(e, path));
                }
                _ => {}
            }
        }
        if create {
            fs::create_dir_all(directory).map_err(|e| io_error(e, directory))?;
        } else if !database.exists() {
            return Err(Failure::new("NOT_INITIALIZED", "请先在该工作目录执行 init"));
        }
        let mut flags = if writable {
            OpenFlags::SQLITE_OPEN_READ_WRITE
        } else {
            OpenFlags::SQLITE_OPEN_READ_ONLY
        };
        if create {
            flags |= OpenFlags::SQLITE_OPEN_CREATE;
        }
        let connection = Connection::open_with_flags(&database, flags)?;
        connection.busy_timeout(Duration::from_secs(5))?;
        connection.pragma_update(None, "foreign_keys", true)?;
        Ok(connection)
    }

    pub fn init(&self) -> Result<Value> {
        let scope = GitScope::load(&self.root)?;
        let mut connection = self.open(true, true)?;
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let version: i32 = transaction.pragma_query_value(None, "user_version", |r| r.get(0))?;
        let application: i32 =
            transaction.pragma_query_value(None, "application_id", |r| r.get(0))?;
        let created = application == 0 && version == 0;
        if created {
            let count: i64 = transaction.query_row(
                "SELECT count(*) FROM sqlite_master WHERE name NOT LIKE 'sqlite_%'",
                [],
                |r| r.get(0),
            )?;
            if count != 0 {
                return Err(Failure::new(
                    "SCHEMA_MISMATCH",
                    "已有数据库不是依赖工具数据库",
                ));
            }
            transaction.execute_batch(
                "CREATE TABLE objects (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    kind TEXT NOT NULL,
                    locator TEXT NOT NULL,
                    UNIQUE(kind, locator)
                );
                CREATE TABLE dependencies (
                    dependent_id INTEGER NOT NULL REFERENCES objects(id),
                    dependency_id INTEGER NOT NULL REFERENCES objects(id),
                    reason TEXT,
                    PRIMARY KEY(dependent_id, dependency_id)
                );
                CREATE INDEX dependencies_reverse ON dependencies(dependency_id, dependent_id);",
            )?;
            transaction.pragma_update(None, "application_id", APPLICATION_ID)?;
            transaction.pragma_update(None, "user_version", 1)?;
        } else {
            check_schema(&transaction)?;
        }
        let upgraded = !created && version == 1;
        if created || upgraded {
            transaction.execute_batch(
                "CREATE TABLE analyses (
                    object_id INTEGER PRIMARY KEY REFERENCES objects(id) ON DELETE CASCADE,
                    fingerprint TEXT NOT NULL,
                    revision INTEGER NOT NULL,
                    note TEXT
                );",
            )?;
            transaction.pragma_update(None, "user_version", SCHEMA_VERSION)?;
        }
        self.commit_write(transaction, Some(&scope))?;
        Ok(
            json!({"created": created, "upgraded": upgraded, "root": self.root, "database": self.database_path(), "schema_version": SCHEMA_VERSION}),
        )
    }

    pub fn apply(&self, operations: &[Operation]) -> Result<Value> {
        let scope = if operations
            .iter()
            .any(|op| !matches!(op, Operation::Remove { .. }))
        {
            Some(GitScope::load(&self.root)?)
        } else {
            None
        };
        let mut connection = self.open(false, true)?;
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        check_schema(&transaction)?;
        let mut results = Vec::new();
        for (index, operation) in operations.iter().enumerate() {
            match self.apply_one(&transaction, operation, scope.as_ref()) {
                Ok(result) => results.push(result),
                Err(mut error) => {
                    error.details["operation_index"] = json!(index);
                    error.details["rolled_back"] = json!(true);
                    return Err(error);
                }
            }
        }
        // Recheck submitted content just before commit, including early operations in a batch.
        for (index, operation) in operations.iter().enumerate() {
            if let Operation::Analyzed {
                file, fingerprint, ..
            } = operation
            {
                self.verify_fingerprint(&self.locator(file)?, fingerprint)
                    .map_err(|mut error| {
                        error.details["operation_index"] = json!(index);
                        error.details["rolled_back"] = json!(true);
                        error
                    })?;
            }
        }
        self.commit_write(transaction, scope.as_ref())?;
        Ok(json!({"results": results}))
    }

    fn commit_write(
        &self,
        transaction: rusqlite::Transaction<'_>,
        scope: Option<&GitScope>,
    ) -> Result<()> {
        if let Some(scope) = scope {
            scope.verify(&self.root).map_err(|mut error| {
                error.details["rolled_back"] = json!(true);
                error
            })?;
        }
        transaction.commit()?;
        Ok(())
    }

    fn apply_one(
        &self,
        connection: &Connection,
        operation: &Operation,
        scope: Option<&GitScope>,
    ) -> Result<Value> {
        if let Operation::Analyzed {
            file,
            fingerprint,
            expected_revision,
            note,
        } = operation
        {
            return self.mark_analyzed(
                connection,
                scope.expect("analysis has Git scope"),
                file,
                fingerprint,
                *expected_revision,
                note,
            );
        }
        if let Operation::Forget { file } = operation {
            let locator = self.locator(file)?;
            let scope = scope.expect("forget has Git scope");
            if scope.files.contains(&locator) {
                return Err(Failure::new(
                    "FILE_STILL_TRACKED",
                    "仍被 Git 跟踪的对象不能通过 forget 清理",
                ));
            }
            let Some(object) = find_object(connection, &locator)? else {
                return Ok(json!({"op": "forget", "locator": locator, "changed": false}));
            };
            let count: i64 = connection.query_row(
                "SELECT count(*) FROM dependencies WHERE dependent_id=?1 OR dependency_id=?1",
                [object.id],
                |r| r.get(0),
            )?;
            if count > 0 {
                return Err(Failure::new(
                    "OBJECT_IN_USE",
                    "对象仍有关联，请先显式删除关联",
                ));
            }
            connection.execute("DELETE FROM objects WHERE id=?1", [object.id])?;
            return Ok(json!({"op": "forget", "locator": locator, "changed": true}));
        }
        let (dependent, dependency) = match operation {
            Operation::Add {
                dependent,
                dependency,
                ..
            }
            | Operation::Remove {
                dependent,
                dependency,
                ..
            }
            | Operation::Update {
                dependent,
                dependency,
                ..
            } => (self.locator(dependent)?, self.locator(dependency)?),
            Operation::Forget { .. } | Operation::Analyzed { .. } => unreachable!(),
        };
        match operation {
            Operation::Add { reason, .. } => {
                let scope = scope.expect("add has Git scope");
                scope.require(&dependent)?;
                scope.require(&dependency)?;
                self.require_file(&dependent)?;
                self.require_file(&dependency)?;
                let from = ensure_object(connection, &dependent)?;
                let to = ensure_object(connection, &dependency)?;
                if let Some(old_reason) = relation_reason(connection, from.id, to.id)? {
                    if &old_reason != reason {
                        return Err(Failure::new(
                            "RELATION_EXISTS",
                            "关系已存在且原因不同，请使用 update",
                        ));
                    }
                    return Ok(json!({"op": "add", "changed": false,
                        "relation": Relation { dependent: from, dependency: to, reason: reason.clone() }}));
                }
                connection.execute(
                    "INSERT INTO dependencies VALUES (?1, ?2, ?3)",
                    params![from.id, to.id, reason],
                )?;
                Ok(json!({"op": "add", "changed": true,
                    "relation": Relation { dependent: from, dependency: to, reason: reason.clone() }}))
            }
            Operation::Remove {
                expected_reason, ..
            } => {
                let from = find_object(connection, &dependent)?;
                let to = find_object(connection, &dependency)?;
                let count = if let (Some(from), Some(to)) = (from, to) {
                    if let Some(expected) = expected_reason
                        && relation_reason(connection, from.id, to.id)?
                            .as_ref()
                            .and_then(Option::as_ref)
                            != Some(expected)
                    {
                        return Err(Failure::new(
                            "RELATION_CHANGED",
                            "待删除关系已改变或不存在，未执行清理",
                        ));
                    }
                    connection.execute(
                        "DELETE FROM dependencies WHERE dependent_id=?1 AND dependency_id=?2",
                        params![from.id, to.id],
                    )?
                } else {
                    if expected_reason.is_some() {
                        return Err(Failure::new(
                            "RELATION_CHANGED",
                            "待删除关系不存在，未执行清理",
                        ));
                    }
                    0
                };
                Ok(
                    json!({"op": "remove", "changed": count > 0, "dependent": dependent, "dependency": dependency}),
                )
            }
            Operation::Update {
                new_dependent,
                new_dependency,
                reason,
                clear_reason,
                ..
            } => {
                if reason.is_some() && *clear_reason {
                    return Err(Failure::new(
                        "INVALID_OPERATION",
                        "reason 与 clear_reason 不能同时提供",
                    ));
                }
                if new_dependent.is_none()
                    && new_dependency.is_none()
                    && reason.is_none()
                    && !clear_reason
                {
                    return Err(Failure::new(
                        "INVALID_OPERATION",
                        "update 至少需要一个修改项",
                    ));
                }
                let from = find_object(connection, &dependent)?;
                let to = find_object(connection, &dependency)?;
                let (Some(from), Some(to)) = (from, to) else {
                    return Err(Failure::new("RELATION_NOT_FOUND", "要修改的依赖关系不存在"));
                };
                let Some(old_reason) = relation_reason(connection, from.id, to.id)? else {
                    return Err(Failure::new("RELATION_NOT_FOUND", "要修改的依赖关系不存在"));
                };
                let new_from = self.locator(new_dependent.as_deref().unwrap_or(&dependent))?;
                let new_to = self.locator(new_dependency.as_deref().unwrap_or(&dependency))?;
                let scope = scope.expect("update has Git scope");
                scope.require(&new_from)?;
                scope.require(&new_to)?;
                self.require_file(&new_from)?;
                self.require_file(&new_to)?;
                let next_from = ensure_object(connection, &new_from)?;
                let next_to = ensure_object(connection, &new_to)?;
                if (from.id, to.id) != (next_from.id, next_to.id)
                    && relation_reason(connection, next_from.id, next_to.id)?.is_some()
                {
                    return Err(Failure::new(
                        "RELATION_EXISTS",
                        "目标依赖关系已存在，未覆盖",
                    ));
                }
                let next_reason = if *clear_reason {
                    None
                } else {
                    reason.clone().or(old_reason.clone())
                };
                let changed =
                    (from.id, to.id, &old_reason) != (next_from.id, next_to.id, &next_reason);
                connection.execute(
                    "UPDATE dependencies SET dependent_id=?1, dependency_id=?2, reason=?3
                    WHERE dependent_id=?4 AND dependency_id=?5",
                    params![next_from.id, next_to.id, next_reason, from.id, to.id],
                )?;
                Ok(json!({"op": "update", "changed": changed,
                    "relation": Relation { dependent: next_from, dependency: next_to, reason: next_reason }}))
            }
            Operation::Forget { .. } | Operation::Analyzed { .. } => unreachable!(),
        }
    }

    pub fn scope(&self) -> Result<Value> {
        let scope = GitScope::load(&self.root)?;
        let initialized = self.database_path().exists();
        let mut outside_objects = Vec::new();
        let mut outside_relations = Vec::new();
        let mut object_count = 0;
        let mut relation_count = 0;
        if initialized {
            let mut connection = self.open(false, false)?;
            let transaction = connection.transaction()?;
            check_schema(&transaction)?;
            let objects = {
                let mut statement =
                    transaction.prepare("SELECT id,kind,locator FROM objects ORDER BY locator")?;
                statement
                    .query_map([], |r| {
                        Ok(Object {
                            id: r.get(0)?,
                            kind: r.get(1)?,
                            locator: r.get(2)?,
                        })
                    })?
                    .collect::<std::result::Result<Vec<_>, _>>()?
            };
            object_count = objects.len();
            for object in objects {
                if !scope.files.contains(&object.locator) {
                    outside_objects.push(object.clone());
                }
                for relation in adjacent(&transaction, object.id, Direction::Dependencies)? {
                    relation_count += 1;
                    if !scope.files.contains(&relation.dependent.locator)
                        || !scope.files.contains(&relation.dependency.locator)
                    {
                        outside_relations.push(relation);
                    }
                }
            }
            transaction.commit()?;
        }
        scope.verify(&self.root)?;
        let issues: Vec<_> = scope
            .files
            .iter()
            .filter_map(|file| self.require_file(file).err())
            .collect();
        let invalid =
            !outside_objects.is_empty() || !outside_relations.is_empty() || !issues.is_empty();
        let data = json!({"root": self.root, "repository": scope.repository, "initialized": initialized,
            "tracked_file_count": scope.files.len(), "tracked_files": scope.files, "object_count": object_count, "relation_count": relation_count,
            "outside_objects": outside_objects, "outside_relations": outside_relations, "issues": issues});
        if invalid {
            let mut error = Failure::new(
                "SCOPE_VIOLATION",
                "范围检查发现旧的范围外记录或无效文件；清单见 data，未修改数据库",
            );
            error.data = Some(data);
            return Err(error);
        }
        Ok(data)
    }

    pub fn query(&self, input: &str, direction: Direction, transitive: bool) -> Result<Value> {
        let locator = self.locator(input)?;
        let scope = GitScope::load(&self.root)?;
        scope.require(&locator)?;
        let mut connection = self.open(false, false)?;
        let transaction = connection.transaction()?;
        check_schema(&transaction)?;
        let start = find_object(&transaction, &locator)?;
        let mut paths: HashMap<i64, Vec<String>> = HashMap::new();
        let mut pending = VecDeque::new();
        let mut matches = Vec::new();
        let mut edges = BTreeMap::new();
        let mut excluded = BTreeMap::new();
        let mut checked = BTreeMap::from([(locator.clone(), ())]);
        if let Some(object) = &start {
            paths.insert(object.id, vec![locator.clone()]);
            pending.push_back(object.clone());
        }
        while let Some(object) = pending.pop_front() {
            for relation in adjacent(&transaction, object.id, direction)? {
                if !scope.files.contains(&relation.dependent.locator)
                    || !scope.files.contains(&relation.dependency.locator)
                {
                    excluded.insert(
                        (
                            relation.dependent.locator.clone(),
                            relation.dependency.locator.clone(),
                        ),
                        relation,
                    );
                    continue;
                }
                let next = match direction {
                    Direction::Dependencies => &relation.dependency,
                    Direction::Dependents => &relation.dependent,
                };
                checked.insert(next.locator.clone(), ());
                if !paths.contains_key(&next.id) {
                    // A shortest path is enough; all traversed edges retain alternative evidence.
                    let mut path = paths[&object.id].clone();
                    match direction {
                        Direction::Dependencies => path.push(next.locator.clone()),
                        Direction::Dependents => path.insert(0, next.locator.clone()),
                    }
                    paths.insert(next.id, path.clone());
                    matches.push(json!({"object": next, "distance": path.len() - 1, "path": path}));
                    if transitive {
                        pending.push_back(next.clone());
                    }
                }
                edges.insert(
                    (
                        relation.dependent.locator.clone(),
                        relation.dependency.locator.clone(),
                    ),
                    relation,
                );
            }
        }
        matches.sort_by(|a, b| {
            a["distance"]
                .as_u64()
                .cmp(&b["distance"].as_u64())
                .then_with(|| {
                    a["object"]["locator"]
                        .as_str()
                        .cmp(&b["object"]["locator"].as_str())
                })
        });
        let data = json!({
            "root": self.root, "query": {"kind": "file", "locator": locator, "id": start.map(|o| o.id)},
            "direction": match direction { Direction::Dependencies => "dependencies", Direction::Dependents => "dependents" },
            "transitive": transitive, "matches": matches, "relations": edges.into_values().collect::<Vec<_>>()
        });
        scope.verify(&self.root)?;
        transaction.commit()?;
        let issues: Vec<_> = checked
            .keys()
            .filter_map(|path| self.require_file(path).err())
            .collect();
        if !issues.is_empty() || !excluded.is_empty() {
            let code = if !excluded.is_empty() {
                "SCOPE_VIOLATION"
            } else if issues.iter().all(|e| e.code == "FILE_NOT_FOUND") {
                "FILE_NOT_FOUND"
            } else {
                "INVALID_OBJECTS"
            };
            let mut error = Failure::new(
                code,
                "查询已完成，但存在缺失、无效或范围外对象；范围内的已有依赖保留在 data 中",
            )
            .detail(json!({"issues": issues, "excluded_relations": excluded.into_values().collect::<Vec<_>>()}));
            error.data = Some(data);
            return Err(error);
        }
        Ok(data)
    }
}

fn check_schema(connection: &Connection) -> Result<()> {
    let version: i32 = connection.pragma_query_value(None, "user_version", |r| r.get(0))?;
    let application: i32 = connection.pragma_query_value(None, "application_id", |r| r.get(0))?;
    if application != APPLICATION_ID || !(1..=SCHEMA_VERSION).contains(&version) {
        return Err(Failure::new("SCHEMA_MISMATCH", "不支持此数据库标识或结构版本；未自动覆盖或迁移")
            .detail(json!({"expected_version": SCHEMA_VERSION, "actual_version": version, "application_id": application})));
    }
    Ok(())
}

fn find_object(connection: &Connection, locator: &str) -> Result<Option<Object>> {
    Ok(connection
        .query_row(
            "SELECT id, kind, locator FROM objects WHERE kind='file' AND locator=?1",
            [locator],
            |r| {
                Ok(Object {
                    id: r.get(0)?,
                    kind: r.get(1)?,
                    locator: r.get(2)?,
                })
            },
        )
        .optional()?)
}

fn ensure_object(connection: &Connection, locator: &str) -> Result<Object> {
    connection.execute("INSERT INTO objects (kind,locator) VALUES ('file',?1) ON CONFLICT(kind,locator) DO NOTHING", [locator])?;
    find_object(connection, locator)?
        .ok_or_else(|| Failure::new("DATABASE_ERROR", "无法读取刚登记的对象"))
}

fn relation_reason(connection: &Connection, from: i64, to: i64) -> Result<Option<Option<String>>> {
    Ok(connection
        .query_row(
            "SELECT reason FROM dependencies WHERE dependent_id=?1 AND dependency_id=?2",
            params![from, to],
            |r| r.get(0),
        )
        .optional()?)
}

fn adjacent(connection: &Connection, id: i64, direction: Direction) -> Result<Vec<Relation>> {
    let filter = match direction {
        Direction::Dependencies => "d.dependent_id",
        Direction::Dependents => "d.dependency_id",
    };
    let sql = format!("SELECT a.id,a.kind,a.locator,b.id,b.kind,b.locator,d.reason
        FROM dependencies d JOIN objects a ON a.id=d.dependent_id JOIN objects b ON b.id=d.dependency_id
        WHERE {filter}=?1 ORDER BY a.locator,b.locator");
    let mut statement = connection.prepare(&sql)?;
    let rows = statement.query_map([id], |r| {
        Ok(Relation {
            dependent: Object {
                id: r.get(0)?,
                kind: r.get(1)?,
                locator: r.get(2)?,
            },
            dependency: Object {
                id: r.get(3)?,
                kind: r.get(4)?,
                locator: r.get(5)?,
            },
            reason: r.get(6)?,
        })
    })?;
    let result = rows.collect::<std::result::Result<Vec<_>, _>>()?;
    if result
        .iter()
        .any(|r| r.dependent.kind != "file" || r.dependency.kind != "file")
    {
        return Err(Failure::new(
            "UNSUPPORTED_OBJECT_TYPE",
            "首版只支持 file 对象",
        ));
    }
    Ok(result)
}

#[cfg(test)]
mod scope_tests {
    use super::*;

    #[test]
    fn index_membership_change_rolls_back_the_write_transaction() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        GitScope::git(root, &["init", "-q"]).unwrap();
        fs::write(root.join("a"), "first").unwrap();
        GitScope::git(root, &["add", "a"]).unwrap();
        let workspace = Workspace::new(root).unwrap();
        workspace.init().unwrap();
        let scope = GitScope::load(&workspace.root).unwrap();
        // Staging new contents does not change the membership boundary.
        fs::write(root.join("a"), "second").unwrap();
        GitScope::git(root, &["add", "a"]).unwrap();
        scope.verify(&workspace.root).unwrap();
        let mut connection = workspace.open(false, true).unwrap();
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .unwrap();
        ensure_object(&transaction, "a").unwrap();
        fs::write(root.join("b"), "new").unwrap();
        GitScope::git(root, &["add", "b"]).unwrap();
        let error = workspace
            .commit_write(transaction, Some(&scope))
            .unwrap_err();
        assert_eq!(error.code, "INDEX_CHANGED");
        assert_eq!(error.details["rolled_back"], true);
        assert_eq!(
            connection
                .query_row("SELECT count(*) FROM objects", [], |r| r.get::<_, i64>(0))
                .unwrap(),
            0
        );
    }
}
