# 0832 — drug-name-lookup-stops-making-searches-partial

Status: OPEN.

Milestone: 7

## Outcome

A trial or article search that names a drug returns `complete` when its own source answered, under the load of 25 searches at once; the drug-name lookup no longer turns the reply `partial` by timing out.

## Why

In `biomed-agent-bench/results/baseline-1.0.md` (build c2511ee90, which is current main), 70 of 109 article searches and 48 of 95 trial searches came back partial because the ChEMBL and openFDA name lookup timed out. An agent holding a partial answer leaves for curl, which is the main reason BioMCP's modes did not beat curl on tokens.

## Proof

- Proof: a routine case with a slow recorded name source shows the search complete with a notice naming the names it could not widen; a release-suite run of 25 concurrent searches against the recorded sources returns no partial reply caused by the lookup. The bench reruns the baseline's Sonnet MCP mode and the partial count falls below 10%.

## Cause

- Developer note: 2026-11-11. Measured by job j37631 on the build machine with the baseline binary (c2511ee90), live sources, a cold cache per process, at 1, 5 and 25 searches at once. Each row is one request's wire time from `--observe`.
- The time goes to ChEMBL, and load is not needed to lose it. One search alone (`search trial imatinib`) came back partial after 7.8 s: MyChem answered in 0.28 s, then the first ChEMBL request ran 6.37 s without an answer and was cut off. ChEMBL's exact-name requests took 0.2 to 5.3 s each when they answered (median 1.3 s over 34 requests), against 0.26 s for MyChem, 0.4 to 0.9 s for the openFDA label and 0.15 s for ClinicalTrials.gov. Load makes it worse: with 25 at once, 7 of 12 first ChEMBL requests got no answer at all, against none of 5 with five at once.
- The lookup asks MyChem, then ChEMBL, then the openFDA label, one after another, inside one third of the command's 20 s. ChEMBL needs three requests in a row (preferred name, synonym, then the molecule), spaced one a second. So a lookup that reaches ChEMBL needs about 4.5 s when ChEMBL is quick and usually runs out of its 6.4 s: 12 of 16 did. When it does, the openFDA label is never asked, yet the reply lists it as failed with `deadline-exceeded`.
- MyChem settles the lookup alone, in under 0.3 s, when it holds one structure for the name (13 of 31 searches). ChEMBL is reached when MyChem has several structures (imatinib, metformin, trastuzumab), rows without a structure (pembrolizumab) or no row at all. The baseline's partial searches were mostly the last kind, whole phrases and trial IDs that name no drug: `GS-US-621-6289`, `apitegromab SAPPHIRE`, `germline mutation rate GWAS ...`. All 15 partial trial and article searches in the Sonnet `biomcp-mcp` records show the same two rows, `chembl` and `openfda-label` failed with `deadline-exceeded`, and the read run (`r2/v3-10-bixlenvo-ddi-2026`) shows it twice.
- Why the reply turns partial: the lookup leaves its timed-out sources in the reply as `failed`, and the status rule counts every failed source in a reply, whether the search asked it for records or only for names (`status::item`, the trial search's `item_state`, the article page's `finish`). The registry or PubMed had answered in full each time.

## Fix chosen

- Developer note: 2026-11-11. Two changes, both in the lookup.
- A lookup that does not finish no longer counts against the search. Its sources are listed as `skipped` with the reason, and carry the notice that names the term searched as typed. The reply is `complete` when its own source answered. Adverse-event and label searches keep their present rule, because there a name source is also the searched source.
- ChEMBL is not asked when MyChem answered with no row at all. MyChem's query reads its own copy of ChEMBL's preferred names and synonyms, so such a name has none in ChEMBL to add, and this is the case that cost the baseline its wait: those searches now spend about 0.7 s on names in place of 6.7 s.
- Not chosen: a longer budget, which lengthens the wait; a shorter one, which at anything under 4.5 s only stops ChEMBL ever answering; dropping ChEMBL from the lookup, which would lose the official-name and one-molecule rules that rest on it; racing the lookup against the search, which ChEMBL would never win.
- Left: a real drug name that MyChem cannot settle (several structures, or no structure) still waits up to 6.4 s on ChEMBL and is then searched as typed, complete, with the notice. Removing that wait needs ChEMBL's names held locally, which is a new data asset and its own ticket.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/search_names.rs` and `aliases.rs`, the shared resolver budget, and idea 'Explain an unattempted drug-name fallback' in `pm idea list`. One run to read: `baseline-1.0-sonnet/r2/v3-10-bixlenvo-ddi-2026/biomcp-mcp/mcp.out` in the bench's data folder on the build machine.
- PM decision: 2026-11-11. Diagnose first with a bounded timing run and put the cause in this ticket before choosing the fix: a cached name table, one shared lookup per call, a longer budget, or the lookup made non-blocking with the reply saying which names were searched. Prefer the fix that removes the wait over one that lengthens it.

## Keeps

- Keeps: a real source failure of the searched source still reports partial or failed; name lists stay bounded and frozen across pages; no result is presented as widened when it was not.

## Defers

- Defers: making widening the default for every drug filter is ticket 0830 on branch `1.1`, which builds on this.

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `crates/biomcp-fetch/src/`, `tests/cases/drug/`
