# 0363 — Fix the CDC WONDER VAERS reply decoding

Status: COMPLETE.
Landed: 8aad84cb036d8e6d686eaa3822ad152d3a87e4c7

Milestone: 5

## Outcome

CDC WONDER VAERS replies with complete XML labelled `text/html; charset=ISO-8859-1`, such as the MMR query that returned `bad_reply`, are admitted and decoded for that source only. The remaining deadline failures moved to 0386 (PM ruling m10791 ask 18).

- Why: the docs lead's product findings report (2026-10-27), item E5, and the PM's live health run on the release candidate. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: the GWAS, VAERS and InterPro adapters, their deadlines and the health probe budget.
- Rules: PM ruling 2026-10-27 (m10791): CDC WONDER replies with complete XML labelled `text/html; charset=ISO-8859-1` are admitted and decoded as XML for that source only, with the fix proved by replay; other sources stay strict. One separate COVID19 retry runs after CDC's required spacing, within the remaining budget. At most three bounded live requests per case per attempt, no keys. A longer deadline needs a measured reason.
- Proof: each fixed cause has a replay or fixture case that failed before the fix; any source limit is stated in the reply or help.
- Changes: depends on the causes.
- Defers: nothing.
