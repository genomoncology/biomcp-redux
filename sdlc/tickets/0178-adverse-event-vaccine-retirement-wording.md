# 0178 — Apply Ian's vaccine search ruling to adverse event retirements and docs

Status: OPEN.

Milestone: 3

Depends on: 0161
Depends on: 0162
Depends on: 0182

## Outcome

VAERS source selection, the CVX routes and 0134's `cvx sync` retirement return the vaccine wording that follows Ian's ruling on losing 0.9 vaccine search, and the roadmap, ledger and docs record the same ruling. This ticket waits for that ruling; it was split from 0161 and 0162 so their nonvaccine work could start.

- Why: adverse-event.md leaves 0.9 vaccine search without a reachable source, and only Ian can accept that loss.
- Starts from: adverse-event.md's VAERS, CDC WONDER and CVX accounting; the retirement table and usage errors from 0161; 0134's `cvx sync` retirement; the ledger and docs from 0162.
- Rules: if Ian accepts the loss, VAERS selection says "Vaccine adverse-event search is unavailable in this slice; the captured CDC endpoint denied access." and CVX routes say "Vaccine identity lookup and sync are unavailable in this slice.", 0134's `cvx sync` message moves to that wording, and the ledger moves VAERS, CDC WONDER and CVX to the dropped line with the design's reason. If he keeps the requirement as blocked, both messages say vaccine search is blocked pending a source access investigation and their roadmap boxes stay open with that reason. Neither claims vaccines are safe or redirects to FAERS. No retirement rewrites the query. No parity claim is made before the ruling.
- Proof: outside-in cases first. A case per vaccine spelling asserts its message with zero provider requests and a size ceiling. `make docs-check` passes with the ledger change. Then make landing-gate runs through pm job.
- Ian ruling 2026-10-25: retry CDC WONDER once the way 0.9 did before deciding the vaccine wording. 0.9 sent its WONDER requests with the user agent `Wget/1.21.4`, and the j12641 capture did not, so its three 403 denials may come from WONDER's client block rather than withdrawn access. This ruling replaces adverse-event.md's bar on a user-agent change for this one capture.
- Retry recipe: one bounded keyless pm job sends 0.9's exact committed `spec/fixtures/vaers/reactions-request.xml` from the 0.9 repository unchanged (MMR), then the same request with `B_1` set to `D8.V10` and then `D8.V1`, as 0.9 built them: POST to `https://wonder.cdc.gov/controller/datarequest/D8` as `request_xml=<XML>&accept_datause_restrictions=true`, user agent `Wget/1.21.4`, at least 16 seconds between requests, each once, with zero retries, redirects, fallbacks or other vaccine codes, a 30-second deadline and a 2 MiB body cap. A denial ends that row. It follows WONDER's data use restrictions, makes no re-identification attempt and archives the exact XML, headers, statuses and bodies to Yellow outside Git. No product code changes in this step.
- After the retry: if WONDER returns aggregate tables, the PM files a VAERS summary ticket for 0.9's three vaccines (MMR, COVID-19, influenza) from the captured fields, with report-count wording that claims no causation, and this ticket's wording moves to that route. If all three are denied again, the PM takes the result back to Ian before any wording lands.
- PM ruling 2026-10-25 after the retry: j13368 returned aggregate tables, so VAERS becomes a supported source in 0182 and this ticket no longer retires VAERS selection. This ticket now owns only the CVX routes and 0134's `cvx sync` retirement, which say "Vaccine identity lookup and sync are unavailable; search vaccine report summaries with `search adverse-event --source vaers`.", and the ledger and docs: VAERS and CDC WONDER move to ported with 0182, and CVX moves to dropped with the design's reason (no proved WONDER mapping). The earlier wording branches above no longer apply. Depends on 0182.
