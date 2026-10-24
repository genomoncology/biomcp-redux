# 0029 — Add the civic section from CIViC

Status: OPEN.

Milestone: 1

## Outcome

`get gene <input> civic` returns CIViC's gene ID, description, variant count and link, tells a present-but-empty entry from an absent one, and reports a CIViC failure as `unavailable` beside an intact card, proven by gene.md cases 13 and 16 and an ERBB2 data case under their ceilings.

Depends on: 0026

- Starts from: 0026's section pattern and `post_json`; gene.md sections 2 (examples 5 and 15), 3 (CIViC row), 5, 6 and 8; architecture 3.5 (`Empty` with and without `SourceRecord`); biodata `Source::Civic`, which already exists.
- Changes: `crates/biodata/src/query/gene.rs`, `crates/biodata/src/response/gene.rs`, `crates/biodata/src/response/gene/civic.rs`, `crates/biodata/schemas/**`, `crates/biomcp-adapters/src/lib.rs`, `crates/biomcp-adapters/src/quotas.rs`, `crates/biomcp-adapters/src/civic/**`, `crates/biomcp-query/src/gene/sections.rs`, `crates/biomcp-query/src/gene/sections/civic.rs`, `tests/cases/gene/sections/civic-*.yaml`, `tests/replies/gene/civic/**`, `tests/replies/gene/all/civic/**`, `ROADMAP.md`.
- Checked boundaries, 2026-10-24: `POST https://civicdb.org/api/graphql` with `Content-Type: application/json` and `{ a: gene(entrezSymbol: "VSIR") { id name entrezId description link variants { totalCount } } c: gene(entrezSymbol: "ERBB2") { ... } }` returned 200 `application/json; charset=utf-8` with VSIR `{"id":14308,"name":"VSIR","entrezId":64115,"description":null,"link":"/features/14308","variants":{"totalCount":0}}` and ERBB2 `{"id":20,"entrezId":2064,"link":"/features/20","variants":{"totalCount":126}}`. BRAF has id 5, 115 variants and a 1,680-character description. Lowercase `braf` returns `null`. CIViC now holds ABCC8 (id 5559), MIR21, H3P10 and KRT18P36, so gene.md's ABCC8 absent example is out of date. `genes(entrezIds: [64115, 2064, 999999999])` returned only ERBB2, leaving out VSIR, so the list query is not used. No reply header names a rate limit.
- Unchecked: CIViC's published rate limit. Declare a conservative local quota `civic` of 2 requests per second and 2 in flight in quotas.rs.
- PM decision: 2026-10-24. One aliased `gene(entrezSymbol:)` query per command with each resolved card's HGNC symbol, at most 20 aliases. Each returned entry's `entrezId` must equal HGNC's Entrez ID, or the entry is dropped with a problem. `Section<CivicGene>` holds `id`, `variant_count`, `url` (the `link` joined to `https://civicdb.org`) and `description` whole, in that order. A gene with zero variants and no description is `Empty` with `record { id, url }`. A `null` entry is `Empty` with no record. Any GraphQL `errors` member fails the check as `bad_reply`. The section is never `NotApplicable`. Assertions and evidence items stay with the variant slice (gene.md section 5).
- Keeps: 0026's section behavior. No evidence, assertion or variant list. Files under 300 lines.
- Proof: Record live once into `tests/replies/gene/civic/` with a trimmed HGNC slice holding VSIR, ERBB2 and TP53, and add TP53's reply to `tests/replies/gene/all/civic/`. `get gene VISTA civic` (case 13) asserts `symbol` VSIR, `civic` state empty with record id 14308, under 2,500 bytes. `get gene ERBB2 civic` asserts state data, id 20, `variant_count` at least 100, and `/result/items/0/sections/civic` under 1,000 tokens; it runs through MCP with parity. `get gene ERBB2 civic` over a recorded HTTP 500 (case 16) asserts `/status` partial, `civic` unavailable with its reason and identity present, under 3,200 bytes. The 429, deadline, cut-body and 200-`errors` cases of architecture 5.5 assert the same, each with a later offline step; a synthetic `null` entry proves `Empty` with no record and a synthetic Entrez mismatch proves the drop. Mark edited recordings synthetic. Then make landing-gate runs through pm job after current main. The landing commit ticks CIViC in ROADMAP.md.
- Defers: CIViC evidence and assertions go to the variant slice.
- Evidence: Builds in parallel with 0027, 0028 and 0030–0032; the shared registration lines are listed in 0026's Evidence.
