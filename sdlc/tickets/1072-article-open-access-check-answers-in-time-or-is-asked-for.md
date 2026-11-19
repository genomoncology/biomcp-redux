# 1072 — article-open-access-check-answers-in-time-or-is-asked-for

Status: OPEN.

Milestone: 7

## Outcome

A default `biomcp search article` does not carry a step that fails most of the time. Either the Europe PMC open-access check answers within its time limit on nearly every search, or it runs only when the caller asks for it.

## Why

Article search adds a Europe PMC open-access check to every search. Ticket 1046 measured it at 11 to 19 seconds against a 3-second limit, and on 2026-11-19 it timed out in 3 of 4 default searches. Today the reply still says `complete` when it fails. Ticket 1059 will make a reply with a failed source say `partial`, which is right, but then most default article searches would read `partial` and callers would learn to ignore the word. The check also costs every search up to 3 seconds for a result it rarely gets.

## Proof

- Proof: the developer measures the check on at least 40 varied live searches before and after, at different times, and reports how many answer inside the limit. After the change, either at least 19 in 20 answer in time, or the check does not run unless asked and a default search makes no Europe PMC request.
- Proof: a replayed case for whichever behaviour is chosen; it fails on the base commit.

## Starts from

- Starts from: ticket 1046, which added the check and chose to leave the page status unchanged when it fails; the `--no-access-lookup` option; `sdlc/planning/status-words-proposal.md` on branch `ticket/1059-status-words-match-what-happened`, which counts 82 recorded article replies saying `complete` beside a failed Europe PMC row.
- First find why it is slow: the form of the request (one request per page of results, or one per article; searching by a long list of IDs), Europe PMC's own latency, or the limit. Try the cheapest request that answers "which of these PMIDs are open access". Report the finding to the PM with numbers before choosing.
- PM decision, 2026-11-19: if no request form answers reliably inside the limit, the check becomes something the caller asks for, the reply says in one short line how to ask, and rows show open access as not checked rather than unknown. Do not raise the limit past what an agent would wait for on every search.

## Keeps

- Keeps: a caller who asks for the check and gets a failure is told so. No search result is dropped because the check failed.

## Defers

- Defers: other add-on lookups with the same pattern, such as the drug search name lookups; list any found.
- Deferred, same pattern: drug-name expansion in `search article` and `search trial` reads MyChem and ChEMBL before the search. Live on 2026-11-19 with an empty cache, `search article osimertinib|trastuzumab|imatinib` took 2.0, 2.5 and 2.7 s, against 1.1, 1.3 and 1.4 s with `--exact-term`: two extra requests and about 0.9 to 1.3 s per uncached search. Under 1059 a failed name lookup makes the reply `partial` (PM decision).
- Deferred, same pattern: the stale-status checks in drug and device recall (`drug/recall.rs`, `diagnostic/recall.rs`), companion (`diagnostic/companion.rs`), clearance (`diagnostic/clearance.rs`) and GWAS (`gwas/mod.rs`, `gwas/get.rs`). Each command decides `partial` by hand after it has classified the reply. They send no extra request, so they cost nothing per call. 1059's single owner replaces them.

## Changes

- Changes: article search, its help, cases and docs.

## Progress

- Investigation, 2026-11-19, 14:14 to 14:40 UTC-5, from the Beelink with curl. 44 varied searches (gene variants, drugs, rare diseases); each takes PubMed's top 10 PMIDs and asks four Europe PMC forms in random order, plus NCBI. Time limit 3 s. Europe PMC's current request (one per page: `SRC:MED AND (EXT_ID:… OR …)`, `resultType=core`) answered in time 14/44, median 7.2 s, p90 15.0 s. `lite` did 14/44 (median 8.0 s), `idlist` 14/44 (median 7.5 s) and `idlist` filtered by `OPEN_ACCESS:y` 22/44 (median 3.6 s). So the delay is Europe PMC's search service answering an ID-list query, not the size or form of the result. Earlier the same day, `search article` on 1.0.0 timed out on the check in 3 of 4 default searches, and even a plain `--source europepmc` search failed after 16 s. NCBI's one-request form, a PubMed ESearch of the page's PMIDs with `"pubmed pmc open access"[filter]`, answered 44/44 in time (median 0.21 s, max 0.31 s). Europe PMC did not yet hold 71 of the 440 PMIDs. It called 90 papers not open access that PMC lists as open access, mostly recent deposits; the reverse happened 2 times. NCBI states no licence.

