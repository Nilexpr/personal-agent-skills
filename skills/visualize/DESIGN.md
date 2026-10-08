# Layout and interaction

## Default styles

For a standalone page without an existing design system, copy [visualize.css](assets/visualize.css) into the deliverable and link that local copy. Keep the output independent of the skill's installation path. This bundled stylesheet provides theme variables, buttons, forms, layout utilities, and tables; no Tailwind build is required.

Use semantic native elements with classes such as `.btn`, `.btn-primary`, `.form-control`, `.form-range`, `.form-select`, `.viz-row`, `.viz-grid`, and `.table`. Use `.card` only for meaningful bounded groups. Colors use variables such as `--background`, `--foreground`, and `--primary`; the document can set `data-theme="light"` or `data-theme="dark"` to choose a theme.

The stylesheet contains global document rules. For embedded components, prefer the containing project's styles instead of importing it globally. CSS supplies appearance only: implement tab switching, tooltips, and other behavior using browser features or existing components; chat-provided helpers are not included.

## Composition

- Organize each explanation around a clear question, a dominant visual, and the explanation relevant to its current state. Place the question before the visual; keep controls and feedback next to what they affect. A component may receive its question and surrounding prose from the containing page.
- Connect overview and detail: expanding a subject or level keeps its place in the whole recognizable. Encode relationships through meaningful labels, position, grouping, or other explained visual conventions. Each relationship should communicate more than the existence of a connection.
- Use side-by-side visual and explanation areas when readers need to consult both together. Stack them in reading order on narrow screens, keeping local controls with their visual. Use matching scales and encodings for comparisons; preserve simultaneous views when comparison requires them.
- Introduce substantial sections in a meaningful sequence. Add navigation when needed for orientation, and show the current section or step. Keep definitions, sources, and longer derivations reachable as supporting detail; essential reasoning stays visible.
- Use spacing and alignment to show hierarchy and grouping. Add panels only when they separate meaningful tasks or information. Follow the containing project's visual language; this guide specifies no font sizes or fixed page template.

## Interaction

- Choose actions that answer the current question: inspect an object, compare cases, change a condition, or advance a process. Make the available action and its purpose visible; do not rely on hover to reveal essential controls.
- Start in a meaningful, labeled state. After an action, visibly identify what changed and why; keep the diagram, values, and explanation synchronized. Explain unavailable actions and their prerequisites.
- For a step-through, show the current step and support returning or replaying where useful. Offer a way back to a known state when experimentation would otherwise lose the starting point. Animations illustrate transitions and respect reduced motion.
- Open definitions or details without discarding current inputs, selections, or position. Give dismissible content a clear close or return action and restore focus appropriately. Keep selections distinguishable from hover and keyboard focus.
- Let readers inspect the supporting material for a subject, relationship, or change and return to the same exploration state. Keep qualifications and uncertainty visible where they affect interpretation, including after switching views or conditions.
- Use labeled native controls with keyboard access. Pair visual encodings with text or shape, and keep content usable on narrow screens. Scope component interactions to their instance so embedding does not take over the surrounding page.
