# Verify reading behavior

Derive checks from the selected interactions and the document's real content. Use the supported browser automation already available in the project; record URL/file, environment, viewport, actions, assertions, and result. Mark unavailable checks as unverified rather than substituting source inspection for user actions.

## Content and generation

- Trace a definition through two uses in one section and one use elsewhere. Verify three distinct occurrence destinations and two groups; navigation and preview duplicates must not inflate counts.
- Rename a heading while preserving its identity, then add and remove a use in the maintained input and regenerate. Verify anchors, summaries, search entries, and backlinks stay synchronized. Missing targets and duplicate IDs should be surfaced by the generator/checker, not silently linked to guessed content.
- Use the same variable spelling in adjacent algorithms and an inner shadowing scope. Toggle an outer occurrence and assert exactly the intended binding is selected; test a second symbol and clearing independently.

## A reading journey

1. Begin midway through a paragraph and record its anchor/occurrence, viewport position, and focused control.
2. Open the explanation by click, then repeat with keyboard and touch. Read the definition and relation label; dismiss with the close control and Escape. Assert the source remains visible and focus returns to the initiating control.
3. Follow the complete-definition link, open its backlinks, and select a second occurrence in the same section. Assert the actual landing element and visible marker, not just a changed hash. Return through the provided path and browser Back/Forward; verify both position and focus. Repeat across pages if supported, including reload and unavailable storage when state is persisted.
4. If search exists, check name, section number, no match, result selection, and the declared search scope. Type shortcut characters into editable controls and use IME composition; verify no document command interrupts input.
5. Repeat relevant actions at a narrow viewport and zoom; inspect clipping and the space left for text. Check keyboard order, accessible names and expanded/pressed states, dismissal, and that navigation does not trap focus. Screen-reader testing is separate from DOM/ARIA inspection.
6. Read with JavaScript disabled, then inspect print and reduced-motion modes. Definitions, essential reasoning, code, and links must still be usable. Copy representative steps or notation if applicable. Check console errors and missing resources.

Report passed, failed, and unperformed journeys separately. These checks establish behavior under recorded conditions, not improved comprehension; actual readers must validate that claim.

## Reproduce the bundled example

Open `assets/reading-example.html` directly in a browser; no server, framework, or installation is needed. It demonstrates an authored paragraph, canonical definition, three explicit references in two sections, outer/inner variable scopes, native popovers, generated backlinks, and an in-session return stack. It intentionally has no search, persistence, pagination, or arbitrary nested previews.

For browser automation, open the example using the project's supported local preview and browser/CDP session. Some browser tools prohibit `file:` URLs; use their supported local-web workflow with only this example exposed. A standalone browser installation is not required by this skill. Preserve the caller's tool and installation constraints.

Reproduce the example checks through actual controls:

| Action | Expected result |
| --- | --- |
| Open the first Explain control, then close it | Preview text equals the canonical definition; focus and the source control's viewport offset are restored. |
| Activate Explain with Enter, reach the full-definition link with Tab, then press Escape | The link is keyboard reachable; dismissal returns focus to Explain. |
| Follow the full definition, select the second backlink, then use Return and browser Back/Forward | Each destination receives focus; the second occurrence is visible below navigation; returning restores the original control and offset. |
| Toggle outer `saved`, then `next`, then clear `saved` | Two outer saved occurrences are selected initially; inner and unrelated saved stay clear; next's two selections survive clearing saved. |
| Open and close a preview at a 390-pixel viewport and with touch input | No horizontal document overflow or coverage of the source; touch operates the visible controls. |
| Disable JavaScript and reload; follow a concept link | Explanations, all variable text, and ordinary definition links remain; enhancement-only controls are absent. |
| Emulate print | Definitions and variable text remain visible; interactive navigation and preview controls are hidden. |

Record which rows were actually exercised. Browser tooling may support narrow viewports without touch dispatch, or show a no-script document without being able to operate its links; report those as separate results. These checks do not establish screen-reader speech, cross-browser compatibility, integration with another generator, or improved comprehension.
