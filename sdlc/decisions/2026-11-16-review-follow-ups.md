# Review follow-ups: Orphadata captures, hosted stdio, machine names, release notes, persona budget

Date: 2026-11-16

Decided by Ian, in chat with the PM, 2026-11-16: "Yeah, you can do all six of those", answering the PM's six recommendations on the 2026-11-14 adversarial review (`sdlc/records/2026-11-14-adversarial-review.md`). He added: "we definitely want to do them a couple at a time, not all of them at once" (persona runs).

## Decision

1. The public test captures from Orphadata are replaced by the synthetic set we already hold (finding C3).
2. A hosted server refuses to run without HTTP. Hosted always means HTTP behind the hosting front end (finding C1).
3. Internal machine names in public planning records are replaced with generic names before the history append (finding D7).
4. The hosted saved-work storage speed test may run on AWS.
5. Any ticket that changes something users see (a command, option, filter, source, config key, credential name, output field or tool) updates `release/notes.md` in the same change.
6. Ian approved $130 more for persona runs, on top of the remaining part of the $150 (`2026-11-11-new-owner-and-1-1-measurement.md` in biomed-agent-bench), for the 1.0 baseline. The runs go a couple of personas at a time, not all at once.
