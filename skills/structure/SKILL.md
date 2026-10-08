---
name: structure
description: Maintain GOAL.md, DECISIONS.md, PROGRESS.md, and GLOSSARY.md for cross-session work. Use for goal-related work (including discussions), user choices or corrections affecting later work, or record maintenance.
---

## Records

- A **goal** is a desired end state or maintained state, specified with the scope, constraints, and conditions needed to judge satisfaction.
- **Decisions** are established judgments and confirmed choices that subsequent work should follow, recorded with their basis and applicable scope.
- **Progress** is an evidence-backed record of results achieved toward a goal, with each claim bounded by its recorded scope and conditions.
- A **glossary** records term meanings that subsequent sessions need to interpret consistently under the record root.

```text
<root>/
  DECISIONS.md
  GLOSSARY.md
  goals/<goal-name>/
    GOAL.md
    DECISIONS.md
    PROGRESS.md
```

## 1. Read shared records

Keep all record paths relative to this conversation's `cwd` (`<root>`). Read applicable root Decisions and relevant Glossary entries even for one-off requests.

## 2. Select the goal

Standalone shared-record edits need no goal. For goal-related work, reuse a known goal when its definition covers the request; otherwise compare existing goals by state, scope, constraints, and satisfaction conditions. Clarify unresolved mismatches before dependent work. Propose new goals only for unrelated work needing continued tracking; make definitions independently understandable and judgeable. State applicable goals and read relevant Decisions and Progress, even for small tasks.

## 3. Verify and clarify

Recheck results this task relies on against recorded conditions; correct inaccuracies. Follow established judgments and confirmed choices; propose revisions when their basis or scope changes.

When ambiguity or conflicting usage affects records, clarify meanings using examples, edge cases, or source checks as needed. Distinguish hypothetical cases from facts and use consistent names. A definition is ready when relevant ambiguities are resolved, related concepts are distinguished, and its source or confirmed agreement is clear; otherwise keep it provisional.

## 4. Select what to preserve

Keep only information whose omission would materially hinder later work. Split mixed claims:

- Goal: satisfaction requirements.
- Decisions: judgments and choices with their basis; use root Decisions for shared guidance, otherwise the goal's Decisions.
- Progress: achieved results and evidence.
- Glossary: concise meanings and necessary distinctions or relationships, including common terms when useful.

Evaluate judgments and choices separately from artifacts and results; an artifact's form alone does not preserve intent. Cross-reference existing records instead of duplicating them.

## 5. Propose and apply changes

Create qualifying records; goal-specific records require an approved goal. Reconcile affected entries, replace superseded or duplicate content, and consolidate intermediate results while retaining evidence with continuing value.

These rules cover only records. For Goal, Decisions, or Glossary, show the full new file or exact diff and obtain approval, including renames and deletions; existing approval for that content suffices. Present the proposal and question without routine policy explanations or quotations. Repropose approved content if it no longer applies. Meaning-preserving Glossary edits and Progress updates need no approval. Continue independent work pending clarification or approval.

## 6. Check and report

Record maintenance is complete when qualifying information has appropriate records or scoped proposals, authorized changes are saved, and affected records agree. Check each new or corrected judgment or choice is explicitly captured with its meaning and scope. Report changes, goal results, and unresolved matters.

## Formats

Goal: `# <name>`, `## Goal`, `## Satisfied when`; add `## Why` and `## Constraints` when applicable. Finite goals complete when satisfied; continuing goals remain active.

Decisions: judgment or choice and `Basis` explaining why. Progress: result and `Evidence` providing a source or reproducible check. Glossary: term, definition, and `Basis` identifying its source or confirmed agreement. Include scope and limits where interpretation depends on them.
