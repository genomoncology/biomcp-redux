# 0088 — Search variants by gene and protein, by region, or by an explicit MyVariant query

Status: COMPLETE.
Landed: f29670d5d44b97cd2c9caf9d53b23b16274619d1

Milestone: 2

Depends on: 0087

## Outcome

`search variant --gene G --protein TEXT`, `search variant --region REGION --build B` and `search variant --query TEXT --source myvariant` return MyVariant's candidates in its order with its total and the exact query, with stable pages, through the CLI and MCP.

- Why: protein shorthand is how papers and clinicians name variants, and 0.9 found them only by text guessing.
- Starts from: variant.md sections 2 (coverage interpretation), 5 and 7; the rulings; 0087's types and adapter; the gene resolver.
- Risks the proof must cover: `--gene` and `--protein` build one fixed, echoed field query; a protein that is not a checked amino-acid change is a usage error; a region without a build is a usage error; a raw `--query` is passed as written and echoed; candidates are never collapsed to one.
- Proof: the design's protein-shorthand questions return the accepted record among the candidates; a region case with its build; a raw-query case; a two-page walk with no duplicates or gaps and a warm page with no request; valid zero versus failure; MCP parity; measured ceilings. Then make landing-gate runs through pm job after current main.
- Defers: the 0.9 advanced filters (0096).
