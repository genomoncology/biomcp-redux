# 0185 — Search WHO prequalified diagnostics by marker and manufacturer

Status: COMPLETE.
Landed: 16fc1324f15e26338295bb1e0cab804f913c9db9

Milestone: 4

## Outcome

`search diagnostic --source who-ivd --marker HIV` and `--manufacturer NAME` search the synced WHO IVD bundle, as 0.9's `search diagnostic --disease HIV --source who-ivd` did, and return a five-row page of WHO product headers, each with its exact WHO Product ID and its `get diagnostic ID --source who-ivd` step.

- Why: 0.9 searched WHO's prequalified diagnostics by pathogen, disease or marker, and the WHO lists came from public issue 235. 1.0 keeps only the exact product card from 0173, so a user who does not already know the WHO Product ID loses the search. 0.9 is the quality floor.
- Starts from: 0173's WHO IVD bundle and card; diagnostic.md's WHO rows that defer search; 0.9's WHO IVD search over the `pathogen/disease/marker` and `manufacturer name` columns.
- Rules: `--marker` matches the native `pathogen/disease/marker` field and `--manufacturer` matches `manufacturer name`, both case-insensitive whole-word, with no synonym expansion. Both may combine conjunctively. 0.9's `--disease` with `--source who-ivd` names `--marker`. Rows group by WHO Product ID in source order. Pages use `--limit` and `--offset` with a measured default. A read never downloads.
- Proof: outside-in cases first, watched to fail, over the retained 0173 bundle. Cases assert an HIV marker page, a manufacturer page, both combined, a verified no-match, the `--disease` retirement message and the page walk with no gap or duplicate. CLI and MCP outputs are measured with ceilings at measured +15 percent. diagnostic.md and the ledger replace the deferral. No provider call. Then make landing-gate runs through pm job.
- PM ruling 2026-10-25: search diagnostic help ceilings approved at measured +15%: 2667 stdout and 2666 compact bytes (measured 2319/2318). The growth comes from the WHO source, marker and manufacturer guidance. Whole help keeps its cap.
