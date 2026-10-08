---
name: visualize
description: Turn complex material into explorable visual models of its subjects, relationships, and changes. Deliver interactive browser pages or reusable components, independently or for another skill.
---

# Visualize

Make supplied material easier to understand through meaningful visual interaction while preserving its meaning and qualifications. A visual model represents the subjects, relationships, properties, and changes relevant to the user's question, with traceable supporting evidence. Its form follows the material rather than a fixed template.

## 1. Establish the understanding task

Read the material and relevant context to identify what the user needs to understand. Set the scope and level of detail around that question; users need not prescribe diagrams or controls. Inspect underlying sources when needed to establish meanings or mechanisms. Identify gaps that prevent a reliable explanation instead of inventing missing content.

Use `structure` to consult relevant `GLOSSARY.md` entries under the caller's record root and keep shared meanings consistent. Identify necessary concepts while reading; users need not prepare a complete glossary. Resolve missing or conflicting definitions through `structure` and its record-change rules before relying on them.

## 2. Build an evidence-backed model

Identify the subjects, relevant properties, and meaningful levels of detail. Derive their grouping from the material. Establish what each relationship means, its direction where applicable, and the conditions under which it holds. Shared wording alone does not establish identity or causation.

Where change matters, identify states, events, and the conditions or rules connecting them. Account for the substantive information within scope, including exceptions and uncertainty. Distinguish direct evidence, interpretation, and explanatory simplification. Keep represented relationships and changes traceable to their supporting material.

## 3. Design observation and interaction

Choose representations and actions that answer the understanding task: reveal structure, follow relationships, inspect detail, compare situations, or observe change. Make each interaction produce a useful observation. Keep static content where manipulation adds nothing; offer computed alternatives only when supported by explicit rules.

Connect the overview to details without losing orientation. Maintain subject identity across views. Organize explanations around the current selection or state, showing what to notice and why it matters. Distinguish observed events, inferred sequences, and simulated outcomes; scripted animation is not evidence that a process actually occurred.

Read [Layout and interaction](DESIGN.md) for composition, controls, and default styles.

## 4. Implement a browser artifact

Reuse the caller's project conventions. Choose native browser tools or a framework according to implementation and integration effort; builds and local servers are acceptable. Deliver a standalone page or an embeddable component without chat-host dependencies.

For components, expose necessary inputs and outputs, isolate instance styles and state, and clean up resources on removal. Supply a usage example or preview. When both forms are needed, let the page host the same component.

## 5. Verify and deliver

Reconcile the visual model, explanations, and interaction results with the material and glossary. Verify that grouping, direction, conditions, and uncertainty retain their meaning; use evidence appropriate to the claims.

Open the artifact in a browser. Exercise its main interaction and a meaningful alternative or boundary case; check keyboard access, narrow layouts, runtime errors, and component integration when applicable. Report unperformed checks as unverified.

Return the artifact, opening or integration instructions, verified behavior, and remaining limits. Completion means an accurate, usable explanation. Callers such as `learn` retain responsibility for teaching plans and learning assessment.
