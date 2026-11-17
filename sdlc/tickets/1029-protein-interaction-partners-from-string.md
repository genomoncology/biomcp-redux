# 1029 — protein-interaction-partners-from-string

Status: COMPLETE.
Landed: 95f584612b035a5ab13c6e3c2176dbecfe7bc8c6

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

- Licence, read 2026-11-17 at https://string-db.org/cgi/access: "All data and download files in STRING are freely available under the Creative Commons Attribution 4.0 (CC BY 4.0) license", including commercial use; "The only condition is attribution": cite STRING, say what was changed, and name STRING and its version in the metadata. Derived files may be redistributed with clear attribution. So our use and committing recorded replies are allowed; the source carries the `attribution` label, the reply names `string-db`, and the section carries STRING's `version`. Material STRING imports from other resources stays under those providers' terms; the API reply carries only STRING's own scores and names.
- API, read 2026-11-17 at https://string-db.org/help/api/: `/api/json/interaction_partners?identifiers=&species=&required_score=&limit=&network_type=&caller_identity=` returns rows with `stringId_A/B`, `preferredName_A/B`, `ncbiTaxonId`, combined `score` and channel scores `nscore` (neighborhood), `fscore` (fusion), `pscore` (phylogenetic co-occurrence), `ascore` (co-expression), `escore` (experiments), `dscore` (databases), `tscore` (text mining). Rules: wait one second between calls, no parallel calls, send `caller_identity`, and use a versioned address (`https://version-12-5.string-db.org`) in production. The quota is one call per second, one at a time.
- Live checks 2026-11-17 with curl (no build): `/api/json/version` gives 12.5. `interaction_partners?identifiers=Q14191&species=9606` accepts the UniProt accession directly and resolves to `9606.ENSP00000298139` (WRN). The default `required_score` is 400 (medium confidence); without `limit` STRING returns 10 rows, and `limit=0` returns every partner: 313 for WRN, 2,396 for TP53 (645 KB, 0.66 s). Rows come sorted by score, highest first, one row per partner, all with the same `stringId_A`. An unknown accession and an isoform accession (`Q14191-2`) both return 404 with `[{"Error":"not found",...}]`. Scores at the 12.0 address differ from 12.5, so the adapter pins 12.5.
- Design, following Complex Portal's protein card section: new `string-db` source (adapter `string_db`; the key `string` would collide with the word "string" in transport code, which a ratchet forbids), new `interactions` protein section. One call reads the whole list (`limit=0`, `required_score=400`); the card keeps `total`, STRING's `version`, the asked protein's `string_id`, `min_score` 0.4, and the first `--section-limit` rows, each with `name`, `string_id`, `score` and the channels above zero. An isoform asks for its canonical accession; a 404 "not found" is an empty section. No paged command: rows past 25 are counted in `total`, not listed.
- The deferral this ticket reopens: `sdlc/decisions/2026-10-26-protein-pathway-study-rulings.md:8` deferred STRING because P42771 and Q8N726 (CDKN2A's two products) map to one STRING ID, and said "A later research question that needs STRING reopens it"; comp-bio Q10 is that question. Re-checked live 2026-11-17: both still map to `9606.ENSP00000418915` with identical partners. The reply makes the loss of specificity explicit: `string_id` names the entry, its description and `docs/cautions.md` say one gene's products share one entry, and the case asserts the shared `string_id` for both. Scores are described as functional association confidence, not physical binding (`sdlc/planning/slices/protein.md:197`). The old `get protein ... interactions` and `protein interactions|associations` retirement routes are removed; the `ProteinString` message stays in the error enum because removing a value would break the 1.0 schema baseline, and a new message value was refused by `schema-release-check` (j42199), so `protein interactions` is now an ordinary unknown command.
- j42193 (5a2afc841): built; live calls hit the old retirement route, so nothing recorded. j42199 (4d1b49b5b): STRING replies were all refused as bad replies because STRING serves `text/json` and the JSON reader admitted only `application/json`; the fetch codec now also admits `text/json`. The same job found the structural ratchets (parent module with logic, `deny_unknown_fields`, a `fn text` helper outside `mapped.rs`, and the source key `string` appearing as a word in transport code), the mcp-parity 4,096-byte cap on `schema biomcp.source/1` (4,106; the variant doc is now just "STRING."), and the protein-sections fixture, which now answers STRING with `[]` and counts four requests. Live `help get protein` is 2,160 bytes and `get protein Q14191 all` 9,293 bytes.
- j42208 (e9983717): live recording for the case: `get protein Q14191 interactions --section-limit 3` returned 313 partners, led by RPA3, RPA1 and FEN1 at 0.999; P42771 and Q8N726 both returned `string_id` 9606.ENSP00000418915 (670 partners each); `--disable-sources string-db` gives a skipped section; `health` lists `string-db` as CC BY 4.0, attribution, ok. The case `tests/cases/protein/string-interactions.yaml` replays those UniProt and STRING replies, kept whole with `provenance.json`. j42211 (8f0d965e): regenerate, check-cheap, types, schema-release and the protein, help, mcp and local cases passed.
- Merged origin/main (338880db): main's new `who_eml` source also took registry order 61, so STRING moved to 62; j42214 regenerated the essential-medicines schema with the new source.
- Gate j42216 on 32275b55 (`make --keep-going landing-gate docs-check schema-release-check`): passed.
- Review follow-ups (coordinator, 2026-11-17): `tests/cases/protein/native-sections.yaml`'s `get protein P00533 all` step now replays a live STRING reply for P00533 recorded in j42220 (1,456 partners, SOS2 first) and asserts the interactions section's state, total and first partner; that replay already lacked InterPro, so its domains section stays `unavailable` (`replay-miss`), as before this ticket. Added a `text/json` row to the transport failure-reason table (it passes the media check and reaches the size limit); the file's 600-line cap took a blank line. The `limit=0` comment now says it was observed live, not documented; the cautions say an empty section means STRING does not know the protein or lists no partner at 0.4 or above; `ProteinString` carries a trailing note that it is unsent and kept for the 1.0 schema (`error.rs` is at its 600-line cap); the 2026-10-26 protein ruling notes the reopening by 1029.
- Review follow-up jobs: j42220 recorded P00533's STRING reply (check-cheap there failed only on the transport file's line cap, fixed); j42221 (a37bcc33) check-cheap, transport and protein cases passed; gate j42222 on a37bcc33 passed.
- Land gate j42225 failed because main gained 1031's study-copy-number schema without `string-db`. Merged origin/main 1c682ac65 (clean); j42230 regenerated schemas and docs (only study-copy-number changed), and check-cheap, types, schema-release and the protein, study, drug, mcp and help cases passed. No schema review entry was needed. Gate j42232 on 6aae8c24 passed.
