# 0843 — drug-names-answer-without-waiting-on-chembl

Status: COMPLETE.
Landed: 8f4b7214bbca54216ef612c249c55c6ff1ba0f87

Milestone: 7

## Outcome

A trial or article search naming a real drug gets its widened names without waiting on live ChEMBL requests, so the name step costs well under a second whether or not ChEMBL is slow.

## Why

The drug-name lookup ticket (0832) made an unfinished lookup harmless to the reply's status and skipped ChEMBL when MyChem has no row. It left one wait: a real drug name MyChem cannot settle (imatinib, pembrolizumab) still waits up to 6.4 seconds on three sequential ChEMBL requests, and a timed-out ChEMBL request is not cached, so a repeat pays again. Its review confirmed both.

## Proof

- Proof: first measure, on live sources, how often a real drug name reaches ChEMBL and what it waits; put the numbers here before choosing.
- Proof: a routine case with ChEMBL stalled shows a named-drug search widened and complete in under a second; it fails on the base commit.

## Progress

- Developer note: 2026-11-12. Measured on live sources by job j38352 on the build machine (main 7699f096a), 30 real drug names, `search trial NAME` one at a time, each with a cold cache and then again with the same cache.
- 17 of 30 names reached ChEMBL: imatinib, Gleevec, pembrolizumab, Keytruda, nivolumab, trastuzumab, rituximab, osimertinib, metformin, aspirin, tamoxifen, cisplatin, paclitaxel, lenacapavir, dolutegravir, bictegravir, elacestrant. MyChem settled the other 13 alone.
- ChEMBL answered none of its 52 requests that day: the first came back HTTP 500 after about 5 s and its retry was cut off at the lookup's 6.4 s. All 17 searches waited 6.0 to 6.4 s on ChEMBL (search median 7.8 s, against 1.4 s for the 13 that never reached it) and were searched as typed, unwidened. The repeat with a warm cache waited the same 6.7 s, because a failed ChEMBL answer is not cached.
- What that says about the candidates. Caching a failed answer helps only the repeat; the first search of each name still waits 6.4 s and still widens nothing while ChEMBL is down. Combining or parallelising the requests cannot reach well under a second either: ChEMBL has no single request for "preferred name or synonym", one request took 0.2 to 5.3 s even when ChEMBL was healthy (ticket 0832), and that day each one stalled.
- A cheaper route than a local asset exists. MyChem already holds ChEMBL's molecule records (`chembl.molecule_chembl_id`, `pref_name`, `molecule_synonyms` with their types), and BioMCP already reads it under its terms. One MyChem request scoped to ChEMBL's preferred name and synonyms answers in 0.28 to 0.41 s live (imatinib, Gleevec, pembrolizumab, Keytruda, metformin, trastuzumab, nivolumab, aspirin, insulin), and each found exactly one ChEMBL molecule with that exact name, except Gleevec, which two molecules state, as ChEMBL's own filter also finds. For valsartan it states the same 19 names in the same order as ChEMBL's API did when the FAERS case was recorded.

## Fix chosen

- Developer note: 2026-11-12. The name lookup reads ChEMBL's record from MyChem's copy instead of ChEMBL's API, in one request, with ChEMBL's own rules kept: a molecule counts only when its preferred name or a synonym is the typed name (ignoring case); two such molecules give no names; the names are the preferred name then the synonyms; official synonym types still survive the name cap. A page MyChem cuts short is read as unfinished, as before. The reply lists MyChem as the source asked and ChEMBL as the record's source, and the notice still names the ChEMBL record. ChEMBL's API is no longer asked by trial, article or adverse-event name lookups; `get drug --source chembl` and `search drug --source chembl` still use it.
- After the change (job j38365, same names, same day, ChEMBL still failing): 0 of 30 searches touched ChEMBL, all 30 widened, all complete. A name that needs ChEMBL's record now makes two MyChem requests, 0.26 s and 0.07 s on the wire.
- Not chosen: the cached failure and the combined requests, for the reasons above; a local ChEMBL names asset, which MyChem's copy makes unnecessary.
- Left: MyChem's declared pacing of one request a second (`quotas.rs`, BIOTHINGS) holds the second request back, so those names spend about 1.0 s on names in all (search median 2.2 s against 1.5 s for names MyChem settles in one request), against 6.4 s and no widening before. Reaching well under a second for them means either loosening that pacing for small name queries, which is a source-policy change, or reading ChEMBL's fields from the first MyChem page when it is complete, which changes every recorded MyChem name request. Neither is done here. MyChem's copy can also trail ChEMBL's newest release; a drug too new for it is usually missing from MyChem altogether, and such names already skipped ChEMBL after ticket 0832.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/search_names.rs`, the cause written in ticket 0832, and its review.
- PM decision: 2026-11-11. Candidates, cheapest first: cache a timed-out or negative ChEMBL answer for a bounded time; make the three ChEMBL requests one or run them together; hold ChEMBL's drug names locally as a data asset refreshed on a schedule. A local copy is a new data asset with its own `sdlc/data/` entry and a licence read (ChEMBL is CC BY-SA 3.0, which hosted use must attribute); take that route only if the cheaper ones do not remove the wait.

## Keeps

- Keeps: no result presented as widened when it was not; names frozen across pages; a real failure of the searched source still reports partial or failed.

## Defers

- Defers: whether a name source that was asked and timed out should read `failed` rather than `skipped` while the reply stays `complete`. The review raised it; it changes the shared status rule and needs a design review first. Until then `skipped` with its reason stands.

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `crates/biomcp-fetch/src/`, `tests/cases/search/drug-names/`

## Amendment, 2026-11-12

- PM decision: 2026-11-12. Accept the fourth route, reading ChEMBL's molecule records from MyChem's copy of them, because it is simpler than the three listed candidates and adds no data asset.
- PM decision: 2026-11-12. Accept about 1 s on names for a drug that needs ChEMBL's record as meeting the outcome for now, against 6.4 s and no widening before.
- PM decision: 2026-11-12. The two routes to well under a second, loosening MyChem's pacing for small name queries and reading ChEMBL's fields from the first MyChem reply, are deferred to a later ticket.
