# 0109 — Capture a real keyed OncoKB variant annotation

Status: OPEN.

Milestone: 2

Depends on: 0095

## Outcome

One bounded keyed OncoKB variant annotation is captured from landed main and replayed as a case, so the OncoKB (keyed) variant roadmap box can be ticked. It never blocks code landing.

- Why: 0095 lands on synthetic keyed proof because no approved recipe lets a product smoke inherit the user's OncoKB key.
- Approved by Ian, 2026-10-24: a product smoke may inherit the configured OncoKB key, and the agent never inspects, prints, copies, records or stores it, as the NCI recipe already does. The key may be inactive because it was issued for development. A rejected key ends the attempt: record the status code through pm progress, make no retry with another key, and tell the PM.
- Demo fallback, PM ruling 2026-10-24: OncoKB's demo instance answers without a key, and it holds only BRAF, TP53 and ROS1. If the key is rejected, the same job may capture BRAF V600E once from the demo instance with no key, to prove the reply shape against real data. That capture is labeled demo, stays outside the repo, replays as a case from outside the repo, and ticks the roadmap box, as Ian ruled on 2026-10-24. No one asks Ian for a new key. The product never calls the demo instance: a fallback that covers three genes would read as "no annotation" for every other gene.
- Proof: one pm job from landed main runs `get variant` with the `oncokb` section for one public example; the recorded reply holds no key; the replay case runs from outside the repo if OncoKB's terms bar storing the reply. A failure is recorded through pm progress with the roadmap left unchecked.
