# 维护个人 Skills 与 AGENTS.md

本文件记录已取得的结果及验证范围；长期判断与选择见 [Decisions](DECISIONS.md)，目标范围与约束见 [Goal](GOAL.md)。

## revise

- **revise 已按 writing-for-agents 与 writing 全量优化，并完成既定范围的全局安装。**
  - Evidence: 2026-10-12 按本目标 [Decisions](DECISIONS.md) 的确认结果，将[revise](../../skills/revise/SKILL.md)共同规范收敛为 writing 的全文评审和 Matt 同类 skill 的主体结构参考；保留范围、设计预期、独立理解评审及上下游依赖维护。全文由 744 词调整为 579 词，SHA-256 为 `9a017ebcb975efcae67872032596ab1229aff0bae7977818c110e07680f54b4a`。
  - Evidence: 本会话按两份写作指导及 SKILL-MECHANICS 全量检查正文；应用前，新建不继承编写历史的评审会话，只提供候选稿与正常引用材料。其对用途、范围、调用、流程和写入条件的解释与已确认预期一致，未发现阻断理解的问题；保存内容与被评审候选稿逐字一致。
  - Evidence: 在 Fish 中执行 `env DISABLE_TELEMETRY=1 npx skills add /Users/lvhongwei/workspace/project/personal-agent-skills --skill revise structure --global --agent cline codex cursor github-copilot zed --yes`，skills 1.7.2 退出码为 0，报告两项均已安装到 Cline、Codex、Cursor、GitHub Copilot、Zed。`~/.agents/skills/revise` 与源码文件集合和全部字节一致；两侧格式校验、相对引用和差异空白检查通过。structure 同步结果见下节。
  - Evidence: 依赖工具已复核上下游并更新两条关系说明；AGENTS.md、README 和既有调用方继续适用。Matt 结构参考入口位于项目 `references/matt-skills/README.md`，属于不纳入 Git 和依赖图的本地参考，缺失时需补齐可读来源。
  - Scope: 已完成源码、相关记录和安装同步，未提交或发布。独立理解评审不等于原场景重放、实际任务效果或各客户端自动触发已验证；既有全仓覆盖待复核项仍按依赖工具结果处理。

- **revise 初版已按确认稿实现，并安装到既定全局范围。**
  - Evidence: 2026-10-09 新增 [SKILL.md](../../skills/revise/SKILL.md)，保留已确认正文并加入本仓库管理范围限定；全文 615 词、4404 字节，SHA-256 为 `7ebbc98daf1b947d08946314c855fabaa0f47df4a70b4517a3461490ee186e6f`。README 已加入入口，设计目的与修订边界已保存到本目标的 Decisions。
  - Evidence: 在 Fish 中执行 `env DISABLE_TELEMETRY=1 npx skills add /Users/lvhongwei/workspace/project/personal-agent-skills --skill revise --global --agent cline codex cursor github-copilot zed --yes`，退出码为 0；安装器报告已复制到 `/Users/lvhongwei/.agents/skills/revise`，覆盖 Cline、Codex、Cursor、GitHub Copilot 和 Zed。源码与安装目录的文件集合及内容逐字节一致；两侧 `quick_validate.py`、README 入口检查和 `git diff --check` 均通过。
  - Scope: 按已确认稿保存，全文超过 600 词精简目标 15 词；未逐客户端验证自动发现与触发，未进行行为评测，不能据此宣称使用效果改善。

## structure

