# 0083 — Page trial sites and contacts

Status: COMPLETE.
Landed: 61c58de58935aa8f4ed30a3e8edead3a1a1be666

Milestone: 2

Depends on: 0082

## Outcome

`get trial NCT… locations` pages the registry's sites with facility, city, country, coordinates, site status and the contacts attached to each site. `get trial NCT… contacts` returns central contacts and officials, each in its own role.

- Why: T09 and T18 ask about sites and recruitment; a trial-level radius match is not a site subset, and a site status is not the trial's status.
- Starts from: trial.md (T09, T18, the facility and radius finding, budgets); the rulings; 0082's card.
- Risks the proof must cover: a contact never moves to another site; a record with no sites is an empty section; the trial's overall status and each site's status stay separate; contacts appear only for the requested trial.
- Proof: T09 pages NCT03734029's 208 sites in registry order with coordinates and statuses, and a walk counts 208; T18 asserts NCT02987543 `COMPLETED` with its update date and site statuses apart; NCT02578680 asserts an empty locations section; MCP parity; measured ceilings. Then make landing-gate runs through pm job after current main.
- Defers: distance from a point to each site, which the registry does not return.
