# 0905 — size-limits-root-cause

Status: OPEN.

Milestone: 7

## Outcome

A root cause analysis (`skills/rca.md` in the agents repo) of why size limits keep forcing developers to trim wording ticket by ticket. It ends with ranked fixes, and the smallest one is done or ticketed, so adding a source, option or next step no longer means hunting for spare characters.

## Why

On 2026-11-13, five tickets ran into a size limit and trimmed other text to fit:
- 0889 (Orphanet prevalence) hit the connect-time text caps in `scripts/check_front_map.py` (ticket 0836: must not grow). Hosted single now has 1 character to spare.
- 0893 (MAUDE event type) pushed the MCP `search_adverse_event` tool past its 2,000-character limit (`crates/biomcp/tests/runner/typed_schemas.rs:182`). The developer shortened two other flags' titles.
- 0894 (counts by date) hit the same 2,000-character limit and shortened three descriptions.
- 0886 (phenotype ranking) raised a help size budget.
- 0896 (FAERS labels) raised a synthetic card's size ceiling by 205 bytes.

0904 (runnable related queries) will add more to replies.

## Proof

- Proof: the RCA's write-up in this ticket's Progress. It counts the cases, sorts them by cause and names fixes with cost and reach, and the smallest fix lands or has its own ticket.

## Starts from

- Starts from:
  - `scripts/check_front_map.py`;
  - `crates/biomcp/tests/runner/typed_schemas.rs`;
  - the help budget and reply size checks;
  - ticket 0836 and the tickets named above (their Progress sections and branches).

## Keeps

- Keeps: the reasons the limits exist (what MCP clients load on connect, token cost to agents) unless the RCA shows a limit protects nothing.

## Defers

- Defers: nothing.

## Changes

- Changes: depends on the findings.

## Progress

- RCA run 2026-11-13 by a fresh subagent, read-only from git (origin/main 4cd92a23f and the ticket branches). The scans behind the counts were scratch and are not kept.

### Cases

Four kinds of size limit. The two that force trimming are new and tight. The two that are only raised are old and routine.

