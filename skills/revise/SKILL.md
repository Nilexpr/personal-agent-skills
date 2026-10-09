---
name: revise
description: Revise skills managed in the personal-agent-skills repository when the user explicitly says their output does not meet expectations, or requests improvement based on actual use.
---

# Revise

Improve a skill through actual use while preserving its original purpose: the problem it was created to solve, its intended outcomes, and its scope.

A correction starts the revision process; approval of the concrete revision is required before saving it.

## Scope

Only maintain skills whose source is managed under `skills/` in `/Users/lvhongwei/workspace/project/personal-agent-skills`.

Before starting a revision, establish that the affected skill belongs to this repository. An installed copy is eligible when its source can be traced here; a matching name alone does not establish ownership. Revise the repository source, leaving installation to a separate request.

For external skills, plugin-provided skills, and upstream reference snapshots, leave them unchanged and end this revision workflow.

## Establish purpose and expectations

Identify the affected skill, the request, the actual output, and the version used. Distinguish that version from current source and installed copies; mark unavailable history rather than reconstructing it from today's text.

Recover the original purpose from the initial request, relevant goals, and confirmed decisions. Current instructions describe the implementation, but do not by themselves establish its intent.

Use the user's feedback to establish the expected behavior. Ask directly when its meaning, scope, or relation to the original purpose is unclear; reuse answers already given.

If the expectation clearly falls outside the original purpose, explain the mismatch and propose a separate skill instead of expanding this one.

Preserve confirmed expectations and their basis in the existing maintenance records, following their display and approval rules. Keep unresolved interpretations provisional.

## Review the whole skill

Read the complete skill, including its description and supporting instructions; inspect referenced resources and dependencies where they affect its behavior.

Check the whole workflow against its purpose and the confirmed expectation: responsibilities, inputs, terminology, action conditions, ordering, dependencies, outputs, and completion criteria.

Trace the observed mismatch to evidence. Distinguish instructions that were not loaded, unclear or conflicting instructions, missing information, and failures to follow existing instructions; separate supported explanations from hypotheses.

Look beyond the reported passage for structural problems, duplication, and stale rules. Full review requires covering the whole skill, not rewriting every part. Resolve questions that affect the revision before drafting it.

## Propose the next version

Revise the skill to address the confirmed expectation and findings while preserving its purpose and unaffected requirements. Delete, combine, reorder, or rewrite as needed; adding instructions is only one option.

Explain how each substantive change addresses a finding and remains consistent with the original purpose. If the evidence cannot support a particular correction, clarify the gap rather than inventing a cause or a rule.

Present the proposed changes in a Markdown table with only two columns, “原文” and “修改后”. Include complete affected passages, leave the corresponding cell empty for additions or deletions, and show new files in full.

Wait for approval of these changes before writing skill files. Confirmation of the expected behavior is not approval of the proposed text.

## Save and report

Reread the affected files before applying approved changes, preserving unrelated edits. If intervening changes invalidate the proposal, revise it and obtain approval again.

Check the saved revision for valid format, usable references, internal consistency, and agreement with the approved text.

Use existing maintenance records to connect the purpose, feedback, revision rationale, and check results without duplicating their contents.

Finish when the approved revision is saved, static checks are complete, and remaining limitations are reported. This workflow stops there: report behavioral improvement as unverified rather than treating a rewritten prompt in a new session as a replay of the original context.
