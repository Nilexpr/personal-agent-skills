---
name: structure
description: Maintain GOAL.md, DECISIONS.md, PROGRESS.md, and GLOSSARY.md for cross-session work. Use for goal-related work (including discussions), user choices or corrections affecting later work, or record maintenance.
---

# Structure

Structure preserves what a later session needs to continue work: intended outcomes, choices that guide action, established results, and shared meanings. Keeping these roles separate helps readers distinguish an expectation, an agreed approach, evidence of what happened, and the meaning of a term.

## Records

The record root is this conversation's `cwd`. Keep record paths relative to it.

```text
<root>/
  DECISIONS.md
  GLOSSARY.md
  goals/<goal-name>/
    GOAL.md
    DECISIONS.md
    PROGRESS.md
```

- A **goal** is a desired end state or maintained state, specified with the scope, constraints, and conditions needed to judge satisfaction. `GOAL.md` requires `# <name>`, `## Goal` (state and scope), and `## Satisfied when` (criteria); add `## Why` and `## Constraints` when applicable. Finite goals complete when satisfied; continuing goals remain active.
- **Decisions** are established judgments and confirmed choices that subsequent work should follow, recorded with their basis and applicable scope. Each entry requires the judgment or choice and `Basis` explaining why. Use root Decisions for shared guidance, otherwise the goal's Decisions.
- **Progress** is an evidence-backed record of results achieved toward a goal, with each claim bounded by its recorded scope and conditions. Each entry requires an achieved result and `Evidence` providing a source or reproducible check.
- A **glossary** records term meanings that subsequent sessions need to interpret consistently under the record root. Each entry requires the term, its definition, and `Basis` identifying its source or confirmed agreement. Include necessary distinctions or relationships, including common terms when useful.

Include scope and limits where they affect interpretation.

Classify each claim by its role. A requirement defining goal satisfaction belongs to Goal; a choice about pursuing it belongs to Decisions. For example, choosing an installation method is a decision, while confirming a particular installation succeeded is progress. One event can produce both claims; preserve their connection while recording each meaning once.

## 1. Read shared records

Read applicable root Decisions and relevant Glossary entries even for one-off requests.

## 2. Select the goal

Standalone shared-record edits need no goal. For goal-related work:

- Compare the request with existing goals by desired outcome, scope, constraints, and satisfaction conditions; reassess the match as the work develops.
- Reuse a goal for work toward the same outcome. Propose a separate goal when the work has an independently judgeable outcome and needs continued tracking, even when it supports an existing goal or shares its project.
- Make proposed goals independently understandable and judgeable. Clarify unresolved boundaries before work that depends on them.

State applicable goals and read their relevant Decisions and Progress, even for small tasks.

## 3. Verify and clarify

Recheck results this task relies on against recorded conditions; correct inaccuracies. Follow established judgments and confirmed choices; propose revisions when their basis or scope changes.

When ambiguity or conflicting usage affects records, clarify meanings using examples, edge cases, or source checks as needed. Distinguish hypothetical cases from facts and use consistent names. A definition is ready when relevant ambiguities are resolved, related concepts are distinguished, and its source or confirmed agreement is clear; otherwise keep it provisional.

## 4. Select what to preserve

Every retained claim adds future reading, verification, and maintenance work. Keep information whose omission would materially hinder later work, and retain the basis needed to interpret it. Where accessible sources already carry the detail, preserve the conclusion, its scope, and a source reference.

Split mixed claims and route each by its purpose using the [record definitions and requirements](#records). Evaluate judgments and choices separately from artifacts and results; an artifact's form alone does not preserve intent. Cross-reference existing records instead of duplicating them.

## 5. Propose and apply changes

Create qualifying records; goal-specific records require an approved goal. Reconcile affected entries according to what changed:

- A new confirmed choice replaces earlier guidance for the same subject and scope; retain the basis needed to understand the change.
- A result verified for an earlier version remains historical evidence for that version. Recheck claims about the current version and state any verification still needed.
- Intermediate results are consolidated into the final result, with links to evidence that still matters for continuation or verification.
- Repeated meanings are maintained in one authoritative location and referenced elsewhere.

This keeps current guidance easy to find while preserving evidence within the conditions under which it was established.

These rules cover only records. Before changing records, show the changes in a two-column Markdown table: original text on the left, revised text on the right. Preserve complete affected passages; show the full content for a new file. Leave the corresponding cell empty for additions or deletions.

For Goal, Decisions, or Glossary, obtain approval, including renames and deletions. Existing approval covering the change waives asking again, but not showing the changes.

When approval is needed, ask directly without routine policy explanations or quotations. Repropose approved content if it no longer applies. Meaning-preserving Glossary edits and Progress updates need no approval. Continue independent work pending clarification or approval.

## 6. Check and report

A scoped proposal identifies the target record and proposed content while approval or application is pending. It becomes a saved update after any required approval is obtained and the change is applied.

Maintenance for the current turn is complete when qualifying information has applicable records or scoped proposals, authorized changes are saved, and affected records agree. Check each new or corrected judgment or choice is explicitly captured with its meaning and scope. Report saved updates, pending proposals, goal results, and unresolved matters separately.