- **First-read ceilings** (`scripts/check_front_map.py:50`, set by command map and examples in first read (0836) on 2026-11-11 at exactly main's size that day). 2 tickets in 2 days:
  - Orphanet prevalence on disease cards (0889) raised typed and hosted by 1 character each (commits 0e830c863, 96bc55de3). Then a PM decision said to keep the ceilings, so it reverted the raise (0a2594244) and trimmed (8aa456a3b): it stopped listing `orphadata` under search disease, which was a real fix because search never used it, and it merged the "help" phrase in the typed and hosted headers. Hosted single is left with 1 character to spare.
  - ClinGen dosage regions (0888), on its branch, raised hosted single from 2,518 to 2,521 because search gene now names ClinGen (e4c77e9c2). No ruling is recorded. That clashes with 0889's decision and its 1-character margin, so whichever of the two lands second will fail the check.
  - The same thing happened before, with the old one-tool `biomcp` description ceiling, which also listed every command. Gene docs and accounting (0034) planned to drop the artifact list if it didn't fit 1,500 bytes. Article docs and accounting (0047) had the PM choose between trimming and raising. The PM raised the ceiling for pathway docs and accounting (0228, to 2,993 bytes) and again for protein docs and accounting (0244, to 3,563). The front map's own limit went from 2,500 (generated front-door map, 0586) to 4,000 so that each section could be paired with its source (front map pairs sections with sources, 0602).
- **Typed per-tool budget** (`crates/biomcp/tests/runner/typed_schemas.rs:182`, 2,000 characters of name, description and input schema; set by typed argument descriptions and size (0625) on 2026-11-04 and never raised). 2 tickets, both trimmed `search_adverse_event`:
  - MAUDE event type filter (0893): the tool went to 2,112 characters, so the developer gave `--expand-drug-names` and `--name-snapshot` short titles.
  - Adverse-event counts by received date (0894): the developer cut the `--aggregate` title to "Report counts." and shortened `--limit` and `--offset`.
- **Help size caps** (per-command `max_bytes` in `tests/cases/*/help.yaml` and the whole-help ceiling of 106,000 bytes). 49 landings on main raised one and 4 lowered one. On branches, phenotype set ranks genes (0886) raised search phenotype help from 1,763 to 3,132 to 3,662 bytes and shortened the `--search-mode` description. ClinGen dosage regions (0888) raised gene help to 2,435. I found one older trim: commit 0d764cab3, "Fit the author source summary inside the help limit". The ticket it belongs to isn't recorded.
- **Reply size caps** (per-case `max_bytes` and the runner's envelope tables, such as `faers_get.rs`). 57 landings on main raised one and 10 lowered one. FAERS codes labelled (0896) raised the synthetic card by 205 bytes. It was 7 bytes over the old cap on the comparable count.
- How these were counted: the help and reply numbers come from first-parent landings on main whose diff changed a `max_bytes` or `max_tokens` value. That is about one landing in five of the 530 ticket landings. The trims come from ticket Progress sections and commit messages, so a trim nobody wrote down would be missed.

Two causes explain every case.

- **Trims (0889, 0893, 0894, 0886's description, and 0888's raise outside the rules): the limit is today's size, and the text grows with the catalog.**
  - The first-read ceilings are main's size on 2026-11-11 with nothing to spare. The 2,000 budget is a round design target ("under 2,000 characters per tool", `sdlc/planning/typed-mcp-tools.md:55`), and `search_adverse_event` had already reached it.
  - Both texts are generated by listing every item:
    - typed instructions list each tool with all its sources (`crates/biomcp/typed_front_map.rs:330`);
    - hosted single lists one row per command with its sources (`crates/biomcp/src/mcp/hosted.rs:348`);
    - each typed tool's description lists its sources and section-to-source pairs (`crates/biomcp/src/mcp/typed/description.rs:100,121`);
    - one adverse-event tool carries the filters of FAERS, MAUDE and VAERS together.
  - So every new source or option has to cost characters, and with no room left, the only way in is to cut someone else's words. Two older rules point opposite ways. Generated front-door map (0586) fails the build if a source is missing from the map. Command map and examples in first read (0836) fails it if the map grows at all. Nothing says how many characters a new source is worth, so the PM ends up ruling one character at a time, and two lanes on the same day got opposite answers (0888 and 0889).
- **Raises (help and reply caps, 0886 and 0896): the rules work as written.** Measured size plus 15 percent, with no ask (`sdlc/decisions/2026-10-26-measured-entity-caps-need-no-ask.md`). Raising costs one measurement and one number edit, and nobody trims. Leave these as they are.

### What each limit protects, and whether the number is real

- **First read:** it protects token cost. `sdlc/records/2026-11-11-token-gap-root-cause.md` found the skill is re-read every turn and costs about 15% of a typed repeat. That cost is real, but the numbers 2,009, 1,996 and 2,518 measure nothing except the day they were set.
  - The only real client limit on record is Claude Code cutting instructions at 4,096 characters (`sdlc/records/2026-11-04-mcp-skills-and-design-research.md:83`). Every mode is well under it, and single mode is the closest at 3,669.
  - Using that record's own numbers (651k tokens a repeat for a 2,900-token skill, so about 225 re-reads a repeat), one character costs about 55 tokens a repeat. That is my estimate. So 0889's extra 16 characters cost about 900 tokens a repeat, under 0.03% of a 3.3M to 4.3M-token repeat.
  - The skill, which is the part re-read most, has about 2,977 characters to spare (2,580 against 5,557). After 0889, the typed and hosted instructions have 1 to 6 to spare and single mode has 132 (0889's measurements).
- **Typed per-tool budget:** it protects what an agent pays when a tool-search client loads a matched tool whole (`typed-mcp-tools.md:55`).
  - The number is a design target. It is not a client limit. The original target also counted the output schema, which typed tools no longer send.
  - Clients that load every tool up front are already guarded by the separate 48 KiB catalog cap (typed tools fit eager clients, 0683). 0683 itself says that cap doesn't prove an optimal size.
  - 0893's overrun of 112 characters is about 28 tokens on a turn that loads that tool. A typical typed reply is about 3,420 tokens (token-gap record).
- **Help caps:** they catch unnoticed growth in text that is read only on request. The 15 percent is arbitrary but cheap to live with.
- **Reply caps:** they protect the second-largest token cost (token-gap record, cause 2: replies 25% smaller save about 12%). Measured plus 15 percent is a fair alarm for that.

### Is text built by listing everything where a pointer would do?

Partly, but listing sources up front is a design choice backed by evidence, so it should stay:

- Generated front-door map (0586) found that a short generated map in the first read roughly halved what agents read.
- Front map pairs sections with sources (0602) cut cost from 0.89× to 0.70× by pairing each section with its source.
- In Claude Code, only tool names and the instructions load at start. So the instructions are the only place an agent sees sources before it searches.
- Tool schemas must carry every option, because clients refuse unknown argument names before BioMCP sees them (typed tools callable from their descriptions, 0833).

Where a pointer or grouping would do:

- Hosted single repeats the same source list on separate search and get rows. Grouping them would save characters, but I haven't measured how many.
- Help's "Fields:" line lists every field of every reply shape (`crates/biomcp/src/fields.rs:305`). That is why 0886's help doubled, but help is read only on request.

So shrinking the text buys room once. Fixing how the limits are set removes the cause.

### Fixes, smallest first

1. **Give the first-read ceilings room, and write down the rule.** Set each ceiling at 0836's measured size plus a stated allowance, say 5%:
   - typed and hosted about +100 characters, hosted single about +125, single about +180, all still under Claude Code's 4,096 cut;
   - a raise inside the allowance needs no ask, and going past it needs a PM ruling;
   - the check prints the margin left.

   Cost: one line in `check_front_map.py` plus a short decision in `sdlc/decisions/`, under an hour. By the estimate above, the allowance costs about 0.15% of a repeat's tokens. It prevents 0889's three commits and PM ruling and 0888's raise outside the rules, and it settles the clash between them before either lands. It keeps 0836's protection, because putting the map and examples back in the first read would still fail. I recommend doing this first.
2. **Raise the typed per-tool budget to 2,400** and keep the 48 KiB catalog cap as the guard for clients that load everything. Add a comment saying 2,000 was a design target, and have the size report print each tool's margin. Cost: one constant and its comment, under an hour. It prevents 0893's and 0894's trims, and the next filter added to adverse events or any other shared tool. It costs about 100 tokens on a turn that loads the largest tools. Optionally, put back the words 0894 cut from `--aggregate` so typed agents can see that the received-year and received-month counts need dates.
3. **Group identical source lists in the hosted single instructions** (`hosted.rs:348`), as typed already does by tool. Cost: about an hour plus regenerating the outputs. It frees room once but doesn't remove the cause. Do it only if fix 1's allowance runs low.
4. **Charge generated lists by the item, and limit hand-written text separately.** The checks would split each first-read text and tool into fixed text and generated lists. Fixed text can't grow without a ruling, and lists grow by a stated price per source, command or option. Cost: half a day to a day across `check_front_map.py`, `typed_schemas.rs` and the generators. It removes the cause for good and would have prevented all four trimming cases. It is more machinery than fixes 1 and 2 need, so hold it unless the allowance runs out within a milestone.
- Leave the help and reply caps alone. Runnable related queries (0904) will grow every card and first page at once and will have to remeasure many reply caps. It should plan one measuring job for all of them, not fix them one at a time.

### Reach beyond this repo

- The code is all in this repo. The habit isn't: setting a limit at today's measured size with no room is the exact-size ratchet that Ian dropped for code size (`agents/instructions/decisions/2026-10-20-features-and-outside-in-coverage-replace-the-size-ratchet.md:3`, "I certainly don't want to keep editing a single file thing").
- `agents/skills/pm-ticket.md:49` asks a ticket only to point at the repo's ruling on a limit.
- Recommend a `pm send jarvis` noting this pattern, so the rules owner can decide whether shared guidance should say that a size limit names what it protects and how much room it leaves. Not sent, per instructions.

### PM decision, 2026-11-13

Fixes 1 and 2 go ahead now, in this ticket. The 0836 limits came from a review (`sdlc/records/2026-11-11-token-gap-root-cause.md`), not from Ian, and their cost is real but their numbers are main's size on the day. Fix 1: first-read ceilings get 5% room over the 0836 measured sizes; growth inside the room needs no ask, growth past it needs a PM ruling, and the map stays out of the first read. Fix 2: the per-tool typed budget rises to 2,400, with the 48 KiB all-tools cap kept as the guard. Fixes 3 and 4 wait until fix 1's room runs low. 0904 plans one measuring job for its growth. Reach: the exact-size ratchet habit is mailed to Jarvis.
