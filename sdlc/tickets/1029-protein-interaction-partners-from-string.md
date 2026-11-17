# 1029 — protein-interaction-partners-from-string

Status: OPEN.

Milestone: 8

## Outcome

`get protein ID` (or `get gene`) has a section listing STRING interaction partners with their combined scores and evidence channels, from STRING's public API, licence checked.

## Why

`sdlc/planning/sources-to-add.md:53`; comp-bio Q10 (bench `personas/comp-bio/runs/2026-11-16/analysis.md` 10.2).

## Proof

- Proof: a replayed case for WRN asserting the top partners and scores; a live check recorded in Progress.

## Starts from

- Starts from: an existing adapter of the same shape, and the source's own documentation and terms.

## Keeps

- Keeps: every existing command form.

## Defers

- Defers: nothing.

## Changes

- Changes: a new adapter or section, its help, schemas, cases, recordings and a release-notes line.

## Progress

- Licence, read 2026-11-17 at https://string-db.org/cgi/access: "All data and download files in STRING are freely available under the Creative Commons Attribution 4.0 (CC BY 4.0) license", including commercial use; "The only condition is attribution": cite STRING, say what was changed, and name STRING and its version in the metadata. Derived files may be redistributed with clear attribution. So our use and committing recorded replies are allowed; the source carries the `attribution` label, the reply names `string`, and the section carries STRING's `version`. Material STRING imports from other resources stays under those providers' terms; the API reply carries only STRING's own scores and names.
- API, read 2026-11-17 at https://string-db.org/help/api/: `/api/json/interaction_partners?identifiers=&species=&required_score=&limit=&network_type=&caller_identity=` returns rows with `stringId_A/B`, `preferredName_A/B`, `ncbiTaxonId`, combined `score` and channel scores `nscore` (neighborhood), `fscore` (fusion), `pscore` (phylogenetic co-occurrence), `ascore` (co-expression), `escore` (experiments), `dscore` (databases), `tscore` (text mining). Rules: wait one second between calls, no parallel calls, send `caller_identity`, and use a versioned address (`https://version-12-5.string-db.org`) in production. The quota is one call per second, one at a time.
- Live checks 2026-11-17 from the Beelink with curl (no build): `/api/json/version` gives 12.5. `interaction_partners?identifiers=Q14191&species=9606` accepts the UniProt accession directly and resolves to `9606.ENSP00000298139` (WRN). The default `required_score` is 400 (medium confidence); without `limit` STRING returns 10 rows, and `limit=0` returns every partner: 313 for WRN, 2,396 for TP53 (645 KB, 0.66 s). Rows come sorted by score, highest first, one row per partner, all with the same `stringId_A`. An unknown accession and an isoform accession (`Q14191-2`) both return 404 with `[{"Error":"not found",...}]`. Scores at the 12.0 address differ from 12.5, so the adapter pins 12.5.
- Design, following Complex Portal's protein card section: new `string` source (adapter `string_db`), new `interactions` protein section. One call reads the whole list (`limit=0`, `required_score=400`); the card keeps `total`, STRING's `version`, the asked protein's `string_id`, `min_score` 0.4, and the first `--section-limit` rows, each with `name`, `string_id`, `score` and the channels above zero. An isoform asks for its canonical accession; a 404 "not found" is an empty section. No paged command: rows past 25 are counted in `total`, not listed.
