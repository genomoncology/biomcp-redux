# 0449 — Show when get variant chose the build

Status: OPEN.

Milestone: 5

## Outcome

When `get variant` runs without `--build`, the query echo shows the build BioMCP chose and that it was a default.

- Why: the BioData review of BioMCP 1.0's data model (2026-10-28), ask 16. Ian ruled that incomplete or incorrect 1.0 data model types are fixed before the tag, because after it the release gate freezes every pattern, type set and schema ID.
- Starts from: `crates/biomcp-query/src/variant/mod.rs:326-335` and `crates/biomcp-adapters/src/myvariant/mod.rs:189`.
- Rules: the query echo still leaves out defaults that did not change the request; this default changes it, so it shows.
- Proof: `get variant rs113488022` without `--build` asserts the shown build and default.
- Changes: the query echo, schemas and cases.
- Defers: nothing.
