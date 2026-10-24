# 0106 — Move BioMCP's query, response and source types out of biodata

Status: OPEN.

Milestone: 2

Depends on: 0105

## Outcome

`biodata` holds only neutral entity values, identifiers, coordinates, notation and closed vocabularies. BioMCP's query and request types, response envelopes, cards, pages, sections, steps, result caps, the `Source` enum and source query builders such as the MyVariant `source_query` move to a BioMCP-owned crate, with the same wire output and generated schemas.

- Why: the shared plan's rules 7 and 16 keep disclosure logic and API-shaped types out of biodata so it can serve another consumer and later merge; BioMCP's architecture had put them there. This reverses architecture section 2's rejection of a types crate beside biodata, because that rule now serves a second consumer.
- Starts from: architecture sections 2, 3 and 8; `crates/biodata/src/query/**`, `response/**`, `source.rs`; the schema generator and ratchets.
- Changes: one new crate, named in architecture section 2 with its allowed dependencies (biodata, serde, schemars behind the schema feature); the dependency ratchet; every importing crate; architecture 2 and 3.
- Keeps: every schema name and byte of case output; `Source` keeps its kebab-case CLI spellings; no behavior change.
- Proof: generated schemas are byte-identical; the full case suite passes unchanged; the dependency ratchet refuses a biodata import of the new crate. Then make landing-gate runs through pm job after current main.
