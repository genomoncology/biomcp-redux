# 0386 — Explain the GWAS, InterPro and CDC WONDER deadlines

Status: OPEN.

Milestone: 5

Depends on: 0363

## Outcome

The deadline failures left after 0363 are each explained: `search gwas --gene BRAF` past 15 seconds, the InterPro health probe past 3 seconds, and the CDC WONDER COVID19 query that received no status or body within 20 seconds after the source's required spacing. Each is fixed, or the reply and help state the source's own limit and the next step a user can take.

- Why: split from 0363, which lands the replay-proved CDC XML codec fix (coordinator m10791 ask 18, PM ruling 2026-10-27). 0363 used six physical requests and the causes remain unknown. Ian's ruling that 1.0 ships every known fix.
- Starts from: 0363's archived witnesses, the three adapters' request shapes and the source documentation.
- Rules: at most three new bounded live requests in total, no keys, and no longer deadline without a measured reason. Source behavior BioMCP cannot change is stated, not hidden.
- Proof: each fixed cause has a replay or fixture case that failed before the fix; each source limit appears in the reply and help, proved by a case.
- Changes: depends on the causes.
- Defers: nothing.
