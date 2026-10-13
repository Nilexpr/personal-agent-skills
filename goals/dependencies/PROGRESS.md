# 依赖管理工具

- **当前 Git 跟踪范围已从空库重新完成依赖初始化。**
  - Evidence: 2026-10-10 本会话以 CLI `scope` 确认 35 个跟踪文件、0 个对象及 0 条关系；保存 `coverage` 指纹和分析版本后完整阅读这 35 个文件的工作区内容，在同一 `batch` 中登记 120 条带具体原因的关系及 35 份分析记录，未执行示例预填。`coverage --only remaining` 返回空集合：已分析 35、未分析 0、需复核 0、无法读取 0、无关联 0。
  - Evidence: 逐文件完成 70 次双向直接查询，并检查 2 次间接查询；端点、原因、计数和路径与本轮登记一致，逐文件 SHA-256 与分析指纹一致，Git 跟踪路径集合没有增减。终端展示使用现有 release CLI 的 `tree`、`tree --only unlinked` 和关联查询。
  - Scope: 仅覆盖本次 35 个 Git 跟踪文件；未跟踪的 `src/coverage.rs` 和根 `references/` 未登记关系或分析记录。覆盖完成不证明发现全部隐含依赖；未修改 Git 索引、重建程序或重跑 Rust 测试。
  - 候选处理与待核对: 2026-10-12 的 [learn 修订](../maintain-personal-skills/PROGRESS.md#learn)已在视觉分支明确调用 visualize 并登记关系，原候选已解决。初始化时 AGENTS.md 的外部 `writing-for-agents` 入口候选，已由同日的[项目维护修订](../maintain-personal-skills/PROGRESS.md#revise)明确为本地 writing 入口并登记。源码一致性问题仍在：此前只读复核确认 `src/main.rs` 缺少现有 release CLI 已提供的 `tree`、`coverage`、`analyzed` 和 `--text` 入口，仍由实现维护任务处理；本次未重新核对或改写工具实现。

- **旧关联数据已清空，重新初始化从空库开始。**
  - Evidence: 2026-10-10 按用户要求删除旧 `.dependencies/`、`analysis-2026-10-10/` 和 `analysis-2026-10-10-tracked/`，移除 README 的旧页面入口；重新执行 `init` 后，版本 2 数据库的 objects、dependencies、analyses 均为 0。旧会话“全仓依赖分析与可视化”（`01a123f0-3c0a-72f3-aeff-409a5e0fe514`）已归档。
  - Scope: 本轮重新初始化以此空库为起点，重新分析当前 Git 跟踪文件，不复用旧关联或旧分析完成状态；下列实现与迁移结果仅作为历史验证记录。

- **CLI 目录树与逐文件分析覆盖已实现。**
  - Evidence: 2026-10-10 新增 `tree`、`coverage`、`analyzed`；目录树基于完整 Git 跟踪清单，支持无关联、未分析、内容变化和剩余文件筛选，关联查询支持 `--text`。分析记录绑定 SHA-256 与版本号，可和关系修改一起批量提交，内容变化或并发冲突导致整批回滚。
  - Evidence: [34 项 CLI 集成测试](../../scripts/dependencies/tests/cli.rs)和 [1 项索引变化回滚测试](../../scripts/dependencies/src/lib.rs)通过；Clippy（告警视为错误）、release 构建、格式和 diff 检查通过。实际仓库的 Git 路径集合、35 个文件的指纹及双向关联数量经独立核验一致。
  - Evidence: 2026-10-10 曾验证本地数据库由版本 1 升级到 2，35 个对象和 102 条关系及原因逐项保持一致，SQLite 完整性和外键检查通过；旧库与升级副本随后按用户要求删除。
  - Scope: 原有关系不自动作为分析完成证明；该次实现完成时，35 个跟踪文件均无新版分析记录，尚未重新执行全仓语义分析。新增 `src/coverage.rs` 当时未加入 Git 索引，因此不在该次跟踪范围内。该次实现工作未修改 Git 索引或发布，保留其他会话已有改动；本轮初始化结果见上方新记录。

- **Git 跟踪范围约束与数据迁移已完成。**
  - Evidence: 2026-10-10 新增只读 `scope`、Git 双端点校验、范围外查询过滤、显式 `forget` 和带原因校验的删除；批量提交前复核索引范围。[26 项 CLI 集成测试](../../scripts/dependencies/tests/cli.rs)与 [1 项索引变化回滚测试](../../scripts/dependencies/src/lib.rs)通过，Clippy、release 构建和格式检查通过。
  - Evidence: 当时仅将已确认的 10 个项目文件加入索引，在一个事务内删除 731 条参考资料关系和 206 个范围外对象；保留的 35 个对象、102 条关系及原因逐项一致。当时的范围、数据库完整性及外键检查通过；迁移报告和恢复材料随后按用户要求删除。
  - Evidence: 当时的 140 次双向直接／间接查询、页面数据及交互逻辑核验通过，记录过 1 个待核对候选和 3 个循环分组；报告与页面随后按用户要求删除，不作为新一轮分析输入。
  - Scope: 未重新推断全部语义依赖，未复测浏览器渲染、键盘和窄屏布局；没有提交、全局安装或发布，其他会话改动未一并暂存。

- **此前曾完成全磁盘范围分析；旧产物已删除，此项仅保留历史验证范围。**
  - Evidence: 2026-10-10 曾分析 241 个磁盘文件（其中 references 下 206 个），登记 833 条关系，并通过 964 次双向直接／间接查询及完整性检查；当时记录过 4 项待核对候选、11 个循环分组。该范围随后被 Git 跟踪范围取代，旧数据库、报告和可视化已按用户要求删除。
  - Scope: 范围包含未提交文件与被 Git 忽略的本地参考资料，排除 Git 内部数据、编译产物、运行数据库、缓存和历史 ZIP；当前项目核心文件读取全文，长参考材料按用途与关系段落核对，逐文件注明方式，不保证发现所有隐含语义依赖。页面交互逻辑检查通过；Chrome 调试连接不可用且内置浏览器安全策略拒绝 file://，实际渲染、键盘及窄屏验证未完成。

- **Rust CLI 首版已实现并通过本地验证。**
  - Evidence: 2026-10-10 在 [scripts/dependencies](../../scripts/dependencies/README.md) 实现同步 CLI，SQLite 保存对象及有向依赖，支持增删改、批量事务、直接与间接查询、原因与路径输出、稳定 JSON 错误。对象 ID 与定位字段分开，数据库结构版本为 1；使用说明与扩展边界见工具 README。
  - Evidence: `cargo test --manifest-path scripts/dependencies/Cargo.toml --locked` 的 [17 项集成测试](../../scripts/dependencies/tests/cli.rs)通过，覆盖方向、循环、重复、批量新增和既有更新/删除回滚、缺失文件、路径、并发初始化、重复写入、12 个进程写入不丢失、读隔离、锁超时及未知结构版本保护。Clippy（告警视为错误）、release 构建及[打包脚本](../../scripts/package-skills.sh)排除构建产物和运行数据库的检查通过。
  - Scope: 在本机 macOS、Rust 1.95.0 上验证，未做跨平台运行验证、全局安装或发布；首版只接受文件，段落与目录规则仍未实现。

- **已在实际仓库验证代表性依赖登记和查询。**
  - Evidence: 2026-10-10 执行 `init`，再用 [repository.json](../../scripts/dependencies/examples/repository.json) 登记四条有依据的关系；再次登记全部返回 `changed:false`。查询核心实现 [src/lib.rs](../../scripts/dependencies/src/lib.rs) 的直接依赖方得到工具说明与工具 Progress，间接查询额外得到仓库 README，路径为 `README.md → scripts/dependencies/README.md → scripts/dependencies/src/lib.rs`。查询 structure 源码的依赖方得到 skills 维护 Progress。
  - Scope: 数据保存在被 Git 忽略的 `.dependencies/graph.sqlite3`。示例只覆盖上述资料，不代表已经初始化整个仓库的全部依赖；LLM 仍负责补充关联、解释分析范围和处理待核对事项。

- **已比较 Foam、Obsidian 的关联记录、查询索引和局部定位方式。**
  - Evidence: 2026-10-09 查阅 [Foam 核心图实现](https://github.com/foambubble/foam/blob/main/packages/foam-core/src/model/graph.ts)、[Foam 块引用](https://docs.foam.md/features/block-anchors/)、[Obsidian 数据存储](https://help.obsidian.md/Files+and+folders/How+Obsidian+stores+data)和[内部链接](https://help.obsidian.md/Linking+notes+and+files/Internal+links)。这些参考区分文本中的关联事实与派生图或缓存，并支持文件内的章节、文本块定位。
  - Scope: 这是 2026-10-09 的文档和源码调研，未完成参考产品实测。2026-10-10 已确认 SQLite 选型，此前独立 JSON 清单建议未采用。

- **旧 Python 扫描原型已移除，交付入口统一指向 Rust CLI。**
  - Evidence: 2026-10-10 核对 `scripts/file-links.py` 和 `scripts/check-file-links.py` 与本会话初始副本逐字一致后删除；README 仅保留最终工具的入口和用途标签。
  - Scope: 不沿用原型的 Markdown 自动扫描行为，也不将其测试作为最终工具的验证证据。
