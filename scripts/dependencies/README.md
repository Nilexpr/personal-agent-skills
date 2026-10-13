# 依赖管理 CLI

LLM 显式登记依赖，工具将关系和逐文件分析记录保存在每个工作目录自己的 SQLite 数据库中。管理范围是该目录内被 Git 索引跟踪的文件，分析工作区当前内容。A 依赖 B；B 变化后，查询 B 的依赖方得到 A。工具按主动调用时的文件内容核对分析状态，不推断依赖、不后台监控，也不执行 `git add`。

## 构建与检查

在仓库根目录执行（需要 Rust 工具链和 C 编译器，SQLite 随 rusqlite 编译；运行时需要 Git）：

```sh
cargo build --manifest-path scripts/dependencies/Cargo.toml --locked --release
cargo test --manifest-path scripts/dependencies/Cargo.toml --locked
cargo clippy --manifest-path scripts/dependencies/Cargo.toml --locked --all-targets -- -D warnings
```

可执行文件位于 `scripts/dependencies/target/release/dependencies`，无需全局安装。以下示例从仓库根目录运行；其他位置可使用二进制的绝对路径及 `--root /path/to/workspace`。`--help` 和各子命令的 `--help` 提供参数说明。

## 初始化与日常操作

```sh
# 只读列出 Git 跟踪范围，尚未 init 也可执行
scripts/dependencies/target/release/dependencies scope

# 当前 Git 工作目录初始化；旧版本升级为结构版本 2，保留已有对象 ID 和关系
scripts/dependencies/target/release/dependencies init

# 终端目录树：全部 Git 跟踪文件，包括没有关系或尚未登记对象的文件
scripts/dependencies/target/release/dependencies tree
scripts/dependencies/target/release/dependencies tree --only unlinked

# 新 session 获取剩余文件、内容指纹和分析记录版本
scripts/dependencies/target/release/dependencies coverage --only remaining

# Progress 依赖 skill 源码；原因由 LLM 根据实际内容判断
scripts/dependencies/target/release/dependencies add \
  goals/maintain-personal-skills/PROGRESS.md skills/structure/SKILL.md \
  --reason '进展记录保存了源码版本及验证结果，源码变化后需要核对其当前状态描述'

# 源码变化后查谁依赖它，默认只查直接关系
scripts/dependencies/target/release/dependencies dependents skills/structure/SKILL.md
scripts/dependencies/target/release/dependencies dependents skills/structure/SKILL.md --transitive
scripts/dependencies/target/release/dependencies dependents skills/structure/SKILL.md --text

# 查 Progress 依赖哪些对象
scripts/dependencies/target/release/dependencies dependencies goals/maintain-personal-skills/PROGRESS.md

# 修改原因、清空原因、替换端点；未指定的部分保留
scripts/dependencies/target/release/dependencies update \
  goals/maintain-personal-skills/PROGRESS.md skills/structure/SKILL.md --reason '需要核对版本与安装状态'
scripts/dependencies/target/release/dependencies update \
  goals/maintain-personal-skills/PROGRESS.md skills/structure/SKILL.md --clear-reason

# 关系删除不要求文件仍存在；不存在的关系返回 changed:false
scripts/dependencies/target/release/dependencies remove \
  goals/maintain-personal-skills/PROGRESS.md skills/structure/SKILL.md
```

`update A B --new-dependent C --new-dependency D` 将 A→B 改为 C→D，两个新端点参数均可单独使用。若新关系已存在则报错，不覆盖它。登记和更新后的两个端点必须被 Git 跟踪且文件存在；更新允许把旧的范围外端点修复为范围内端点。删除允许端点已经缺失或不再被跟踪。同端点、同原因的重复 `add` 返回 `changed:false`，原因不同则返回 `RELATION_EXISTS`，由调用者明确更新。

## 已有仓库的批量登记

初始化分析先用 `coverage` 获取完整文件清单、内容指纹及分析状态，再由 LLM 分批阅读、判断依赖并说明依据。已跟踪的未提交修改、已加入索引的新文件均纳入，未跟踪文件排除。数据库保存逐文件分析记录；未确认的候选关系仍由 LLM 在分析结果中说明，不混入正式依赖。以下数组可保存为 `relations.json`，执行 `dependencies batch relations.json`，也可通过 stdin 输入：

```json
[
  {
    "op": "add",
    "dependent": "goals/maintain-personal-skills/PROGRESS.md",
    "dependency": "skills/structure/SKILL.md",
    "reason": "进展记录包含该源码的版本与验证结果"
  },
  {
    "op": "update",
    "dependent": "goals/maintain-personal-skills/PROGRESS.md",
    "dependency": "skills/structure/SKILL.md",
    "reason": "源码变化后核对当前版本及安装描述"
  }
]
```

