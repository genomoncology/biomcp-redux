# 0889 — orphanet-prevalence-on-disease-cards

Status: OPEN.

Milestone: 7

## Outcome

A disease card carries Orphanet's epidemiology: prevalence (type, value, class, geography, source, validation) and inheritance and age of onset, from Orphadata's open files, so a counselor can quote a cited prevalence to a family.

## Why

In the rare-disease persona run of 2026-11-13 (biomed-agent-bench `personas/rare-disease/runs/2026-11-13/log.md`, analysis beside it; code-mode run in `runs/2026-11-13-raw/`), BioMCP had no prevalence (`incidence` is SEER cancer sites only) (log.md Q10). The code-mode agent read Orphadata's `en_product9_prev.xml` and `en_product9_ages.xml` (CC BY 4.0) and quoted Kabuki syndrome's birth prevalence 1.16/100,000 and point prevalence 3.1/100,000 with PMIDs (raw log.md:215, 247-250).

## Proof

- Proof: a recorded case where MONDO:0016512 (via Orphanet:2322) carries the prevalence rows and inheritance in the recorded Orphadata files; a disease with no Orphanet link says so.

## Starts from

- Starts from: `crates/biomcp-adapters/src/disease_sources/`; Orphadata licence CC BY 4.0, attribution required.

## Keeps

- Keeps: SEER incidence for cancer sites.

## Defers

- Defers: patient-organization links (GARD's API host does not resolve; its pages are a JavaScript app).

## Changes

- Changes: a new Orphadata adapter, `crates/biomcp-query/src/disease/`, `crates/biomcp-types/`, the sources list, `tests/cases/disease/`, `docs/`

## Progress

- Measured live on 2026-11-13: `en_product9_prev.xml` is 16.2 MB (6,728 disorders, 17,108 estimates) and `en_product9_ages.xml` 6.9 MB (7,790 disorders); both are about 0.3-0.65 MB gzipped on the wire, `application/xml`, dated 23 Jun 2026 (the "July 2026" release on https://www.orphadata.com/epidemiology/). Each file states CC-BY-4.0 in its own `Availability/Licence` element; the legal notice and https://sciences.orphadata.com/faq/ confirm CC BY 4.0 with citation, and ask AI services to state that they use Orphanet/Orphadata data. No published rate limit.
- MONDO to ORPHA: MyDisease's MONDO record carries `mondo.xrefs.orphanet` (MONDO:0016512 lists `2322`); MONDO:0007254 breast cancer and MONDO:0007843 Kabuki syndrome 1 list none. BioMCP follows only these stated links.
- Built as a synced bundle like GTR and fda-cdx: new source `orphadata` (CC BY 4.0, attribution) with `biomcp sync orphadata` replacing both files atomically, a new `prevalence` disease section read with `--source orphadata` for a MONDO, DOID or Orphanet ID, and a `biomcp.disease-prevalence/1` reply. Sync refuses a file whose licence is not CC-BY-4.0. Each estimate keeps type, qualification, class, mean value, geography, source text with PubMed IDs, and validation; a class-only estimate's placeholder 0.0 is left out (6,350 of 6,358 class-only rows). The MyDisease card offers the prevalence command when MONDO states an Orphanet link. A concept with no link reports `orphadata` skipped with `no-join-id`.
- Live run: `sync orphadata` stored 14,518 rows from 23.1 MB; `get disease MONDO:0016512 prevalence --source orphadata` returns the four Kabuki estimates (1.16/100,000 at birth in New Zealand and Australia, 3.1/100,000 point prevalence in Europe, 3.1 at birth in Japan) with PMIDs, onset and inheritance.
- Case `tests/cases/disease/orphadata.yaml` replays both files cut to Kabuki syndrome and live MyDisease replies. `biomcp --record` cannot capture this sync: `sync_set` fetches through `leg.call`, which records under the ordinary 16 MiB limit, and `en_product9_prev.xml` becomes 17,053,377 bytes as JSON text (`crates/biomcp-fetch/src/bundle/set.rs:89-104`, `record.rs:21`; filed as its own ticket), so the Orphadata exchanges were cut from the live capture by hand, as for GenCC; provenance is in the replies README.
- First read (PM decision 2026-11-13: keep the 0836 ceilings): adding `orphadata` put typed at 2,010 characters (ceiling 2,009), hosted typed at 1,997 (1,996) and hosted single at 2,534 (2,518). Search disease no longer lists `orphadata`, since search never used it: its schema advertises ols, mydisease, monarch and seer, and get disease keeps all five. The typed and hosted headers each say `help` once for its four topics instead of twice. Measured after the merge of origin/main (gate job j39555 on a4da8a6e): typed 2,003 (ceiling 2,009), hosted typed 1,990 (1,996), hosted single 2,517 (2,518), single 3,537 (3,669), skill 2,580 (5,557). The ceilings in `scripts/check_front_map.py` are unchanged.
- Orphadata's terms (https://www.orphadata.com/legal-notice/, read 2026-11-13) ask that "Any use of our data distributed under the CC BY 4.0 licence in LLM/AI-based tools/services made available by the user to third parties should be accompanied by a statement explaining that Orphanet/Orphadata data has been used in the development of said tool/service but that any result based on this source data may not be a true reflection of the Orphanet/Orphadata data and as such Orphanet is not responsible for the quality of outputs based on our data." Every `orphadata` source row in a reply now carries that statement as its terms, with the legal notice as `terms_url`, and the case checks it. Orphanet data reached through other disease sources keeps the shared Orphanet component terms.
- Review follow-ups: PubMed IDs are now kept when the source text spaces them (`18442953 [PMID]`) or labels them (`Source: 33080481[PMID]`); the trailing digits before `[PMID]` are taken after trimming. The live file had 30 estimates that lost some or all PMIDs before. `tests/cases/disease/orphadata-synthetic.yaml` replays a synthetic pair with a class-only 0.0 row and both PMID forms, and the runner trial `disease-orphadata/other-licence-keeps-set` syncs that pair, then refuses a CC-BY-NC-4.0 prevalence file and reads the first set unchanged. `docs/cautions.md` says a MONDO Orphanet link can name a related or broader Orphanet entity.
