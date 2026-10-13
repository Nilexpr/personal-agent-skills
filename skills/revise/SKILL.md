---
name: revise
description: Design and maintain skills under skills/ in the personal-agent-skills source project. Use when creating, changing, or removing a project skill or its supporting files.
---

# Revise

Keep the project's skills consistent in writing and organization as they evolve. Use writing for text quality and Matt's comparable skills for task-specific structure, while preserving each skill's confirmed purpose.

## Scope

Select the source project using [structure's record-root rule](../structure/SKILL.md#records). Confirm that the selected directory is the personal-agent-skills project; clarify an unknown location before editing. Work on its `skills/` sources and supporting files. Installed copies are evidence, not editing targets.

Read that project's `AGENTS.md` and referenced standards. Resolve project paths, maintenance commands, and structure records from this root, including in conversations started elsewhere. Installation and publication require explicit requests.

## Design expectations

Use the project's structure skill to read applicable goals and Decisions before proposing changes. Establish the skill's purpose, responsibilities, expected behavior, and intended change from these records and the request. Compare the current implementation with recorded intent; reuse confirmed choices and raise conflicts or missing premises that affect the design. For a new skill, establish the expectations not yet recorded.

For feedback about actual use, inspect the relevant output and version. Keep evidence and proposed explanations distinct.

## Form the revision

Read the whole affected skill and its supporting instructions and resources. For creation, inspect neighboring capabilities before adding another. Assess the whole design, then change what the request and findings require.

For the main organization, use the source project's `references/matt-skills/README.md` to find and read a Matt skill doing comparable work. Follow the task's shape: stages for dependent actions, rounds for interaction, objects and updates for persistent state, or concepts for a reference skill. Keep comparable skills similarly organized while adapting to their purpose; explain the structural choice. Matt's examples guide organization beyond writing's existing guidance. If the reference is unavailable, report the gap and obtain an accessible source before completing this assessment.

Use the project's `skills/writing/SKILL.md` to review the complete revised text and necessary supporting instructions. Keep its writing principles there rather than reproducing them in this workflow.

## Independent understanding review

Before writing source files, freeze each changed skill's complete candidate version. Give a fresh session with no authoring history only that candidate and its normally available references. Withhold prior feedback and intended answers. Ask it to explain purpose, scope, invocation, actions, and outputs from the text, and identify ambiguities or missing premises. It reviews understanding without executing the skill. For removal, review surviving entries and callers for clear retirement or replacement.

Compare its interpretation with the confirmed expectations. Resolve material comprehension gaps and repeat in another fresh session until they agree. Changes to meaning or necessary context invalidate a previous pass. If the session or required materials are unavailable, report the blocked review and leave the candidate unapplied. This checks understanding, not actual behavior.

## Apply and complete

Show complete affected passages in a two-column Markdown table, “原文” and “修改后”; leave cells empty for additions or deletions and show new files in full. Apply changes covered by existing implementation authorization; otherwise obtain approval of the proposed changes.

Follow the project's AGENTS.md maintenance procedure before and after changes: use the dependency tool to inspect upstream sources and downstream consumers, judge what needs updating, and reconcile affected files, relationships, and records. Use structure for decisions and evidence. Finish when that procedure's completion conditions hold, reporting unresolved work and separating comprehension, static checks, and behavioral evidence.
