# 0350 — Drop token ceilings that only restate a byte ceiling

Status: COMPLETE.
Landed: ef4547881a4aca2c811a84f207f6820a215176fa

Milestone: 5

## Outcome

Output budget fixtures whose token ceiling only restates their byte ceiling, including the pairs 1337/335, 171/43 and 172/43, use the existing byte-only form. Token ceilings that set a tighter limit than the byte ceiling stay.

- Why: the coordinator's RCA (m10387, 2026-10-27) found that 0334 and 0339 each failed a job because a byte ceiling changed for legitimate output growth while its derived token ceiling did not. The runner checks bytes first, so these token ceilings add no limit. Logged under 0060.
- Starts from: the output budget fixtures and the runner's existing byte-only syntax.
- Rules: fixtures only; no new runner syntax, checker or job.
- Proof: the landing gate passes; each removed token ceiling was at or above the token count its byte ceiling allows.
- Changes: output budget fixtures.
- Defers: nothing.
