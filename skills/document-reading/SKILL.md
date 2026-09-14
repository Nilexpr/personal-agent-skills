---
name: document-reading
description: Design, implement, or review HTML reading interactions for technical documents, reference manuals, tutorials, and long explanations, especially concept lookup, cross-references, scoped highlighting, and returning to the reading position. Use for document reading behavior, not general site styling or teaching plans.
---

# Document reading

Improve understanding within the text and preserve continuity when consulting supporting material.

## Workflow

1. Inspect the maintained content, generator, semantic markup, navigation, and representative reading journey. Identify where readers lose a concept, relationship, or position; retain the existing authoring pipeline. Finish with concrete reading problems and their source locations.
2. Establish the semantic contract below before adding interactions. Resolve ambiguous meanings from the content or its author; leave uncertain relationships unasserted.
3. Read [interaction patterns](references/interactions.md) to select the smallest useful set. For each selected capability, state its location, action, visible effect, reading benefit, and dismissal or return behavior. A short document may need only semantic markup and ordinary links. When comparing with ECMA-262, consult [source evidence](references/ecma-evidence.md); distinguish observed mechanisms from proposed improvements.
4. Implement in the existing components or generator, using native HTML and small local CSS/JavaScript where sufficient. Keep essential explanations in reading order. Add controls only after their enhancement initializes; retain functional links and readable definitions without JavaScript. Treat imported content as data and use the project's safe rendering path.
5. Run the selected journeys using [verification](references/verification.md). Verify destinations and return positions, not merely event handlers or screenshots. Report implemented choices, checks actually run, and remaining limits; static review alone does not establish working interaction.

## Semantic contract

- Give each definition a stable ID and canonical location; keep summaries with that source. Link explicit occurrences to it, recording each occurrence's ID, owning section, target, and relationship such as “defines,” “uses,” or “illustrates.” Derive backlinks from those forward links, counting occurrences separately from sections.
- Use explicit scope and symbol IDs for variable associations. A same-name fallback may only claim text matches within a declared container; distinguish shadowed bindings and unrelated names. Generate indexes with content updates and detect duplicate IDs, missing targets, and stale summaries.
- Map typography to roles: headings express hierarchy, `strong` importance, `em` emphasis, `dfn` definitions, `var` variables, and `code` literal syntax. Domain notation may need separate classes and a legend. Give links a recognizable treatment; retain distinctions beyond color. For Chinese prose, prefer clear labels and spacing over extensive synthetic italics.
- Specify position restoration for each lookup: source occurrence, page/anchor, viewport offset, and initiating focus. Closing a preview restores its trigger without moving the text; following a destination preserves a usable return path. Use session state only when navigation needs it, and durable storage only for deliberate reader preferences.

## Example

For a minimal implementation of preview, backlinks, scoped variables, and return navigation, inspect [the standalone example](assets/reading-example.html). It is an adaptable reference, not a required page template; its reproducible checks are in [verification](references/verification.md).
