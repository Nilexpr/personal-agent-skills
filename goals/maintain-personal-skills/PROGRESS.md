# 维护个人 Agent Skills

- **`structured-records` 原版已完成全局安装。**
  - Evidence: 此前安装输出确认 Cline、Codex、Cursor 和 GitHub Copilot 可用；2026-09-10 本次修订前，已核对仓库文件与 `../../skills/structured-records/SKILL.md` 内容一致。
  - Scope: 这是此前安装结果；本次仓库修订后，已安装文件与源文件不再一致。

- **`structured-records` 已加入共用术语表与中间结果清理规则，并完成独立评审。**
  - Evidence: [SKILL.md](../../skills/structured-records/SKILL.md)；2026-09-10 在工作区根目录运行 `python3 【原电脑本地路径已移除】 skills/structured-records`，输出 `Skill is valid!`；最终文件 SHA-256 为 `b934d29dc2b4a8e100253b4d16a8dde30e06a65e20416bb2eaba1c82f99f9412`。本次对话中，不继承讨论历史的独立评审通过三个请求推演检查规则，并复核了目标批准前的只读调查边界。
  - Scope: 验证限于文件格式、独立阅读及情景推演，尚未进行真实项目行为验证；新版尚未重新安装。

- **已提供个人 skills 的离线打包脚本。**
  - Evidence: [package-skills.sh](../../scripts/package-skills.sh)；2026-09-10 在工作区根目录运行 `bash -n scripts/package-skills.sh`，退出码为 0。既有记录报告 `personal-agent-skills-20260902-214204.zip` 曾通过归档校验及解压比对。
  - Scope: 本次仅检查脚本内容和语法，未执行打包；原 ZIP 当前不在工作区，历史归档验证结果未复验。