`batch` 还接受 `remove`、`forget`、`analyzed`，以及 `update` 的 `new_dependent`、`new_dependency`、`clear_reason` 字段；字段名使用下划线。`reason` 与 `clear_reason:true` 互斥。数组按顺序执行，后项可使用前项刚创建的关系。整个数组为一个事务：任一项失败，之前的关系修改、对象登记和分析记录全部回滚，错误包含从 0 开始的 `operation_index`。未知操作和字段会被拒绝；空数组为无操作。

```sh
scripts/dependencies/target/release/dependencies batch < relations.json
```

批量文件自身的路径相对于调用目录；数组中的对象路径相对于 `--root`。该命令只登记输入，不调用模型，也不承诺找出所有隐含依赖。

## 目录树与分析覆盖

`tree` 默认输出终端目录树，`tree --json` 与 `coverage` 返回同样的 JSON 数据。目录来自完整 Git 清单，不从数据库对象反推；目录只用于展示层级，不是依赖对象。每个文件显示直接依赖数、直接依赖方数和分析状态。某个文件的具体关联用 `dependencies FILE --text` / `dependents FILE --text` 查看，保留原因；加 `--transitive` 查看间接路径。省略 `--text` 返回原有 JSON。

`tree` 和 `coverage` 均接受 `--only`：

| 值 | 选出的文件 |
|---|---|
| `all`（默认） | 全部 Git 跟踪文件 |
| `unlinked` | 两个方向都没有范围内已登记关系 |
| `unanalyzed` | 可以读取，但没有分析记录 |
| `stale` | 当前内容指纹与上次分析不同 |
| `remaining` | 所有尚未完成项：未分析、内容变化及无法读取 |

筛选只影响返回的 `files` / 目录树，`shown` 是展示数量，`summary` 始终统计完整范围。树保留筛选文件的祖先目录；特殊文件名中的换行和终端控制字符转义显示，JSON 保留实际路径。范围外旧记录不参与关联计数，命令返回 `SCOPE_VIOLATION` 和有效清单，不静默清理。

`coverage` 的每个文件包含 `path`、当前内容的 SHA-256 `fingerprint`、`analysis_status`、`analysis_revision`、上次登记的 `analyzed_fingerprint`、可选 `note`、两个方向的关联数量及读取 `issue`。状态如下：

- `unanalyzed`：没有分析记录，即使已被登记为别人的依赖也不算分析完成。
- `analyzed`：LLM 登记的内容指纹与当前文件一致；允许无任何依赖。
- `stale`：文件内容已经改变，需要重新分析。恢复为同样内容时可复用原记录。
- `unavailable`：缺失、越界符号链接或读取失败；保留文件条目，指纹为空，不计为完成。

`summary.complete` 仅在全部文件当前内容都有分析记录，且没有读取或范围异常时为 `true`；`remaining` 为未分析、内容变化及无法读取的数量之和。`unlinked` 与这些状态独立，不能和状态数量相加。完成只代表覆盖记录齐全，不证明 LLM 阅读充分、找全语义依赖或已处理所有候选关系。

新 session 的操作顺序：

1. 执行 `init` 创建或升级数据库，再执行 `coverage --only remaining`，保存本批文件的 `fingerprint` 和 `analysis_revision`。
2. 读取文件及必要上下文，判断依赖。单纯主题相似或出现链接不自动成为依赖；需要说明被依赖方变化后为什么应检查依赖方。
3. 将关系修改和每个已分析文件的 `analyzed` 操作放进同一个 `batch`。没有发现关联也要提交该文件的分析记录；没读完或读取失败的文件不提交完成记录。
4. 再次执行 `coverage --only remaining`，处理新增、遗漏及内容已变化的文件。清单按本次调用的 Git 范围重新生成，不需要维护额外的会话任务表。

独立登记命令为 `analyzed FILE --fingerprint HASH --expected-revision N [--note TEXT]`，其中 `HASH` / `N` 必须替换为分析前取得的实际值。批量操作对象如下（占位符须替换为实际指纹）：

```json
{
  "op": "analyzed",
  "file": "docs/design.md",
  "fingerprint": "<coverage 返回的 64 位小写 SHA-256>",
  "expected_revision": 0,
  "note": "已阅读当前内容，未发现需要登记的依赖"
}
```

每次成功登记将该文件的分析版本加一；初始版本为 0。内容不一致返回 `CONTENT_CHANGED`，版本已被其他调用更新返回 `ANALYSIS_CHANGED`，整批回滚；不能仅换用新指纹或版本号重试，应重新核对内容或另一会话的结果。工具提交前再次核对本批分析文件的内容及 Git 成员集合。文件系统、Git 与 SQLite 无法构成跨系统原子快照，提交后的外部改动由下次 `coverage` 识别。

