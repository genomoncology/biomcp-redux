# 0187 — Ship the agent skill guide for the 1.0 command tree

Status: COMPLETE.
Landed: c396fe547b4ab74cf3531c01c5875df37b2c0005

Milestone: 5

## Outcome

`biomcp skill`, `biomcp skill list`, `biomcp skill render` and `biomcp skill install DIR` ship in 1.0 as in 0.9, with a guide and worked examples rewritten for the 1.0 tree. Every command shown in the guide comes from a passing case.

- Why: public issue 114 asked for agent skills, 0.9 shipped `biomcp skill`, and issue 239 led to packaging it as a plugin. architecture.md lists `skill` as a housekeeping command and says the guide is rewritten from cases, but no ticket owns it.
- Starts from: 0.9's skill command and guide; architecture.md's housekeeping and skills lines; the landed entity cases and generated help.
- Rules: the guide is generated or checked from case run steps so it cannot drift. `skill install` writes only inside the named directory. The guide tells agents that report counts are not risk and that a listing is not a recommendation, matching the entity help.
- Proof: outside-in cases for each subcommand, a docs check that every guide command parses and maps to a case, and size ceilings. Then make landing-gate runs through pm job.

- Added 2026-10-26 from the milestone 4 review: the accounting covers 0.9.3's `skill status` leaf as well, the proof reuses the existing executable-example check from case run steps instead of a new guide checker, and this ticket is the ruling 0275 cites for every `skill` row.
