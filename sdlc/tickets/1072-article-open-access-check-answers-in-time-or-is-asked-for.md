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
