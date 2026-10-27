# 0363 — Diagnose the GWAS, InterPro and VAERS live failures

Status: OPEN.

Milestone: 5

## Outcome

The causes of these live failures are found and fixed or recorded with the source's own limit: `search gwas --gene BRAF` hitting `deadline_exceeded` after about 15 seconds, CDC WONDER VAERS returning `bad_reply` for MMR and `deadline_exceeded` for COVID19, and InterPro and Semantic Scholar health probes timing out at 3 seconds.

- Why: the docs lead's product findings report (2026-10-27), item E5, and the PM's live health run on the release candidate. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: the GWAS, VAERS and InterPro adapters, their deadlines and the health probe budget.
- Rules: at most three bounded live requests per case per attempt, no keys. A longer deadline needs a measured reason.
- Proof: each fixed cause has a replay or fixture case that failed before the fix; any source limit is stated in the reply or help.
- Changes: depends on the causes.
- Defers: nothing.
