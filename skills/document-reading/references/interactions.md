# Interaction patterns

Select by the reading problem and available semantic data. The following behaviors are design options, not a checklist to add to every document.

## Understand a concept without losing the sentence

At an explicit concept reference in the paragraph, keep an ordinary link to its definition. If repeated detours interrupt reading, add an adjacent “Explain [concept]” button; click, tap, or keyboard activation opens a short sourced explanation with clearly named relationships and a link to the complete definition. This answers “what does this mean here?” without replacing the surrounding argument.

Use an inline region, nonmodal popover, or panel suited to the available space. Opening it should not unexpectedly reposition the source. Provide a visible close control and Escape; restore initiating focus on dismissal. If the preview contains links, make them keyboard reachable and keep it available while the pointer moves into it. Hover may supplement the visible control, never be its only entry. For long or nested explanations, prefer the full definition and a return path rather than stacked floating windows. An existing native Popover API implementation can handle focus and light dismissal; test the actual browser behavior.

## See where an idea is used

Near the definition heading, offer “Used in (N occurrences)” when reverse lookup answers a real question. Activation opens a list grouped by section; each occurrence has a distinct link and a meaningful snippet or ordinal. Selecting one lands on that occurrence, briefly marks it, and keeps the list accessible for comparing uses. Provide “Return to [source]” when the browsing flow leaves the original place.

Build the list from maintained forward references, excluding navigation and preview copies. A target used twice in one section yields two occurrences and one section. Label relationship types so examples are not mistaken for prerequisites or definitions. For small documents, an ordinary static backlink list is sufficient; a resizable panel earns its complexity only when comparison requires persistent space. If resizing is offered, include a keyboard alternative and preserve readable body space on narrow screens.

## Follow a variable through an explanation

At a variable occurrence inside an algorithm or derivation, provide an operable toggle whose label identifies the variable and scope. Click, tap, Enter, or Space marks occurrences of the same symbol in that scope; a second activation clears them. Readers can follow the value across steps without rereading every line.

Expose the toggle state, scope, and match count in text; pair color with an outline or another visible cue. Independent symbols may remain selected simultaneously when comparison helps. Scope IDs distinguish same-spelled variables in adjacent algorithms and nested bindings. Derive bindings from existing structured input when available; otherwise annotate them explicitly. A container/text matcher must disclose its approximation and keep nested-scope matches out unless intentionally included.

## Find material and retain orientation

For a long hierarchy, place a contents navigator beside the article or behind an operable narrow-screen control. Selecting a heading goes to a stable anchor; reading updates the current section and its ancestors without stealing focus or continually rewriting browser history. Keep the current entry visible without preventing manual directory browsing.

Add search when readers know a name or section number but not its location. Put a labeled input in navigation and state whether it searches headings, indexed concepts, or full text. Results should expose their destination context; selection lands at the matching occurrence or heading and marks it. Provide empty and no-result states. If shortcuts are offered, publish them and ignore input, textarea, select, editable content, IME composition, and modified keystrokes. Support ordinary browser Find even when there is a custom index.

A section permalink is an ordinary shareable link; an explicitly labeled copy button must report clipboard success or failure. Optional bookmarks place chosen destinations in a reader list and provide removal. Save preferences with a document/version namespace and tolerate unavailable storage. Add cross-page state restoration only if the document actually spans pages; do not introduce pagination solely to imitate a reference site.

## Reveal domain relationships

At a structured entity such as a grammar production or API operation, a labeled “Related operations” control may list its typed relations and precise destinations. Activating one lets readers move from syntax to behavior or from an interface to examples. This requires authored or generated relation data; shared spelling does not establish a relationship. Keep domain-specific notation and filters only when they answer questions in this document.

## Keep text readable in every mode

Choose a restrained role-based type system and preserve the original logical order. Important conclusions belong in the main text; optional derivations can be disclosed when their labels explain what is inside. Keep notes, examples, warnings, and normative requirements distinguishable through labels and structure.

On narrow screens or zoom, reposition supporting UI so it does not cover the passage it explains. Confine horizontal overflow to genuinely wide code, equations, or tables. Respect reduced motion for target highlights and scrolling, retain focus visibility in light and dark appearances, and provide print output with definitions, examples, and meaningful links. Copying an algorithm should retain its sequence; verify numbering and special notation if the document depends on them.
