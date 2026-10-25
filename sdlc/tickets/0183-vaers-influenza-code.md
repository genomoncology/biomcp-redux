# 0183 — Find WONDER's current influenza vaccine code for VAERS summaries

Status: OPEN.

Milestone: 3

Depends on: 0182

## Outcome

VAERS summaries cover influenza, as 0.9 did, using WONDER's current native influenza vaccine code or codes, or adverse-event.md records with evidence why influenza cannot be served.

- Why: 0182's capture j13383 sent 0.9's `FLU` code for all three groupings and got HTTP 500 with identical 233-byte bodies and no tables, while MMR and COVID19 returned tables. 0.9's code may no longer match WONDER's vaccine list.
- Starts from: the j13383 evidence; 0182's adapter, recipe and settings; WONDER's published VAERS help and request form, which list the vaccine codes.
- Rules: read the published code list first, without a data request. Choose only codes the list names for influenza vaccines; never guess spellings. Then one bounded keyless pm job sends the three groupings once for each chosen code under 0182's settings, with zero retries or fallbacks, archived to Yellow. If the list names several influenza codes, ask the PM before choosing which to support. Several codes are never summed into one total. A denied or failed capture ends this ticket with that evidence recorded, and influenza stays deferred.
- Proof: outside-in cases replaying the new capture, as in 0182, with measured ceilings. Then make landing-gate runs through pm job.
- PM ruling 2026-10-25: support all fourteen native WONDER influenza vaccine types found in j13551 as separate selectable codes, each with its native label and its own per-table totals. No group is summed, merged or aliased to a broad influenza selector, and one report may list several vaccines. The text `influenza`, `flu` or `FLU` returns no data: it fails before I/O and lists the fourteen native codes with their native labels so the caller picks one. The labels are kept exactly, including FLUX(SEASONAL) meaning no brand name and FLUX(H1N1) meaning unknown manufacturer, and the selector carries the exact native value with its qualifier. Capture: one bounded keyless pm job sends the three groupings once for each of the fourteen codes (42 requests) under 0182's settings, at least 16 seconds apart, zero retries or fallbacks, 2 MiB each, archived to Yellow. A code that is denied, fails or returns no tables is left out and listed in adverse-event.md. A clinical reviewer checks the listing wording.
