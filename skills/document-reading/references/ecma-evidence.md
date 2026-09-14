# ECMA-262 reference evidence

Use this when explaining the design's relationship to ECMA-262 or rechecking a mechanism before borrowing it. These observations are evidence for options, not requirements for other documents.

## Sources and verification boundary

Primary reference: [Block](https://tc39.es/ecma262/#sec-block), also available in the official [statements and declarations multipage chapter](https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-block).

On 2026-09-14, the multipage chapter was read through web retrieval, and the following official assets were downloaded and inspected as text. The query string identifies the requested URL, not an immutable version; use hashes to distinguish later responses.

- [Reading script](https://tc39.es/ecma262/assets/js/ecmarkup.js?cache=xCscAdeW), SHA-256 `5ab8e90b799c7c016cd1557aada6b278d9556b45cdf83b6896c87266a631ce97`.
- [Reading styles](https://tc39.es/ecma262/assets/css/ecmarkup.css), SHA-256 `8bef2688107197ac28abe81b62a61100904cec548e223d03a10ac7ea7b6b2fc7`.

The configured Chrome MCP could not connect. A separate headless probe could not start because the Playwright-required browser executable was absent. Thus this verification establishes source behavior and generated index data, not a fresh live-page interaction test. Earlier investigation supplied in the task is a research lead, not a substitute for current evidence.

## Mechanisms confirmed in the downloaded assets

| Reading mechanism | Source locator | What the implementation establishes |
| --- | --- | --- |
| Name/section lookup | `Search.prototype.search`, `getKey`, `fuzzysearch`, `selectResult` | Navigation search matches section-number prefixes or fuzzy index keys; a single character gives no results, the list is capped at 50, and Enter selects the first result. It is not general full-text search. |
| Current section in contents | `findActiveClause`, `revealInToc` | Scrolling updates the section and ancestors, marks the leaf, and adjusts the directory's own scroll position to reveal it. |
| Definition/heading tools | `Toolbox.init`, `findReferenceUnder`, `updatePermalink` | The floating tools expose an ordinary Permalink anchor, a Pin action, and References. Pin adds a navigation bookmark; it does not hold a definition popup open. |
| Reverse references | `referencePane.showReferencesFor`, `dragStart` | Reference occurrence IDs are mapped to sections and grouped; repeated uses get separate links, and pointer dragging changes panel height. |
| Local variable tracking | `findContainer`, `findLocalReferences`, `toggleFindLocalReferences`, `installFindLocalReferences` | Clicking a `var` matches its HTML content among variables below the nearest clause container; seven highlight classes are available and a second click clears that name. Descendants are queried, so this is not binding analysis and may include nested content. |
| Syntax-to-operation lookup | `sdoBox`, `referencePane.showSDOs` | Pointer entry on a grammar alternative exposes a Syntax-Directed Operations link; relation data drives the operations list and destinations. |
| Shortcuts and state | `doShortcut`, `initState`, `persistPinEntries`, `beforeunload` | `u`, `e`, `?`, and `m` dispatch annotation/help/mode actions; that handler guards editable targets and modifiers. Pins use local storage; the reference panel, directory, search, and directory scroll have session restoration. Other shortcut handlers need their own input checks. |
| Typography by role | `var`, `var.field`, `emu-nt`, `emu-t`, `emu-const`, `emu-val`, `code`, `emu-note` | Variables, nonterminals, terminals/code, specification values/constants, fields, and notes have different treatments. Styles also include system dark appearance and local overflow rules. |

Parsing the script's generated bibliography found four reference IDs for `sec-blockdeclarationinstantiation`, in three sections: block evaluation, switch evaluation, and the legacy block-level function section (two occurrences). This checks the count without activating the panel. The earlier `oldEnv` counts were not independently repeated and are not assumed as an invariant.

## Adaptation choices

The generic skill preserves explicit concept links, occurrence-based backlinks, scoped tracking, semantic typography, and reading continuity. It adds visible keyboard/touch entry points, dismissal and focus behavior, and explicit binding IDs where warranted. The inspected variable installer is click-based; no general definition-preview mechanism was established in this inspection. The bundled popover example is an original design choice, not a claim about ECMA-262.

Optional implementation-navigation links in the [official ecmarkup repository](https://github.com/tc39/ecmarkup): `js/menu.js`, `js/sdoMap.js`, `js/multipage.js`, `js/listNumbers.js`, `js/superscripts.js`, and `css/elements.css`. Fetch their current contents if a future task needs to reuse their implementation; the maintained source files were not retrieved successfully in this run. Likewise, [print.css](https://tc39.es/ecma262/assets/css/print.css) is a follow-up source for print details, not a verified print/PDF result here.
