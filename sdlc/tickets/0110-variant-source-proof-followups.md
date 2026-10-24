# 0110 — Capture positive CIViC, Cancer Hotspots and gnomAD release proof

Status: COMPLETE.

Milestone: 2

Depends on: 0090
Depends on: 0091
Depends on: 0089
Depends on: 0111
Depends on: 0112
Depends on: 0113

## Outcome

Three bounded captures from landed main close the variant source-proof gaps that gate roadmap boxes, never code landing:

- CIViC: a positive exact variant membership join, such as native Variant 12 (BRAF V600E, CA123643) and its molecular profile evidence, recorded and replayed after 0111. The landed capture proves only a native zero.
- Cancer Hotspots: an exact join between the variant's residue and alternate and a hotspot row, recorded and replayed after 0113 adds a transcript-qualified match. If 0113 closes without code, the capture proves a positive residue-level candidate row instead.
- gnomAD: release evidence captured with the same request as its counts, so `PopulationFrequency.dataset_version` can be typed, captured for 7-140753336-A-T after 0112. `gnomad_r4` is a moving selector, not a release, so landed counts stay native without a typed frequency.

- ClinVar: a real SCV whose classification is drug response, such as Variation 666267 with SCV000965609.1 (drug response to dabrafenib), after 0089 lands. The version in the future reply controls. It is recorded and replayed to show the label kept native with `statement_unmapped: true`. 0089 lands on a synthetic row.
- Proof: one pm job per source from landed main, keyless, with existing retries only; each success becomes a replay case (from outside the repo where terms bar storage) and ticks its roadmap box; each failure is recorded through pm progress with the box left unchecked. No refresh loops.
- Closed 2026-10-24 by PM: CIViC, gnomAD and ClinVar captures were accepted with clinical review. The Hotspots attempt stopped with `no_join_id` before any request, because CA123643's identity carried no gene; 0115 makes one candidate capture instead.


## Bounded source evidence

The four jobs built published main `60daa16ecbc1c824a0292c578dc61ed435640726`. Each ran one keyless command with an empty recording directory, isolated home, config and cache, and the existing transport retry policy. No refresh loop or additional target-source attempt ran. Each job exited zero; the Hotspots command returned a partial result rather than proving its source. No product code changed. The roadmap marks only the three successful source proofs; Hotspots remains unchecked.

| Source | Job | Command after `biomcp` | Observed result |
| --- | --- | --- | --- |
| CIViC | j11326 | `get variant CA123643 civic --section-limit 50 --format json --record DIR` | HTTP 200. The native simple molecular profile 12 contains Variant 12 with CA123643 and preserves separate evidence and assertion page actions. This proves only the returned bounded pages. The source supplies both IDs; neither was assumed from the other. |
| Cancer Hotspots | j11330 | `get variant CA123643 hotspots --section-limit 50 --format json --record DIR` | Only CAR was called. The resolved input supplied no usable gene, and Hotspots returned `no_join_id` before any Hotspots HTTP request. No positive candidate row was proved. Leave its source box unchecked. |
| gnomAD | j11332 | `get variant CA123643 population --section-limit 50 --format json --record DIR` | CAR and gnomAD returned HTTP 200. The actual response for `7-140753336-A-T` supplies exome AC 2, AN 1460618, and one VA `ALL` cohort with matching counts and `derivedFrom.version` 4.1.0. The output uses that observed release. Genome and VA genome are null. This proves the exome observation only. No allele frequency was calculated. |
| ClinVar | j11333 | `get variant clinvar:666267 clinvar --section-limit 50 --format json --record DIR` | HTTP 200. Actual VCV000666267.3, RCV000824811.1 and SCV000965609.1 retain the drug-response label. The SCV output preserves `statement_unmapped: true`, the source germline domain, the Dabrafenib response condition and the separate somatic observation origin. This does not claim full lossless SCV detail: the source TraitRelationship names Pancreatic Adenocarcinoma, which this output does not expose. |

### External evidence and request identity

All cited paths below are relative to Yellow's `/home/ian/workspace/data/biomcp-redux/0110-source-proofs/`. Raw bodies, actual recordings, output and replay stay outside Git. Every size and SHA-256 in each original manifest was checked after transfer and archive extraction on Yellow.

