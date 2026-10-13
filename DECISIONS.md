# 个人 Agent Skills

- **根目录 references/ 作为本地参考资料忽略，不纳入 Git；skill 自带的参考文件继续随源码维护。**
  - Basis: 用户于 2026-10-09 要求忽略参考目录，以减少 Git 中需要维护的文件数。
  - Scope: `.gitignore` 使用 `/references/`；资料保留在本机，打包时存在则包含、缺失则跳过。新检出的仓库不含这些资料，已有指向它们的链接属于本地资料入口。

- **主要参考 Matt Pocock 的 skills，不再将 Waza 作为参考对象。**
  - Basis: 用户于 2026-10-08 明确当前主要参考 [Matt Pocock skills](https://github.com/mattpocock/skills/tree/main/skills)，并明确取消 Waza，认为其没有参考意义。

- **参考 OpenAI 的 skills 与提示词设计指导，评审本项目的 skills 和 AGENTS.md。**
  - Basis: 用户于 2026-09-24 确认纳入 [Rethinking skills and prompts for GPT-6 Astra](references/rethinking-skills-and-prompts-for-gpt-6-astra.md)，用于检查指令是否仍有必要、是否过度限制执行。
  - Scope: 作为现有结构与写作参考的补充；采用针对 Astra 的建议时，考虑本仓库其他目标模型的适用性。

- **术语表使用 `GLOSSARY.md`，有符合记录条件的术语时再创建。**
  - Basis: 用户于 2026-09-24 确认术语表名称及按需创建原则，与 [structure](skills/structure/SKILL.md) 的文件分工一致。

- **中英文内容按一句英文、下一行对应中文的方式展示；diff 保留增删标记。**
  - Basis: 用户在任务 `01a07ef7-c9bf-7b30-88d4-2950c415b7f7` 中明确要求采用类似沉浸式翻译的逐句对照形式。

- **带读先完整呈现一章节或一大段含义相对完整的原文与逐句翻译，再统一分析；可给出总评或指出值得回读的部分，确认用户理解后再进入下一节。**
  - Basis: 用户于 2026-10-09 要求按内聚、独立的大段带读，并于 2026-10-10 进一步明确，不要每讲两句话就插入分析，应完整读完一章节或一大段后再评价。
  - Scope: 本项目后续带读；英文原文与中文逐句对照连续呈现，分析放在整节阅读之后。

- **带读优先关注非实验版的通用 agent 工作流，代码专项内容后置，实验版暂缓。**
  - Basis: 用户于 2026-10-09 明确希望先看 agent 工作流相关的 skill，写代码在其当前场景中占比较小；2026-10-10 进一步要求实验版先跳过。
  - Scope: Matt skills 的后续带读选题与顺序；已读内容保留既有进度。

- **本项目的编写标准保存在 Goal 的约束中，通用 skill 保持跨项目可复用。**
  - Basis: [维护目标的 Constraints](goals/maintain-personal-skills/GOAL.md#constraints) 规定了本项目的体量、表达和验证要求；原记录已确认这些项目标准不写入通用 skill。

- **本仓库的 skill 使用 `npx skills` 安装，默认全局安装到 Cline、Codex、Cursor、GitHub Copilot 和 Zed。**
  - Basis: 沿用用户已确认的 structure 全局安装范围；用户于 2026-10-09 要求将默认范围明确记录，供后续安装直接引用。

- **本仓库的 README 仅作为内容索引，保留入口链接和用途标签；具体规则、操作说明与记录由对应文件承载。**
  - Basis: 用户于 2026-10-08 要求 README 做成指针式的内容地图，不承载实际内容，并明确这是长期约定。
  - Scope: 本仓库 README 的后续维护。
