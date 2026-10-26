# Vertical slices

BioMCP 1.0 grows one entity at a time. A slice takes one entity with its sources from investigation to landed commands. Simpler entities come first and complex ones build on them. Tickets come out of each slice, never from a list made up front, as `decisions/2026-10-23-slices-not-preplanned-tickets.md` says.

BioMCP 0.9 is not the spec. It is the quality floor: 1.0 must answer at least as well, and matching 0.9 is a starting point, never the goal or the definition of good. A slice looks critically at every 0.9 command, section and option for its entity and keeps only what a real question needs.

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

Run bounded source captures with `make source-capture PLAN=... OUT=...` (`scripts/source_capture.py`); the plan supplies every selector and the command retains and verifies the evidence archive.

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

Every slice design gives a token budget for each command's default JSON output and for each section, measured on the slice's harvested cases, and compares it with 0.9's output for the same questions in tokens and in facts carried. A 1.0 default output that costs more tokens than 0.9 for the same question needs a stated reason. The cases assert the budget with a byte or token ceiling, so growth fails a check instead of drifting. The one exception is the source name list in the schema output: when a slice adds sources and components its design approved, that list's byte and token ceilings move to the measured size plus 15 percent without a new ruling, and the earlier sizes stay as history. The [measured-caps decision](../decisions/2026-10-26-measured-entity-caps-need-no-ask.md) governs entity help, command and section steps, and Markdown: set each cap to the measured size plus 15 percent, rounded up, and record the measurement job and cap in the owning ticket. The reviewer checks the measurement. A PM ruling is required before exceeding a global whole-help, MCP description or full tool-list cap, exceeding a design allocation by more than 50 percent, or removing content to fit.

### 5. Build outside-in

Each ticket writes its cases first and watches them fail, then builds the least code that passes, then removes what the cases do not need. Each kept source's exact-request captures exist before its shared types are accepted, and type review checks each field's absence, multiplicity and collection scope against those captures. Types and schema land before adapters, adapters before commands.

## What a slice delivers

- **Types.** The entity's identifiers, closed sets and records in `biodata`, following `rust-standards.md`.
- **Schema.** The header, card and page schemas, generated, checked in and versioned.
- **Adapters.** One module per source the design keeps, in the shape `architecture.md` gives, each with a recorded 429, 500, timeout and broken body case.
- **Commands.** `search <entity>`, `get <entity>` and the drill-downs the design names, through the CLI and the MCP tool, printing the envelope.
- **Cases.** At least one harvested case for every command and every option. Each case tells a story, names its harvest source, records the date its answer was verified against the live source, and asserts the facts a clinician's question turns on: which result comes first, what order, what count and what status.
- **Budget.** The progressive disclosure layers and the token budget per command and section, with the 0.9 comparison, enforced by ceilings in the cases.
- **Accounting.** Every 0.9 command and option for the entity ends as ported, renamed or dropped, with the ruling named in its ticket.

## Rules learned from 0.9

BioMCP 0.9's worst answers came from well-formed replies that hid a mistake. Every slice follows these rules.

- **Every filter has a known-positive case.** Each option that narrows a source's query has a case whose facts were checked against the source's own direct query on the capture date. The case asserts those captured facts, not a count that stays fixed forever. A filter mapped to the wrong field returns a valid zero that no reply check can catch, as 0.9's trial filters and FDA year search did.
- **A section exists only where its source sends the field.** The slice records, from a live capture of the public tier BioMCP uses, which fields each source returns. The slice offers a section only after a capture proves the public tier carries its fields. A field that the tier carries but some records lack stays optional per record. A source's public tier that lacks a field never reads as "none", as 0.9's drug interactions did.
- **Returned identity must match requested identity.** BioMCP compares the resolved, checked identifier it requested with the one the source returns, never literal alias or DOI spelling. A record whose identity differs is dropped with a problem. An ambiguous input returns candidates and never a guess.
- **Coordinates always carry their genome build.** A query coordinate needs an explicit build, and a coordinate with no build is a usage error. The build is echoed in `query`. A returned coordinate is labeled with the build its source states, never with the query's build by assumption.

## What a slice does not do

- Add a source no example needs.
- Interpret, classify or score clinical meaning.
- Guess meaning from text with regexes, term lists, query rewriting or rerankers, as `decisions/2026-10-24-no-text-heuristics.md` says.
- Keep a 0.9 behavior because it exists.
- Write a ticket for a later slice. Later slices start from their own investigation.

Each slice's investigation reads every closed public issue in the 0.9 repository that touches its entity or sources, and its legacy accounting names each one as kept, dropped by ruling with a reason, or not applicable. A real user's request that 0.9 shipped is kept unless Ian rules otherwise. Known issues for slices not yet started: author, issue 289 (`get author orcid:ID` with lowercase visibility values); release, issues 239 and 241/281 (plugin and container channels); skills, issue 114.

Scope: a slice serves biomedical research questions about any human health issue, including clinical questions such as cancer care and rare disease, as framed in `decisions/2026-10-23-the-user-is-an-agent.md`. BioMCP is entity-driven access to data sources and APIs: download a source's data as a bundle, search for entities, and get or fetch one entity with its sections and relations. A command is out of scope when it downloads, installs or runs another bioinformatics tool. Earlier drops that cited the cancer-care scope alone (DisGeNET and NIH RePORTER) are reconsidered only when a real question needs them.
