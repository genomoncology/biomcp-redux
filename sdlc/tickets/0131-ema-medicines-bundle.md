# 0131 — Sync the EMA medicines bundle and read EMA regulatory records

Status: OPEN.

Milestone: 3

Depends on: 0124

Depends on: 0125

## Outcome

`biomcp sync ema` downloads EMA's `medicines.json`, validates it and replaces the local copy atomically with a `biomcp.sync/2` receipt. `search drug TEXT --source ema` returns native EMA candidates from the bundle, `get drug EMEA/H/C/000406 --source ema` returns the exact EMA card, and `get drug EMEA/H/C/000406 regulatory --source ema` returns native application, status and date fields as `AuthorizationEvent` records with the source timestamp.

- Why: the bank's EU authorization questions (D02, D06, D22, D24) need EMA's own records, and drug.md keeps the medicines file under the approved bundle decision. The 0116 recheck marks the EMA exact-ID get unproven.
- Starts from: drug.md's EMA rows, the bundle inventory and cases R01 and B01; the existing shared `sync <bundle>` contract and atomic bundle recipe; 0124 and 0125.
- Rules: only `medicines.json` enters the initial bundle; the other EMA feeds wait for a matching question. Reads never trigger a sync. A failed validation keeps the old usable copy. Both Tibsovo records stay separate, D06 keeps EMA's structured 2016-02-01 date, and D24's withdrawn application never means absence from every European route. No EMA event reaches a card, and nothing infers a US status. `identity_level` comes from the EMA record type, never from a field's shape.
- Proof: outside-in cases first. Before adapter code, a bounded keyless pm job syncs the bundle once and captures positive exact-ID gets for EMEA/H/C/000406, EMEA/H/C/004124, EMEA/H/C/004480 and EMEA/H/C/005621, and a known positive for the search. Cases S04 for `ema`, G01 for EMA IDs, R01 (D02, D06, D22, D24 and the two Tibsovo records) and the EMA half of B01 (recorded size and hash, synthetic partial download, malformed file, and two concurrent readers during one replacement each seeing a complete old or new copy, and zero sync requests on reads) pass with size ceilings. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
