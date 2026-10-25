# 0136 — Measure 0.9 on the drug and disease questions behind verified network containment

Status: OPEN.

Milestone: 3

## Outcome

BioMCP 0.9 runs the drug and disease slices' harvested questions with every request passing through a capture that counts it, and the drug and disease designs gain 0.9's tokens, requests and facts per question beside 1.0's. The drug and disease token budget boxes can then be ticked.

- Why: j11400 showed that the old 0.9 binary bypassed the capture proxy, so its drug outputs were excluded and the drug comparison is unmeasured. The disease design's 0.9 observations also carry unknown request counts.
- Starts from: the j11400 diagnosis; the drug and disease designs' question lists and token tables; the slices.md budget rule.
- Proof: first prove containment with one 0.9 command whose every request appears in the capture and none reaches the network another way. Then one bounded keyless pass over the harvested questions within the designs' approved request caps, with no refresh loops. Results go into the two designs' token tables. A failure to contain 0.9 is recorded through pm progress, and the tables stay marked unmeasured.