指纹按调用时流式读取工作区文件计算，包括二进制文件；筛选仍需要核对全部文件，因此耗时随仓库内容量增加。工具不判断文件是否适合 LLM 分析，也不把二进制文件或大文件自动算作完成。记录只保存最近一次分析，不保存正文或历史版本。依赖对象变化不会自动改写其他文件的分析记录，LLM 仍须通过 `dependents` 查询和判断影响。

## 检查与清理旧的范围外记录

`scope` 返回 `tracked_files`、数据库对象与关系数量，以及 `outside_objects`、`outside_relations` 和文件存在性 `issues`。检查发现问题时退出 2，清单仍在 `data` 中；不会删除记录或创建数据库。

普通查询只遍历范围内的节点和边。遇到范围外旧边时返回 `SCOPE_VIOLATION`，有效结果仍在 `data`，被排除的边在 `error.details.excluded_relations`；不会通过这些旧边连接其他文件。直接查询未跟踪文件返回 `FILE_NOT_TRACKED`。已跟踪但磁盘缺失的文件仍返回 `FILE_NOT_FOUND` 并保留已有查询结果。

清理前先核对清单并保存可恢复数据。通过 `remove A B --expected-reason '预览时的原因'` 可在原因改变或关系消失时拒绝删除；不提供该参数时保留原有幂等删除行为。`forget FILE` 显式移除不再被跟踪且没有任何关系的旧对象，仍有关系或仍被跟踪则拒绝。两者都可放入一个批量事务：

```json
[
  {"op":"remove","dependent":"A.md","dependency":"references/B.md","expected_reason":"预览时的原因"},
  {"op":"forget","file":"references/B.md"}
]
```

`expected_reason` 为可选字符串，`forget` 不会级联删除关系。只有 `remove` 的操作允许在非 Git 旧工作目录清理；新增、更新、查询、检查和 `forget` 均要求可读取 Git 工作树。所有清理均由调用者显式指定，不自动清空数据库。

本仓库的代表性验证输入见 [repository.json](examples/repository.json)，含 LLM 根据当前文件用途判断的四条依赖。它只覆盖工具说明、相关进展和仓库入口，不是整个仓库的全量依赖清单。可在仓库根目录执行：

```sh
scripts/dependencies/target/release/dependencies init
scripts/dependencies/target/release/dependencies batch scripts/dependencies/examples/repository.json
scripts/dependencies/target/release/dependencies dependents scripts/dependencies/src/lib.rs --transitive
```

## JSON 与错误约定

除帮助和版本信息、`tree` 默认的终端文本以及关联查询的 `--text` 模式外，命令向 stdout 输出一个 JSON 对象，成功退出 0，失败退出 2。JSON 模式的错误不混入非 JSON 日志；消费者应检查退出码、`ok` 和 `error.code`。查询缺失文件或覆盖检查异常仍然失败，但 `data` 保留已有结果；文本模式打印有效结果及错误说明。参数解析失败统一返回 JSON。

```json
{
  "ok": false,
  "data": {
    "query": {"id": 2, "kind": "file", "locator": "B.md"},
    "matches": [
      {"object": {"id": 1, "kind": "file", "locator": "A.md"}, "distance": 1, "path": ["A.md", "B.md"]}
    ]
  },
  "error": {
    "code": "FILE_NOT_FOUND",
    "message": "查询已完成，但存在缺失、无效或范围外对象；范围内的已有依赖保留在 data 中",
    "details": {"issues": [{"code": "FILE_NOT_FOUND", "message": "文件不存在", "details": {"locator": "B.md"}}]}
  }
}
```

以上是省略部分字段的示例。完整查询结果还包含 `root`、`direction`、`transitive`、`relations`；每条关系携带端点对象和 `reason`。每个匹配对象返回一条最短 `path`，路径始终沿“依赖方→被依赖方”排列。直接查询只返回当前一层，间接查询返回可达关系及其中的循环边；查询起点不列为自己的匹配项，自依赖边仍保留在 `relations` 中。替代路径可从返回的关系追溯，不枚举所有路径。

修改结果放在 `data.results` 中，按操作顺序返回 `op`、`changed` 及关系或端点信息。一般失败时 `data:null`；查询发现失效对象和 `scope` 检查发现问题时保留 `data`。已跟踪的既有文件未登记时，查询成功且结果为空，`query.id:null`。

稳定错误类型：