- **structure 已支持默认工作区及显式指定记录根，并随 revise 完成全局同步。**
  - Evidence: 2026-10-12 仅替换[SKILL.md](../../skills/structure/SKILL.md)的记录根段落：未指定时使用会话 cwd，用户或调用流程明确指定时使用该路径，并区分记录根相对路径与记录文件路径的解析基准。与本轮开工快照比较，其余内容逐字一致；当前全文 963 词，SHA-256 为 `a02df84d9fad0dfe1be571b3db9c8aab4363a637cb7141b87636277e9c223e99`，格式、相对链接与差异空白检查通过。
  - Evidence: 此前确认的记录分工与独立目标选择继续保留，依据见 [Decisions 的 structure 小节](DECISIONS.md#structure)。2026-10-09 的 938 词版本及其校验值 `42a317de107f981bf27eba329f73e35f9bf323f11c81b0734a010c917020a946` 属于历史源码；此前按[默认范围](../../DECISIONS.md)安装的是 908 词版本，当时通过内容一致性与格式检查。
  - Evidence: 2026-10-12 因新版 revise 直接引用记录根规则，使用上方同一安装命令同步 structure；安装器报告覆盖既定五个 Agent。`~/.agents/skills/structure` 的文件集合和全部字节与当前源码一致，主文件 SHA-256 为 `a02df84d9fad0dfe1be571b3db9c8aab4363a637cb7141b87636277e9c223e99`，源码与安装副本格式校验通过；本轮未改 structure 正文。
  - Scope: 当前源码已安装；全文沿用表达完整性优先的约束，记录根选择之外的既有规则保持不变。跨会话执行及各客户端自动调用效果仍未验证。

- **依赖管理工具已从本维护目标拆分。**
  - Evidence: 2026-10-09 按用户确认新建 [依赖管理目标](../dependencies/GOAL.md)及其 Decisions 和 Progress，四条既有工具决定原样迁移，专属约束移入新 Goal；本目标保留入口，README 同步索引并移除旧扫描原型的工具入口。
  - Scope: 后续工具需求、设计、实现与验证在依赖管理目标推进；structure 的修订继续在本目标维护。

- **历史维护问题已回查，并据此保留关键边界。**
  - Evidence: 沿 [Structure Skill](codex://threads/01a07ef7-c9bf-7b30-88d4-2950c415b7f7) 读至最早一页，核对 57 个返回轮次中的用户纠正与相关答复，交叉查阅 [Writing Skill](codex://threads/01a0c863-8822-7c33-8cf4-f2eacaab0b9f)、[列出项目维护文件](codex://threads/01a0d265-6e6c-7a00-a35d-4b9a0b08b682) 和 [AgentMD](codex://threads/01a0c7d2-b18e-7243-b305-ee954f33ee9b)。现文覆盖讨论入口、小请求仍遵循适用决定、长期判断与产物成果分别记录、单次具体变更确认，以及相关目标冲突先澄清的要求。
  - Scope: 属于历史证据回查与静态推演，部分长答复使用工具返回的截断内容，未全量重读工具输出；不能据此保证后续会话不会再犯。

## learn

- **learn 已按完整评审修订学习方式、完成判断、记录维护和视觉协作，源码尚未进行真实教学验证。**
  - Evidence: 2026-10-12 用户授权按评审问题优化后，重写 [SKILL.md](../../skills/learn/SKILL.md)、[PLANNING.md](../../skills/learn/PLANNING.md)、[RECORDS.md](../../skills/learn/RECORDS.md) 和 [LESSONS.md](../../skills/learn/LESSONS.md)，设计依据见 [Decisions 的 learn 小节](DECISIONS.md#learn)。主文件从 653 词调整为 591 词，配套文件依次为 203、297、250 词；主文件 SHA-256 为 `2f649e3e71f0829e1558567aff38d0d9fda16dc7e008f6643f6a27d670bdf369`。
  - Evidence: `quick_validate.py`、修改范围的 `git diff --check` 及四份正文与本目标 Decisions 的 23 处本地链接和锚点检查通过。文本推演覆盖连续阅读、不同帮助程度的应用、暂缓前置缺口、错误答案更正、跨会话恢复和单个视觉产物；这些检查不构成实际教学测试。
  - Evidence: 已登记视觉分支到 visualize、学习记录到 structure、规划到记录及主文件到设计决定的关系，并更新相关职责说明；双向查询和间接查询确认调用路径。README 用途标签、调用配置、visualize、structure、writing、revise、项目规则及依赖示例经影响复核仍适用，无需改写；依赖目标中原先未落实的 learn/visualize 候选已解决。
  - Scope: 保留四份正文，分别承载共同教学规则、规划调整、记录维护和可选视觉交付，均低于 600 词精简目标。仅修改仓库源码、相关记录和本地依赖数据，未安装、提交或发布；尚未执行真实教学、跨会话恢复或页面集成实测。开工时根 DECISIONS、agentmd/global.md 和 writing 的既有 stale 状态不属于本次 learn 修订结果。

## 其他 skills

- **Pi 课件重复问题的 learn 候选已撤回，当前仅准备 writing 修订。**
  - Evidence: 2026-10-13，用户明确纠正修改范围为 writing，并要求同一语义只能在一个位置维护；已据此准备 `/private/tmp/writing-single-source-2026-10-13.md` 及同名 `.diff`，替代此前 learn 交付说明候选。
  - Scope: learn 不属于本次修改范围，旧候选不再适用；writing 正式源码尚未应用，独立理解评审仍待完成，未安装或验证行为效果。

- **writing 已补充跨上下文文本的使用要求，本次修订待全局同步。**
  - Evidence: 2026-10-09 安装的 [SKILL.md](../../skills/writing/SKILL.md) 版本为 1059 词，主文件 SHA-256 为 `9681a5d51be970385e0165a9a8690e7a71f5e8320e58474b71e4a23d9cd42987`。当时通过 Fish 中的 `npx skills` 按[默认范围](../../DECISIONS.md)完成全局安装，源码与安装目录的文件集合及全部字节一致，两侧格式校验通过；命令、安装位置与逐文件校验值见[安装证据](../../references/writing-review-2026-10-09/INSTALLATION.json)。
  - Evidence: [Matt 规则评审](../../references/writing-review-2026-10-09/review-matt.json)和[自身规则评审](../../references/writing-review-2026-10-09/review-self.json)分别检查了同一份 1009 词[冻结稿](../../references/writing-review-2026-10-09/writing.md)，两项建议已落实为评审完成约定和逐句识别 no-op 候选；[比较结果](../../references/writing-review-2026-10-09/COMPARISON.json)保留取舍依据。
  - Evidence: 2026-10-10 按确认稿新增跨上下文文本要求，明确覆盖跨 session 提示词，并要求发送或保存前从接收者可获得的信息检查可用性；[全局规则源码](../../agentmd/global.md)加入 writing 调用入口，相关决定已保存。writing 当前为 1145 词，SHA-256 为 `b9868c54bd9d0ec8911394769fa80763816cd08b2ac80c96e59a4ce86bf40553`，格式校验与改动的空白检查通过。
  - Scope: 本次完成仓库源码与记录修改，未重新安装 writing 或全局规则；只读核对已安装 writing 仍为 2026-10-09 版本，已安装全局规则尚无新增入口。既有独立评审未覆盖本次修订，行为效果仍待验证。

- **document-reading 已移除，research 已退出候选维护范围。**
  - Evidence: 2026-10-09 删除 document-reading 的 5 个已跟踪文件及 README 入口，更新 visualize 的历史参考说明；本仓库没有 research 实现，相关候选记录已替换为停止维护决定。检查的既定全局目录中均未发现两项安装。
  - Evidence: 删除前确认 document-reading 无未提交修改；历史内容位于 Git 提交 `8d0cbf48c6133a44436f2f42b612d0337df3c94c` 的 `skills/document-reading/`，research 旧设计记录位于同一提交的 `goals/maintain-personal-skills/CONTEXT.md`。
  - Scope: 未修改其他 skill 正文或上游参考快照；历史报告中的名称仅作当时情况保留。

- **visualize 已按通用视觉建模流程重写，配有设计规范与默认 CSS；全局安装版仍为此前草稿。**
  - Evidence: 2026-10-08 按用户纠正重写 [SKILL.md](../../skills/visualize/SKILL.md)，全文 549 词，SHA-256 为 `0d28587094fd41abf53766384d8e2f980e0628663567f7d19da63c551faa7f70`。流程为明确理解问题、建立有依据的模型、设计观察与交互、实现、验证交付；从材料提取主体、层次、关系及变化，借助 structure 统一定义，未按领域逐项追加规则。
  - Evidence: [DESIGN.md](../../skills/visualize/DESIGN.md) 共 550 词，补充整体与细节对应、关系含义和可返回的依据查阅。Decisions 中此前收窄到复杂代码的条目已按用户要求替换为通用定位；[设计参考](../../references/visualize-design.md)同步说明依据。格式、字数、本地文件链接及空白检查通过。
  - Evidence: 用户同意后新增 [默认 CSS](../../skills/visualize/assets/visualize.css)，与官方 1.0.45 原文件逐字节一致；[来源记录](../../skills/visualize/assets/visualize.source.json)保存版本、路径和校验值。DESIGN.md 说明独立页面的本地引用方式、组件的样式边界和交互需自行实现；静态检查确认该 CSS 无外部导入，URL 仅使用内嵌 data 资源。
  - Evidence: 此前通过 `npx skills` 安装到 Cline、Codex、Cursor、GitHub Copilot 和 Zed 的版本，主文件 SHA-256 为 `690e6a69dabb75f56161be5fb4e29512869d99436577b4fbdca645f3298e5317`；本次确认它与新源码不同。用户此前在聊天中选择 `$visualize` 时传入的路径为 `~/.agents/skills/visualize/SKILL.md`，确认该次选择指向个人安装版。
  - Scope: 本次完成指令与静态结构检查，未重新安装，未执行独立会话行为评测、页面生成或组件集成实测；不能据此认定理解效果已验证。

## 参考资料

- **已完成 Matt 固定快照中全部 38 个 skill 的组织方式分析。**
  - Evidence: 2026-10-12 完整阅读 38 份 SKILL.md 和 27 份附属正文、模板及脚本，核对 38 份调用配置；依据 [SOURCE.json](../../references/matt-skills/SOURCE.json) 验证快照全部 188 个文件或符号链接的 SHA-256，均匹配。[完整索引](../../references/matt-skills/README.md)列出逐项来源。分析从各 skill 的实际组织出发，覆盖用途与边界、概念与状态、流程与交互、产物与维护、分文件与能力组合，并区分常见写法和个别例外。
  - Scope: 来源为提交 `f3fc5632f401156837ee3872f14fe33ccf1024ea`，包含杂项和实验版；这是文本分析，未执行上游工作流、验证行为效果或将归纳作为已批准编写规则。

- **已保存 domain-modeling 带读讨论中的八项 skill 编写参考。**
  - Evidence: 2026-10-09 按用户两轮确认，将最初四点与随后六点合并为 [Decisions 中的八项参考](DECISIONS.md#skill-编写参考)：描述顺序、用户确认、行动条件、示例与概念关系检验、矛盾核对、产物职责、按需创建与及时记录、能力边界。行动条件与概念关系检验并入原有相关条目，其余四项新增；关联本地 Matt 固定快照及 Astra 文章，区分原文要求、讨论判断和适用范围。
  - Scope: 作为后续设计与评审的可选参考；本轮只维护记录，未据此修改 skill 或通用规则。

- **Matt 快照与 Astra 官方原文已保存，当前文件校验通过。**
  - Evidence: 2026-10-08 实查 [Matt 来源清单](../../references/matt-skills/SOURCE.json)中的提交为 `f3fc5632f401156837ee3872f14fe33ccf1024ea`，188 个文件或符号链接的 SHA-256 全部匹配；[阅读索引](../../references/matt-skills/README.md)提供 skill 及附属文件入口。
  - Evidence: [Astra 原文](../../references/rethinking-skills-and-prompts-for-gpt-6-astra.md)为 6517 字节，SHA-256 为 `a1deee4c0b3ca16a20385c27f0692e22d5150fdec81710c04742dd5da29f0468`，与[来源记录](../../references/rethinking-skills-and-prompts-for-gpt-6-astra.source.json)一致；本次 structure 调整已实际阅读并采用其中按需加载、避免过细流程和明确完成条件的建议。
  - Scope: 固定参考快照，不代表已检查上游最新版本；人工理解进度以[带读会话](codex://threads/01a11a1e-e321-70b3-abfa-70553094d80d)及后续对话为准，不以文件读取完成代替用户理解。

- **OpenAI 用户能力分类资料仍可作为选题参考。**
  - Evidence: [2026-09-28 的分类与分析](../../references/openai-user-capabilities-2026-09-28.md)保存当时对 12 类场景、交互形式、平台依赖和可借鉴方向的调查。
  - Scope: 保留历史参考价值；本轮未更新产品事实、账户权益或外部链接，也未将其中建议作为已批准决定。

## 规则与交付脚本

- **根目录参考资料已排除出 Git，当前文件集合已通过提交前检查，尚未暂存或提交。**
  - Evidence: 2026-10-09 添加 `/references/` 忽略规则，`git check-ignore` 确认生效；README 标明本地资料，打包脚本改为参考目录存在时才包含。按 `git ls-files --cached --others --exclude-standard` 收集现存文件，在临时目录中分别验证无参考目录和有参考目录时打包成功。
  - Evidence: 5 个 skill 的 `quick_validate.py` 校验、`scripts/check-rule-installation.py` 的 11 项测试、两个 Shell 脚本的 `bash -n` 及 `git diff --check` 均通过；提交文件集合中的其余相对文件链接有效。
  - Scope: 26 处本地参考链接的目标不会随 Git 保存，新检出仓库需另行提供这些资料；本次未执行暂存、commit 或远端操作，也未补齐历史设计依据或拆分维护目标。

- **两份通用规则曾通过安装一致性核验；global 新增 writing 入口后待同步。**
  - Evidence: 2026-10-08 只读提取 `~/.codex/AGENTS.md` 中的 global 区块、`/Users/lvhongwei/workspace/AGENTS.md` 中的 workspace 区块，与 [global](../../agentmd/global.md)、[workspace](../../agentmd/workspace.md) 逐字节匹配；两处起止标记均唯一。
  - Evidence: [安装脚本](../../scripts/setup-rule.sh)采用追加或替换管理区块的方式。既有 2026-10-08 验证记录确认区块外内容保留、重复安装返回 Unchanged，[安装检查](../../scripts/check-rule-installation.py)的 11 项检查通过。
  - Scope: 上述核验对应 2026-10-08 版本；2026-10-10 的 global 修订与同步状态见本文件 writing 条目。规则安装脚本未改动，独立新会话的执行效果仍待验证。

- **README 保持内容索引，打包脚本交付项目文件。**
  - Evidence: 当前 [README](../../README.md)提供 skill、规则、脚本和记录入口；[打包脚本](../../scripts/package-skills.sh)输入包含 skills、agentmd、goals、references、scripts 及根目录文件，排除会话、Git 内部数据和旧 ZIP。
  - Scope: 旧打包文件是对应时间的历史快照；本轮没有重新打包。Glossary 相关入口的旧提案适用状态见[待处理事项](#待处理事项)。

## 待处理事项

- **旧记录重写提案已不适用于当前文件，后续需重新准备。**
  - Evidence: 2026-10-09 对 `/var/folders/mt/phtb44n50wzg1gc88797czkw0000gn/T/structure-cwd-rewrite-mk5dd3p2/records-rewrite.diff` 执行 `git apply --check`，根目录和维护目标下的 Decisions 均无法匹配；两份 Decisions 与 README 已不同于提案基线。
  - Scope: 旧提案保留作历史参考。继续其中的 Glossary、README 和打包入口工作时，基于当前文件重新形成变更，再按记录规则展示与确认。
