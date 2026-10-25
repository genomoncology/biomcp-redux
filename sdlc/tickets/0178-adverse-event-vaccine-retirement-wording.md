# 0178 — Apply Ian's vaccine search ruling to adverse event retirements and docs

Status: OPEN.

Milestone: 3

Depends on: 0161
Depends on: 0162

## Outcome

VAERS source selection, the CVX routes and 0134's `cvx sync` retirement return the vaccine wording that follows Ian's ruling on losing 0.9 vaccine search, and the roadmap, ledger and docs record the same ruling. This ticket waits for that ruling; it was split from 0161 and 0162 so their nonvaccine work could start.

- Why: adverse-event.md leaves 0.9 vaccine search without a reachable source, and only Ian can accept that loss.
- Starts from: adverse-event.md's VAERS, CDC WONDER and CVX accounting; the retirement table and usage errors from 0161; 0134's `cvx sync` retirement; the ledger and docs from 0162.
- Rules: if Ian accepts the loss, VAERS selection says "Vaccine adverse-event search is unavailable in this slice; the captured CDC endpoint denied access." and CVX routes say "Vaccine identity lookup and sync are unavailable in this slice.", 0134's `cvx sync` message moves to that wording, and the ledger moves VAERS, CDC WONDER and CVX to the dropped line with the design's reason. If he keeps the requirement as blocked, both messages say vaccine search is blocked pending a source access investigation and their roadmap boxes stay open with that reason. Neither claims vaccines are safe or redirects to FAERS. No retirement rewrites the query. No parity claim is made before the ruling.
- Proof: outside-in cases first. A case per vaccine spelling asserts its message with zero provider requests and a size ceiling. `make docs-check` passes with the ledger change. Then make landing-gate runs through pm job.
