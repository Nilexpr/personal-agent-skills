---
name: visual-explaining
description: Explain supplied material through visual models and meaningful interaction. Use for interactive explanations, observable processes, or visual exploration of concepts and relationships.
---

# Visual explaining

Draft scaffold: the responsibilities and initial guidance are established; examples and teaching behavior remain unvalidated.

## Purpose

Lower the effort required to understand supplied information through visual representation. Actively make content interactive wherever manipulation, comparison, or exploration helps understanding. Preserve the original meaning and complete coverage within the requested scope, including conditions, exceptions, uncertainty, and limitations.

## Input

Use supplied, verified material and its source references. Establish the explanation's scope, audience, and intended understanding from the request. Return missing or ambiguous premises to the user or calling workflow for clarification; this skill does not perform independent subject research.

## Workflow

1. Map the source material to a visual model using [Modeling](references/modeling.md). Account for each substantive source point and identify what the learner can observe or manipulate.
2. Organize the explanation around that model: orient the learner, introduce actions or comparisons, explain their effects, and connect the result to the original question. Introduce detail as needed while keeping supporting information accessible.
3. Build the explanation using [Implementation](references/implementation.md), selecting interactions and tools that reduce learning and maintenance effort.
4. Apply [Verification](references/verification.md). Deliver the explanation, how to open or run it, its limits, and the checks actually performed.

## Collaboration

Work independently on a supplied explanation, or within a learning workflow. When used with `learn`, receive its verified material and learning outcome, and return the artifact plus shared learner attempts and unresolved questions for its teaching loop. Course planning and continuing learning records remain with the calling workflow.

For the design basis or community implementation references, read [Sources](references/sources.md).
