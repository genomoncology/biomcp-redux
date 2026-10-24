# 0111 — Send no CIViC ids filter when no profile is selected

Status: OPEN.

Milestone: 2

## Outcome

The CIViC molecular profile request sends `ids` only when the caller selected profiles. With no selection, the request omits the argument and the variable instead of sending `ids: null`. Exact native membership and independent paging stay as landed.

- Why: CIViC's resolver applies `scope.where(id: value)` to any `ids` argument it receives, so `ids: null` can turn a positive query into zero. The landed capture for native Variant 12 (BRAF V600E, CA123643) returned zero, and a clinical reviewer confirmed that variant has profile evidence. The resolver is `server/app/graphql/resolvers/top_level_molecular_profiles.rb` in civic-v2. Variant 12 and molecular profile 12 are different records.
- Starts from: the CIViC adapter and its request builder, and the landed CIViC cases.
- Proof: first diagnose locally from the published resolver and the recorded request, with no provider call. If the null argument is confirmed as the cause, a case with no selected profile asserts the request holds no `ids` argument or variable, and a case with selected profiles still sends them. Existing CIViC cases pass. Then make landing-gate runs through pm job. The positive live capture stays in 0110. If the diagnosis rules out the null argument, record the finding through pm progress and stop.
