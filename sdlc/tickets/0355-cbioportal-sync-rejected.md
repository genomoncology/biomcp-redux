# 0355 — Find why sync cbioportal-studies is rejected

Status: OPEN.

Milestone: 5

## Outcome

The cause of `biomcp sync cbioportal-studies` returning `rejected` in about half a second is found. If it reproduces on an ordinary network without a proxy, it is fixed and joins the 0336 refresh. If it depends on the reporter's proxy route, the reply names the routing cause, and the fix ships in a 1.0 update.

- Why: the docs lead's second-pass product findings (2026-10-27) and the first pass (E5). On the docs lead's cloud machine the sync is rejected, while `curl https://www.cbioportal.org/api/studies?projection=DETAILED` through the same proxy returns 200. The PM's health run also showed cbioportal `syncing` with no sync running (0342).
- Starts from: the cBioPortal studies sync request, its Fetch route and `rejected` mapping, and 0340's proxy changes.
- Rules: one bounded live request per attempt, at most three attempts, no key. Do not widen proxy policy to make it pass.
- Proof: the ticket records the reproduction, or its absence, with and without a proxy route; a fixed cause has a replay or local fixture case that failed before the fix.
- Changes: depends on the cause.
- Defers: nothing.