- **已创建通用学习技能 `learn` 首版。**
  - Evidence: [SKILL.md](../../skills/learn/SKILL.md)；2026-09-10 在仓库根目录运行 `python3 【原电脑本地路径已移除】 skills/learn`，输出 `Skill is valid!`；文件为 38 行、461 个按空白分隔的词。设计参考已通过 GitHub MCP 读取的 [Matt Pocock teach](https://github.com/mattpocock/skills/blob/main/skills/productivity/teach/SKILL.md)，围绕本次要求收敛为资料核实、概念解释、理解校正和学习笔记。
  - Scope: 已检查格式及条款对官方资料、无统一官方来源、资料缺口、概念边界和理解验证的覆盖；尚未进行真实主题的学习效果验证，未安装、发布或提交 Git。

- **已补充 `learn` 的学习顺序与收获检验要求。**
  - Evidence: [SKILL.md](../../skills/learn/SKILL.md)；2026-09-10 依据用户关于逐步引出未知问题和明确学习用途的反馈修订，要求以用户关心的问题为起点，为各步确定问题、前置认识和回答后的作用，并用当前缺口引出下一步，结束时回到起始问题。当前文件 38 行、474 个按空白分隔的词；再次运行 `quick_validate.py skills/learn`，输出 `Skill is valid!`。参考结构已核对 [teach](https://github.com/mattpocock/skills/blob/main/skills/productivity/teach/SKILL.md) 及其 Mission、Resources 和 Learning Record 格式文件。
  - Scope: 完成条款修订及格式校验；真实学习中的动机、理解与迁移效果尚待验证。逐句中英对照的展示约定已存在于根 Context，本次未修改 Goal 或 Context。

- **已为 `learn` 增加可独立完成的学习单元和 HTML 教学说明。**
  - Evidence: [SKILL.md](../../skills/learn/SKILL.md) 定义 unit 并链接 [LESSONS.md](../../skills/learn/LESSONS.md)，后者规定从成果倒推单元、按前置依赖排序、在单元内提供练习与反馈，以及按阅读和交互需要生成 HTML。2026-09-10 运行 `python3 【原电脑本地路径已移除】 skills/learn`，输出 `Skill is valid!`；主文件 40 行、486 个按空白分隔的词，说明文件 33 行、490 个词。已读取本机 teach 原文，并核对 [Android 官方课程](https://developer.android.com/courses/android-basics-compose/course)、[mdBook 阅读说明](https://rust-lang.github.io/mdBook/guide/reading.html) 和 [Brown 互动 Rust Book](https://rust-book.cs.brown.edu/)。
  - Scope: 完成规则编写、格式校验与引用文件检查；另提供状态更新的对话内简化演示，尚未做浏览器交互验证或真实教学验证。本次没有开始安卓课程或修改已安装的 teach。

- **已明确 `learn` 的章节层次、初始大纲和动态调整规则。**
  - Evidence: [LESSONS.md](../../skills/learn/LESSONS.md) 新增 chapter 定义，要求较大主题在教学前展示覆盖既定范围的章节大纲、目标深度、前置依赖和完成证据，并根据学习表现细化单元，以章末综合任务验证单元间的联系；[SKILL.md](../../skills/learn/SKILL.md) 补充深入学习范围和章节规划入口。2026-09-10 运行 `python3 【原电脑本地路径已移除】 skills/learn`，输出 `Skill is valid!`；主文件 40 行、490 个按空白分隔的词，说明文件 45 行、616 个词。
  - Scope: 完成规则修订及格式校验；尚未用完整课程验证大纲覆盖和动态调整效果。没有创建实际学习课程，也未修改 Goal 或 Context。

- **已按 structured-records 的组织方式与 writing-for-agents 的标准简化 `learn`。**
  - Evidence: [SKILL.md](../../skills/learn/SKILL.md) 集中为 Definitions、Workflow、Source checks、Formats、Changes；[LESSONS.md](../../skills/learn/LESSONS.md) 仅保留按需读取的 HTML 要求。2026-09-10 运行 `python3 【原电脑本地路径已移除】 skills/learn`，输出 `Skill is valid!`；重构前后的三个概念定义逐字比对一致。主文件 40 行、500 个按空白分隔的词，HTML 说明 20 行、156 个词，总词数从 1,106 减为 656；已核对本地引用文件存在。
  - Scope: 已检查格式、定义稳定性及既有需求的条款覆盖；未通过实际试教验证行为效果。本次没有修改 structured-records、writing-for-agents、Goal 或 Context。

- **已通过 `npx skills` 全局安装 `learn`，Agent 范围与 structured-records 一致。**
  - Evidence: 2026-09-10 按用户指定的方法，在 Fish 环境执行 `env DISABLE_TELEMETRY=1 npx skills add ../.. --skill learn --global --agent cline codex cursor github-copilot --yes`，退出码为 0，输出 `Installation complete`。随后 `npx skills list --global --json` 确认两者均为 global，Agent 列表均为 Cline、Codex、Cursor 和 GitHub Copilot；learn 安装位置为 `【未随包迁移的参考技能：learn/】`。安装后的 SKILL.md、LESSONS.md 均与仓库逐字节一致，格式校验输出 `Skill is valid!`；其 SHA-256 分别为 `b61d5f64d054cec4910f7f4e355078685668fbf2ae7bb9761a12e32146815252`、`f65c3d527e4b2fb76217f5b74ad78be70eb9ebb9b4932a8bb58048f61e21fac6`。
  - Scope: 已验证正式安装命令、全局范围、Agent 列表和文件内容；尚未在新的学习会话中验证调用效果。安装器对这四个 Agent 使用共享目录的 copy 模式，安装方式由 `npx skills` 管理；仓库继续保留维护版本。

- **已为 `structured-records` 增加用户明确选择或纠正做法后的 Context 提议触发。**
  - Evidence: [SKILL.md](../../skills/structured-records/SKILL.md) 在 Workflow 的公共入口要求：用户选择或纠正方法、约束、偏好后，在结束当前轮次前执行 Recording checks；符合 Context 条件时，在正常回答尾部询问是否按已确认范围记录，并遵守 Changes。2026-09-10 运行 `quick_validate.py skills/structured-records`，输出 `Skill is valid!`；Changes 段落与本次修改前逐字一致，文件 SHA-256 为 `d22cc6e9cd74b4de525bf6c150b868aa498a3dc6c84a119cd308912ccf72802e`。
  - Scope: 已修改仓库版本并验证格式及审批规则保留；尚未同步全局安装，尚未通过后续真实对话验证触发效果。关于 npx skills 安装方式的条目已在用户明确批准后补入[项目根 Context](../../CONTEXT.md)。

- **已补充 `learn` 单元衔接时的理解证据检查，并更新全局安装。**
  - Evidence: [SKILL.md](../../skills/learn/SKILL.md) 的 Workflow 5 首句按用户批准的三句话替换，要求推进对话前检查已有证据，不足时主动邀请针对缺口的简短验证，并允许明确跳过、保留未验证状态；逐字比对确认其他内容未变。2026-09-11 仓库版及安装版均通过 `quick_validate.py`；执行 `env DISABLE_TELEMETRY=1 npx skills add ../.. --skill learn --global --agent cline codex cursor github-copilot --yes`，输出 `Installation complete`，退出码为 0；安装后的两个文件与仓库逐字节一致。
  - Scope: 已验证修改范围、格式和安装一致性；真实对话中的触发效果待后续观察，尚未通过实际学习会话验证。

- **已为 `learn` 补充 HTML 阅读与概念关系呈现要求，并更新全局安装。**
  - Evidence: [LESSONS.md](../../skills/learn/LESSONS.md) 的 Presentation 合并了用户批准的四项要求：结构化呈现与按需联动高亮、关键概念就地展开及有明确含义的关联链接、定义/示例/来源交叉导航与阅读位置保留、必要推理保留及可选细节折叠。2026-09-11 通过 `env DISABLE_TELEMETRY=1 npx skills add ../.. --skill learn --global --agent cline codex cursor github-copilot --yes` 完成全局更新，退出码为 0；格式校验通过，安装后的两个文件与仓库逐字节一致。LESSONS.md 的 SHA-256 为 `a95c53bc4ed0896bd692b521234669288527ff82c91aa82d6ac4260b464662b6`，主 SKILL.md 保持未变。
  - Scope: 已验证规则更新、安装结果及文件一致性；没有重建 Pi 课程页面，实际阅读和交互效果待课程试用验证。

- **已创建独立的文档型 HTML 阅读交互技能 `document-reading`。**
  - Evidence: [SKILL.md](../../skills/document-reading/SKILL.md) 为 27 行、480 个按空白分隔的词，SHA-256 `cb5f6accfb2539f327b6ae610b45eaa7fa9a09c9572dfdca5df9850d02780081`；入口配套交互选型、ECMA 官方资源核对记录、可复现验证步骤和一个无依赖 HTML 示例。2026-09-14 运行 skill-creator 的 `quick_validate.py` 输出 `Skill is valid!`，示例内联 JavaScript 通过 `node --check`，本地 Markdown 链接、静态 HTML ID 和引用目标检查通过。
  - Evidence: 通过 Codex 内置浏览器的独立 HTTP 测试页及 CDP，实测预览与正式定义一致，点击关闭及 Enter/Tab/Escape 后焦点回到触发按钮，返回/浏览器后退前进保持原位置（源按钮 viewport top 在返回前后均为 379.796875 px），三个反向引用分为两个章节。外层 saved 只选中两处，内层及另一章节同名变量不选中；独立选择 next 后清除 saved 保留 next。发现并修正固定导航遮挡，复验引用 top 为 112.171875 px，大于导航 bottom 53.890625 px。390 px 窄屏 scrollWidth 为 390 px；打印时定义和变量文字保留、导航隐藏；禁用脚本后正文及原始链接结构可见。浏览器错误日志为空。
  - Scope: 已保存维护版本，未安装、发布、执行 Git 状态变更操作，也未修改 learn 或 Pi 课程。受工具能力限制，触屏事件及禁用脚本后的真实链接点击未验证；未执行屏幕阅读器、跨浏览器、其他项目生成流程或真实读者理解效果验证。ECMA 的机制依据当前官方脚本、样式和生成索引，未重新完成在线交互实测；准确范围见 [来源记录](../../skills/document-reading/references/ecma-evidence.md)。

- **已加强 `learn` 的反馈跟进和理解问题发现规则，并更新全局安装。**
  - Evidence: [SKILL.md](../../skills/learn/SKILL.md) 增加学习中反馈或困难的即时处理入口，将不充分或矛盾的证据转为针对不确定点的新情境预测/推理检查；Changes 要求区分理解、教材与节奏问题，记录问题、证据、调整和下次检查，将未解决问题带入后续相关单元，并按问题类型验证解决。[LESSONS.md](../../skills/learn/LESSONS.md) 补充练习回答和困难带回教学对话的途径，只使用已共享或检查过的结果作为证据。
  - Evidence: 2026-09-14 仓库版和安装版均通过 `quick_validate.py`；比对确认三个定义、明确跳过规则及章节/计划验证要求保留。通过 `env DISABLE_TELEMETRY=1 npx skills add ../.. --skill learn --global --agent cline codex cursor github-copilot --yes` 安装成功，两个安装文件与仓库逐字节一致。SKILL.md 的 SHA-256 为 `b1a20b52c56e557cb2d759951adac3af91d57919051fc9254479db2a9d67914d`，LESSONS.md 为 `3eaacdba36c7692a9bbbd501f3cb864374d78de245865121413f6540dc7670f7`。
  - Scope: 已验证规则内容、格式和全局安装一致性；未修改其他学习项目的教材或学习记录，持续学习中的问题发现和解决效果仍待实际验证。
