---
name: structure
description: Maintain standard files for goals, guiding judgments and decisions, verified results, and shared terminology when information must persist across sessions or an existing goal is being continued.
---

## Definitions

- A **goal** is a desired end state or maintained state, specified with the scope, constraints, and conditions needed to judge satisfaction.
- **Context** is the established judgments and decisions that guide how goals are pursued across sessions.
- **Progress** is an evidence-backed record of results achieved toward a goal, with each claim bounded by its recorded scope and conditions.
- A **glossary** records term meanings that subsequent sessions need to interpret consistently across the workspace.

## Structure

```text
<workspace>/
  CONTEXT.md
  GLOSSARY.md
  goals/<goal-name>/
    GOAL.md
    CONTEXT.md
    PROGRESS.md
```

This skill manages only the record files shown above; its approval requirements apply only to changes to those files.

Root Context and Glossary are shared across the workspace; goal records apply to one goal. Create Context, Progress, and Glossary when content qualifies; goal-specific records require an approved goal.

## Workflow

Read root Context if present and consult relevant Glossary entries. Follow applicable judgments and definitions; propose revisions when their basis or applicability changes.

When the user selects or corrects a method, constraint, or preference, apply Recording checks before closing the turn. For qualifying Context, present the proposed record and its scope for approval under Changes in the same response.

Check existing goals before deciding whether to create one:

1. Compare the request with `goals/*/GOAL.md` by desired state, scope, constraints, and satisfaction conditions, including any goal the user specifies. Reuse goals that clearly cover it. If a related request cannot be handled under the existing definitions, explain the mismatch and clarify the intended definitions with the user before proceeding. Use what the user has already clarified and ask only about unresolved points.
2. Propose a new goal only when the request is unrelated to existing goals and needs continuing tracking. Clarify new or revised definitions from confirmed information and necessary read-only investigation until independently understandable and judgeable; follow Changes to create or revise the affected `GOAL.md` files.
3. Read and state the applicable goals; read their Context and Progress. Follow applicable Context; propose revisions when its basis or applicability changes. If no goal applies and no continuing tracking is needed, handle the request directly.
4. Verify recorded results before treating them as current facts; correct inaccuracies and retain results with continuing reference value.
5. Use Recording checks, Formats, and Changes to maintain records.

## Recording checks

Split mixed content into claims and route each by purpose; leave unmatched claims unrecorded.

- Would omission materially hinder later understanding, decisions, actions, or verification? If not, omit it.
- Does it define the desired state, scope, constraints, or satisfaction conditions? Propose a Goal change.
- Does it express an established judgment or choice that later sessions need, with a basis and applicable scope? Propose Context.
- Does it record an achieved result with evidence and verification conditions? Record Progress.
- Does a sourced or agreed term definition prevent ambiguity or repeated explanation in later sessions? Propose Glossary.

Keep the latest valid record for each subject and scope, with its basis, evidence, and limits. On each update, reconcile related entries and replace superseded or duplicate content under Changes; reference existing sources.
Retain verified intermediate results only while needed to continue the work; consolidate them into the current result when superseded.

## Formats

`GOAL.md`: require `# <goal name>`, `## Goal` (state and scope), and `## Satisfied when` (criteria). Add `## Why` and `## Constraints` when applicable. Satisfying a finite goal completes it; continuing goals remain active.

Use these templates for Context, Progress, and Glossary; statements or definitions and Basis/Evidence are required. Add Scope or other qualifiers when they affect interpretation.

```markdown
- **<Judgment or decision>**
  - Basis: <reasoning and supporting sources>
  - Scope: <where the conclusion applies>

- **<Achieved result>**
  - Evidence: <source or reproducible check>
  - Scope: <verification conditions and limits>

- **<Term>**: <Definition>
  - Basis: <source or confirmed agreement>
```

## Changes

Before creating, editing, renaming, or deleting `GOAL.md`, `CONTEXT.md`, or `GLOSSARY.md`, show the full proposed file when absent or the exact diff when present, then wait for approval. Apply only approved content; if it no longer applies, reread and propose again.
Glossary wording and formatting edits that preserve meaning need no prior approval.
`PROGRESS.md` needs no prior approval. After changing it, name the goal and summarize the result.
