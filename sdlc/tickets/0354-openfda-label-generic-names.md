# 0354 — Find openFDA labels by generic name

Status: COMPLETE.
Landed: eb9d41c82269dbdef499cc92db17635bdac57bb4

Milestone: 5

Depends on: 0387

## Outcome

`search drug NAME --source openfda-label` finds labels whose `openfda.generic_name` matches the name exactly, as well as labels whose brand name matches. `search drug daraxonrasib --source openfda-label` finds the RASONQUE label.

- Why: the docs lead's second-pass product findings (2026-10-27). The label search queries only `openfda.brand_name`, so a generic name such as daraxonrasib is absent although the label lists it. Clinicians search by generic name.
- Starts from: `openfda_label/mod.rs` query construction, the 0311 complete-brand equality ruling and the label map's `alternate_names`.
- Rules: generic matching uses the same complete-value equality as 0311 brands; no partial or fuzzy match. Each item says whether it matched by brand or generic name.
- Proof: a replay case for daraxonrasib returns the label with a generic match; a brand case still matches by brand; a partial name still does not match.
- Changes: the openFDA label adapter, its cases and recordings.
- Defers: nothing. Joins the 0336 refresh.
