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