| code | 含义 |
|---|---|
| `INVALID_ARGUMENT` / `INVALID_BATCH` / `INVALID_OPERATION` | 命令、批量 JSON 或操作字段有误 |
| `INVALID_ROOT` / `INVALID_PATH` / `OUTSIDE_ROOT` / `NOT_A_FILE` | 根目录、路径或文件类型不符合范围 |
| `FILE_NOT_FOUND` / `INVALID_OBJECTS` | 文件缺失；或查询结果含其他无效对象，细节见 `issues` |
| `RELATION_EXISTS` / `RELATION_NOT_FOUND` | 与现有关系冲突；或待修改关系不存在 |
| `GIT_UNAVAILABLE` / `GIT_ERROR` / `NOT_GIT_WORKTREE` | Git 不可用、读取失败或目录不属于 Git 工作树 |
| `FILE_NOT_TRACKED` / `SCOPE_VIOLATION` | 端点不在跟踪范围内；或发现范围外旧记录、无效文件 |
| `INDEX_CHANGED` | 命令执行期间 Git 跟踪集合改变，写入已回滚 |
| `FILE_STILL_TRACKED` / `OBJECT_IN_USE` | 不能 forget 仍被跟踪或仍有关联的对象 |
| `RELATION_CHANGED` | 清理时的原因与预览不一致，或关系已不存在 |
| `CONTENT_CHANGED` | 提交的分析指纹与当前内容不同，整批回滚 |
| `ANALYSIS_CHANGED` | 分析版本已被其他调用更新，整批回滚 |
| `MIGRATION_REQUIRED` | 旧数据库可读，但须执行 `init` 升级后才能登记分析 |
| `NOT_INITIALIZED` / `INVALID_DATABASE_PATH` | 尚未 init；或数据库位置是符号链接 |
| `SCHEMA_MISMATCH` / `UNSUPPORTED_OBJECT_TYPE` | 不支持的数据库标识、版本或对象类型 |
| `DATABASE_BUSY` | 等待写锁 5 秒后仍未获得，可稍后重试 |
| `DATABASE_CORRUPT` / `DATABASE_ERROR` / `IO_ERROR` | 数据库损坏、其他数据库错误或文件系统错误 |

## 路径、并发与扩展边界

- 默认根目录是当前目录，不自动向父目录寻找数据库。Git 仓库根目录通过 Git 命令发现，工作目录为子目录时仅管理索引与该子目录的交集，数据库位置不变。支持 Git worktree，不递归合并子模块或嵌套仓库。对象路径可为根目录内的相对或绝对路径，规范化 `.` 和 `..` 后以相对路径保存；越界路径拒绝。
- 跟踪集合来自实际工作树索引的 `git ls-files --cached --stage --full-name -z`，排除子模块 gitlink。以 NUL 分隔处理文件名，重复索引条目去重。`.gitignore` 不改变已有索引成员；加入索引但未提交的文件纳入，退出索引的文件不再纳入。Git 读取失败直接报错，不降级成磁盘扫描；不使用调用环境的 `GIT_INDEX_FILE` 替代索引。
- 首版只接受文件。对象 ID 与定位字段分开：相同逻辑路径保持同一 ID，删除关系后保留对象记录。符号链接别名与实际路径是不同对象，指向目录外的符号链接拒绝；改名不会自动推断为同一个对象，应显式修改关系端点。
- 数据位于 `.dependencies/graph.sqlite3`。本仓库忽略运行数据库与构建目录；对其他工作目录不自动修改 Git 配置，不提供跨分支合并或多机器同步流程。
- 同一台机器的多个进程通过 SQLite 事务协调写入；批量操作使用 `BEGIN IMMEDIATE`，查询在一个读事务中获取一致的关系快照。每条命令共用一份跟踪集合，写事务提交前复核；范围改变则回滚，文件内容的暂存更新不改变成员集合。Git 索引、文件系统和 SQLite 不是同一个原子事务，复核完成后仍可能发生外部变化，需由后续调用再次核对。
- 结构版本使用 SQLite `user_version`，应用标识使用 `application_id`。当前版本为 2：`objects(id, kind, locator)` 保存对象，`dependencies(dependent_id, dependency_id, reason)` 保存有向关系，`analyses(object_id, fingerprint, revision, note)` 保存最近的分析记录。`init` 通过单个事务将版本 1 升级到 2，保留对象 ID、关系及原因；原有对象不会自动标为已分析。版本 1 仍可读取和维护关系，登记分析前须升级；未知版本拒绝打开，不自动覆盖。
- ID 独立于定位信息，未来可通过版本迁移增加段落选择器或目录定位规则，同时保留既有对象 ID 和关系；首版不接受其他类型，也不预建通用插件框架。明确 `forget` 范围外孤立对象时一并删除它的分析记录。
- 查询返回待检查范围，关系含义由 LLM 判断；工具只提供 CLI，不提供页面、后台监控、缓存、审计或历史记录。