## PM decisions (2026-11-19, option A)

- PM decision, 2026-11-19: a default article search takes the open-access flag from NCBI, in one request per page sent alongside the title read. The default sends no Europe PMC request.
- PM decision, 2026-11-19: the licence comes only when the caller asks for the Europe PMC lookup. Keep an explicit option for that lookup and name it in help in one short line. A row without a licence shows it as not checked, never as unknown or absent. When the asked-for lookup fails, the reply says so, and under the 1059 rule it reads `partial`.
- PM decision, 2026-11-19: each row says which source its open-access flag came from. The two sources disagreed on 92 of 440 papers; record that count and its direction in `docs/cautions.md`. Say what NCBI's filter means in NCBI's own words, citing its documentation, because "open access" means different things in PubMed Central's subset and in Europe PMC. Do not decide which is right.
- PM decision, 2026-11-19: if the NCBI request fails, the search still returns its rows, the flag shows as not checked, and the status follows the 1059 rule.
- PM decision, 2026-11-19: respect NCBI's request pacing and the repo's quota rules, and state the added time. The second batch of 45 searches is the "after" measurement, together with a run on the built change.
- PM decision, 2026-11-19: list the same pattern elsewhere in Defers, with its cost per call where it can be measured cheaply: drug search name lookups, and the stale-status checks in recall, companion, clearance and GWAS. Do not change them here.
- After measurement (second batch), 14:41 to 15:03 the same day, the same 44 searches: Europe PMC's current form answered in time 20/44, median 3.5 s; NCBI's open-access ESearch answered 43/44 in time (median 0.21 s; one reply took 5.9 s). Over both batches, NCBI answered 87 of 88 in time.
- Change: a default `search article` page asks NCBI, in one ESearch per page beside the title read, which of its PMID rows are in PubMed Central's open-access subset (`pubmed::open_access`, `"pubmed pmc open access"[filter]`, 3-second budget, NCBI's shared request pacing). It sends no Europe PMC request. Rows carry `open_access_source` (`pubmed` or `europepmc`). `open_access_unknown` becomes `open_access_not_checked`, and rows without a licence say `license_not_checked`. `--license-lookup` (MCP `license_lookup`) also asks Europe PMC for licences and its own flag, with a 10-second budget; its flag replaces NCBI's on the rows it holds. A failed NCBI lookup keeps the rows, merges its failure into the PubMed source row with a one-line notice, and offers a fresh retry. The article page, edge page and recommendations schemas move to new majors (`article-page/11`, `article-edges/12`, `article-recommendations/11`) because a field was renamed. Docs: `docs/cautions.md` (both definitions and the 92-of-440 disagreement), `docs/configuration.md` and release notes.
- Cases: `article/search/pubmed-access-rights` now has three steps: `--no-access-lookup` (rows not checked); the default (NCBI flags 36970111 true and 42162665 false from `pubmed`, licences not checked); and `--license-lookup` (Europe PMC flags and `cc_by_nc` / `cc_by_nc_nd`). `pubmed-access-timeout` now asks `--license-lookup`; the 12-second synthetic delay still passes the 10-second budget, so the rows keep NCBI's flags, say `license_not_checked`, and Europe PMC is named `failed deadline-exceeded`. Both fail on base 66a5e8c67, where the option does not exist and no row has `open_access_source`. For every replayed article page that now makes the NCBI request (cases and runner trials, found from reply transcripts in j43144 and j43170), BioMCP's exact ESearch was recorded live with Python into that replay folder. The synthetic drug-name PMIDs (990000xx) return NCBI's real empty answer. The LitSense and LitVar first-step ceilings move to measured plus 15 percent. The concurrent-process article trial compares rows by identity, because its offline searches cannot read flags.
- Built-change run (j43173, 2026-11-19 15:29 to 15:31, debug build, empty cache per search, `--exact-term`): 40 of 40 live searches answered the NCBI lookup, and all 390 rows carry a `pubmed` flag (127 open access). No search sent a Europe PMC request. The same searches with `--no-access-lookup` were faster by a median of 0.15 s (mean 0.18 s), which is the added time.
