# 0149 — Keep every candidate get and the page step in next

Status: COMPLETE.
Landed: 8273066afd38fbf9a12314a5235a71d30f646d54

Milestone: 3

## Outcome

Shared next-step assembly follows architecture 6.10's bound: a page of selectable candidates yields one get per candidate on the page, then the page step, then at most three other steps in the slice's order. No candidate get or page step is dropped to make room for a section or recovery step. Other responses keep the five-step bound.

- Why: `network_steps` cuts query actions at five after adding recovery actions, so 0140 and 0127 lost the page step after five candidate gets. Accepted MyChem search, MyVariant search and candidate-resolving variant get build their steps the same way. Architecture asked for both five steps and one get per candidate, which conflict at normal page sizes; 6.10 now settles it.
- Starts from: `network_steps` and the shared step assembly; the OLS exception in 0140; architecture 6.10.
- Proof: outside-in cases with synthetic or retained replies for a five-candidate page that has more pages and one that is the last page, on each affected command, asserting every candidate get, the page step and the bounded extras. Full and comparable output is measured, and only a case whose measured size grows gets a reported, case-specific ceiling change. No provider call. Then make landing-gate runs through pm job. Affected adapters are accepted after this lands.
