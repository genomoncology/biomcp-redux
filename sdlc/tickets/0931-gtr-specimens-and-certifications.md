# 0931 — gtr-specimens-and-certifications

Status: COMPLETE.
Landed: 172dd7035fa404d07e24642071a1d60c9195c9cc

Milestone: 7

## Outcome

A GTR test card, and a search row where cheap, carries the test's specimen types and its lab's certifications (CLIA and CAP numbers) as NCBI publishes them. The card says where they came from.

## Why

The persona needed specimen and accreditation columns for its test menu and validation packet; BioMCP's GTR card had neither, so it went to E-utilities (log.md:58, 77-80), and code mode filled specimen on 21 of 21 rows from esummary (raw log.md:79-82, 320-322). Evidence: bench `personas/dx-lab-director/runs/2026-11-14/` (j40302, BioMCP f4f726674) and its code-mode pair `runs/2026-11-14-raw/` (j40303); the analyst's source checks are listed at the end of `analysis.md`. NCBI's esummary for test 560508 lists five specimens and CAP 7154701; BioMCP reads `test_version.gz`, which has no specimen column. The same check found that the bulk file leaves the gene cell empty for one current EGFR test (GTR000575663, ARUP's ctDNA T790M test), which E-utilities indexes; note whether the chosen source fixes that too.

## Proof

- Proof: a recorded NCBI reply for test 560508 gives its five specimens and CAP 7154701 on the card.

## Starts from

- Starts from: `crates/biomcp-adapters/src/gtr/`; NCBI E-utilities esummary for GTR (`db=gtr`) and GTR's other bulk files. Check live which source holds specimens and certifications and its terms; prefer a bulk file over per-test calls if one has them.

## Keeps

- Keeps: the 1.0.0 schema rules (optional fields only); GTR search counts and paging; no network reads on search if the current search is offline.

## Defers

