# Vertical slices

BioMCP 1.0 grows one entity at a time. A slice takes one entity with its sources from investigation to landed commands. Simpler entities come first and complex ones build on them. Tickets come out of each slice, never from a list made up front, as `decisions/2026-10-23-slices-not-preplanned-tickets.md` says.

BioMCP 0.9 is not the spec. A slice looks critically at every 0.9 command, section and option for its entity and keeps only what a real question needs.

## The steps

### 1. Harvest real examples

The slice's first ticket gathers real questions about the entity from published papers and real questions that agents and clinicians have asked. Each example records:

- The question as asked.
- Where it came from: a paper's PMID or DOI, or a public issue.
- The answer a careful person would accept, and where that answer lives.

Twenty to forty examples suffice for a first entity. Examples cover the common case, the hard case, the empty case and the case where a source fails. Clinical judgments go to a clinical reviewer. Case sets that cannot be published stay outside the repo and run through the runner's `--cases DIR`.

### 2. See how the sources answer

For each example, the slice asks each candidate source directly and records:

- Which source answers it, and how completely.
- How fast, and how many requests it takes.
- Where sources disagree, and how each spells the same fact.
- Which identifiers come back, and which ones an agent needs to chain the next call.
- What each source's license allows.

This step also measures the 0.9 command on the same examples, so each 0.9 behavior is kept, changed or dropped for a named reason.

### 3. Ask what would delight the agent

The slice answers five to ten questions in its design file. The template:

1. What does the agent ask first, in its own words, and which single command answers it?
2. What is the smallest header that lets the agent choose its next step without another call?
3. Which drill-downs does the agent need next, and does `next` name each one?
4. Which identifiers must come back exact, so the agent can pass them to the next command?
5. Where do sources disagree, and how does the output show it without interpreting?
6. What can fail or come back empty, and how does the status tell those apart?
7. How fast is a cold call and a warm call, and which source sets that speed?
8. What does 0.9 return here that no example needs, and what does it miss?
9. What would let the agent stop early with the right answer?
10. What would a clinician reading the agent's answer need to trust it, such as a source, a date or a version?

A slice may replace a question with one that fits its entity better. It keeps at least five.

### 4. Write the slice design

The slice writes `planning/slices/<entity>.md`. It holds the examples' summary, the source findings, the answers to the delight questions, the types and schema, the command tree with each command's header and drill-downs, the sources kept and dropped with reasons, the progressive disclosure strategy and token budget, and the 0.9 accounting. The PM reviews it on its branch and files the slice's tickets from it.

### Progressive disclosure and token budget

Every slice design states its progressive disclosure strategy. The densest, highest-priority facts for a clinician's question sit at the top of the command and option tree: the search header and the default card answer the common question alone, and each drill-down section or option adds the next most useful layer. Rarely needed or bulky data lives deepest, behind a named section or option, never in the default output.

Every slice design gives a token budget for each command's default JSON output and for each section, measured on the slice's harvested cases, and compares it with 0.9's output for the same questions in tokens and in facts carried. A Redux default output that costs more tokens than 0.9 for the same question needs a stated reason. The cases assert the budget with a byte or token ceiling, so growth fails a check instead of drifting.

### 5. Build outside-in

Each ticket writes its cases first and watches them fail, then builds the least code that passes, then removes what the cases do not need. Types and schema land before adapters, adapters before commands.

## What a slice delivers

- **Types.** The entity's identifiers, closed sets and records in `biodata`, following `rust-standards.md`.
- **Schema.** The header, card and page schemas, generated, checked in and versioned.
- **Adapters.** One module per source the design keeps, in the shape `architecture.md` gives, each with a recorded 429, 500, timeout and broken body case.
- **Commands.** `search <entity>`, `get <entity>` and the drill-downs the design names, through the CLI and the MCP tool, printing the envelope.
- **Cases.** At least one harvested case for every command and every option. Each case tells a story, names its harvest source, records the date its answer was verified against the live source, and asserts the facts a clinician's question turns on: which result comes first, what order, what count and what status.
- **Budget.** The progressive disclosure layers and the token budget per command and section, with the 0.9 comparison, enforced by ceilings in the cases.
- **Accounting.** Every 0.9 command and option for the entity ends as ported, renamed or dropped, with the ruling named in its ticket.

## What a slice does not do

- Add a source no example needs.
- Interpret, classify or score clinical meaning.
- Keep a 0.9 behavior because it exists.
- Write a ticket for a later slice. Later slices start from their own investigation.
