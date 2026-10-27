# 0341 — Use one schema ID spelling and list schema IDs

Status: OPEN.

Milestone: 5

## Outcome

Every `biomcp.*` schema ID uses hyphens between words, such as `biomcp.adverse-event-devices/1`, matching command and source names. `biomcp schema` with no ID lists every schema ID with a one-line description and exits 0. Each command in `biomcp help` names the schema ID of its reply. An unknown schema ID returns an error that points to `biomcp schema`.

- Why: the docs lead's product findings report on main a01667d (2026-10-27), rechecked by the PM on the release candidate build from hosted run 40542150253. IDs mix underscores and hyphens, such as `biomcp.drug-header/2` beside `biomcp.drug_civic/4`, and 1.0 freezes them in the release schema baseline, so the spelling must be settled before the release. Agents have no way to list schemas except `version --verbose`.
- Starts from: the schema ID definitions, the schema command, the help artifact, `release/schema-baseline.json` and the 0314 snapshot recipe.
- Rules: PM ruling 2026-10-27: hyphens. IDs keep their current major numbers, because no release has published them. The baseline is regenerated from the new product source.
- Proof: a check fails if any schema ID contains an underscore; `biomcp schema` lists every ID in the baseline; each help command's schema ID resolves with `biomcp schema <id>`.
- Changes: schema ID constants, the schema command, help, fixtures that name IDs and the schema baseline.
- Defers: MCP `outputSchema` carrying real reply schemas, which goes to a 1.0 update.
