# 0179 — Keep GTR gene values that are not NCBI Gene IDs

Status: OPEN.

Milestone: 4

Priority: high. The full retained GTR bundle cannot sync on main.

## Outcome

A GTR gene row whose `gene_or_SNOMED_CT_ID` is not a positive NCBI Gene ID keeps its exact native value in a typed native identity field with no Gene pivot, and the rest of the bundle syncs. BioMCP does not interpret what such a value means.

- Why: full product sync j13077 on landed 6a7f1c4 failed with bad_reply. Delivery hashes and sizes matched both native bodies. Three captured rows carry `-1`: logical rows 580 (GTR000192572.11), 586 (GTR000192578.9) and 235542 (GTR000567741.1). The adapter parses every nonempty value as a positive NCBI Gene ID and rejects the whole pair.
- Starts from: the 0172 adapter and types; the witnesses and logs in Yellow data/biomcp-redux/0172-gtr-full-product-proof/j13077-postrun-evidence.tar.gz (SHA256 1b8b5dfae84c2e3997750e68f49168a4c1e9397871c1e64f7be2757e2f00e056) and main evidence 2291 (SHA256 d6d7c4ac48d6b82b8d3307e72918cd23ffc49c5b3837ce582ede344cabab8ba6).
- Rules: a positive integer stays an NCBI Gene ID with its Gene pivot. Any other nonempty value is kept exactly as sent in the typed native field, with no pivot, no next step to gene and no guessed meaning. An empty value stays absent. One unusual row never rejects the bundle; a malformed file structure still fails. Schema versions follow the compatibility check.
- Proof: outside-in cases first, built from the three retained source rows plus one positive neighbor, assert the native value, the absent pivot and a successful sync with zero provider requests. Each case has a size ceiling. No licensed content enters the repo. Then make landing-gate runs through pm job. After landing, one bounded run reruns the deferred full retained-bundle product proof from main, with counts and walks, and archives it to Yellow.
