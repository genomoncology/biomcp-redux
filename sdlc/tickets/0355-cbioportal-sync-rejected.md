# 0355 — Find why sync cbioportal-studies is rejected

Status: COMPLETE.
Landed: af36ea6371bbe6296612848b7c0eeb24896c1cfe

Milestone: 5

## Outcome

The cause of `biomcp sync cbioportal-studies` returning `rejected` in about half a second is found. It is fixed either way. If it depends on the reporter's proxy route, the reply names the routing cause and the routing fix ships in 1.0 too.

- Why: the docs lead's second-pass product findings (2026-10-27) and the first pass (E5). On the docs lead's cloud machine the sync is rejected, while `curl https://www.cbioportal.org/api/studies?projection=DETAILED` through the same proxy returns 200. The PM's health run also showed cbioportal `syncing` with no sync running (0342).
- Starts from: the cBioPortal studies sync request, its Fetch route and `rejected` mapping, and 0340's proxy changes.
- Rules: one ordinary CLI sync invocation with its built-in three attempts, every physical request counted against a total of three, no key (PM ruling m10617). Do not widen proxy policy to make it pass.
- Proof: the ticket records the reproduction, or its absence, with and without a proxy route; a fixed cause has a replay or local fixture case that failed before the fix.
- Changes: depends on the cause.
- Defers: nothing.
