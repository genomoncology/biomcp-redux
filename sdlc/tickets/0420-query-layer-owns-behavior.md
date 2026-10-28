# 0420 — Keep query behavior in the query layer and wire syntax in adapters

Status: OPEN.

Milestone: 5

Depends on: 0419

## Outcome

The binary only parses, calls `biomcp_query::run` and spells the result. Drug label bounding, step ordering and caps, step sources for every entity, disease section support and bundle sync move into biomcp-query. Provider query syntax and parameters (Europe PMC, PubMed field tags, ClinicalTrials.gov and NCI parameters) and ChEMBL and Open Targets mapping move into their adapters, whose raw types become private. LitVar's rsID check uses `biodata::RsId`.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`. Rules stated in two layers drift: only article and author follow-ups drop disabled sources, and the query layer returns a card for an unsupported disease section.
- Starts from: `crates/biomcp/src/command/{drug_label,steps,step_sources,disease,unsupported,sync}.rs`, `crates/biomcp-query/src/article`, `trial`, `drug/native/identity.rs`, and the named adapters.
- Rules: behavior stays the same except the two drift defects, which get cases; no new framework.
- Proof: existing cases pass unchanged; a case shows disabled sources dropped from follow-ups for a non-article entity; a direct query call for an unsupported disease section refuses.
- Changes: the files above and cases.
- Defers: nothing.
