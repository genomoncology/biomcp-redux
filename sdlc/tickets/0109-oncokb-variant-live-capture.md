# 0109 — Capture a real keyed OncoKB variant annotation

Status: OPEN.

Milestone: 2

Depends on: 0095

## Outcome

One bounded keyed OncoKB variant annotation is captured from landed main and replayed as a case, so the OncoKB (keyed) variant roadmap box can be ticked. It never blocks code landing.

- Why: 0095 lands on synthetic keyed proof because no approved recipe lets a product smoke inherit the user's OncoKB key.
- Waits on: Ian's approval of a recipe in which a product smoke inherits the configured OncoKB key and the agent never inspects, prints, copies, records or stores it, as the NCI recipe already does.
- Proof: one pm job from landed main runs `get variant` with the `oncokb` section for one public example; the recorded reply holds no key; the replay case runs from outside the repo if OncoKB's terms bar storing the reply. A failure is recorded through pm progress with the roadmap left unchecked.
