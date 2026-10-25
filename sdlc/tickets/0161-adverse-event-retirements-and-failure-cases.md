# 0161 — Retire old adverse event options and prove adverse event failure cases

Status: COMPLETE.
Landed: fe65d282eab5a97168f4bd4fd89e4b82fe36bf38

Milestone: 3

Depends on: 0158
Depends on: 0159
Depends on: 0160

## Outcome

Every 0.9 adverse event command, option and section that adverse-event.md drops or renames returns a fixed usage error that names the supported route or the dropped reason, and makes zero requests. FAERS and MAUDE each report a failed outcome, never an empty one, on a 429, 500, timeout or broken body, and a mismatched returned identity is dropped with a problem and never enters the cache.

- Why: adverse-event.md's accounting drops or renames these options, and the slice rules require failure and identity cases for every adapter.
- Starts from: adverse-event.md's complete legacy accounting and retirement messages, the A24 row and the source failure case row; the existing retirement table and usage errors, including 0134's `cvx sync` retirement; the adapters from 0158 to 0160.
- Rules: `-d` and `--drug` name positional text. Every `--outcome` value names `--serious death` or `--serious hospitalization` for report flags and the `reactions` section for native outcomes, and says the old hospitalization and disability mappings were wrong. `--date-from`, `--since`, `--date-to` and `--until` name `--received-from` and `--received-to`. Bare or `any` `--serious` and the other seriousness values say only death and hospitalization are supported. `--suspect-only`, `--sex`, `--age-min`, `--age-max`, `--reporter`, `--manufacturer`, `--product-code`, device seriousness and every `--count` form, including the fake total fields, say the filter is not supported. `--type faers` names `--source faers`, `--type device`, `devices` and `maude` name `--source maude`, and `--type recall`, `recalls`, `enforcement` and `--classification` say recalls are outside this entity. `--source all` names `--source faers` or `--source maude`. `--device` names positional text with `--source maude`. `-l` names `--limit`. Get `--version` says multi-version get is unproven. The `outcomes` section names the card's seriousness flags and the `reactions` section, `concomitant` names `drugs`, and `guidance` and `all` say to choose a named section. `drug adverse-events` names `search adverse-event TEXT`. `-j` and `--json` name `--format json`. VAERS selection, the CVX routes and 0134's `cvx sync` message are out of this ticket and move to 0178, which waits on Ian's vaccine ruling; this ticket leaves their current behavior unchanged. No retirement rewrites the query.
- Proof: outside-in cases first. A case per retired spelling above asserts its message with zero provider requests. A24 runs synthetic 429, 500, timeout and broken HTTP 200 replies for `faers` and `maude`, labeled synthetic, and asserts a failed outcome with source and terms, never zero reports. A synthetic wrong returned ID or version for each adapter asserts the drop, the problem and no cache admission. D03's verified no-match stays distinct from every failure. Each case has a size ceiling. This ticket adds no filter or exact-ID get and makes no provider call; the filters and exact-ID gets it relies on have known-positive captures through one bounded keyless pm job in 0158 to 0160. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
