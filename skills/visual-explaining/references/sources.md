# Design references

Reviewed during design on 2026-09-22. These sources inform presentation and implementation; they are not authorities for the subject being taught. Recheck current instructions, compatibility, maintenance, and licenses before reuse.

| Reference | What to consult it for | Boundary |
|---|---|---|
| [Learn Claude Code](https://learn.shareai.run/en/) | Persistent objects, staged actions, visible states, chapter progression and recombination | Preset playback does not establish free simulation or learning; reconcile visuals with their source material. |
| [Red Blob Games: making interactive tutorials](https://www.redblobgames.com/making-of/circle-drawing/) | Mapping variables to diagrams, direct manipulation, concept-specific views, and extracting reusable behavior | Implementation examples need version checks; preserve the distinction between teaching models and actual systems. |
| [visual-explainer](https://github.com/nicobailon/visual-explainer) | Representation selection, labeled relationships, reference HTML and diagram navigation | Its viewing interactions and optional fixed-schema renderer do not supply arbitrary subject-model rules. |
| [Anthropic frontend-design](https://github.com/anthropics/skills/blob/main/skills/frontend-design/SKILL.md) | Concise design judgment, information hierarchy, and presentation review | Design guidance does not provide a teaching model or reusable simulation engine. |

Use these references selectively. Add executable assets or helper scripts only when an implemented example establishes a concrete reuse need.