| Source | Evidence directory | Manifest SHA-256 | Files verified |
| --- | --- | --- | --- |
| CIViC | `biomcp-redux-0110-civic-gd6kih3m` | `2ab45e40ebd1ab454c0241abd9538482c652159eb94d75d83c33062e94d9c26a` | 7 |
| Hotspots | `biomcp-redux-0110-hotspots-yliwksky` | `378dad5e888bebbca5afa320a7b70883c70a2111e741d19efec91ae1cbd0f589` | 11 |
| gnomAD | `biomcp-redux-0110-gnomad-0ol5xk89` | `41a623573be1e9f40dd594ebbaca4e54d18b87fa70c20ed2f176492fb31fa1ef` | 10 |
| ClinVar | `biomcp-redux-0110-clinvar-0tmxuwon` | `161360b3a18e2a8f73a6f4286ec6e806533b0e6190a124e4fd13fe0f7490f962` | 9 |

Each directory has `live-output.json`, `live-stderr.txt`, `summary.json`, `proof-recipe.py`, `manifest.json` and nested `record/<source>/` files. The successful gnomAD and ClinVar directories also contain `replay-output.json` and `replay-stderr.txt`. Their live and replay output bytes match, with CLI exit zero for each execution.

The first CIViC summary incorrectly scanned the recording directory without recursing into its source subdirectory, so it reported no requests and skipped replay. The original record and manifest preserve that mistake. Job j11330 replayed that existing CIViC recording without another provider call; its `civic-offline-replay-output.json` equals the original live output, and `summary.json` records replay exit zero. This is not a second CIViC capture.

- CIViC: actual POST `https://civicdb.org/api/graphql`, fetched `2026-10-24T22:27:43Z`; recorded request-body SHA-256 `0b41acb1a9dfc7d7d0c2b1ba8c8d487042c5b38f99cb16a501a0f9371307472b`. Recording `record/civic/b730e62dc615795632478a0b16c721438d0652a4a113c754b228e601cef5afc5.json` has SHA-256 `f131bfb0457a0527027c9967edd1b05c28233ddea3b2c1ede0ea161e1b4ca76c`. Its recorded JSON value contains 1580 bytes with SHA-256 `2d23fb5e5d6034eb3369deac0298b8cc5065f2ab8ada0b7fa103ba608ccc2532`. Live output SHA-256 is `88b54fcc282e2a0d4ab0cb36f90a9279a33730dca6de287fd09a970bc4c62571`.
- gnomAD: actual POST `https://gnomad.broadinstitute.org/api`, fetched `2026-10-24T22:28:57Z`; recorded request-body SHA-256 `98f711dfa74777d15b620ba0c9d7542916f1ec776b69fcdebb07321ff8541d01`. Recording `record/gnomad/adb8b581a9d18af7397cc5defd00cd34fc93ebe4b7df7f1a58d78e2e4288455b.json` has SHA-256 `742b3a583574c98ebb2c0136626eb30ae123712fd15aa05b63b7ab089815ecee`. Its recorded JSON value contains 4324 bytes with SHA-256 `b1049123d561de8066745f4d87032f2e84e3fe02242e674c2537b02234fc078e`. Both output files have SHA-256 `910cc8afaa7ae5b04cd524137e4b3e592bbf8e4d3ecd6c97551ef11badf679c9`.
- ClinVar: actual GET `https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=clinvar&id=666267&rettype=vcv&is_variationid=true&retmode=xml&tool=biomcp`, fetched `2026-10-24T22:29:10Z`. Recording `record/clinvar/b0dc08f8b2e327a3cc14eea0aff96c83994e24add7db09654e86e8c3f801b4ae.json` has SHA-256 `f806ec35a3d4e12667835fa80b205fe9ad7f1e45a7c9cc1e56a0d0d162df0054`. The decoded XML text contains 26973 bytes with SHA-256 `1b82ed1a73d1eddd3c2e796e488ba3e5008508adef2ce445dfc3a1ab816339ba`. Both output files have SHA-256 `ac48b3b8eb0891f1d13616689d4361acb97addcff2b6711a43d14b0d569c8a55`.

The fresh clinical review accepts the three narrow successful source proof scopes. Hotspots remains unproved. The authorized four-job recipe is exhausted; there were no further provider calls. The coordinator owns any subsequent work. The roadmap preserves its approved OncoKB demo proof and direct dbSNP drop.

### Lesson

Inspect the recorder's nested source directories and the chosen command's actual input requirements before its one bounded call. A missing gene can stop a candidate-only Hotspots request before the target source runs; successful CAR resolution does not prove Hotspots coverage. Preserve that failure rather than replacing it with an invented transcript join or a repeated provider attempt.
