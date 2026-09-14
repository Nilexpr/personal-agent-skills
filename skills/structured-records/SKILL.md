---
name: structured-records
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

Root Context and Glossary are shared across the workspace; goal records apply to one goal. Create Context, Progress, and Glossary when content qualifies; goal-specific records require an approved goal.

## Workflow

Read root Context if present and consult relevant Glossary entries. Follow applicable judgments and definitions; propose revisions when their basis or applicability changes.

When the user selects or corrects a method, constraint, or preference, apply Recording checks before closing the turn. For qualifying Context, append a concrete proposal within its confirmed scope to the normal response, using the full draft or exact diff required by Changes. Obtain the required approval before writing.

Handle requests needing no continuing tracking directly, using applicable record rules. Otherwise:

1. Select the specified goal or match the request against `goals/*/GOAL.md`; ask the user when ambiguous.
2. If none matches or a separate goal is requested, clarify its definition from confirmed information and necessary read-only investigation until independently understandable and judgeable; follow Changes before pursuing the goal.
3. Read and state the selected goal; read its Context and Progress. Follow applicable Context; propose revisions when its basis or applicability changes, or the request changes the goal.
4. Verify recorded results before treating them as current facts; correct inaccuracies and retain results with continuing reference value.
5. Use Recording checks, Formats, and Changes to maintain records.

## Recording checks

Split mixed content into claims and route each by purpose; leave unmatched claims unrecorded.

- Would omission materially hinder later understanding, decisions, actions, or verification? If not, omit it.
- Does it define the desired state, scope, constraints, or satisfaction conditions? Propose a Goal change.
- Does it express an established judgment or choice that later sessions need, with a basis and applicable scope? Propose Context.
- Does it record an achieved result with evidence and verification conditions? Record Progress.
- Does a sourced or agreed term definition prevent ambiguity or repeated explanation in later sessions? Propose Glossary.

Keep the minimum sufficient statement and its basis; reference existing sources and remove redundant or no longer useful content.
Record verified intermediate results only when later sessions need them to continue the work. When a final result is established, consolidate supporting evidence and remove intermediate records that no longer have independent reference value.

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

The following approval rules are defaults; user instructions take precedence.

Before creating, editing, renaming, or deleting `GOAL.md`, `CONTEXT.md`, or `GLOSSARY.md`, show the full proposed file when absent or the exact diff when present, then wait for approval. Apply only approved content; if it no longer applies, reread and propose again.
Verify that persisted content matches the approved version.
Glossary wording and formatting edits that preserve meaning need no prior approval.
`PROGRESS.md` needs no prior approval. After changing it, name the goal and summarize the result.
