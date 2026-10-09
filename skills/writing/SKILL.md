---
name: writing
description: Write, revise, and review replies and documents for people and agents.
---

# Writing

## Purpose and cost

**Good writing** is text that faithfully expresses its intended meaning, gives its readers enough information to use it correctly, and keeps avoidable effort low. Its quality depends on the reader and purpose: an explanation must support understanding, a procedure must support action, and a decision record must preserve the choice and its basis. Accurate, sufficient meaning takes priority; brevity improves a text within that boundary.

**Use cost** is the effort required to locate, understand, and apply information. For people, this includes reading, remembering conditions, and connecting separate passages. For agents, **context cost** is the amount of text loaded and retained for a task; **retrieval cost** is the work of locating and reading additional material. Always-loaded text incurs context cost even when irrelevant to the current task, while optional material incurs it when read.

These costs guide tradeoffs. Keeping everything together reduces switching but can increase irrelevant reading; separating material narrows each reading task but adds lookup work. A shorter text may require more guessing or repeated retrieval. Preserve the effort needed for meaningful judgment, and reduce the extra work caused by unclear wording, missing premises, or scattered information.

## Expression and organization

Readers bring general knowledge, but the definitions, choices, and reasons established while drafting may be available only to the author. Supply the premises needed to interpret this text: what its concepts mean, when a rule applies, and why a choice is appropriate. Introduce specialized meanings at first use, beside the rules and qualifications that depend on them. Examples earn their space by clarifying a distinction or showing a condition in use.

In agent instructions, a **leading word** is a compact concept familiar from the model's training, used to guide a related class of behavior. For people reading the same document, that naming choice must also convey an understandable meaning. Prefer familiar, precise names; define local meanings and reuse the names consistently in requests, references, and prose. This connects a recognized need with the relevant guidance and replaces repeated explanations only where the name preserves their meaning.

An instruction needs to identify an action and its conditions. **Positive direction** supplies the desired action directly; a prohibition establishes a boundary but may leave the permitted response unclear. Pair necessary prohibitions with what the reader can do. Distinguish requirements, preferences, and examples so explanatory material carries its intended authority.

A **completion criterion** is the condition used to decide that work is finished. **Clarity** concerns whether it can be checked; **demand** concerns the coverage and depth it requires. "Check one changed item" and "check every changed item" can both be clear while requiring different work. State the necessary scope and evidence of completion. A set of rules can likewise require each applicable rule to be considered.

When different situations require different material or actions, each distinct situation is a **branch**. Creating and deleting a record, for example, may share terminology while requiring different checks. A **pointer** is a reference that identifies additional material and states when to read it: "For deletion, read the removal checks in removal.md." The reader sees the pointer before the target, so it must provide enough information to judge relevance. Lead with the recognizable concept, express each distinct trigger once, and give the detailed explanation in the target.

**Information hierarchy** places content according to when it is needed: immediate steps, reference within the current file, or reference reached through a pointer. Steps order actions; reference supplies definitions, rules, and facts. A document may contain either or both; rules of equal importance can share a level. **Progressive disclosure** places conditional reference behind pointers. Keep essentials needed by every branch in the current file; move substantial branch-specific material behind a pointer when the reduction in irrelevant reading justifies the lookup cost. Each branch's reading path, the passages and references needed for that situation, must include its required context.

Within a file, **co-location** groups a concept's definition, rules, reasons, and exceptions so readers can interpret it without assembling scattered fragments. Connected paragraphs and Markdown should make these relationships visible. Maintain each meaning in one authoritative place and reference it elsewhere, keeping revisions consistent. Even relevant, unique content can become unwieldy through length: this is **sprawl**. Reconsider its distribution by branch or stage, weighing clearer reading against additional navigation and context costs.

## Check and refine

Review from the intended reader's position, using the document and its accessible references. Can the reader identify the purpose, resolve its concepts, find applicable conditions, and make the required judgment or action? Preserve scope, obligation strength, and authorization when revising; distinguish facts, decisions, and inferences, and identify deliberate changes to meaning.

When reviewing a text, apply the relevant principles across the requested scope. The review is complete when that scope has been covered, findings are grounded in text or observed results, and unresolved behavioral questions are identified.

**Pruning** removes content that contributes no necessary meaning or behavior. Check whether each passage still serves the current purpose; remove repetition and stale material, and relocate detail serving another branch. Facts easily obtained from configuration, files, or commands already have an authoritative source. Record the conventions, reasons, and pitfalls those sources cannot explain; copy facts only when saved lookup work justifies keeping the copy current.

A **no-op instruction** adds no useful behavior beyond the model's default for the task. When reviewing agent instructions, examine each sentence within the review scope for possible no-ops. Treat suspected cases as candidates for removal; when uncertain, compare realistic use with and without the instruction. Delete an ineffective sentence as a whole. Explanations of task-specific concepts and decisions remain necessary when their removal leaves the reader guessing.

**Premature completion** means treating a stage as finished before its completion criterion is satisfied. First inspect the criterion and missing premises. One hypothesis is that visible later instructions encourage a transition before the current stage is complete. If the boundary remains inherently difficult to specify and premature completion persists, consider isolating the current stage from later instructions. This requires a separate execution context receiving only the current task and necessary information; moving already-read steps between files leaves them visible. Test this adjustment against observed completion. Likewise, treat claims about wording directing model attention as hypotheses to verify; retain adjustments that improve the required outcome.
