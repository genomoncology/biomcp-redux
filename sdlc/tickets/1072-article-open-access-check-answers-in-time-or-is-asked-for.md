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

## Changes

- Changes: article search, its help, cases and docs.

## Progress

- Investigation, 2026-11-19, 14:14 to 14:40 UTC-5, from the Beelink with curl. 44 varied searches (gene variants, drugs, rare diseases); each takes PubMed's top 10 PMIDs and asks four Europe PMC forms in random order, plus NCBI. Time limit 3 s. Europe PMC's current request (one per page: `SRC:MED AND (EXT_ID:… OR …)`, `resultType=core`) answered in time 14/44, median 7.2 s, p90 15.0 s. `lite` did 14/44 (median 8.0 s), `idlist` 14/44 (median 7.5 s) and `idlist` filtered by `OPEN_ACCESS:y` 22/44 (median 3.6 s). So the delay is Europe PMC's search service answering an ID-list query, not the size or form of the result. Earlier the same day, `search article` on 1.0.0 timed out on the check in 3 of 4 default searches, and even a plain `--source europepmc` search failed after 16 s. NCBI's one-request form, a PubMed ESearch of the page's PMIDs with `"pubmed pmc open access"[filter]`, answered 44/44 in time (median 0.21 s, max 0.31 s). Europe PMC did not yet hold 71 of the 440 PMIDs. It called 90 papers not open access that PMC lists as open access, mostly recent deposits; the reverse happened 2 times. NCBI states no licence.