- Defers: CPT codes (GTR's field is empty upstream: esummary `cptcode ""`).

## Changes

- Changes: the GTR adapter, its card and row types, cases.
- Changes (PM, 2026-11-14, option B approved within this ticket): the shared bulk download in `biomcp-fetch` gains a streamed gzip-XML part format with its own size limit, so GTR's set adds `gtr_ftp.xml.gz`. The XML is decoded as a stream and never held whole; only specimen rows (matched to exact test accession and version) and lab certification rows (with expiry dates) are kept. The existing 128 MiB ceiling for other sets and parts stays. The weekly GTR sync grows from about 57 MB to about 241 MB.
- Not in this ticket: taking genes from a test's listed variants (see GTR000575663 below).

## Progress

Design check, 2026-11-14 (before the PM chose option B).

- `test_version.gz` (read today) already has `CLIA_number`, and the card already shows it (`clia_number 05D0981414` for 560508). It has no specimen or CAP column.
- No small GTR bulk file has specimens or CAP. `/pub/GTR/data/` holds `GTR_test_IDs.csv`, `labs_tests_by_country.txt`, `test_condition_gene.txt/.xlsx`, `test_version.gz`, `test_version_microbe.gz`, `tests_by_method_category.txt` and `gtr_ftp.xml.gz`; `xml_archive/` keeps monthly copies of the XML.
- `gtr_ftp.xml.gz` has both: `GTRLabTest/Specimen/SpecimenType` per test (exact `GTRAccession` + `Version` attributes) and `GTRLab/LabCertification` per lab (Type, Name, Number, Effective/Expiration date). For 560508 v11 it lists the same five specimens as esummary, and lab 61756 lists `CAP 7154701` (expires 2026-10-26) plus CLIA and five state licenses. It is 184 MB gzip, 1.89 GB decoded; 63,256 tests, 32,865 with specimens, 341 labs (170 with CAP, 200 with CLIA). The extracted specimen and certification values are about 4.3 MB.
- The bundle layer cannot take it as is: the whole set is capped at 128 MiB (`MAX_BUNDLE_BYTES`), parts are held and decoded whole in memory, and native TSV decoding is capped at 256 MiB. Taking the XML needs a streamed gzip-XML part format with its own ceiling in `biomcp-fetch`.
- E-utilities `esummary db=gtr` (and `efetch`, which returns the same summary) has `specimens`, `certifications` (CAP 7154701 for 560508; CLIA, NYS CLEP, CAP and ISO15189 for 575663) and `testtargetlist`. It takes the numeric ID without a version and describes the current version only. It batches several IDs per call.
- Terms: the FTP README says GTR data is public domain and asks for attribution to GTR; E-utilities falls under the same NCBI policies already in `SPEC.terms`.
- GTR000575663 (ARUP EGFR T790M ctDNA): the XML has no test-level gene either; EGFR appears only as the gene of the test's listed variant (`FoundIn`). esummary's `testtargetlist` has EGFR. The XML would fix the empty gene cell only if genes were derived from listed variants.

Build, 2026-11-14 (option B).

- `biomcp-fetch` has a new part format, `RowFormat::StreamedXml`, with its own ceilings (512 MiB gzip, 8 GiB decoded). The gzip XML is decoded as a stream through `Row::stream_xml`; only the rows the source emits are kept. The 128 MiB `MAX_BUNDLE_BYTES` ceiling is unchanged and still covers every other part. The compressed body is still held in memory like every other part; the 1.8 GiB decoded XML never is. The XML part skips the transport's body check and is decoded once, after download; other parts keep their check as before.
- GTR's set adds `gtr_ftp.xml.gz` (limits 384 MiB gzip, 4 GiB decoded). The GTR reader keeps `GTRLabTest/Specimen/SpecimenType` keyed by `GTRAccession.Version`, and each lab's `LabCertification` entries of `Type="Certification"` (CLIA, CAP, ISO15189, NYS CLEP, EMQN and others) with name, number and expiration date. State licenses are left out because the card already has them from `test_version.gz`; with them a lab could carry 53 entries (6.4 KB), without them at most 18 (1.3 KB). Test-specific `TestCertification` entries and research tests are not read.
- Reply: `header.specimens` on cards and search rows; `laboratory_certifications` and `xml_release_url` (the file they came from) on the card. All optional, so the 1.0.0 schema rules hold.
- Search rows: specimens added 51-58 bytes per row on average in the EGFR and KMT2D replays (most tests list none; the largest list is 457 bytes). The live 50-row EGFR page went from 43,501 to 47,085 bytes (+8.2%); search time stayed 0.03 s. Name-only pages read the page's specimen rows by their own key, so they never pull a test's relation rows. Case caps that grew follow measured size plus 15%.
- Live sync on the M5 (job j40798 against j40774 on the pre-change code `206b8d689`, both from NCBI): 244.7 MB downloaded against 60.3 MB, 43.7 s against 20.6 s, peak memory footprint 1.33 GB against 1.37 GB, 770,125 rows against 736,989.
- Where the memory goes (resident memory sampled every 0.2 s, j40812 branch against j40813 on `206b8d689`): both peak at the very end, when the whole set is written to the store: 1,889 MiB on the branch, 1,897 MiB on main. That peak was already there on main; it comes from the two TSV parts, whose 737k rows are all kept in memory until the set is published. On the branch the TSV parts finish by about 14 s at about 1.7 GB, as on main. The XML phase (about 17-37 s) then runs at 1.70-1.80 GB, which is the TSV rows plus the 184 MB compressed XML body, buffered whole like every other part. The decoded 1.8 GiB XML is never held, the parser holds one element at a time, and the 33,136 kept XML rows (32,865 tests with specimens and 271 labs with certifications) are a few MB. Streaming the compressed body to disk would lower only the XML phase, not the peak; lowering the peak means writing every part's rows to the store as they are parsed, which is a change to the shared bulk download beyond this ticket. The 120 s sync budget still has room. The live card for GTR000560508.11 shows the five specimens and CLIA 05D0981414 (expires 2026-05-29) and CAP 7154701 (expires 2026-10-26).
- GTR000575663.2 (ARUP EGFR T790M ctDNA): the card now shows CLIA, NYS CLEP, CAP 4096301 and ISO15189, but no specimens (the XML lists none) and still no gene count. The XML names EGFR only as the gene of the test's listed variant; taking genes from listed variants is not in this ticket.
- Cases: `diagnostic/gtr-specimens.yaml` replays the EGFR recording with the XML cut to those tests' labs and specimens (values unchanged) and proves the five specimens and CAP 7154701 on the card and specimens on both search paths. The synthetic GTR case checks entity decoding, that a state license is left out and that a bad accession is a refused element. `crates/biomcp-fetch/tests/bundle/streamed.rs` checks that a decoded-size overflow is refused and keeps the prior set.
- Review fix: a stored set now records its streamed parts' bytes as well (GTR's is about 245 MB), and copying a stored set from an older cache file refused anything over 128 MiB, which would have failed every GTR read after a cache-format change until a manual sync. The copy now accepts up to the held ceiling plus the streamed ceiling (`bundle/transfer.rs`); only a streamed part can push a set past 128 MiB. `crates/biomcp-fetch/tests/bundle/streamed.rs` copies a 245 MB-sized set with a streamed part from an older cache.
- The measurements above came from a temporary job script (live sync with `/usr/bin/time -l`, and a memory sampler every 0.2 s against `206b8d689`), since removed; its runs are jobs j40774, j40798, j40812 and j40813.
