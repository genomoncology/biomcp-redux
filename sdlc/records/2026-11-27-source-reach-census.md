# Source reach census

Date: 2026-11-27. BioMCP 1.0.0 at 4efd1dcb2426. A census of how much of each source's own records BioMCP can reach, run after a six-question pilot of the CIViC MCP benchmark showed that about 23% of CIViC's reviewed evidence had no BioMCP route (tickets [1202](../tickets/1202-civic-non-allele-profiles.md) and [1203](../tickets/1203-civic-drug-submitted-evidence.md)). CIViC itself is not repeated here.

## Method

For each source the census started from the source's own data model, API or release, not from BioMCP's commands. It listed the source's record types and entry points, mapped each to a BioMCP command or marked it as having none, counted what the source holds (its own count endpoints, small release files, or a random sample whose size is stated), and probed awkward records live: records without the usual ID, rare types, unreviewed or withdrawn records, records with no linked gene or drug. It looked for four failure shapes from CIViC: a record type with no route; a join key that silently limits reach; a reply that says `complete` or `empty` when BioMCP could not look; and a default filter nobody asked for. Three code reads covered the same sources for hard-coded filters, dropped rows and status rules.

Limits. FDA sources were read through BioMCP only, so their totals are BioMCP's own. No licensed release (OncoKB, COSMIC, OMIM) was fetched. Large files were sampled, not scanned. Nothing here judges whether a source's content is clinically right. Counts are as of the date above. The failing case of every ticket's first witness below was run again by the census lead against the binary and, where the source has an open API, against the source; other rows rest on one helper's run and say "read from code" or "sample of N" where that is so.

## Result in one table

| Source | Reach measured | Worst gap |
| --- | --- | --- |
| ClinVar | About 94.7% of 4,714,028 records by ClinVar ID; totals match ClinVar | 249,528 records with no aggregate classification read as `complete` and `empty` though they hold submissions |
| ClinGen Evidence Repository | 68.3% of 13,968 expert-panel records | 4,384 records with no summary document ID return zero rows |
| ClinGen validity and dosage | Gene-first: 100 of 100 random genes match the downloads | Disease-first by OMIM or Orphanet ID returns nothing; actionability (463 curations) has no route |
| GenCC | Gene-first counts match the export | 71.8% of rows were submitted under OMIM or Orphanet IDs, which the disease-first route does not match |
| CPIC | Guidelines and recommendations 100%; pairs 574 of 635 | Diplotypes (112,820 rows) only by exact spelling and allele order; six tables have no route |
| ClinPGx | Gene routes match the source; 95.1% of clinical annotations | 254 annotations with no gene, some level 1A, are unreachable; no variant-first or drug-first route |
| GWAS Catalog | Every total compared matched the catalog | About 18.6% of associations lose their variant (no rsID); 42,583 unpublished studies have no route |
| Reactome | Human pathway, event and participant counts match exactly | Genes with no UniProt accession (70 microRNAs) read as having no pathways |
| WikiPathways | 1,131 human pathways by ID | 94 pathways lose every gene under `complete`; no route from a gene |
| ClinicalTrials.gov | Totals equal the registry's for all 613,799 studies, by type, status, phase and results; no hidden default | Obsolete NCT numbers; help's sponsor example returns 1 of 3,051 |
| NCI Clinical Trials Search | Status counts sum to its 27,775 trials | Minor: page cap and problem noise |
| gnomAD | 83% of 178 sampled SNVs by VCF-style ID; HGVS form reaches the rest | Same allele is `absent` by one input form and present by another; mitochondrial and structural variants read as empty |
| MyVariant.info carried fields | 47% of sampled ClinVar records carry dbNSFP | COSMIC and CADD are empty on the default GRCh38 build |
| cBioPortal | Mutation and copy-number counts match on the cases tried | Every chromosome X variant counts as zero under `complete`; structural-variant profiles (8.8%) have no route |
| HGNC and MyGene | 66 of 68 sampled symbols across all locus types resolve; the two misses are honest | None serious |
| UniProt | The 20,429 reviewed human entries, with full feature totals | Out-of-scope accessions say `failed` as if the source were down |
| InterPro | Not measured broadly | A protein with 26 entries fails; unintegrated signatures are dropped |
| openFDA label and Drugs@FDA | Not countable without FDA's files | Vaccines and cell and gene therapies read as having no United States record |
| FDA Purple Book | Reference products that have biosimilars | A biosimilar's own name, and a reference product without biosimilars, return zero |
| FDA device sources | 244 companion diagnostics listed in full | A PMA with over 100 rows returns no card |
| FAERS, MAUDE, VAERS | Not countable | FAERS counts any drug role and does not say so |
| ChEMBL | All 41 sampled molecules resolve by ChEMBL ID | 15.4% of parent drugs read as having no targets because the rows sit on a salt |
| Open Targets | 12 of 26 sampled drugs | Every drug without a mechanism of action fails |
| Drug identity (MyChem.info) | 32 of 38 sampled names | The default lookup fails for common antibodies |
| Monarch, MONDO, HPO | 43 of 43 sampled MONDO diseases return cards; 24 of 24 empty sections checked were truly empty | An obsolete MONDO ID returns an empty section |
| Orphadata | 85.3% of 11,645 disorders | The 1,715 disorders with no MONDO mapping fail |
| PubMed, Europe PMC, PubTator, LitVar | Counts compared matched | LitVar reads as no literature when no rsID was picked; PubTator species entities are dropped |

## Lessons

1. **The failure is general, not a CIViC accident.** Every cluster had at least one record class that BioMCP's own entities did not anticipate: records with no aggregate classification, documents without an ID, microRNAs, biologics, salts, positional variant names, unpublished studies, disorders without a cross-reference. Mapping a source must start from the source's list of record types, with a count beside each, and end with a route or a written "no route" for every one.
2. **A join key is a reach limit, so measure it.** Each route needs some ID before it can ask: an allele ID, an rsID, a UniProt accession, a MONDO ID, a parent ChEMBL ID, a document ID. The share of the source's records that lack the key is the share BioMCP cannot reach. That share was never measured before a route shipped. It ranged here from 0.6% (obsolete NCT numbers) to 31% (repository records without a document ID).
3. **"Could not look" must never read as "nothing there".** The most harmful findings are all this shape: `complete` with zero rows, `empty`, `absent` or `no-record` when no question was put to the source, or when rows came back and could not be compared. A reply needs three distinct outcomes: the source was asked and holds nothing; the source holds something that could not be matched (return it as a labelled candidate or count it); the source was not asked (say why). Any of the last two makes the reply `partial`. Some routes already do this well, such as the variant-to-GWAS section's `no-join-id`.
4. **A scope rule must not look like an outage, and an outage must not look like a scope rule.** Out-of-scope proteins return `failed`, `bad-reply`; a valid pathway file returns `bad-reply`; a disabled source returns `no-record`. The reason codes exist; they are applied to the wrong cases.
5. **Fixed choices belong in help and in the reply.** The gnomAD dataset, the MyVariant assembly, the FAERS drug role, the cBioPortal sample list, the Purple Book's reference-only search, Monarch's inclusion of descendant diseases and CPIC's removed pairs are each fixed in code and stated nowhere a caller sees. ClinicalTrials.gov and ClinVar show the opposite, good case: no default filter, totals equal to the source's.
6. **Two routes to one source must agree.** The census found the same source sorted two ways (GWAS), matched two ways (gnomAD by input form), and scoped two ways (Monarch genes and phenotypes), as CIViC's drug and variant routes disagreed on status.
7. **Tests drawn from famous records cannot find these.** The fixtures lean on BRAF V600E, TP53, DPYD, imatinib, chromosome 7 and ClinVar variation 13961. Every gap above sits in a class with no fixture. Test records should be drawn from the source's release by class, including the classes with few members, and the expected value should come from the release, not from what BioMCP returned when the fixture was recorded.
8. **A deliberate limit still needs to be said.** Several gaps were chosen in earlier tickets (exact diplotype strings, gene-first pharmacogenomics, reference-only Purple Book search, the 100-row PMA limit, `not-applicable` pathways for non-coding genes) and their tests assert the limiting behaviour. A choice to limit reach is fine when the reply states it; none of these did.
9. **Where the outside-in method had already been used, reach was complete.** Trial search, ClinVar row counts, CPIC recommendations, ClinGen gene-first lookups, Reactome counts, STRING, DDInter and UniProt features all matched their source exactly. The gaps cluster where a route was built for a handful of bank questions.

## Tickets, ranked by harm to a user who trusts the reply

Harm here means: how likely a careful reader is to draw a wrong conclusion from a reply that looks clean, weighted by how often the record class occurs. All are open; none is built.

| Rank | Ticket | Gap |
| --- | --- | --- |
| 1 | [1204](../tickets/1204-erepo-rows-without-document-id.md) erepo-rows-without-document-id | About 31% of the repository's expert-panel classifications come back as zero rows today, so an agent reads a classified variant as unclassified. |
| 2 | [1205](../tickets/1205-clinvar-records-without-classification.md) clinvar-records-without-classification | ClinVar holds 249,528 records (5.3% of 4,714,028) whose review status is "no classification provided"; each has real submissions, and all of them read as `complete` and `empty` today. |
| 3 | [1206](../tickets/1206-drug-lookup-fails-for-antibodies.md) drug-lookup-fails-for-antibodies | The default lookup fails today for pembrolizumab, nivolumab, adalimumab, rituximab, bevacizumab and denosumab, so the most common oncology biologics have no default drug card. |
| 4 | [1207](../tickets/1207-recurrence-counts-chrx-as-zero.md) recurrence-counts-chrx-as-zero | cBioPortal writes chromosome X as `23`; today every chrX variant gets `observed_sample_count: 0` under `status: complete`, which reads as "never seen". |
| 5 | [1208](../tickets/1208-complete-status-when-a-section-could-not-look.md) complete-status-when-a-section-could-not-look | The census found the CIViC failure shape, "could not look" reported as "nothing there", in four more places; an agent that reads `status` alone treats each as a clean negative. |
| 6 | [1209](../tickets/1209-gnomad-population-misses-held-alleles.md) gnomad-population-misses-held-alleles | The same allele returns data by HGVS and `absent` by VCF-style ID today, because some forms are resolved through MyVariant.info's index, which lacks many gnomAD exome-only variants. |
| 7 | [1210](../tickets/1210-chembl-rows-on-salt-forms.md) chembl-rows-on-salt-forms | ChEMBL records many mechanisms on the salt; a lookup by the parent's ID reads as "no targets" today. |
| 8 | [1211](../tickets/1211-opentargets-drug-without-mechanism-fails.md) opentargets-drug-without-mechanism-fails | Open Targets sections fail outright today for any drug without a mechanism, and `search drug` lists those drugs, so search leads to a failing get. |
| 9 | [1212](../tickets/1212-gene-validity-by-omim-or-orphanet-id.md) gene-validity-by-omim-or-orphanet-id | A disease-first lookup by OMIM number reads as "no curated genes" today, though 71.8% of GenCC submissions were made under OMIM or Orphanet IDs. |
| 10 | [1213](../tickets/1213-cosmic-cadd-empty-on-default-build.md) cosmic-cadd-empty-on-default-build | MyVariant's GRCh38 index carries neither COSMIC nor CADD, so the default build returns `empty` for the best-known somatic variants. |
| 11 | [1214](../tickets/1214-gwas-variants-without-rsid.md) gwas-variants-without-rsid | An estimated 18.6% of the catalog's 1,206,717 associations name their variant by position; today those rows come back with no variant at all. |
| 12 | [1215](../tickets/1215-wikipathways-genes-lost.md) wikipathways-genes-lost | Ninety-four human pathways lose every gene today under `status: complete`, and six valid pathways read as a source failure. |
| 13 | [1216](../tickets/1216-pathways-for-genes-without-protein-id.md) pathways-for-genes-without-protein-id | Reactome maps 70 human microRNAs to 27 pathways; today each reads as "no pathways", with Reactome marked `ok` though it was never asked. |
| 14 | [1217](../tickets/1217-cpic-diplotype-spelling-and-order.md) cpic-diplotype-spelling-and-order | CPIC's 112,820 diplotype rows are reachable only by an exact string today, and help's own example misses. |
| 15 | [1218](../tickets/1218-pgx-annotations-without-a-gene.md) pgx-annotations-without-a-gene | 254 of ClinPGx's 5,192 clinical annotations (4.9%) name no gene, including level 1A ones, and every PGx route today starts from a gene. |
| 16 | [1219](../tickets/1219-gwas-trait-name-matching.md) gwas-trait-name-matching | A capital letter fails the whole page today, and a common short name reads as "no associations". |
| 17 | [1220](../tickets/1220-litvar-variant-without-rsid.md) litvar-variant-without-rsid | A LitVar search for one of the commonest EGFR alterations reads as "no literature" today. |
| 18 | [1221](../tickets/1221-gnomad-dataset-scope-stated.md) gnomad-dataset-scope-stated | The dataset is fixed to gnomAD r4 in code, so a mitochondrial variant gnomAD publishes reads as "no variant record". |
| 19 | [1222](../tickets/1222-fda-biologics-center-products-read-absent.md) fda-biologics-center-products-read-absent | An FDA-licensed CAR-T therapy reads today as having no United States record. |
| 20 | [1223](../tickets/1223-purple-book-search-biosimilar-names.md) purple-book-search-biosimilar-names | A biosimilar's own name returns zero rows today with a notice that reads as if the product were newer than the file. |
| 21 | [1224](../tickets/1224-orphanet-only-disorders.md) orphanet-only-disorders | 1,715 of Orphadata's 11,645 disorders (14.7%) have no MONDO mapping and fail today. |
| 22 | [1225](../tickets/1225-trial-obsolete-nct-alias.md) trial-obsolete-nct-alias | A reader who holds an older NCT number from a paper gets a refusal or a clean zero today. |
| 23 | [1226](../tickets/1226-trial-sponsor-name-match.md) trial-sponsor-name-match | Help's own example, `--sponsor "MD Anderson"`, returns one study today; the registry lists 3,051 under "M.D. Anderson Cancer Center". |
| 24 | [1227](../tickets/1227-trial-phase-filter-exact.md) trial-phase-filter-exact | Help says "Exact source phase", but `--phase 1` includes 17,251 phase 1/2 studies. |
| 25 | [1228](../tickets/1228-uniprot-out-of-scope-accessions.md) uniprot-out-of-scope-accessions | A scope rule reads as a source outage today, and 306,643 unreviewed human entries plus every non-human entry return `failed`, `bad-reply`. |
| 26 | [1229](../tickets/1229-interpro-domains-fail-on-valid-proteins.md) interpro-domains-fail-on-valid-proteins | A valid protein with 26 InterPro entries fails today as a source error. |
| 27 | [1230](../tickets/1230-clingen-actionability-route.md) clingen-actionability-route | ClinGen publishes 463 adult and pediatric actionability curations and BioMCP has no route to any. |
| 28 | [1231](../tickets/1231-gwas-unpublished-studies.md) gwas-unpublished-studies | 42,583 of the catalog's studies (17.2% of all study records) are unpublished, and each reads as not found today. |
| 29 | [1232](../tickets/1232-clinvar-entry-by-rcv-scv.md) clinvar-entry-by-rcv-scv | Papers and reports cite RCV and SCV accessions; today both are refused, and large copy-number and range variants can be reached only by a VCV number nothing in BioMCP can find. |
| 30 | [1233](../tickets/1233-cbioportal-structural-variant-profiles.md) cbioportal-structural-variant-profiles | 226 of cBioPortal's 2,573 molecular profiles (8.8%) are structural-variant profiles, the only cBioPortal record of fusions, and none is reachable. |
| 31 | [1234](../tickets/1234-faers-drug-role-stated.md) faers-drug-role-stated | A FAERS count is read as "reports about this drug"; today it includes reports where the drug was only concomitant, and nothing says so. |
| 32 | [1235](../tickets/1235-pma-large-application-card.md) pma-large-application-card | Long-lived devices, the ones with the most history, return no card today. |
| 33 | [1236](../tickets/1236-who-eml-name-matching.md) who-eml-name-matching | Paracetamol reads as not on the Essential Medicines List today. |
| 34 | [1237](../tickets/1237-gwas-variant-section-sort.md) gwas-variant-section-sort | The first page of the variant section today can omit the strongest associations, and the two routes disagree. |
| 35 | [1238](../tickets/1238-car-protein-only-alleles.md) car-protein-only-alleles | A protein-only description, the form most reports use, fails today as a source error. |
| 36 | [1239](../tickets/1239-pubtator-species-and-cell-line.md) pubtator-species-and-cell-line | Two of PubTator's six entity types are dropped today with no problem and no notice. |

Gaps found and not ticketed, because they are depth of a record rather than reach, or small: GenCC evidence fields (PMIDs, report links) are not returned; UniProt comment types beyond function and location; Human Protein Atlas returns 5 summary labels of 136 fields; EMA product flags and referrals; Monarch's descendant scope is undisclosed; disease IDs from MeSH, UMLS and ICD-10 return zero rows on search; Cellosaurus search rows lack the contamination warning; 46 Evidence Repository curations have no allele ID; ClinVar genotype records lose their subject; several ClinicalTrials.gov fields and the results "more information" module are not shown. Each is in the tables below and named in a ticket's Defers line where one applies.

## Proposed standing check

Two layers, because the landing gate must stay offline and fast and most reach numbers move with the sources.

**Layer 1, in the landing gate: pinned slices, offline, under a minute in total.** Small files taken from each source's own release are replayed through the real decoders and mappers. One assertion for all: for every class of record in the slice, the reply returns the rows the slice holds, or refuses with a stated reason and a status that is not `complete`; no class returns `complete` with zero rows. Sources and slices:

- ClinVar: one recorded record per variation type and per review status, including a no-classification and an included-only record.
- ClinGen and GenCC: the validity, dosage and region downloads whole (about 1.7 MB) and a GenCC sample by submitted disease namespace.
- ClinGen Evidence Repository: a sample of records with and without a document ID.
- CIViC: the census test tickets 1202 and 1203 already require.
- WikiPathways: the human gene file and pathway list (under 1 MB) against a pinned set of pathway files.
- Reactome: a slice of its UniProt, Ensembl and miRBase mapping files joined to HGNC.
- CPIC: the pair table and a diplotype sample looked up in both allele orders.
- cBioPortal: recorded mutation rows for genes on chromosomes X, Y and MT.
- ClinicalTrials.gov: the registry's published value lists for status, phase, study type and intervention type, each of which must parse.
- Shipped lists: every name in the Purple Book file and the WHO Essential Medicines list resolves to its own row.

Cost: fixtures of a few megabytes in the repository; an estimated 30 to 60 seconds added to the gate; no network. Open-licence sources only; ClinPGx rows stay out of the repository under its noncommercial terms.

**Layer 2, a weekly scheduled job: live counts against a recorded floor.** For each source, read the source's own totals and a random sample per record class, run the sample through the built binary, and fail when the reach share drops below the last recorded figure or a new enum value or record class appears. Estimated run times at one to two requests per second: ClinVar 3 minutes; ClinGen, Evidence Repository and GenCC 10; CPIC 8; ClinPGx 2; GWAS Catalog 20 to 30; Reactome 10 to 15; WikiPathways 5; ClinicalTrials.gov 3; NCI 2 (needs its key); gnomAD 15; ChEMBL 3 to 15; Open Targets 15 to 45; drug identity 25; Orphadata 10; UniProt and InterPro 4. That is about two and a half to three and a half hours of polite wall time a week if run in sequence, a few thousand requests, and no money. FDA sources can be checked only by sampling through BioMCP itself, since the test harness must not read FDA sites; their layer is a fixed list of awkward products (a vaccine, a cell therapy, a biosimilar, a discontinued-only drug, a PMA with over 100 rows) with expected states.

Recommended order to build: Layer 1 for ClinVar, the Evidence Repository, ClinGen and GenCC, WikiPathways and cBioPortal first, since those hold the highest-harm gaps and their slices are small and openly licensed; then the weekly job for ClinVar, GWAS, ChEMBL and drug identity. Each ticket above names its census in its Proof where one applies, so the checks arrive with the fixes.

## Not checked

- Suspect-only FAERS counts, Drugs@FDA and label totals, and anything else that would need FDA's own files.
- OncoKB and AlphaGenome content; only how they report themselves.
- gnomAD structural variants and repeats live; that finding is read from code.
- The full ClinVar release; shares come from ClinVar's count filters and a sample of 1,123.
- Whether the six sampled ClinGen Allele Registry misses (6 of 100 ClinVar spellings) are the registry's or BioMCP's.

## Tables by source

### ClinVar

Totals are E-utilities `esearch` counts (one Variation ID is one VCV record) taken on 2026-11-27 against ClinVar build 2026/11/24, which reports 4,714,028 records; shares are estimates from a random sample of 1,123 live records (about plus or minus 3 points at 50%).

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| VCV (variation archive) | 4,714,028 | `biomcp get variant clinvar:N clinvar`, `VCV000000000`, bare `N`; `biomcp variant assertions clinvar:N --source clinvar` | Rows returned for about 94.7% (4,714,028 minus 249,528 evidence-only minus 715 included-only = 4,463,785). Variation 13961 matched ClinVar exactly (88 rows = 45 SCV + 38 RCV + 5 VCV area rows) | Requested version ignored: `biomcp get variant VCV000013961.100 clinvar` returns version 143 with no notice (ClinVar holds version 100 with 39 submissions). Retired ID reads complete and empty: `biomcp get variant clinvar:1373707 clinvar` (77 of 1,200 random IDs, 6.4%, have no record) |
| Evidence-only records (review status "no classification provided") | 249,528 (5.3%) | same | 0%; 11 of 11 tried returned empty. Sample share with no label in any area was 6.1% (68 of 1,123) | Complete but empty: `biomcp get variant clinvar:4517766 clinvar` (status complete, section empty; ClinVar holds 1 SCV and 1 RCV); `biomcp variant assertions clinvar:4517766 --source clinvar` returns absent, exit 1. Cause: the mapper in `crates/biomcp-adapters/src/clinvar/map.rs` lists three classification areas and skips the rest without a problem. The code read adds that an SCV with no classification element, and any classification group outside the three known areas, also yield zero rows with no problem (inferred from code) |
| Included-only variations (inside a haplotype or genotype only) | 715 (probably all of them; 2 of 2 sampled were included) | same | 0% | Failed with a generic reason: `biomcp get variant clinvar:242709 clinvar` (status failed, reason bad-reply); also `clinvar:242673` and `biomcp variant assertions clinvar:242709 --source clinvar --kind vcv`. ClinVar answers normally; `crates/biomcp-adapters/src/clinvar/raw.rs` requires a classified record. Classified haplotype members work (`clinvar:242724`) |
| RCV and SCV (condition record and submission) | Not counted separately; variation 13961 has 38 RCV and 45 SCV | Rows inside the VCV reply (`--clinvar-kind`) | Reached through the parent VCV only | No accession route: `biomcp get variant RCV000014992 clinvar` and `biomcp get variant SCV000061601 clinvar` are usage refusals (exit 2). Lower-case `vcv000013961` is refused |
| Small variants (SNV 4,297,772; deletion 197,224; duplication 86,222; indel 21,377; insertion 16,480; microsatellite 41,554) | as listed | `biomcp get variant` by any ID form or exact HGVS | 98.0% of sampled VCVs have a small-variant HGVS spelling (1,101 of 1,123). 6 of 6 sampled small variants with no ClinGen or dbSNP link resolved by HGVS | Whole-exon or range deletions cannot be named: `biomcp get variant NM_000251.2:c.1387-?_1510+?del clinvar` fails (car rejected); 4 of 43 sampled deletions were of this form |
| Copy number gain and loss | 25,965 and 24,142 | VCV or `clinvar:N` only | 1.2% of the sample (13 of 1,123); about 90,000 VCVs have no small-variant HGVS by extrapolation (gain and loss alone 50,107) | `biomcp get variant NC_000001.10:g.(?_11078770)_(11217368_?)dup clinvar` failed, exit 4; `biomcp search variant --region chr8:133267699-134578639 --build GRCh37` failed; the ClinVar title as input is a usage error. By VCV it works (`biomcp variant assertions clinvar:3392078 --source clinvar` gave 3 rows) |
| Other types: inversion 1,669; translocation 287; complex 106; fusion 5; haplotype 640; protein only 93; "Variation" 982; tandem duplication 1 | as listed | VCV | Sampled records returned data; haplotypes come back as one record with members | Fusion `clinvar:15248` returned 3 unmapped rows with no subject. Inversion HGVS spellings untested |
| Diplotype (genotype set) | 596, all drug-response | VCV | Rows come back | All rows unmapped with no genotype subject: `biomcp get variant clinvar:633883 clinvar` (9 of 9). The mapper treats any non-haplotype as a single allele (inferred from code; live replies agree) |
| Classification values and review statuses | Review: no assertion criteria 136,491; single submitter 3,352,318; multiple submitters no conflict 768,279; conflicting 183,959; expert panel 23,051; practice guideline 663. Somatic origin 10,862; oncogenic text hits 1,937 | Default, no filter | All areas and statuses returned by default | Drug response, risk factor, association, other, not provided and protective come back as unmapped rows with the native label but no `subject` (`clinvar:5066`, `83027`, `16609`, `495317`, `97974`) |
| Identifier gaps | No rsID: sample 39.6% (445 of 1,123). No ClinGen allele ID: 39.8%. Multi-gene 11.8%. GRCh37 only 1.1%. Mitochondrial 3,138. Y 3,938 | VCV, `clinvar:N`, HGVS; rsID where one exists | 7 of 7 sampled records with neither ClinGen nor rsID still resolved by HGVS | Multi-allelic rsID: `biomcp get variant rs113488022 clinvar` is partial with `no-join-id` and a mislabelled `value.unknown` problem code; `biomcp variant assertions rs113488022 --source clinvar` fails; `biomcp get variant rs74517255 clinvar` pairs the ClinVar ID with the chrY copy only. `chrM` is refused, `chrMT` works. The code read adds that the variant section takes the first ClinVar ID while the assertions command refuses unless there is exactly one (inferred from code) |
| Gene-level counts | Per gene, for example BRAF 1,723 | `biomcp get gene BRAF clinvar` | Totals agree with ClinVar for BRAF 1,723, SHOX 496, MT-TS1 43, DNAAF11 436 | Counts only, germline only (BRAF has 69 somatic-origin, 25 oncogenic-text and 15 Tier I records not counted). No list route: `biomcp search variant --gene BRAF` is refused; no classification filter |
| Other: MyVariant carried ClinVar block; CAR identity noise | MyVariant's ClinVar fields | `biomcp search variant ... --fields myvariant` | Used for filters and for the `clinvar:N` ID | The `myvariant` field is null in every reply tried. Every CAid or HGVS reply carries `representations.expression` and `representations.build` `value.unknown` problems, which makes a complete reply look noisy |

Honest behaviour confirmed: totals matched ClinVar on every check; no default filter on review status or classification; somatic and oncogenicity rows returned by default; paging states the total and a `next` step; haplotype records carry their members; unmapped labels are kept natively; RCV and SCV refusals name the accepted forms; rsID candidates without a join are labelled `unavailable`.

Not checked: a scan of the full variant summary file; separate RCV and SCV totals; the ClinVar-side dbSNP filter (53.9% have one) disagrees with the sample (60.4% have a dbSNP link) and was not reconciled; the 715 included-only count comes from a review-status term and was confirmed on 2 records only (the code read could not find a live ID in three tries); inversion HGVS spellings; genotypes other than the drug-response diplotypes; CAR alleles that list two ClinVar IDs.

### ClinGen gene validity and dosage

Counted on 2026-11-27 from the source's own downloads (validity CSV rows after the banner, gene dosage CSV and TSV lists, region TSV), then compared with live replies for 100 random genes.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Gene-disease validity | 3,741 rows, 3,087 genes, 2,192 MONDO diseases. Definitive 2340, Limited 555, Moderate 462, Disputed 201, Strong 84, No Known Disease Relationship 50, Refuted 49 | `biomcp get gene X clingen`; `biomcp get disease MONDO:N gene-validity [--source clingen]` | Gene-first 100% (100 genes, 0 mismatches). Disease-first by MONDO ID: every row reachable, 4 of 4 diseases matched. By OMIM or Orphanet ID: 0% | `biomcp get disease OMIM:182212 gene-validity` is complete with 0 items while `MONDO:0008426` returns 11 (ClinGen 2, GenCC 9); all 3,741 ClinGen rows are MONDO, so the OMIM/Orphanet miss is mostly GenCC (see GenCC). No hierarchy: `biomcp get disease MONDO:0007254 gene-validity` matches 0 ClinGen rows (18 sit under MONDO:0016419) |
| Vocabulary and ordering | ClinGen says "Refuted", GenCC says "Refuted Evidence" | `--classification` | Exact match only | `biomcp get disease MONDO:0016248 gene-validity --classification refuted` returns ClinGen's 3 and shows GenCC count 0 with state ok. `biomcp get disease MONDO:0044970 gene-validity --limit 50` returns 50 ClinGen rows and no GenCC (GenCC starts at offset 235 of 495) |
| Gene dosage sensitivity | 1,720 genes in the CSV; 1,541 rows in the detail TSV | `biomcp get gene X clingen` | Text labels 100% of the sample; score and PMID block for 1,540 of 1,720 = 89.5% | 180 genes (10.5%) show a label but no evidence block, with state ok: `biomcp get gene CASK clingen`, `ARSL` (175 of the 180 have an evaluated HI or TS) |
| Dosage regions | 516 rows (HI score 40: 359; 0: 91) | `biomcp search gene ISCA-NNNN`, `--band B`, `--region chrN:a-b`, `--regions-only` | 7 of 7 ISCA probes exact; band `16p11.2` returned the 6 regions the TSV lists; interval `chr16:29600000-30200000` returned the 1 overlap. Band reach 515 of 516 and coordinate reach 508 of 516 are from the TSV, not run live for each | Name search finds nothing: `biomcp search gene "16p11.2 recurrent region"` is complete with 0 rows (source holds ISCA-37486 and ISCA-37400); 59 region names contain "includes GENE" and `biomcp get gene TBX6 clingen` does not show its region. 8 regions have location `tbd` and 1 has no cytoband. The code read adds that interval and band searches silently skip rows with an unparseable location or band, and band search misses cross-arm spans (inferred from code) |
| Recurrent-CNV BED track | 179 intervals | `none` | Not measured | Names and coordinates only, no scores |
| Empty gene section | 51 of the 100 sampled genes have no validity and no dosage | `biomcp get gene RGS5 clingen gencc` | n/a | ClinGen section shows `state: data` with no rows, while GenCC shows `state: empty` for the same case |
| Other: genes with no Entrez ID | n/a | gene section | n/a | Dosage evidence status is absent without a problem (inferred from code); symbol-only rows are refused with a problem |

Honest behaviour confirmed: gene-first counts equal the downloads; old symbols resolve (GBA to GBA1, PARK2 to PRKN); coverage states `matched 0`; location `tbd` and missing HI scores pass through as the source has them; score 40 comes through.

Not checked: the gene dosage "downloadall" file (no response); the recurrent-CNV BED beyond a name comparison; all 516 regions by live call; the "Animal Model Only" class (not in the export).

### ClinGen Evidence Repository

Counted on 2026-11-27 from the repository's paged summary list (14 pages of 1,000).

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Germline variant interpretations (expert panels) | 13,968 records, 13,909 distinct CAids, 49 panels. VUS 4156, Pathogenic 3764, Likely Pathogenic 3011, Likely Benign 1656, Benign 1381. 3,924 records have more than one version | `biomcp variant assertions CAxxx --source erepo`; `biomcp get variant CAxxx erepo`; `biomcp get variant clinvar:N erepo` | 68.3% of records (9,541 of 13,968) and 68.6% of variants (9,536 of 13,909). 16 of 16 records with a document ID returned the right row; 18 of 18 without one returned total 0 | 4,384 records (31.4%) have no `PCERDocID` and are dropped: `biomcp variant assertions CA16609653 --source erepo` is partial with `value.unknown` and total 0. 4,373 CAids reach nothing; 9 lose one of two documents (`CA024693` shows Pathogenic and hides Likely Pathogenic). Worst panels: Ocular Anterior Segment, Gastric Cancer and Potassium Channel Arrhythmia 100%; Limb Girdle Muscular Dystrophy 405 of 432; Myeloid Malignancy 542 of 1,662; 2025 records 2,500 of 3,735. Cause: `crates/biomcp-adapters/src/erepo/raw.rs` makes the document ID required |
| Records with no CAid (exon deletions, large duplications, delins) | 46 (Myeloid Malignancy 13, Familial Hypercholesterolemia 11) | `none` (route needs a CAid) | 0 of 46; ClinVar-ID route 0 of 2 tested | `biomcp variant assertions clinvar:217107 --source erepo` fails with "requires one CAid"; `biomcp get variant clinvar:217107 erepo` and `clinvar:89065` are absent with no word that a curation exists |
| Records with no ClinVar ID | 836 (300 with a CAid and a document ID) | `biomcp variant assertions CA346329946 --source erepo` | Works | `biomcp get variant CA346329946 erepo` returns the row with an empty identity |
| Criteria met or not met, version history | `metCodes`, `unMetCodes`, `versionsList` in every record | `biomcp variant assertions CAxxx --source erepo --assertion PCER... [--version V]` | Works (CA395136420; CA2573051038 shows v1.2.0 Uncertain Significance against current v2.2.0 Likely Pathogenic) | List rows omit criteria and versions and do not say older versions exist. `--version` without `--assertion` is a usage error by design |
| Somatic or oncogenicity curation | None; all records are germline | n/a | n/a | Nothing missing |
| Other: gene, condition or panel listing | present in source | `none` | n/a | Dropped by design (code read) |

Honest behaviour confirmed: dropped rows give `partial`, not `complete`; version history and criteria work; records with `retracted` null (PTEN `CA000559`) work.

Not checked: the all-classifications stream (timed out); the classifications download file.

### ClinGen Allele Registry

No release count exists (hundreds of millions of alleles, no small download), so reach was judged from live probes of the registry and the tool.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Canonical allele (CA ID) | Not countable | `biomcp variant normalize --hgvs H --source car`; `biomcp get variant CAxxx` | Transcript c. and GRCh37 genomic probes resolved to CA123643 | One `genomicAlleles.hgvs` `value.unknown` problem on normalize; `representations.expression` and `.build` problems on most CAR-resolved replies |
| Protein-only allele (PA ID) | For example PA094029 for `NP_004324.2:p.Val600Glu` | `biomcp variant normalize --hgvs 'NP_004324.2:p.Val600Glu' --source car` | 0 of 1 | Failed with `@id value.unknown`; the registry returns PA094029 and matching transcripts. Cause: `crates/biomcp-adapters/src/allele_registry/raw.rs` accepts only CA IDs. `--source all` resolves it through Mutalyzer |
| Large variants and exon-level HGVS | CAR rejects over 10,000 bp and `?` boundaries | same | Failed with the source's reason shown (`VariationTooLong`, `HgvsParsingError`) | Honest |
| Promoter and 5' UTR by transcript position | `NM_198253.3:c.-124C>T` rejected by all three resolvers at source | `biomcp get variant NM_198253.3:c.-124C>T`; `variant normalize --source all` | Failed for all three | Reply says failed with no reason text (sources name `IncorrectHgvsPosition` and `EOUTOFBOUNDARY`); `NM_000518.5:c.-78A>G` also failed with no reason |
| Resolver differences | Three resolvers (CAR, Mutalyzer, VariantValidator) | `variant normalize --source car or mutalyzer or variantvalidator or all` | Old transcript version `NM_004333.3`: CAR unavailable, Mutalyzer resolved (2 mappings), VariantValidator resolved (9). `NC_000017.11:g.43077335_43077336dup`: CAR unavailable, Mutalyzer resolved (10). Mitochondrial `NC_012920.1:m.12015T>C`: CAR and Mutalyzer resolved, VariantValidator unavailable | `unavailable` is used both for "source returned no record" and "source cannot take this class". A Mutalyzer-only identity has no CAid, so every later section is `unavailable` with `no-join-id` (`--resolver mutalyzer`) |
| CAid coverage of ClinVar HGVS | n/a | CAR REST | 94 of 100 random ClinVar SNV or indel IDs returned a CAid (SNV 84 of 89, indel 10 of 11) | The 6 misses were not diagnosed |
| Other | n/a | n/a | n/a | Transcript forms are requested for MANE only; enrichment of a CAR-resolved variant joins only through GRCh37, so a GRCh38-only allele gets no enrichment and no problem (inferred from code) |

Honest behaviour confirmed: CAR's own limits are shown with the source's reason; `variant normalize` shows each source's outcome separately.

Not checked: CAR counts (no small release).

### ClinGen actionability

Counted on 2026-11-27 from the source's adult and pediatric summary downloads.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Actionability curations (adult and pediatric) | Adult 259 (257 gene-condition, 2 variant-condition, 8 retracted), 155 documents; pediatric 204 (202 + 2), 135 documents; assertion tables 3,540 and 3,671 rows; about 309 distinct gene symbols | `none` | 0% | No help text, source name or code mention. `biomcp get gene FGF23 clingen gencc` shows no validity, no dosage and nothing about actionability, and cannot say it was not checked |
| Other: criteria specifications | n/a | `none` | 0% | No code anywhere (code read) |

Honest behaviour confirmed: none; the findings report no statement of absence for this source.

Not checked: nothing further listed.

### GenCC

Counted on 2026-11-27 from the export in the form the adapter reads (30,388 rows); the adapter's local copy held 30,383 rows (retrieved 2026-11-20) with no stale hint.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Submissions | 30,388 rows, 6,112 genes, 8,144 MONDO diseases, 19 submitters. Strong 11314, Definitive 5814, Supportive 5070, Limited 4414, Moderate 3152, Disputed Evidence 307, No Known Disease Relationship 258, Refuted Evidence 59. Disease as submitted: OMIM 16,737, MONDO 8,575, Orphanet 5,076 | `biomcp get gene X gencc`; `biomcp get disease MONDO:N gene-validity [--source gencc]` | Gene-first 100% (100 genes, 0 mismatches; FGFR3 53 of 53, TP53 12 of 12, FGF23 9 of 9). By MONDO: 4 of 4 diseases matched. By the OMIM or Orphanet ID the submitter used: 0% | Complete but empty: `biomcp get disease OMIM:182212 gene-validity` (GenCC holds 6 rows filed under that ID) and `biomcp get disease Orphanet:676 gene-validity` (3 rows). 21,813 of 30,388 rows (71.8%) were filed under OMIM or Orphanet; the table in the findings says 71.7% for the same share. Cause: `crates/biomcp-adapters/src/curation.rs` compares the requested ID only with the MONDO column |
| Submitter evidence fields | PMIDs 23,178 rows (76%); report URL 14,427 (47%); criteria URL 29,141 (96%); notes 4,206 (14%); original disease ID for all | `none` (not returned) | 0% of those fields | The adapter keeps 12 fields (`crates/biomcp-adapters/src/gencc.rs`) |
| Gene with no GenCC rows | n/a | `biomcp get gene X gencc` | `state: empty` | Honest |

Honest behaviour confirmed: gene-first counts equal the download for every gene tried; the name route works (`biomcp get disease "Shprintzen-Goldberg syndrome" gene-validity` gives 11 items); symbol-only rows are refused with a problem; the gene section marks every gene unavailable if any read failed (conservative, from code).

Not checked: a GenCC submission filed under OMIM or Orphanet with no MONDO cannot exist in the export, so it could not be probed.

### CPIC

Counted on 2026-11-27 from CPIC's API with the exact-count header and a one-row limit.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Gene-drug pairs | 635 rows (61 removed). Level C 284, D 134, A 96, B/C 56, B 36, C/D 15, A/B 9, Retired 5 | `biomcp search pgx --gene G --source cpic [--cpic-level L]` | 574 of 635 = 90.4% (the view drops removed pairs); 121 genes, each reachable; provisional pairs (255 of 574) returned and flagged | Removed pairs are hidden without a word: `biomcp search pgx --gene CYP2A7P1 --source cpic` gives total 0. Case slips read as empty: `--gene g6pd` and `--cpic-level c` give complete, 0 rows. A gene CPIC lacks reads complete with no identity: `biomcp get pgx --gene BRAF --source cpic` (CPIC `gene` has 132 rows, no BRAF) |
| Drug-first entry | 324 drugs (301 in the view) | `none` | 0% | `biomcp search pgx --drug warfarin --source cpic` is `usage.retired` (the source lists 7 genes for warfarin); `--drug-id` needs a key such as `RxNorm:`, names are refused (568 of 574 pairs are RxNorm; MT-RNR1 uses `RxNorm:NA`) |
| Guidelines | 29 guidelines, 35 genes | `biomcp get pgx guidelines --gene G --source cpic` | 29 of 29 | `biomcp get pgx guidelines --gene CYP1A2 --source cpic` is empty with no pointer to ClinPGx, which holds two DPWG annotations |
| Recommendations | 2,159 rows, 130 gene-drug combinations, 109 drugs, 19 genes; 1,432 rows (66%) name two or more genes | `biomcp get pgx recommendations --gene G [--drug-id RxNorm:N]` | 2,159 of 2,159; CYP2D6 1,360 = source | `biomcp get pgx recommendations --gene CYP4F2` is empty with no reason; the notice for `--drug-id RxNorm:4053` says "lists no G6PD pair" though a removed row exists |
| Diplotypes | 112,820 rows, 19 genes (RYR1 60,378; G6PD 17,765; CYP2D6 16,836; DPYD 3,570) | `biomcp get pgx diplotype --gene G --diplotype EXPR --source cpic` | Exact string only; 0 rows reachable without the exact spelling | `biomcp get pgx diplotype --gene CYP2D6 --diplotype '*4/*1'` is empty (the source is too; it holds `*1/*4`). `biomcp get pgx --gene CYP2C19 --source cpic diplotype --diplotype '*2/*1'` is empty while `*1/*2` works. The help example `biomcp get pgx --gene DPYD --source cpic diplotype --diplotype '*1/*2A'` is empty; CPIC spells it `c.1905+1G>A (*2A)/Reference`, and a test asserts the empty reply as expected |
| Genes | 132 | `biomcp get pgx --gene G --source cpic` | 132 of 132 | See BRAF above |
| Gene results (phenotype terms) | 101 and 208 | `none` | Not reachable as a list | |
| Alleles and definitions | 1,378 alleles, 1,354 definitions; 22 genes | `none` | 0% | No route returns allele function or definition |
| Allele frequencies and populations | 391,760 and 4,012 | `none` | 0% | Deliberately deferred |
| Test alerts | 2,213 | `none` | 0% | |
| Publications | 1,155 | `none` (pair rows carry PMIDs only) | 0% direct | |
| Other: drug as a record; change log, file artifacts, statistics | drug 324 | `none` (drug works only as a filter) | n/a | 23 drugs sit only in removed pairs |

Honest behaviour confirmed: totals matched rows in every probe (pairs 79 of 79, CYP2C9 25 of 25, recommendations 1,360); provisional and level C/D pairs are returned and labelled; no default level or status filter; empty drug-key lookups carry a notice naming the pair or phenotypes CPIC has; multi-gene lookups agree (TPMT and NUDT15 both 35); copy-number and fusion allele names work as exact strings; paging states totals; rows failing identity checks emit a problem and make the reply partial (inferred from code).

Not checked: counts for the allele location and sequence location tables; whether drug names differ in spacing or case.

### ClinPGx

Counted on 2026-11-27 from the ClinPGx API, which refuses a request with no criterion, so counts are per evidence level and per agency.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Clinical annotations | 5,192 (level 1A 314, 1B 18, 2A 46, 2B 25, 3 4,479, 4 310) | `biomcp get pgx annotations --gene G --source clinpgx --kind clinical` | 4,938 of 5,192 = 95.1% (those with a gene). CYP2C9 74 = 74; CYP2D6 122 = 122 | 254 (4.9%) have no gene (1 level 1A, 244 level 3, 9 level 4): `biomcp get pgx annotations --gene rs12777823 --source clinpgx` fails not-found while PA166134655 (level 1A, warfarin) exists. Reachable only by `--annotation-id` |
| Annotation detail | One per accession | `biomcp get pgx --annotation-id PA166134655 --source clinpgx` | Works for any accession | Only when the user already holds the accession |
| Guideline annotations | 214 (CPIC 79, DPWG 113, CPNDS 6, RNPGx 16) | `--kind guideline` | By gene, all sources (SLCO1B1 14 = 14; DPWG-only F5 and CYP1A2; CPNDS UGT1A6; RNPGx ABCB1) | Gene-less share not measurable from the minimal view |
| Label annotations | 1,462 (FDA 542, EMA 340, PMDA 52, HCSC 399, Swissmedic 129) | `--kind label` | By gene, all five agencies (DPYD 12 = 12; BRAF 29 = 29) | No agency or drug filter |
| Filters | n/a | all routes need a gene symbol | n/a | No drug, level, agency or variant selector: level 3 is 86% of annotations and a user pages 50 at a time. Default page is 3 clinical rows and `total_kind` is `observed` |
| Variant annotations | 1,155 for CYP2C9 alone | `none` | 0% | |
| Pathways, VIPs, haplotypes, phenotypes, automated annotations, drugs, genes | not counted | `none` | 0% | `biomcp search pgx` rejects `--source clinpgx` by design |
| Other: sweep for dropped rows | n/a | n/a | No drop in 9 gene reads | Refused rows would give a partial reply with an observed total; a collection over 4 MB would give an incomplete problem (inferred from code; not hit) |

Honest behaviour confirmed: totals always matched the source count; levels 3 and 4 are shown and labelled; `--gene` with `--annotation-id` is a usage error by design; an unknown symbol says not-found.

Not checked: total variant annotations; pathway, VIP and haplotype counts; the gene-less share of label and guideline annotations; whether the full view differs from the minimal one; the HTTP 429 and 500 failure paths; a sweep of all 5,192 rows. ClinPGx is noncommercial, so bulk downloads were not fetched.

### GWAS Catalog

Totals come from the REST API (`page.totalElements`), data release 2026-11-17, API v2, on 2026-11-27. Class shares are estimates from random samples: 14,000 associations (28 pages of 500, 1.16% of 1,206,717) and 200 studies.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Published studies | 247,699 (196,925 with full summary statistics; 50,774 without) | `biomcp get gwas GCST...`; `biomcp search gwas --pubmed N --studies` | Any known GCST or PMID. No route by trait, gene or text. Sample of 200: 149 (74.5%) have zero curated associations; 164 (82%) have summary statistics | `biomcp search study GCST000035` is complete and empty (cBioPortal only); `biomcp search gwas --study GCST90559242` gives total 0 with no hint that the study is summary-statistics-only |
| Unpublished studies | 42,583 (17.2% of study records) | `none` | 0% | `biomcp get gwas GCST91007236` is absent or not-found while the catalog serves it at its unpublished-studies endpoint; `biomcp search gwas --study GCST91007236` is complete with 0 |
| Associations | 1,206,717 (639,668 in studies with full summary statistics; 567,049 without) | `biomcp search gwas` with `--gene`, `--variant`, `--trait`, `--trait-id`, `--study`, `--pubmed`; `biomcp get gwas association:ID`; `biomcp get variant rsID gwas` | rsID-bearing about 81.4%; mapped gene about 81.2%; EFO trait 100%; PMID 100%. Totals matched the API in every case tried (FGFR2 212; HLA-DRB1 2,086; paging to offset 14,040 worked) | No hidden p-value cut (10.1% of sampled rows are above 5e-8, 32.2% have effect allele `?`) |
| Non-rsID variant identifiers (`chr:pos`) | About 18.6% (2,607 of 14,000) | `--variant` rejects them | Reachable by study, PMID, trait or gene, without the allele | Allele dropped with `value.unknown`: `biomcp get gwas association:117751220` is complete with a problem; `biomcp search gwas --trait "Alzheimer disease" --limit 5` is partial and its top row (p 9e-734) lacks `chr19:44908684`; `biomcp search gwas --variant chr4:6965877` is a usage refusal. Cause: `crates/biomcp-adapters/src/gwas/map.rs` |
| Associations with no mapped gene | 18.8% (2,636 of 14,000) | Not reachable by `--gene` | 0% by gene | Reachable by rsID (`biomcp get variant rs771354939 gwas`). Author-reported genes exist only in `loci` |
| Haplotype and SNP-by-SNP interaction associations | Haplotype about 0.03% (4 of 14,000; about 350 estimated). Interaction 0.29% (41 of 14,000; about 3,500 estimated) | Reachable by any member rsID | Reached | Not labelled: `biomcp get gwas association:15710339` (5-SNP haplotype) and `association:62069681` show alleles like independent SNPs; the flags are absent from the mapped schema |
| Two routes for one rsID | rs2981582; rs7903146 has 353 associations | `biomcp get variant rs2981582 gwas --section-limit 5` against `biomcp search gwas --variant rs2981582 --limit 5` | Same data | The variant route is unsorted: first row p=1e-12 and the p=2e-76 row is fifth; search sorts p ascending. For rs7903146 the variant route starts at p 1e-11 and search at p 3e-1315 |
| Traits | 24,496 | `--trait` (exact label), `--trait-id`, `--include-child-traits` | Studies' trait mappings: EFO 55%, OBA 26%, MONDO 15%, HP 4%, GO 0.5% | Free text: `biomcp search gwas --trait "type 2 diabetes"` is complete with 0 (the label "type 2 diabetes mellitus" has 9,655). Children excluded by default: `--trait-id MONDO_0004975` 6,852 against 7,149 with children; `MONDO_0004992` 397 against 27,077. Retired ID `EFO_0000305` is complete with 0 (404 at the source). Lower-case `mondo_0004989` fails bad-reply (source returns 2,094). A label in another case fails the whole page: `biomcp search gwas --trait "Breast carcinoma" --limit 3` (source returns 2,094) |
| SNPs, genes, parent mappings | SNPs 570,476; genes 38,441 | exact rsID only; `--gene` | Associations reached; SNP and gene cards `none` | `biomcp search gwas --gene FGFR2 --extended-geneset --limit 1` fails deadline-exceeded |
| Publications and ancestries | 7,907 publications | `--pubmed`; `biomcp get gwas GCST... ancestries` | Any known PMID | PMID 18438407 is absent in the catalog (404); unpublished ancestries `none` |
| Other: BioMCP gene and disease cards | n/a | n/a | n/a | No `next` pivot to `search gwas` from `biomcp get gene FGFR2` or `biomcp get disease MONDO:0004989` |

Honest behaviour confirmed: totals matched the API for every case tried; `--p-value` is opt-in and states the count is before the cut; no hidden ancestry, assembly or date filter; gene match is case-insensitive; timeouts and bad IDs are reported as failed with a reason; a haplotype row survives an rsID search because any member allele matches (from code).

Not checked: merged or retired rsIDs (none of 400 SNPs read had a merge flag); an association or study with no EFO mapping (0% observed); GxE study cards (7 of 200 studies); file sizes; the test budget files; associations with background traits (121 of 14,000); the share of associations whose only variant is not an rsID (the code read lists it as unmeasured; the sample gives 18.6% carrying a `chr:pos` identifier).

### Reactome

Reactome release 98. Counts come from the pathway list download (23,652 rows), the relation file and the ContentService; reach was compared live. The tool's pathway surface is human only and plain search uses Reactome only.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Pathways, all species | 23,652 rows, 16 species; human 2,889, mouse 1,849, chicken 1,845, rat 1,836, cattle 1,825, pig 1,808, dog 1,787, zebrafish 1,698, Xenopus 1,623, Drosophila 1,593, C. elegans 1,413, Dictyostelium 1,088, S. pombe 874, S. cerevisiae 851, P. falciparum 660, M. tuberculosis 13 | `biomcp get pathway ID` | Human 12.2% of rows; 0% of the other 20,763 | `biomcp get pathway R-MMU-5673001` is a usage error saying human only (honest) |
| Human top-level pathways | 29 | `biomcp search pathway --top-level` | 29 of 29 | |
| Human sub-pathways | 2,889 (851 with children, 2,038 leaves); Disease root 789 | `biomcp get pathway R-HSA-n`; `biomcp search pathway TEXT` | Any stable ID | No disease flag or filter; no parent-child tree (events list is flat). `biomcp get pathway R-HSA-5673001.3` is a usage error although the card's own `version` is `R-HSA-5673001.12`; numeric and lower-case IDs are refused |
| Contained events | R-HSA-5673001: 81; R-HSA-1643685: 2,963; R-HSA-162582: 3,002 | `biomcp get pathway ID events`; `biomcp pathway events ID` | Counts match the source exactly; not capped | 12 of 81 items (and 11 of 2,963) are bare database IDs because the source returns bare integers |
| Reactions, complexes, catalysts, regulation | Under Disease alone: 1,332 reactions, 457 failed reactions, 381 black-box events | `none` | 0% | `biomcp get pathway R-HSA-5675206` (a reaction from the events list) is absent with an `identity value.unknown` problem; the record exists but is not a pathway |
| Reference entities of a pathway | R-HSA-5673001: 332; Disease: 3,416 | `biomcp get pathway ID genes`; `biomcp pathway genes ID` | 332 of 332; 3,416 of 3,416; `--gene-products` removes 53 and says so | `biomcp pathway genes R-HSA-1643685 --limit 200` is complete although 478 references have no HGNC gene (disclosed in `gene_count` and 71 problems) |
| Entry by gene symbol | All human genes with a UniProt ID | `biomcp get gene X pathways`; `biomcp search pathway --gene X` | TP53 46 of 46; UBC 205 of 205; BRAF 16 of 16 | The gene card shows 25 of 205 with no `next` |
| Entry by UniProt accession | Reactome's own key | `biomcp search pathway --gene P15056` | 0% | Complete with 0 rows and an `hgnc value.unknown` problem (the source returns 16); also `P00533`. A test named "uniprot" runs `--gene EGFR` |
| Entry by miRNA | 70 human miRNAs in 27 pathways, 113 rows; MIR21 maps to R-HSA-9725371 and R-HSA-8943723 | `biomcp get gene MIR21 pathways`; `biomcp search pathway --gene MIR21` | 0 of 70 | Card says `not-applicable`; search is complete with 0 rows and the Reactome row reads ok though Reactome was never asked. Cause: no UniProt list, so a zero-call leg counts as ok (`crates/biomcp-query/src/pathway/gene.rs`). A test enshrines the `not-applicable` reply |
| Entry by ChEBI compound | Water 544 pathways; aspirin (ChEBI:15365) 3 | `none` | 0% | `biomcp search pathway "ChEBI:15377"` is complete with 0 rows |
| Entry by GO term, disease name | n/a | text search | `biomcp search pathway "GO:0006915"` returned Apoptosis; `biomcp search pathway melanoma` 47 | By name text only, not by annotation |
| Other: interactors, analysis service, non-human pathways | n/a | `none` | 0% | Analysis dropped by design; union search total is labelled `read` (106) against 492 Reactome and 56 WikiPathways hits |

Honest behaviour confirmed: human-only scope is stated; `biomcp get pathway R-HSA-999999999` is absent; counts match the source; `biomcp get gene TRPS1 pathways` is empty because the source is; Entrez and Ensembl inputs resolve; search text results equal the source's (BRAF 13).

Not checked: the UniProt and Ensembl mapping downloads (skipped), so the share of genes with Reactome pathways is unmeasured; retired stable IDs; per-pathway disease and inferred flags; whether the HGNC resolver explicitly rejects UniProt and miRBase IDs (seen from behaviour only); Ensembl-only Reactome participants.

### WikiPathways

Release 20261110. Counts come from the pathway list JSON, the human GPML zip (all 1,131 files parsed) and the GMT file. The pathway list total is 2,214 (41 species) in one findings file and 2,218 (1,087 non-human, 49%) in the code read; the first file's non-human count is 1,083.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Pathways, all species | 2,214 or 2,218 (see above); human 1,131, cattle 241, mouse 214, rat 129, S. cerevisiae 106, C. elegans 60, zebrafish 55 | `biomcp get pathway WPn`; `biomcp search pathway Q --source wikipathways` | Human catalog matches the source; non-human 0% | `biomcp get pathway WP1` (mouse) is absent with no problem entry, while Reactome gives a usage error for non-human IDs. `biomcp search pathway "IL-9 signaling" --source wikipathways` is complete with 0 although WP10 (mouse), WP8 (rat) and WP974 (cow) exist. Lower-case `wp5434` is a usage error |
| Search | 1,131 human titles | `biomcp search pathway Q --source wikipathways` | 100% by title substring | Titles only: `biomcp search pathway "tumor necrosis factor" --source wikipathways` gives 0 (one human description matches). No search by gene, disease, tag or description. WikiPathways is excluded unless `--source wikipathways` is given |
| Gene-type nodes | 74,068 data nodes: GeneProduct 43,314; Metabolite 13,247; Protein 10,755; Pathway 3,279; RNA 1,610; Key Event 909; Unknown 611; Complex 236; Event 83 | `biomcp pathway genes WPn`; `biomcp get pathway WPn genes` | Only GeneProduct, Protein and RNA are read: 55,691 of 74,068 = 75.2% | Metabolite, Pathway, Complex, Event and Key Event nodes are counted in `physical_entities` but never listed. 132 of 1,131 pathways have no gene-type nodes and answer complete with total 0 (`biomcp pathway genes WP3604`: 519 metabolites) |
| Gene IDs on gene-type nodes | Ensembl 28,308 (50.8%); Entrez 16,408 (29.5%); UniProt-TrEMBL 6,516 (11.7%); HGNC 1,657 (3.0%); no ID 1,046; RefSeq 444; other | same | About 83.3% resolve by namespace; 16.7% (9,318 nodes) lost (inferred from namespaces). 575 of 1,001 gene-bearing pathways lose at least one gene node; 94 (9.4%) lose all | Counted under `gene_count.unmapped` with status still complete and no list: `biomcp pathway genes WP4324` gives 0 of 297 (the GMT lists 62); `WP5098` 0 of 123 (GMT 90); `WP5485` 0 of 31 (GMT 25); `WP5584` 0 of 69; `WP5355` 55 of 96; `WP5287` about 49% with 78 nested pathway nodes not returned; `WP100` lists 20 and counts 18 EC-number nodes unlisted. `WP5434` matched (509) |
| Pathways whose GPML lacks the `Data-Source` attribute | 6 of 1,131 | same | Fail | `biomcp get pathway WP5639 genes` is `unavailable` bad-reply, exit 4, although it holds 28 genes (also WP5620 71, WP5255 1, WP5637 42, WP5594 21; WP5423 holds 0). Cause: `crates/biomcp-adapters/src/wikipathways/gpml_raw.rs` |
| Metabolites, nested pathways, interactions, ontology tags, communities, authors | 13,247 metabolite nodes; 3,279 pathway nodes | `none` | 0% | See gene-type nodes |
| Entry by gene | GMT: 9,029 distinct human Entrez IDs in 991 pathways | `none` (`--gene` is Reactome only) | 0% | `biomcp get gene FAM170B pathways` is empty (Reactome only) and `biomcp search pathway --gene FAM170B` gives 0 rows while WP5352 lists FAM170B |
| Description fields | `getPathwayInfo.json` | `biomcp get pathway WPn` | Description (truncated), `cited_in`, revision shown | Authors not shown |
| Events | WikiPathways has no event model | `biomcp pathway events WP5423` | `usage` | The refusal does not say why |
| Other: bundle freshness | Catalog fresh for 7 days, never auto-refreshed | n/a | n/a | A stale or syncing catalog adds a sync step and makes the reply partial (from code) |

Honest behaviour confirmed: human-only scope is stated in help; `physical_entities` shows every node count; node counts and `unmapped` are disclosed; duplicates and bad identity are refused at sync with problems (from code); `biomcp pathway genes WP4010` (no gene nodes) is complete with 0, true to source.

Not checked: nothing is listed beyond the limits above. Every pathway call also emits a `data_nodes.Pathway.xref value.unknown` problem for a normal node (seen on WP100, WP5434 and WP5485).

### ClinicalTrials.gov

The registry total is 613,799 from the v2 API (`countTotal`) on 2026-11-27, the same dataset date the tool reports; tool counts come from `--count-only`.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| All studies, study type, status, country, sponsor | 613,799. Interventional 468,448; observational 143,282; expanded access 1,079. Status: not yet recruiting 30,279; recruiting 65,480; enrolling by invitation 5,315; active not recruiting 22,304; completed 336,378; suspended 1,756; terminated 34,845; withdrawn 17,057; unknown 98,316; available 251; no longer available 546; temporarily not available 37; approved for marketing 245; withheld 990 | `biomcp search trial --count-only` with `--study-type`, `--status`, `--country`, `--sponsor` | Equal to the registry for every filter offered (60+ comparisons) | Search rows lack `study_type`, so observational, interventional and expanded access mix in a result. Expanded-access subtypes are neither a filter nor on the card. `--country "Viet Nam"` is a usage error (valid label "Vietnam" 1,386) |
| Phase | Early 1: 6,555; 1: 66,453; 2: 91,207; 3: 50,315; 4: 36,179; not applicable 242,533; 1/2 17,251; 2/3 7,676 | `--phase early-1, 1, 2, 3, 4, not-applicable, 1/2, 2/3` | Equal to registry numbers | `--phase 1` and `--phase 2` include combined phases while help says "Exact source phase": `biomcp search trial --phase 1 --limit 1 --fields nct_id,phases` returns NCT03515551 with PHASE1 and PHASE2 (pure phase 1 about 49,202 and pure phase 2 about 66,280, arithmetic not measured) |
| Records missing a value | Phase missing 145,484 (23.7%); no interventions 62,358; no locations 61,320; maximum age under 18: 37,469 | `none` as filters | 0% by filter | `get` works on them (NCT06652451 arms empty; NCT03051633 locations empty). `biomcp search trial --age 5` gives 65,200 (includes adult and all-ages studies) so pediatric-only cannot be isolated |
| Intervention type | DRUG 214,981; device 77,420; biological 30,309; procedure 61,526; radiation 10,849; behavioral 66,411; genetic 2,903; dietary supplement 18,590; combination product 3,493; diagnostic test 19,937; other 126,314 | `--intervention` (text search) | 0% by type | `--intervention DRUG` gives 250,514 against 214,981; a device query by text gives 92,273 against 77,420 |
| Obsolete NCT alias | 9 of 1,500 sampled studies (0.6%) carry an alias; extrapolation not measured | `biomcp get trial NCT01141231`; `biomcp search trial --id NCT01141231` | Fail | Get fails (`failed`, `rejected`) and search is complete with 0, while the registry redirects to NCT02589938 and `AREA[NCTIdAlias]` finds it. Cause: no redirect handling and an ID filter that does not match aliases (`crates/biomcp-adapters/src/ctgov/filters.rs`, inferred) |
| Sponsor | Amgen lead 1,058; any role 1,603 | `--sponsor`, `--sponsor-role any`, `--sponsor-type` | Equal to registry | The help's own example `biomcp search trial --sponsor "MD Anderson"` is complete with 1 row; the registry holds 3,051 under "M.D. Anderson Cancer Center". Broadening fires only at zero matches. `--sponsor Amgn` gives 0 with a notice |
| Results | 81,215 with posted results (13.2%) | `--has-results`; `biomcp get trial X results` | Search equal to registry. NCT05398445: serious 37, other 7, outcomes 30 = registry | The "more info" module (limitations and caveats, PI agreement, point of contact) has no record kind. A single `--limit 200` call returned 13 of 92 rows (byte cap) with a continuation |
| Locations and documents | NCT00087178: 606 sites | `biomcp get trial X locations`; `biomcp get trial X documents`; `biomcp trial document X FILE --find` | 606 = 606, paged. NCT02894502 protocol read is partial with state damaged (honest) | Documents: no search for "has protocol" |
| Card fields | per record | `biomcp get trial X` | Core modules present | Not carried: IPD sharing, oversight flags, organisation and secondary IDs, collaborators, keywords, MeSH, design info, `stdAges`, EA subtype, status-verified date (checked against the registry record for NCT05398445) |
| Secondary IDs | per record | `biomcp search trial 2022-501540-15-00` (and `20210142`, `NCI-2009-00193`, `CDR0000594729`) | 4 of 4 found by free text | Not shown on the card |
| Paging and defaults | n/a | `--limit` max 100; offsets to 5,000 then `--next-page` | `--offset 6000` is a usage error stating the limit | No default status, type, phase, country, date or lead-sponsor filter; sort is the registry's own |
| Other: non-US radius | n/a | `biomcp search trial --lat 35.68 --lon 139.69 --distance 50km --count-only` | 6,040 | Not independently verified |

Honest behaviour confirmed: no row drops in 1,100 sampled rows (0 problems); withheld records (990) work; empty arms or locations are true; lower-case IDs get a usage error naming the canonical form; condition synonym expansion and drug-name widening are stated in notices; an uncommon biomarker spelling search states the spellings it tried; a country mismatch fails the page loudly and unknown enum values become field problems (from code).

Not checked: the tests were not read (the findings infer they use famous trials); non-US radius counts; alias share across the registry; design info fields.

### NCI Clinical Trials Search

The total 27,775 comes from `biomcp search trial --source nci --count-only`; every NCI count is the tool's own, because NCI's API needs a key that was not used.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Trials and status | 27,775. Active 6,610; Administratively Complete 4,541; Approved 125; Closed to Accrual 5,237; Closed to Accrual and Intervention 1,066; Complete 8,716; Enrolling by Invitation 90; Temporarily Closed to Accrual 393; Temporarily Closed to Accrual and Intervention 13; Withdrawn 984; Inactive 0 | `biomcp search trial --source nci [--status]` | Status counts sum to 27,775 exactly | `--limit 100` is a usage error that does not say the cap is 50 |
| Phase | NA 7,740; I 6,067; I/II 3,077; II 7,594; II/III 270; III 2,221; IV 252 | `--phase` | Sums to 27,221; 554 (2.0%) have no phase | Same unfilterable-missing shape as ClinicalTrials.gov |
| Trials without an NCT ID | 0 of 500 sampled (upper bound about 0.6%) | `biomcp get trial` takes NCT IDs only | Approximately 0% affected | `biomcp get trial NCI-2020-02976 --source nci` is a usage error |
| Disease concepts | C3224: 1,242 | `--condition C3224`; `--condition-id MONDO:...`; `biomcp get trial X --source nci --section diseases` | 1,242 for C3224 and MONDO:0005105; `MONDO:0007254` selected NCIT:C9335 with 3,729 | Free text `--condition melanoma` is a usage error (honest); `--keyword melanoma` gives 2,159. The "exact concept, no descendants" rule is not in `--condition` help. `--condition-id` needs exactly one NCIt cross-reference |
| Biomarker concepts | PD-L1 37; ROS1 24; NTRK 45; KRAS 36 | `biomcp search biomarker --source nci --name X`; `biomcp get biomarker C96022 --source nci` | C96022: concept count 328; trial search 326 (320 inclusion, 8 exclusion) | `--source nci` is required on `search biomarker`; variant-level names ("G2032R") give 0 |
| Organizations | `--name Mayo`: 13 | `biomcp search organization --source nci --name Mayo` | Not measured | Complete with 6 `value.unknown` problems on `contact_email` across 13 rows |
| Sites, contacts, references, documents | per trial | `biomcp get trial X --source nci --section locations` | 0 | Named as unsupported (`unsupported_sections`, partial); honest |
| Records ClinicalTrials.gov also holds | n/a | `biomcp get trial NCT01865851 --source nci` | Absent | Honest; NCI and ClinicalTrials.gov never merge |

Honest behaviour confirmed: a disabled source gives `failed` with the source state `skipped` and reason `disabled`; with no key the code gives `failed` plus a key step (inferred from code); free-text disease and `--id` are refused clearly; rows without an NCT ID or NCIt code are refused with problems (none seen in a 50-row sample).

Not checked: an actual missing-key run; an independent NCI count; the intervention and disease vocabularies (never requested); organization reach.

### gnomAD

Counts are from gnomAD's own API (v4.1, GRCh38). The reach figures come from a sample of 200 random v4 variants drawn from a pool of 20,660 rows for TP53, BRCA2 and EGFR (178 were SNVs), each looked up in MyVariant's hg38 index.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| v4.1 short variants | Browser: 807,162 exomes and 76,215 genomes (not counted here); 20,660 rows for the three genes | `biomcp variant population INPUT`; `biomcp get variant X population` | By gnomAD's VCF-style ID or rsID: 148 of 178 sampled SNVs found (83%); 30 missing (17%, all exome-only; about 1 in 6, interval roughly 12 to 23%). Without an rsID only 11 of 41 found (27%); with an rsID 137 of 137; AC0-filtered 18 of 33 (55%) | Absent where gnomAD holds the allele: `biomcp variant population 17-7673174-A-T` is `absent` while `biomcp variant population NC_000017.11:g.7673174A>T` returns data (6 of 6 missing alleles behaved so). `biomcp variant population 17-7661867-C-A` is absent while the NC_ form returns exome ac 0 with filter AC0. `biomcp variant population clinvar:17677`, `rs80357906` and `17-7661911-C-T` (and `rs1393282382`) fail, though `NM_007294.4:c.5266dup` returns data. 22 sampled indels were not tested. Cause: the VCF and rsID path resolves identity through MyVariant hg38 with no CAR fallback (inferred from code) |
| Multi-allelic rsID | n/a | `biomcp variant population rs113488022` | Candidates | Top status complete while the section is `unavailable` with `no-join-id` (also `rs1801133`); a biallelic rsID works |
| v2.1 (GRCh37) | exists | `none` | GRCh37 inputs are crosswalked through CAid to v4 | A variant present only in v2 is not-found. `crates/biomcp-adapters/src/gnomad/variant.rs` fixes the dataset to `gnomad_r4` |
| Mitochondrial | v3.1 mitochondrial records exist: the code read found `M-3243-A-G` with an 56,383, 6 heteroplasmic, 0 homoplasmic carriers; the census probes found "Variant not found" for `M-8860-A-G` and `M-12015-T-C` in both r3 and r4 and could not confirm what mitochondrial data exists | `none` | 0% | `biomcp get variant 'NC_012920.1:m.3243A>G' population` is complete with state empty and the notice "gnomAD found no variant record"; `M-8860-A-G` also empty. A fix would say "not in gnomad_r4" |
| Structural variants, short tandem repeats, CNV | exist (`gnomad_sv_r4`, `gnomad_cnv`) | `none` | 0% | SV IDs refused (usage); `biomcp get variant NC_000004.12:g.3074876CAG[40]` fails; help says none of this |
| Ancestry and sex strata | 29 group rows for BRAF V600E | `biomcp variant population CA123643 --detail` | Returned (dataset 4.1.0, exome ac 2 an 1460618) | |
| Filtered or failed-QC rows | For example TP53 `17-7661952-G-A` filter AS_VQSR | same | Filters carried | |
| Gene constraint | One v4 row per gene | `biomcp get gene X constraint` | BRAF and TP53 match the API; protein-coding genes with an Ensembl ID 17 of 17 returned data; non-coding, pseudogene and other groups `not-applicable` (43 of 43 sampled) | MT-ND4 and MT-CO2 `empty` (gnomAD has no constraint); `--constraint-release v4.0` and `v4.1` return partial with reason `syncing` on the test machine; the default is labelled only `v4` |
| Other | n/a | n/a | n/a | `joint.dataset_version` `value.unknown` on every successful population reply (inferred from code); help does not name the dataset |

Honest behaviour confirmed: the dataset is shown in the reply; filters and flags (MUC16, TTN) are carried; chrY SRY gives empty and gnomAD has no row for it; multi-allelic candidates are labelled rather than guessed; SV input is refused with usage.

Not checked: whether gnomAD v4.1 or r3 holds any chrM site beyond the one the code read found (the two sets of probes disagree, see the Mitochondrial row); 22 sampled indels; constraint for pseudogenes at gnomAD; the constraint metrics file (not fetched).

### MyVariant.info (with COSMIC, CADD and dbNSFP carried fields)

The hg19 index holds 1,458,004,628 records (MyVariant metadata, build 20250624); the hg38 total is not reported separately (1.51e9 covers both). Component shares come from a non-uniform sample of 207 ClinVar-carrying and 207 gnomAD-exome-carrying hg38 records (9 random-offset hits per chromosome for chr1 to 22 and X).

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| dbNSFP | Both indexes | `biomcp get variant X predict`; `conservation` | 47% of ClinVar-carrying and 46% of gnomAD-carrying hg38 records; 0 of 24 non-SNV ClinVar records | Missing is reported as empty or `unavailable (not-found)` |
| CADD | hg19 index only | `predict` | With the default GRCh38 identity CADD is absent; a GRCh37 genomic input returned `cadd.phred` 32.0 | Undisclosed: `biomcp get variant CA123643 predict` shows dbNSFP only |
| COSMIC | hg19 index only; 0 of 207 sampled hg38 records | `biomcp get variant X cosmic` | `biomcp get variant "chr7:g.140453136A>T" cosmic --build GRCh37` returns COSM476 | `biomcp get variant CA123643 cosmic` is `empty`; `NM_004333.6:c.1799T>A --build GRCh37` is also empty (identity resolves through GRCh38). `biomcp search variant --source myvariant --query '_exists_:cosmic' --build GRCh38` is complete with 0 against 1,024,494 for GRCh37. `COSM476` as input is a usage error. COSMIC `deadline-exceeded` on TP53 frameshift and splice probes |
| Other index components (SNPedia 5,907 records; cgi, civic, docm, exac, geno2mp, grasp, mutdb, wellderly and more) | hg19 only | `none` (SNPedia declared in code but no section) | 0% | `_exists_:exac` matches 10,195,872 but the reply shows identity only |
| Identity input forms | n/a | `biomcp get variant INPUT` | Resolved: rsID (multi-allelic gives candidates), CAid, ClinVar IDs, genomic HGVS (NC_, or `chr...` with `--build`), c. on MANE or old version or ENST or LRG, VCF-style, `GENE p.` forms | `biomcp get variant "BRAF V600E"` queries hg19 (candidates `build: GRCh37`, no `build_defaulted`) though help says GRCh38 is the default, and returns two candidates that are the same allele. Gene-only shorthand for indels, exons, fusions, amplification, repeats and promoter is refused (usage) with accepted forms. `NP_004324.2:p.Val600Glu` fails; `NM_004333.3:c.1799T>A` fails with no reason text |
| Variant classes (best-working form) | n/a | `biomcp get variant ... all` | SNV, MNV, insertion, deletion, duplication, splice site, UTR, promoter genomic, chrX and chrY resolved | Promoter by transcript `NM_198253.3:c.-124C>T` and 5' UTR fail with no reason text. Intronic duplication given as genomic HGVS fails (`NC_000017.11:g.43077335_43077336dup`). Large deletions: `biomcp get variant clinvar:563563 all` fails while `biomcp get variant clinvar:563563 clinvar` returns 3 assertions. HGVS that `search variant` itself emits (`chr17:g.43077335_43077336dup`, `chr9:(?_2130392)_(2185324_?)del`) fails when fed back |
| Mitochondrial by input form | n/a | `biomcp get variant 'NC_012920.1:g.12015T>C' all` against `chrMT:g.12015T>C --build GRCh38` | Different reach for the same allele | NC_ form: predict, conservation, cosmic, hotspots `unavailable (not-found)`. chrMT form: predict and conservation data; GWAS and hotspots `no-join-id`; AlphaGenome unsupported |
| Search | n/a | `biomcp search variant --gene/--protein/--coding/--region/--query` | `--gene TERT --coding 'c.-124C>T'` found 1; `--region chr5:1295113-1295113 --build GRCh38` 5 hits | Without `--build` it uses hg19 (the reply shows `build: null`). Region offset max 9999: an 80 kb BRCA1 window reports total 65,165 but only 10,000 are reachable. `--protein E746_A750del` is a usage error, so protein-level indels have no search route. Protein search drops non-SNV hits with no rsID or ClinVar ID and unconfirmed reference base (reported in `excluded`, status still complete; inferred from code) |
| Alleles in the CAR registry | n/a | resolver `car` | 94 of 100 random ClinVar SNV or indel IDs returned a CAid | The 6 misses were not diagnosed |
| Other | n/a | n/a | n/a | `status: complete` while a section is `unavailable` (`biomcp variant population rs1801133`); rejected HGVS gives `failed` with `value.unknown` and no reason text (sources name `IncorrectHgvsPosition`, `EOUTOFBOUNDARY`) |

Honest behaviour confirmed: `get variant` states `build_defaulted: true` for GRCh38; refused input lists the accepted forms; chrY SRY population `empty` is right; a retired ClinVar record (636502) reads empty; `variant normalize` explains each source's outcome.

Not checked: COSMIC content (licensed, binary only); SPDI (refused); whether protein search excludes a somatic-only MNV (code read, no probe); how many CAR representations are dropped.

### Cancer Hotspots

Rows were compared with the service's by-gene endpoint; for EGFR the API returns 24 rows and the tool returns 24.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Single-residue hotspots | By-gene rows with type "single residue" | `biomcp get variant X hotspots` | EGFR 24 of 24 | Returns the whole gene's rows as `membership: candidate`, not the asked residue: `biomcp get variant CA175996 hotspots --section-limit 50`; TP53 synonymous gives 120 unrelated rows; a BRAF delins gives the gene's 12 rows. BRCA1 is empty (correct). Disclosed in help as "candidate residue counts" |
| In-frame indel hotspots | EGFR 745-759 (155 tumors), 764-774 (33), 709-710 (9) | same | 3 of 3 for EGFR | The `type` field is dropped, so the reader infers indel from the residue text (`crates/biomcp-adapters/src/hotspots/mod.rs`) |
| Cancer-type breakdown, q-values, position, tumor counts | In every row (L858: lung 143, bowel 1) | `none` | 0% | Not carried |
| 3D hotspots | The probe returned HTTP 500 "Input not found" | `none` | n/a | |
| Dependence on MyVariant | n/a | n/a | n/a | The section needs a MyVariant-derived transcript consequence, so it is `unavailable (not-found)` where MyVariant lacks the allele and `no-join-id` for chrMT, even though the gene rows exist |

Honest behaviour confirmed: the candidate nature of rows is stated in help; no hotspots for a gene without any returns `empty`.

Not checked: the 3D endpoint beyond one probe; the service version (not stated in replies).

### cBioPortal

Counts come from the portal's molecular-profile list (2,573 profiles in 554 studies).

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Studies | 554 | `biomcp search study TEXT`; `biomcp get study ID` | 553 of 554 by text "a" | A local snapshot about 3 days old; text match on ID, name and description only |
| Mutation profiles | 548 profiles in 548 studies | `biomcp get study ID mutations --gene G`; `biomcp variant recurrence` | EGFR in `luad_tcga_pan_can_atlas_2018` 86 = 86; BRAF V600E in `msk_impact_2017` 295 samples = source (295 of 592 rows; 10,945 selected samples) | Recurrence needs `--study`, `--profile` and `--sample-set` (no default cohort, stated). Recurrence on chrX returns zero: `biomcp variant recurrence 'NC_000023.10:g.66943552A>G' --source cbioportal --gene AR --study msk_impact_2017 --profile msk_impact_2017_mutations --sample-set msk_impact_2017_all --build GRCh37` is complete with `observed_sample_count: 0` and `examined_rows: 302`, while the source has 6 samples carrying T878A and `biomcp get study msk_impact_2017 mutations --gene AR` returns 302 rows. The source uses chromosome `"23"` and the matcher cannot compare it, counts the row nowhere and still reports complete (`crates/biomcp-query/src/variant/recurrence/matching.rs`). Three studies use `"23"`; chrY and chrM likely behave the same (inferred) |
| Pure insertions and deletions | EGFR exon 19: 117 of 191 deletion rows in one study | `biomcp variant recurrence NC_000007.14:g.55174772_55174786del ...` | 0% | Refused as failed ("pure insertions and deletions are unavailable"), by design. Multi-allelic `rs113488022` also fails |
| Discrete CNA | 355 profiles | `biomcp get study ID cna --gene G` | EGFR LUAD 26 amplified and 3 deep-deleted, both | Counts only, no sample list; calls of plus or minus 2 only |
| Structural variants and fusions | 226 profiles in 226 studies (8.8% of profiles); 4 EML4 rows in one profile | `none` | 0% | `biomcp get study luad_tcga_pan_can_atlas_2018 mutations --gene EML4 --molecular-profile luad_tcga_pan_can_atlas_2018_structural_variants` is refused with no `next` to the portal |
| Expression, protein, methylation, continuous CNA, generic assay | mRNA 884 (519 Z-score, 365 continuous, 2 discrete); RPPA 179; methylation 71; continuous CNA 50 + 56; generic assay 201 | `none` (listed under `profiles` only) | 0% | 1,421 of 2,573 profiles (55%) are neither mutation nor discrete CNA |
| Samples, sample lists, files | `msk_impact_2017`: 10,945 samples, 60 lists, 18 files | `biomcp get study ID samples`, `sample-lists`, `files` | 100% | List members not returned by design |
| Clinical data, gene panels | present | `none` | 0% | A user cannot compute a frequency (selected size is never an allele denominator) |
| Study defaults | n/a | `biomcp get study ID mutations` | n/a | The sample list is hard-coded to the first all-cases list (`crates/biomcp-adapters/src/cbioportal/studies/rows.rs`) and is not in help or the reply; the profile defaults to the first mutation or discrete profile (documented and echoed) |

Honest behaviour confirmed: BRAF and EGFR counts equal the source; a page of 3 rows reports `observed 0`, `more true`, `all_pages_read false`; the multi-allelic rsID refusal; the MNV `observed 0` is correct (the portal holds no such row).

Not checked: TP53 R175H (209 over 4,954 rows, not cross-checked); per-study mutation row counts; chrY and chrM recurrence (inferred); a study with several discrete profiles.

### OncoKB and AlphaGenome

Both are licensed sources. OncoKB had no key registered; AlphaGenome content was not examined beyond how it reports itself.

| Source | How it reports itself | Failure modes found |
|---|---|---|
| OncoKB | `biomcp get variant CA123643 oncokb`: top status `complete`, `result.resolution: unresolved`, section `{"state":"skipped","reason":"no-key"}`, source row `use: licensed`, no problem entry. Same with `--tumor-type Melanoma` and for `biomcp get gene BRAF oncokb`. Listed inside `all` as `skipped (no-key)` | A reader checking only the top status misses that a whole licensed source was skipped. Missing build or tumor type gives `skipped (no-join-id)` the same way (inferred from code). The test asserts the skip but not that status should be partial. Protein-change, copy-number and structural-variant endpoints are never requested |
| AlphaGenome | With a key: source row `use: licensed, state: ok` and a `data` section with scorers (for example `gene-mask-lfc-rna-seq`, page total 17,342) for SNVs on GRCh38. Otherwise `unavailable` with reason `unsupported` (GRCh37, chrMT), `no-join-id` (indels, genomic HGVS without a unique allele) or `bad-reply` | `bad-reply` is unexplained for chrY SRY, TP53 synonymous, BRCA1 splice, TERT promoter and 3' UTR probes, while BRAF V600E and BRCA2 stop-gain return data |

Honest behaviour confirmed: both label their own state in the section and source row.

Not checked: OncoKB content (not tested with a key); AlphaGenome content and key handling.

### HGNC and MyGene

HGNC counts come from the complete set (45,470 approved records) and the withdrawn file (5,293: 3,490 merged or split, 1,803 entry withdrawn) read on 2026-11-27; reach comes from a stratified random sample of 68 `get gene` calls (seed 8, one duplicate).

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Approved protein-coding genes | 19,299 (20 have no UniProt ID in HGNC) | `biomcp get gene SYM`; `biomcp search gene Q --locus-group protein_coding` | 23 of 23 resolve | `biomcp get gene HSD17B8 --section protein` says `not-applicable` and `biomcp get protein HSD17B8` is absent, yet `biomcp search protein --gene HSD17B8` finds reviewed human Q92506 (also TRD, TRG, KANGD1). Cause: the protein section is keyed on HGNC's UniProt ID |
| Approved non-coding RNA, pseudogene, other | 10,470; 14,698; 1,003 | same | 18 of 18, 9 of 9, 16 of 16 resolve | Other sections say `not-applicable` or `empty`, which is honest. 330 pseudogenes have a UniProt ID (2 of 6 sampled got protein data) |
| Records without Entrez or Ensembl ID | 904 without Entrez; 2,659 without Ensembl | `biomcp get gene SYM` | Resolve | Constraint and expression say `not-applicable` without Ensembl; KANGD1 resolves with no MyGene |
| Mitochondrial genes | 56 | `biomcp get gene MT-*` | 3 of 3 | MT-CO2 constraint empty |
| Previous and alias symbols | Previous 12,719 genes; alias 22,679 genes | `biomcp get gene OLD` | Previous 3 of 3; alias 2 of 2, 1 ambiguous (Eos listed IKZF4 and PRSS33) | 739 alias or previous symbols equal another approved symbol: `biomcp get gene VARS2` returns VARS2 only, with no hint that VARS1 used the alias |
| Withdrawn symbols | 3,474 merged or split into one; 16 into several; 1,803 entry withdrawn | `biomcp get gene OLD` | 9 of 9 resolve to successors; 5 of 5 several-successor cases return `ambiguous` listing all; 6 of 6 withdrawn return `withdrawn` | `biomcp search gene RN7SL232P` is complete with total 0 while `biomcp get gene RN7SL232P` says withdrawn |
| Gene groups and families | 28,694 approved records carry one | `none` | Not measured | `search gene` has only `--band`, `--region`, `--locus-group` and text |
| Other ID forms | n/a | `biomcp get gene INPUT` | Entrez, Ensembl, HGNC ID and any letter case of a symbol resolve | `biomcp get gene Trp53` (mouse) is absent, `P04637` and `ENSG00000290825` not-found |
| MyGene summaries and positions | about 45,470 HGNC genes | `biomcp get gene` identity block | `mygene` ok for 58 of 68 sampled | `value.unknown` on 7 of 68 sampled items. HSD17B8 lists 6 positions (chromosome 6 plus 5 `HSCHR6_MHC_*` alternate contigs): the main span is kept, the alternates raise two problems, status stays complete. SLCO1B7 shows a `hgid` mismatch, empty spans and partial. `crates/biomcp-adapters/src/mygene/map.rs` |
| Non-human, Entrez-only or Ensembl-only genes, discontinued Entrez | MyGene holds many species | `none` | 0 | Human only by design |
| Section states by group (68 sampled) | n/a | n/a | Protein-coding: protein data 18, not-applicable 5; constraint data 17; expression data 17. ncRNA 18: all protein `not-applicable`. Pseudogene 9: protein data 2. Other 16: expression data 6 | HPA returns 404 for the empty expression cases, so `empty` is honest |
| Other: constraint release | n/a | `biomcp get gene TP53 --section constraint` | v4 shown | See gnomAD |

Honest behaviour confirmed: non-coding genes give `not-applicable`, not failures; ambiguous symbols list every candidate; withdrawn successors are named; one transient FZD3 expression failure was labelled `unavailable` and cleared on repeat.

Not checked: OncoKB; STRING for non-human proteins; HPA bulk download; an InterPro census (12 proteins sampled); proteins with 50 or more isoforms (largest found had 17); the GenCC, ClinGen, ClinVar and CIViC gene sections.

### UniProt

Counts are `x-total-results` from UniProt's search on 2026-11-27.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Reviewed human entries | 20,429 (all entries 154,222,205; reviewed 576,073) | `biomcp get protein ACC`; `biomcp search protein --gene G` or `TEXT` | All by accession; by symbol only through HGNC's UniProt IDs | `biomcp search protein --gene Trp53` is complete with 0 and does not state human-only scope. `biomcp search protein TP53` total 757 (5 returned, truncated, pool ID) |
| Unreviewed (TrEMBL), mouse, viral, bacterial | 153,646,132 (human 306,643); mouse 261,799; viral 1,109,724; bacterial 75,545,359 | `none` | 0 | Reported as a failed source: `biomcp get protein A0A0A0MS99` (unreviewed human ABCC1), `P02340`, `P03366`, `P00722`, `P12345` all give `failed` with `bad-reply` and `reviewed_human value.unknown`, so scope refusal reads like an outage. `biomcp search protein --gene ABCC1 --reviewed=false` is a usage error although a positive exists |
| Secondary and deleted accessions | not counted | `none` | 0 of 2 | `biomcp get protein Q13290` is `failed` with `rejected` (UniProt redirects to P42126); `biomcp get protein A0A008APQ8` is `failed` with `bad-reply` (deleted) |
| Isoforms | 10,683 human reviewed entries with alternative products | `biomcp get protein P38398-2`; `biomcp protein isoforms ACC`; `biomcp protein sequence P38398-2` | 1 of 1; DMD 17 of 17 | |
| Features | TP53 1,520 (15 types); TTN 1,317 (17 types) | `biomcp protein features ACC`; `biomcp get protein ACC --section features` | TP53 1,520 of 1,520 over 9 pages of 200, kinds match the source; TTN 1,317 of 1,317 over 8 pages | `value.unknown` on `features.description` on every call (EGFR also `features.featureId`), status complete |
| Function, subcellular location, alternative products | 17,361; 17,363; 10,683 | `get protein --section function`; `isoforms` | Returned | Subcellular location appears only under `biomcp get gene --section protein`, not under `get protein` |
| Other comment types | Disease 5,350; tissue specificity 10,280; interaction 13,138; cofactor 1,820; catalytic activity 4,872; PTM 5,594; polymorphism 853; pharmaceutical 51; allergen 6 | `none` | 0 | Dropped without a note: `biomcp get protein P04637 --section all` returns function text but no disease comment (inferred from code: only function and subcellular fields are requested) |
| Cross-references | PDB 8,969 human reviewed; AlphaFold 20,277 | `biomcp protein structures ACC` | P04637: 318 PDB, 5 shown by default | AlphaFold not seen (PDB only) |
| Other | 125 human reviewed with no gene name; 1 fragment | `biomcp get protein ACC` | Not measured | `biomcp get protein FAM86JP` works (Q05BU3); `biomcp get protein TRG` is absent (no UniProt) |

Honest behaviour confirmed: feature totals equal the source; page totals and `next` are stated; isoform get returns name and length.

Not checked: proteins with 50 or more isoforms; nothing further beyond the unreviewed-search deferral, which the design notes base on there being no positive example, while one was found.

### InterPro

Counts are the `count` field of InterPro's API on 2026-11-27.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| InterPro entries | 72,944 (family 29,970; domain 38,055; homologous superfamily 3,519; repeat 386; conserved site 782; active site 133; binding site 82; PTM 17) | `biomcp protein domains ACC`; `biomcp get protein ACC --section domains` | Integrated entries on a protein matched for 11 of 12 sampled proteins (P04637 9 of 9, P00338 7, Q9NPG1 6, P00533 15, Q92506 4) | `biomcp protein domains P27708` (26 entries) is `failed` with `bad-reply`; proteins with 20, 17, 15 and 14 entries worked, and the cause is unknown (possibly a 25-row limit in `crates/biomcp-adapters/src/interpro/raw.rs`) |
| Proteins with no entries | InterPro returns HTTP 204 | `biomcp protein domains Q05BU3` | n/a | Reported `failed` with `bad-reply` instead of empty |
| Unintegrated member signatures | 41,894 entries (`entry/all` 201,822) | `none` | 0%; present on 10 of 11 sampled proteins (TP53 1, LDHA 5, FZD3 2, EGFR 7, BRCA1 4, TTN 9, KRAS 7) | Silently dropped: `biomcp protein domains P01116` returns 4 rows while InterPro adds 7 unintegrated hits; `native_total` equals the integrated count. The adapter rejects any row whose source database is not InterPro (inferred from code) |
| Member signatures of integrated entries | Pfam, CDD, PANTHER and more | inside each entry | TP53 shows PANTHER PTHR11447 and PRINTS PR00386 | |
| Other | n/a | n/a | n/a | Every row is tagged taxon 9606 by the tool (inferred from code) |

Honest behaviour confirmed: non-domain types (conserved site, homologous superfamily) are returned.

Not checked: a per-type census of everything the tool returns across all proteins (12 sampled).

### Complex Portal, STRING and Human Protein Atlas

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Complex Portal complexes | 21,162 (human 17,816; other organisms 3,346) | `biomcp protein complexes ACC`; `biomcp get protein ACC --section complexes` | TP53: 11 admitted of 20 native (the other 9 do not list P04637 as a participant; `native_total` is shown). EGFR empty, true to source | Other organisms: `none` (human proteins only) |
| Complex Portal members | CPX-900 has 21 members including a zinc atom | `biomcp protein complex-members Q16514 --complex CPX-900` | Works, small molecules kept | The help example `biomcp protein complex-members P00533 --complex CPX-715` fails (CPX-715 is BRCA1-BARD1); a wrong pairing returns `failed` instead of "not a member"; `CPX-1056` with P00533 fails with `bad-reply` (not diagnosed) |
| STRING interaction partners | EGFR 1,456 at score 400 (5,328 at 150); TP53 2,396 | `biomcp get protein ACC --section interactions` | EGFR 1,456 = the STRING API at 400 | Score cutoff (400) and species (9606) are not options, only stated in the reply (`min_score`, version 12.5); other species `none` |
| HPA per-gene record | 136 top-level fields (checked for TP53) | `biomcp get gene SYM --section expression` | Five summary fields only (RNA tissue specificity, distribution, cancer specificity, main and additional subcellular location) | Per-tissue values, single-cell, pathology and cancer prognostics (30+ sets), blood, brain and cell-line expression, and tissue protein data have no route and no `next`; the design notes describe "top tissues" that the reply lacks. One transient `unavailable` (FZD3) |
| HPA genes with no record | HTTP 404 | same | `expression: empty` (5 tested) | Honest |

Honest behaviour confirmed: Complex Portal exact-participant admission is disclosed; STRING defaults are visible in each reply; non-protein complex members are kept.

Not checked: STRING for proteins absent from STRING or non-human; the HPA bulk file; why `CPX-1056` fails.

### openFDA drug label

Counts are totals the tool itself printed; nothing was fetched from any FDA site. Field lists come from the adapter code and from recalled openFDA documentation, not a fresh read.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Labels by name (Rx, OTC, unapproved, homeopathic) | Names resolved to 339 labels (loratadine), 78 (warfarin), 79 (arnica montana), 23 (imatinib) | `biomcp get drug NAME or SETID or APPLNO label --label-section ...` | Works for named labels; one label chosen with the total and 5 alternatives shown | Bare read: `biomcp get drug imatinib label` returns `label.state: empty` with status complete (a hint lists 29 sections). `--label-section` is a free string: `warnings_and_precautions` reads `warnings_and_cautions`; an invented section is indistinguishable from a real absent one except by the hint |
| Combination-only ingredients | n/a | `biomcp get drug sacubitril label --label-section boxed_warning` | n/a | Absent while `biomcp search drug sacubitril --source openfda-label` returns 40 labels and `get drug Entresto label` works; `emtricitabine` says 3 labels share the name while search finds 55 |
| CBER biologics (vaccines, CAR-T, gene therapy) | n/a | by name or set ID | Zolgensma works by name; Kymriah by set ID only | `biomcp get drug Kymriah label --label-section boxed_warning` is absent (set ID `aad3ba54-dfd3-4cb3-9e2b-c5ef89559189` works); Shingrix is absent even by set ID (3 set IDs exist in NLM's DailyMed) and `biomcp search drug Shingrix --source openfda-label` is complete with 0 |
| Animal drug labels | n/a | `none` | n/a | `biomcp get drug carprofen label ...` is absent with no statement that animal drugs are out of scope |
| Application-type search | n/a | `biomcp search drug --source openfda-label --application-type nda,anda,bla` | n/a | `biomcp search drug pembrolizumab --source openfda-label --application-type anda` returns 19 rows by free text (labels mentioning pembrolizumab, not its products), with a notice |
| Name resolution | n/a | n/a | n/a | Name get reads only the label's `openfda.brand_name` or `generic_name`, so a label without that block resolves only by set ID or text fallback (code read) |
| Label sections | 29 (Gleevec), 18 (Claritin), 35 (Jantoven) | `--label-section a,b`; `:NUMBER` subsections | `spl_medguide`, `purpose`, `do_not_use`, `recent_major_changes` proven | |
| Other: carried MyChem fields | n/a | label-adjacent routes | n/a | `value.unknown` problems on `drugcentral.xrefs.pdb_chem_id` and `chembl.molecule_synonyms` (rofecoxib, metformin) |

Honest behaviour confirmed: a missing section gives a hint ("has no boxed_warning section"); the pick of one label (original packager, brand, most indications, newest) is printed with the count of others; OTC labels carry their own sections.

Not checked: every one of about 90 label fields; the share of labels lacking an `openfda` block; a source-side census.

### Drugs@FDA

Counts are totals the tool printed; nothing was fetched from FDA.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Applications | imatinib 19, tenofovir 105, dolutegravir 36, loratadine 57; sponsors Teva 800, Mylan 458, Sandoz 515 | `biomcp search drug NAME --source drugsfda`; `biomcp get drug APPLNO or NAME approvals` | Works; a search with no name or applicant is a usage error; no `--status` filter | One application per drug is picked ("NDA/BLA before ANDA, then earliest approval"): `biomcp get drug imatinib markets`, `approvals` and `label-history` choose NDA021335 (discontinued capsule), so the market shows `Discontinued` while NDA021588 is Prescription |
| Products | dolutegravir 38 tentative approval, 8 prescription, 2 discontinued; loratadine 43 OTC, 13 discontinued | in each row | Visible and labelled | No default OTC or prescription filter |
| Submissions and documents | NDA021588: 47 submissions; rofecoxib 26 | `biomcp get drug APPLNO approvals`; `--document-id N` | Pages of 10 (max 200); letters read | |
| Label history | NDA021588: 33 versions | `label-history --label-section X --find TEXT` | Opens up to 10 of 33 | Reports "27 of 33 could not be read" with status partial (honest) |
| EU-only or biosimilar names | n/a | `biomcp get drug Zessly markets` | n/a | Shows REMICADE (BLA103772) as Zessly's FDA market through an ingredient lookup with a notice, `matches 1`, while `biomcp search drug infliximab --source drugsfda` has 6 applications |
| CBER biologics | not in Drugs@FDA | `none` | `biomcp get drug Kymriah or tisagenlecleucel or Shingrix approvals` absent | Never says CBER is out of scope: `biomcp get drug Yescarta markets` is complete with US `no-record`, `matches 0` (Yescarta is BLA 125643 in the Purple Book seed) |
| Disabled source | n/a | `biomcp get drug Keytruda markets --disable-sources drugsfda` | n/a | The US row reads `no-record`, `matches 0` while the source row is `skipped` and the reply can be complete (`crates/biomcp-query/src/drug/markets/read.rs`) |
| Date window | n/a | `biomcp search drug a --source drugsfda --approved-from 2026-10-01 --sort newest` | Applied after reading all name matches (stated; no cap seen to 800 applications) | A bare date window is a usage error; `--applicant "ELI LILLY" --approved-from 2025-01-01` returned 3 |
| Therapeutic equivalence codes | n/a | `none` seen | Not measured | Not present in rows |

Honest behaviour confirmed: EMA-only brands (`Translarna`, `Kisplyx`) are absent with a notice naming candidates; `biomcp get drug ataluren markets` shows drugsfda `no-record` and EMA found; tentative approvals are labelled; `supplement_purpose` "not-stated" is reported as partial.

Not checked: therapeutic equivalence codes; any openFDA count query.

### FDA Purple Book

Counts are totals the tool printed from a seeded snapshot (October 2026, notice "1 month old", monthly manual download with `biomcp sync purple-book --file`). The seed's size is given as about 2,000 lines in one findings file and about 2,270 rows in another.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| 351(a) reference products with no biosimilar | not countable | `none` by their own name | 0 | `biomcp search drug Keytruda or pembrolizumab or Kymriah or Enhertu or Recombivax --source purple-book` all complete with 0; Yescarta, which is in the seed, also returns 0 |
| 351(k) biosimilars and interchangeables | infliximab 4 (Inflectra, Renflexis, Ixifi, Avsola; reference Remicade); adalimumab 57 presentations; insulin glargine 3, all interchangeable | `biomcp search drug REFERENCE --source purple-book [--presentations --strength --device]` | Works by reference name, proper name or BLA: Humira 10, Remicade 4, Herceptin 6, Lantus 3 | A biosimilar's own name or BLA is complete with 0: `biomcp search drug Inflectra --source purple-book` (BLA125544 is item 1 of the infliximab reply); `infliximab-dyyb`, `"infliximab dyyb"`, `BLA125544`, `Amjevita`, `adalimumab-atto` too. `biomcp search drug Zirabev --source purple-book` returns 0 with the notice "Products approved or designated after October 2026 are not included" although `biomcp search drug bevacizumab` lists Zirabev (BLA761099, approved 2019). Cause: `crates/biomcp-adapters/src/purple_book/decode.rs` matches only reference names and BLAs |
| Exclusivity dates, presentations | Semglee first interchangeable exclusivity "November 15, 2022" | fields on rows; `--presentations` | Shown | |
| Snapshot age | n/a | notice | n/a | Source row ok and status complete at any age |

Honest behaviour confirmed: reference-name searches work; the snapshot age is stated.

Not checked: the seed was not counted as a source count; lookup by BLA of a 351(a) product has no route to test.

### FDA drug recall, Drug Safety Communications, safety signals and withdrawn accelerated approvals

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Drug recall enforcement records | 18,194. Class I 1,764; II 14,704; III 1,725 (sum 18,193, one short, unexplained). Ongoing 2,924; completed 425; terminated 14,845 (sum 18,194). January 2026: 64; 2012: 534; 2000 to 2011: 41 | `biomcp search drug --source fda-drug-recall`; `biomcp get drug D-0082-2027 --source fda-drug-recall` | Works (`metformin` 91; Class I Argatroban card complete) | `--status "Not Yet Classified"` returned partial with a MyChem `deadline-exceeded`; unrelated MyChem `value.unknown` lines; `--classification` accepts only Class I, II, III; a row without a valid recall number is declined with a problem and a refusal |
| Drug Safety Communications | 44 (list starts 2020-01-14; seed 2026-11-20; about 50 seed rows per the code read) | `biomcp search drug --source fda-dsc`; `biomcp get drug UUID --source fda-dsc` | By name, title and page text | Older items not read: `biomcp search drug fluoroquinolone --source fda-dsc` is complete with 0 plus a hint that zero matches does not establish that none was issued. `biomcp search drug pembrolizumab --source fda-dsc` is partial with total 0 and failed source rows for a known name (cause not examined). A communication whose text was unread can match only on title, yet zero matches still read complete (inferred from code; a test asserts it) |
| Potential safety signals | `semaglutide` 11; `pembrolizumab` 7; `Tecvayli` 4; `bleeding` 3 | `biomcp search drug QUERY --source fda-safety-signals` | Needs a query | Always partial and `stale` (`unread_quarters: 31`, seed 2026-11-21); newest pembrolizumab items are from 2022 |
| Withdrawn cancer accelerated approvals | By withdrawal year: 2026: 5; 2024: 5; 2023: 4; 2022: 7; 2021: 7; 2020: 2; 2013: 1; 2012: 2; 2011: 3; none in 2025, 2019 to 2014 or 2010 | `biomcp search drug QUERY --source fda-withdrawn-approvals` (name, indication or year) | Works | One row per indication (Tazverik 2, Ukoniq 2, Imbruvica 2); `biomcp get drug tazemetostat regulatory` is absent (no withdrawal section on `get drug`). The seed (about 35 rows, 2026-11-21) is used only when the live read fails and is flagged stale. `biomcp search drug 2025` is complete with 0 and could not be verified |

Honest behaviour confirmed: the 2020 cutoff is stated by hint; stale and partial states are labelled; the date window of recalls is optional and stated.

Not checked: FDA's own pages for any of these lists; whether 2025 truly had no withdrawal.

### FDA device sources: companion diagnostics, PMA, 510(k), device recall

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Companion diagnostics | 244 rows (235 individual, 9 group); device kinds P 232, DEN 5, H 3, K 2, B(R/K) 2 | `biomcp search diagnostic --source fda-cdx [--biomarker --name --drug-application --regimen-partner --device-id --date-from --sort date]` | By drug: all. By biomarker: individual rows only | `--biomarker EGFR` returns 33 rows and drops the group rows (complete, with a `filter.unsupported` problem); `--biomarker KRAS` 21; `--name gefitinib` returns 11 including 5 group rows. Help states the group-table limit. 6 of 244 (2.5%) lack `device_ids`. Date and identity problems: 3 problems in 244 rows. Stray `fda-cdx next value.unknown` problem on every 510(k) get |
| CDx devices with no pivot | HDE (`H...`) and CBER (`BK...`, `BR...`) devices | `none` | 0 | `biomcp get diagnostic H140006 --source fda-pma` and `--source fda-510k` are usage errors; `biomcp get diagnostic BK241074 --source fda-510k` too. `DEN260027` via `--source fda-510k` works |
| PMA submissions (originals and supplements mixed) | 57,548 | `biomcp search diagnostic --source fda-pma [--name --manufacturer --product-code --intended-use --decision-from --decision-to]` | Works; originals and supplements cannot be filtered apart | A PMA with more than 100 rows returns no card, no supplements and no regulatory text: `biomcp get diagnostic P890003 --source fda-pma` (and P950037) is partial with `collection.incomplete` and no paging `next`; only `--supplement S486` works. Share of PMAs over 100 rows not measured. One missing or duplicate supplement number fails the whole reply (code read, inferred) |
| PMA supplements and summaries | P150035 42 supplements; P170019 59 | `... supplements --section-limit`; `summary`; `regulatory --supplement S057` | P170019 summary: 24 fragments of a 58-page PDF | P890003 summary failed (HTTP 404); older PMAs have no SSED and the reply warns it does not prove absence |
| 510(k) and De Novo | 176,747 | `biomcp search diagnostic --source fda-510k [--name --classification-name --manufacturer --product-code --decision-from --decision-to --aggregate product-code]` | `--aggregate product-code` for glucose gives 58 codes whose sum 1,038 matches the total | Card has 8 fields: no clearance type, panel, third-party flag or summary; no filter for De Novo only (De Novo is in the same total); HDE `none` |
| Device recall | 59,705 recall records. Enforcement class I 3,696; II 35,887; III 1,041 (sum 40,624; a different unit) | `biomcp search diagnostic --source fda-device-recall [--name --firm --product-code --initiated-from --initiated-to]`; `--classification` | `--product-code LWP` 113; `--name pacemaker` 282; card complete (Z-0355-2027) | `--classification` with `--product-code` is refused; a Class I row from search can be absent from get: `biomcp get diagnostic Z-0409-2027 --source fda-device-recall` (a notice says 1 of 2 rows has no exact record) |
| Other: openFDA device endpoints | classification, registration and listing, UDI, COVID-19 serology | `none` | 0 | No route |

Honest behaviour confirmed: an unmatched product code returns an honest 0; recall rows without an exact record are flagged in a notice; the aggregate states whether its sum matches the total.

Not checked: the share of PMAs above 100 rows; 510(k) clearance types (not carried); anything on FDA's site.

### FAERS, MAUDE and VAERS

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| FAERS reports | pembrolizumab 109,152 (108,969 with `--exact-term`); Keytruda 109,152; Kymriah 4,700; Shingrix 2,461; aspirin 888,311; imatinib 67,898 | `biomcp search adverse-event DRUG` | Works; names are widened by default (a notice, not in the query echo) | Counts the drug in any role: no `drugcharacterization` term, so report 27325180 (68 drugs) counts for pembrolizumab. Neither help nor reply says so (live for the count, code for the cause) |
| FAERS seriousness and filters | Serious any 95,711; death 19,427; non-serious 13,441 only by subtraction | `--serious any, death, hospitalization, lifethreatening, disabling, congenital-anomaly, other`; `--aggregate reactions` or `received-year` | Works | No filter for role, country, reporter, outcome, age or sex; no non-serious filter. `--reaction pneumonitis` alone and an unfiltered search are usage errors |
| FAERS per report | seriousness, reporter and country, drug roles, reactions | `biomcp get adverse-event ID [drugs or reactions or unii]` | Works | |
| MAUDE reports | Product code LWP 160,780; Medtronic 3,393,840; `pacemaker` by generic name 768,401; LWP with "Battery Problem" 5,543 | `biomcp search adverse-event --source maude --product-code LWP` and `--manufacturer`, `--problem` | Works | Event types sum to 160,755 (malfunction 67,940; injury 90,926; death 1,872; other 17), 25 short; help says empty types fall outside every count |
| MAUDE card | `biomcp get adverse-event 51244117 --source maude`: 1 device, 2 narratives | sections `devices`, `narrative` | Works | Product problem codes, patient outcomes and problems, report source and remedial action are not carried; problem is usable only as a filter |
| VAERS | n/a | `biomcp search adverse-event Shingrix --source vaers` | MMR, COVID-19 and listed influenza vaccines only | Every other vaccine is a `usage.retired` refusal ("supported vaccines are MMR and COVID19..."); the pinned dataset vintage 03/27/2026 is never echoed (inferred from code) |

Honest behaviour confirmed: a non-drug name gives complete with 0 and no widening; per-report fields are shown; the VAERS refusal is explicit.

Not checked: a suspect-only FAERS count against the source (needs an FDA host); VAERS totals (CDC, not censused).

### ChEMBL

Counts are `total_count` values from ChEMBL's API on 2026-11-27. Reach comes from a stratified sample of 41 molecules drawn at random offsets (7 approved small molecules, 7 approved biologics and similar, 14 phase 0.5 to 3, 3 withdrawn, 5 named with no phase, 2 natural products, 3 unnamed compounds).

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Molecules | 2,921,148 (49,099 with a preferred name). Max phase 4: 4,225; 3: 1,892; 2: 9,054; 1: 1,613; 0.5: 25 (all 16,809 have a name). Small molecule 1,920,259; unknown 404,621; protein 22,799; antibody 1,032 | `biomcp get drug NAME or CHEMBLID --source chembl`; `biomcp search drug NAME --source chembl` | By ID 41 of 41; the default MyChem name lookup resolved 32 of 38 named molecules (84%) | Name search is preferred-name only until it returns zero rows (notice): `biomcp search drug imatinib --source chembl` returns CHEMBL941 only, while `Gleevec` returns 2 (the salt is hidden from the generic-name search) |
| Mechanisms | 7,561 rows | `biomcp get drug X targets --source chembl` | Counted from the full mechanism table: 862 of 5,594 parent molecules (15%) have mechanisms only on a salt or child; 1,626 rows (21.5%) sit on a non-parent molecule | Reads empty for the parent: `biomcp get drug montelukast targets --source chembl` is absent (parent CHEMBL787 has 0, `montelukast sodium` CHEMBL1200681 has 1); `biomcp get drug CHEMBL941 targets --source chembl` is absent (CHEMBL1642 has 4) while Open Targets returns 5. Cause: `crates/biomcp-adapters/src/chembl/mod.rs` sends the exact molecule ID only |
| Indications | 60,055 | `biomcp get drug X indications --source chembl` | Sample consistent with source | `biomcp get drug imatinib indications --source chembl` returns total 52 (CHEMBL941); the source has 82 for CHEMBL1642 and 134 under the parent; osimertinib ChEMBL 12 against Open Targets 24 |
| Withdrawn flag | 364 | `none` | n/a | Not a filter and not shown on the card |
| Warnings, metabolism, ATC | drug warnings 2,304; metabolism 2,147; ATC classes 5,579 (3,651 molecules) | `none` (ATC only through `xrefs --source mychem`) | 0 | |
| Activity, assay, target | 24,527,044; 1,970,438; 18,552 | not from drug routes | n/a | |
| Names with a slash, combinations, regimens | n/a | `biomcp get drug lopinavir/ritonavir --source chembl` | n/a | `failed` (MyChem leg rejected); `Truvada` absent; `Kaletra` absent on ChEMBL; `FOLFOX` usage |
| Section misuse | n/a | `biomcp get drug osimertinib xrefs --source chembl` | n/a | Absent with an empty section instead of a usage error |
| Cross-reference row | n/a | `biomcp get drug "imatinib mesylate" --source chembl` | n/a | Partial with `cross_references.xref_id value.unknown` (a row dropped, flagged) |

Honest behaviour confirmed: the sampled classes (ADC, radioligand, gene therapy, cell therapy, vaccines resolved to a component record, thalidomide, rofecoxib, selamectin, curcumin, artemisinin, ginkgo) all resolve; of 5 empty sections spot-checked, 4 were truly empty in ChEMBL; paging shows total and `next`.

Not checked: maximum phase of salts against parents beyond the counted mechanism table; why 5 upper-case ChEMBL-style names fail on the default route.

### Open Targets

Platform 26.09 (API 26.9.0), GraphQL. Reach comes from the same 41-molecule sample as ChEMBL.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Drugs | 19,170 | `biomcp get drug CHEMBLID --source opentargets`; `biomcp search drug X --source opentargets` | 26 of 41 sampled molecules exist in Open Targets; BioMCP returns 12 of those 26 (46%) | 14 of 26 (54%) fail with `bad-reply` and `value.unknown @record` when Open Targets has `mechanismsOfAction: null` (all 14 do), including approved drugs: `biomcp get drug CHEMBL3301579 indications --source opentargets` fails although Open Targets holds 1 indication (urothelial carcinoma, approval); `biomcp search drug enfortumab --source opentargets` lists it. Cause: a required-field rule in `crates/biomcp-adapters/src/opentargets/map.rs` (inferred) |
| Indications and mechanisms of a drug | per drug (osimertinib 24 indications) | `... indications --source opentargets`; `... targets --source opentargets` | As above | `biomcp get drug "enfortumab vedotin" indications --source opentargets` works (29 rows). Indications cover every clinical stage with the stage shown per row |
| Drug warnings, adverse events, pharmacogenomics, similar entities, literature, parent and child molecules | not counted | `none` | 0 | The code read adds that linked targets, cross-references and withdrawal flags are never requested |
| Targets and diseases | 78,733 and 45,896 search hits | `none` (gene and disease source lists have no Open Targets) | 0 | Tractability, evidence by datasource, associations, clinical reports, credible sets: none. Disease IDs appear only inside drug indications |
| Search noise | n/a | `biomcp search drug imatinib --source opentargets` | n/a | Returns CHEMBL803 cytarabine as a third (fuzzy) hit |

Honest behaviour confirmed: the 12 sampled molecules reported `absent` are truly absent in Open Targets; `failed` is loud rather than silent.

Not checked: pharmacovigilance and tractability counts (no cheap count call).

### MyChem.info and drug identity

No document count was taken. Reach comes from the 38 named sample molecules and the 14 approved molecules tried by ChEMBL ID.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Biologic names on the default route | n/a | `biomcp get drug NAME` | Default lookup resolved 32 of 38 named molecules (84%) | `biomcp get drug pembrolizumab` is `failed` (`not-found`; MyChem returns a first-hit `_id` 187051801 that is a 404 at fetch); `adalimumab`, `nivolumab`, `rituximab`, `bevacizumab` fail with `bad-reply`; `denosumab` fails (its ID redirects to another `_id`). `biomcp get drug Keytruda` and `--source chembl` work; trastuzumab works |
| Case of a name | n/a | same | 5 of the 6 failures | `biomcp get drug AVOBENZONE` is `failed` while `avobenzone` is complete (also EDOTECARIN, RONOPTERIN, RUGONERSEN, ENFORTUMAB); upper-case `IMATINIB`, `OLAPARIB`, `TRASTUZUMAB` work; cause not found |
| ID inputs | MyChem resolves them itself | `biomcp get drug ID` | InChIKey works. ChEMBL ID works on the default route only when MyChem's `_id` is the ChEMBL ID (7 of 14 approved sample molecules) | DrugBank, UNII, CAS, RxCUI, PubChem CID and numeric CIViC ID all fail for osimertinib: `biomcp get drug DB09330` is `failed` with `bad-reply`; `biomcp get drug CHEMBL1315667` fails by default and works with `--source chembl`. `biomcp get drug X xrefs --source mychem` prints exactly these IDs (40 rows for osimertinib) and none can be fed back |
| Failure wording | n/a | n/a | n/a | An unresolved ID, a CID redirect and a typo come back `failed` (`bad-reply` or `not-found`), never `usage`; a nonsense name (`zzzqqqxx`) is a `usage` error, exit 2 |
| Names that work | n/a | `--source mychem or chembl or opentargets` | Generic INN, US brand (Gleevec), salt, research code (AZD9291), synonym | Latin INN `osimertinibum` is `usage`; EU-only `Glivec` is absent on MyChem but resolves on ChEMBL and EMA |
| Combinations and regimens | n/a | `biomcp get drug "lopinavir/ritonavir"` | n/a | `failed` with MyChem `rejected` (also `emtricitabine/tenofovir`); `Truvada` absent; `FOLFOX` usage |
| Carried components (drugbank, chembl, drugcentral, gtopdb, unii, ndc, openfda) | `get drug imatinib xrefs --limit 200`: 137 rows (ndc 64, drugsfda 32, mmsl 4, unii 2, rxnorm 2, nddf 2, vandf 2) | `biomcp get drug X xrefs --source mychem` | The NDC rows are MyChem's, not openFDA's directory | `drugcentral.xrefs.pdb_chem_id value.unknown` drops a field with status complete on MyChem but `partial` on ChEMBL for the same kind of drop |
| Search | n/a | `biomcp search drug NAME` (default source MyChem, limit 10, max 50) | n/a | `total` counts MyChem source rows (15 for pembrolizumab, not 15 drugs). Name get reads at most 250 rows and says so |
| Identity across sources | n/a | n/a | n/a | No cross-source identity: each source resolves the name itself and the first ranked exact match wins (alternatives and total are disclosed). UNII and RxCUI are never join keys, by design |

Honest behaviour confirmed: `--source chembl` resolves names where the default route fails; `name_resolution` shows the total and up to 5 alternatives.

Not checked: why 5 upper-case names fail while others pass; DrugBank full content (licensed carried field).

### EMA

Counted from EMA's medicines JSON (2,765 rows, stamp 2026-11-27T18:07:56Z) and referrals JSON (593 rows).

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Human medicines | 2,365: authorised 1,569; withdrawn 379; application withdrawn 270; refused 67; lapsed 27; opinion 23; expired 17; revoked 10; withdrawn from rolling review 2; suspended 1 | `biomcp get drug NAME or EMEA/H/C/... regulatory --source ema`; `biomcp search drug NAME --source ema` | 11 of 11 sampled across statuses resolved (Sovrima, Mylotarg 2008, Oxbryta, Acomplia, Evlarco) | `biomcp get drug Mylotarg regulatory --source ema` has two records (refused 2008, authorised 2018); the earliest authorisation is picked and the other is listed as an alternative |
| Veterinary medicines | 400 (authorised 310, withdrawn 58, application withdrawn 18, refused 4, opinion 5, lapsed 2, expired 2, suspended 1) | same | Reached (Stronghold, Kexxtone) | No human-only filter; the two mix in one search |
| Flags: generic 398, biosimilar 198, orphan 253, advanced therapy 36, conditional 59, exceptional 66, accelerated 50, PRIME 46, additional monitoring 473 | as listed | `none` | 0 | Reply keys are `record`, `status`, `dates`, `indication`, `attribution`; `biomcp get drug Hefiya regulatory --source ema` has no biosimilar flag (the source says Yes) |
| ATC, pharmacotherapeutic group, MeSH area, INN, species | 2,365 human ATC (133 blank); 107 blank INN; 93 blank active substance | `none` | 0 | |
| Referrals | 593 | only the decision date, folded into a medicine's dates | 0 as an entity | `biomcp get drug "iron sucrose" regulatory --source ema` is absent while a referral for parenteral iron products (Article 31) holds it |
| Orphan designations, DHPCs, shortages, PIPs | not in the two files | `none` | 0 | Separate EMA datasets, not counted |
| Search filters | n/a | `biomcp search drug --source ema` | n/a | No filter for status, human or veterinary, orphan or biosimilar: "withdrawn EU oncology medicines" has no route |

Honest behaviour confirmed: no row drops seen; blank `opinion_status` (2,705 of 2,765) is handled; an absent reply carries an honest notice.

Not checked: Translarna by regulatory read; EMA's separate datasets.

### DDInter

Counted from 15 CSV partitions (about 34 MB), DDInter 1.0, last update 2020-09-04.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Interaction rows | 542,355 rows; 236,834 distinct pairs (each pair appears in two files) | `biomcp get drug X interactions --source ddinter [--partner P]` | Osimertinib: 470 partners (238 major, 227 moderate, 5 minor) = the CSV exactly | Default page 5 (max 200, `next` given). The reply does not say the data are from 2020 |
| Levels | Moderate 322,401; unknown 114,294 (21%); major 80,592; minor 25,068 | `levels` counts in the reply | Verified | |
| Distinct drugs | 1,972 IDs (home page says 1,833 approved) | by name, through other sources' aliases | 5 of 38 sampled named molecules found (all in the CSV) | `biomcp get drug "imatinib mesylate" interactions --source ddinter` is absent although help promises one trailing salt word (`imatinib` gives 672); `Coumadin` absent while `warfarin` gives 864; `vitamin K` absent. Aliases work for `Tylenol` (625), `salbutamol` and `albuterol` (753), `adrenaline` and `epinephrine` (385) |
| Partner names | n/a | `--partner P` | `--partner warfarin` on osimertinib returns 1 row | `--partner Coumadin` and `--partner "warfarin sodium"` are complete with 0 rows although the CSV holds the pair; the reply does not say the name was unrecognised |
| Mechanism, management and alternatives text | on the website per pair | `none` | 0 | Reply hint points to the openFDA label |
| Drug-food and drug-disease | not offered by this release | n/a | n/a | |

Honest behaviour confirmed: counts equal the CSV; `Coumadin` as a primary name returns a clear `absent`.

Not checked: nothing further listed.

### WHO Essential Medicines and prequalification

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| WHO Essential Medicines (eEML): medicines | 650 distinct names (24 with a parenthetical, 62 combinations, 36 with a salt word) | `biomcp get drug NAME essential-medicines --source who-eml`; `biomcp search drug NAME --source who-eml` | Exact eEML name only, case-folded; about 9% (about 60 of 650) have a name a user would not type | `biomcp get drug paracetamol essential-medicines --source who-eml` is absent (eEML says "Paracetamol (acetaminophen)"); `acamprosate` absent while "Acamprosate calcium" works. Absent reads the same as "not on the list" |
| eEML listings and rejected applications | 1 to 14 listings per medicine; rejected applications shown only with `showRemoved=1` | reply `listings` (page 5) | Verified for pembrolizumab (5 of 14 read) and imatinib (3) | `biomcp get drug osimertinib essential-medicines --source who-eml` is `failed` with `bad-reply` because the page counts 1 recommendation but shows no heading (a removed application) |
| WHO prequalification: finished pharmaceutical products | 657 rows (finished product 625, biotherapeutic 31, blank 1). Basis: full 426, abridged 139, alternative listing 77, blank 15; 91 rows have no date | `biomcp search drug NAME --source who-pq`; `biomcp get drug NAME markets --source who-pq` | Dolutegravir 52 rows; `--therapeutic-area oncology` 18 of 18 | Source state `stale` turns every reply partial even with rows (`biomcp search drug imatinib --source who-pq` is partial with 0, the honest reading). The dolutegravir row `ANDA 208355 USFDA` has no date (`not-published`) |
| Therapeutic areas | HIV/AIDS 255; tuberculosis 127; malaria 103; reproductive health 62; Covid-19 20; neglected tropical diseases 19; oncology 18; hepatitis 18 | `--therapeutic-area` | Exact | |
| Prequalified vaccines and APIs | not counted | `none` | 0 | By design; reach is by INN text only |

Honest behaviour confirmed: stale WHO data does not say absent.

Not checked: nothing further listed beyond known limits.

### Monarch, MONDO and OLS, MyDisease, and HPO

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Monarch disease-to-phenotype and gene associations | Disease-phenotype 268,833 (negated 726; not negated 266,900); causal gene 14,027; correlated gene 2,003; gene-phenotype 1,043,507 | `biomcp get disease X phenotypes or genes --source monarch`; `biomcp get phenotype HP:X diseases or genes` | 20 of 43 sampled diseases have phenotype rows and all 19 non-obsolete empties are empty in HPO's annotation file too; NCL genes 43 of 43; Meckel syndrome 34 causal + 1 correlated | Negated rows are mixed in and counted: `biomcp get disease ORPHA:79406 phenotypes --source monarch --section-limit 50` gives 27 rows, 16 with `association.negated true` (HPO has 16 NOT and 11 other). Closure: `biomcp get disease MONDO:0007254 genes --source monarch --section-limit 10` shows rows whose object is MONDO:0016419 and total 22 counts descendants, unstated; phenotype-to-disease is direct only. No taxon filter on disease genes |
| Obsolete MONDO IDs | 4 of 43 sampled | `biomcp get disease MONDO:0020668 phenotypes --source monarch` | Card says `obsolete: true` and `replaced_by` | Section is complete with 0 rows although MONDO:0010047 has 51 phenotypes and 2 genes; `MONDO:0014463` to `MONDO:0000208` (22 phenotypes) likewise |
| Direct against closure counts | `HP:0002197`: HPO direct 116, Monarch 630. `HP:0001250`: HPO direct 2,567 (2,558 diseases, 14 NOT), Monarch 5,350 | `biomcp get phenotype HP:0002197 diseases --section-limit 1` | Tool 115; `HP:0001250` tool 2,561 | Descendants are not rolled up and the reply does not say so; the reason Monarch's number is larger is unconfirmed |
| Monarch records with no route | Variant-disease 205,336; genotype-disease 10,322; model_of 10,324; inheritance 8,944; location 1,017; chemical-disease not counted | `none` | 0 | Variant-disease for NCL has 774 rows in Monarch. Non-HGNC subjects get no next step and no problem (inferred from code) |
| MyDisease and MONDO | MyDisease 352,248 documents; MONDO 36,094 classes (32,137 active, 3,957 obsolete) | `biomcp get disease MONDO:X` | 43 of 43 sampled cards complete, including 4 obsolete | `value.unknown @ mondo.xrefs.icd11` on Huntington disease, status complete |
| Disease IDs other than MONDO | MONDO xrefs: DOID 11,761; OMIM 9,806; Orphanet 9,054; OMIM phenotypic series 612; UMLS 21,650; MeSH 8,002; SCTID 8,888; EFO 2,386; ICD10CM 2,152; MedDRA 1,398; NCIt 7,331; OncoTree 552 | `biomcp get disease DOID:12858 or ORPHA:399 or OMIM:143100` | DOID, ORPHA and OMIM resolve through MyDisease (with a "may be broader" notice) | `biomcp get disease MESH:D006816`, `UMLS:C0020179`, `ICD10:G10`, `EFO:0000373`, `OMIMPS:PS163950` and `MONDO_0007739` are usage errors; `biomcp search disease MESH:D006816` is complete with 0 rows, while MyDisease returns MONDO:0007739 for `mondo.xrefs.mesh:D006816`. NCIt needs `--source ols`. Free text `biomcp get disease "Huntington disease"` is usage |
| Orphanet-only disorders | Orphadata 11,645 disorders; 1,715 (417 with prevalence) have no MONDO mapping in either direction | `biomcp get disease ORPHA:699029` | Card, phenotypes, genes and hierarchy fail | `failed` with `not-found`; `ORPHA:90051 synonyms` fails with `bad-reply`; only `prevalence --source orphadata` works. Genes and phenotypes on unmapped disorders are small today (179 and 18) because they come from Monarch |
| OLS (MONDO, DOID, NCIt, HP) | Terms: MONDO ontology 63,573 (only 36,094 are MONDO classes), DOID 19,825, NCIt 209,910, HP 32,772, EFO 94,705, ORDO 16,296 | `biomcp search disease TEXT [--ontology doid or ncit]`; `biomcp get disease ID synonyms or xrefs --source ols` | `search disease huntington` 17; `--ontology doid` 3; `--ontology ncit` 141 | EFO and ORDO not offered. Search excludes obsolete terms (an obsolete label returns the replacement, unstated). `biomcp search disease carcinoma --offset 9990` returns 0 rows with total 1,220 and no error. MONDO results can include non-human (OMIA) terms with a notice. ID-shaped queries return 0 (OLS also returns 0) with no explanation |
| HPO phenotype terms and annotations | 32,772 terms; annotation file v2026-09-01: 286,651 rows, 12,880 diseases (OMIM 8,478; ORPHA 4,355; DECIPHER 47), 733 NOT rows | `biomcp get phenotype HP:X`; `... synonyms or xrefs or ancestors or hierarchy` | Subontologies resolve; 99.0% of annotated diseases (12,753 of 12,880) and 99.4% of rows map to an active MONDO class (a join of the MONDO and HPO files) | `biomcp search phenotype HP:0001250` returns ranked genes, not the term (stated in help). Obsolete HPO terms not probed |
| Other: defaults | n/a | n/a | n/a | Disease search page 5 (max 50); Monarch genes 2 and phenotypes 1 per page by default |

Honest behaviour confirmed: the obsolete card states `obsolete` and `replaced_by`; `biomcp get disease MONDO:0004173 prevalence --source orphadata` is absent with `orphadata: skipped no-join-id`; every sampled empty phenotype section was also empty in HPO's file.

Not checked: `biomcp search disease --source hpo --phenotype ...`; obsolete HPO terms; why Monarch's own total is larger; the full content of OMIM (licensed carried field).

### Orphadata

Counted from downloaded XML files dated 2026-06-23 (products 1, 4, 6 and 9).

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Disorders and cross-references (product 1) | 11,645: disease 4,773; category 2,175; malformation syndrome 2,064; clinical subtype 1,103; morphological anomaly 532; clinical group 498; etiological subtype 290; particular clinical situation 72; histopathological subtype 64; clinical syndrome 57; biological anomaly 17 | used internally to join ORPHA to MONDO | 9,930 of 11,645 (85%) have a MONDO mapping in at least one direction | 1,715 have none (see Monarch, MyDisease table) |
| Prevalence (product 9) | 6,728 disorders, 17,155 rows (point prevalence 8,304; cases or families 3,619; annual incidence 3,068; prevalence at birth 2,070; lifetime 47) | `biomcp get disease ORPHA:X prevalence --source orphadata` | Huntington (ORPHA:399) 32 of 32 rows; works for Orphanet-only codes (`ORPHA:699029`, `221126`, `90051`). By MONDO ID only when MONDO carries an Orphanet cross-reference (27 of 43 sampled MONDO IDs were skipped `no-join-id`) | `biomcp get disease MONDO:0004173 prevalence --source orphadata` is top-level `absent` with `orphadata: skipped no-join-id` (the code read says it should be partial). 1,873 "Not yet validated" rows are labelled `validation` |
| Onset and inheritance (product 9 ages) | 7,790 disorders (onset 7,032; inheritance 6,872) | same reply | Included | |
| Genes (product 6) and phenotypes (product 4) | 4,245 disorders, 8,503 rows; 4,355 disorders, 116,664 rows | `none` as Orphadata (genes and phenotypes arrive via Monarch) | Not measured | |
| Classifications, functional consequences, linearisation | not downloaded | `none` | 0 | |
| Version label | n/a | reply attribution | n/a | Attribution says "[2025-03-03]" (the database software version string) while the file is dated 2026-06-23 |

Honest behaviour confirmed: a missing Orphanet link is reported with the reason `no-join-id`; validation status is labelled.

Not checked: the classification products (not downloaded).

### SEER

The 73-site catalog comes from `biomcp search disease --source seer --limit 50`; no outside count was taken.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Cancer sites | 73 (All Cancer Sites Combined plus 72 specific, including 4 breast subtypes, 4 lung histologies, 5 brain types) | `biomcp get disease SITECODE incidence or survival --source seer` | A site must be found by SEER name or code | `biomcp get disease MONDO:0044915 incidence --source seer` is a usage error ("not an ontology ID"): no route from an ontology ID |
| Incidence | By site and stage (5 rows for lung) | `--dataset {1,3,4} --period {...}` | `biomcp get disease 47 incidence --source seer --dataset 4 --period 5 --section-limit 3`: 5 rows, all stages rate 47.172044 per 100,000, count 434,448, "SEER 21 Registries (2000-2023)" | Dataset and period are required (no defaults). 4 `value.unknown` problems per reply (`dataset_membership`, `registry_membership`, `standard_population`, `observed_or_delay_adjusted_rate`) with status complete. No stage filter |
| Survival | By site and interval {1,2,3,5,10} | `biomcp get disease 47 survival --source seer --dataset 4 --interval 5` | Works | Period is not required and silently uses `year_range 31` ("1975-2022"); 5 `value.unknown` problems (`modeling_method`, `eligibility`, `age_adjustment` and more) |
| Stratifiers | sex, race, age range | `--sex`, `--race`, `--age-range` | Defaults (both sexes, all races, all ages) are shown in `requested_selectors` and `returned_selectors` | Selector codes are a compiled closed list, so a newer period code is refused before any call (code read) |
| Mortality, SEER prevalence | exist | `none` (`biomcp get disease 47 prevalence --source seer` is usage) | 0 | |
| Catalog metadata | n/a | every reply | n/a | `catalog.publisher_date` and `catalog.publisher_version` come back as `value.unknown` on every SEER reply |

Honest behaviour confirmed: dataset and period are stated; stratum defaults are visible in each row.

Not checked: nothing further listed.

### PubMed, Europe PMC and PMC

Counts come from NCBI `esearch` (count type) and Europe PMC `hitCount` on 2026-11-27.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| PubMed records | 41,511,705; no abstract 11,705,577 (28%); non-English about 4.9M; before 1975 4,491,495 | `biomcp get article PMID`; `biomcp search article` (default source pubmed) | Reached (a 1960 record works) | A record with no abstract carries no `abstract_parts` and no statement of it |
| Retracted, concern and correction notices | Retracted 35,904; expression of concern 4,168; erratum 217,087; preprint 73,604 | `types` in get and search rows | Retracted flagged (Wakefield 9500320 with its notices); erratum and preprint flagged; `--exclude-retracted` works (5 rows against 6) | The notice records have no `types` (`20137807`, `38660329`); `--type` has no erratum, retraction, preprint, letter, editorial or comment value |
| Ahead of print, review, case report, guideline, clinical trial, comment, letter | 250,769; 4,115,317; 2,593,683; 44,183; 1,065,791; 1,074,582; 1,374,488 | `types`; `--type` | Reached | Future dates pass through (42685739 shows 2027-05-19) |
| PubMed books | 40,069 (35,708 with abstract) | `biomcp get article 32134616` | PMID reached, with `text_withheld disabled` | `biomcp get article NBK554742` and `biomcp article text NBK554742` are usage errors |
| Europe PMC preprints | 1,299,466 | `biomcp get article PPR1191464`; `--source europepmc` | Card works | `biomcp get article 10.22541/au.173561968.84978762/v1` (DOI of a Europe PMC-only preprint) is `absent`; `biomcp article citations PPR1191464` is a usage refusal; full text `deadline-exceeded` twice. By default (pubmed only) `biomcp search article "Hereditary Orotic Aciduria uncommon presentation"` is complete with 0 while `--source europepmc` finds 47, and no `next` points to it |
| Europe PMC other record classes | Patents 4,229,296; Agricola 1,026,549; theses 176,799; Chinese Biological Abstracts 142,377; CiteXplore 3,941; NHS Evidence 2,868; bookshelf 5; PMC-only 1,005,699 | `none` (PMC-only partly) | 0%; `biomcp get article PMC2705918` works where PubMed has the PMCID | |
| PMC and open access | In PMC 11,747,341 (28%); OA subset 8,026,306; Europe PMC full text 11,481,499; open access 7,966,841 | `biomcp get article X --section fulltext`; `biomcp article text X` | `biomcp get article 37638106 --section fulltext` data (cc_by, 48 parts); author manuscript `35254056` data (tdm, manuscript true) | `biomcp article text 38376333` and `40073356` return complete with the section `unavailable (not-open)`, while `get article 40073356 --section fulltext` is partial and `38376333` complete. `not-open` merges not in PMC, in PMC but not OA, and embargoed. `42685739` (Europe PMC says open access) is `unavailable not-open` (reason not rechecked) |
| Supplements | per article | `biomcp article supplement 28944238 MGG3-5-553-s002.xlsx` | xlsx parsed (6 sheets); `37638106` lists 2 `.tiff` files | TIFFs are not readable and carry no state saying so |
| Merged search | n/a | `biomcp search article "KRAS G12C sotorasib" --source pubmed --source europepmc --source semantic-scholar --source litsense --source pubtator --limit 5` | Partial when one source fails (LitSense `deadline-exceeded`) | Source counts 556, 2,996, 3,554, failed, 1,672; `page.total` 638 is the merged pool, not any source's total. Europe PMC pool 100 of 106 (truncated, stated) |
| Other: `problems` noise | n/a | n/a | n/a | `electronic_date value.unknown` appears on many PubMed searches whenever the source date is empty |

Honest behaviour confirmed: retraction is stated on the original; DOIs and PMCIDs resolve through PubMed (`10.1038/ng.356`, `PMC2705918`); truncation is stated; `article text` with a DOI is refused as the help says.

Not checked: LitSense beyond one timeout; Semantic Scholar rate-limit behaviour; whether patents or Agricola leak into Europe PMC search; `--license-lookup`; whether an erratum's original links forward to the correction.

### PubTator, LitVar and iCite

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| PubTator annotations | Per paper: Gene, Variant, Disease, Chemical, Species, cell line | `biomcp get article X --section annotations` | Gene, Variant, Disease and Chemical kinds | Species and cell-line mentions are dropped with no problem (`crates/biomcp-adapters/src/pubtator/map.rs`): `biomcp get article 19282848 --section annotations` shows gene 5, disease 5, chemical 2, variant 1, while the export holds disease 16, gene 8, species 7, variant 4, chemical 4 (same-name mentions are also merged). `33123537` loses 1 species. A paper not yet processed (`43102389`) is partial with `rejected`, which is honest. Cell-line mentions were not probed live |
| LitVar variants | Variants with an rsID | `biomcp search article --source litvar --variant rs113488022` | 32,344 papers (pool 9,999, truncated, stated); gene plus protein change resolves to an rsID (BRAF V600E 32,344; TP53 p.R175H 3,093; KRAS p.G12C 8,975) | `biomcp search article --source litvar --gene EGFR --variant p.E746_A750del` is complete and empty while LitVar2 holds 76 papers under rs121913421; `--variant rs121913421` returns 76. `--gene EGFR --variant c.2235_2249del` lists the exact candidate but no rows; `--gene BRCA1 --variant c.5266dupC` lists a candidate (481) and no rows; fusion `--gene EML4 --variant EML4-ALK` is complete and empty (autocomplete also returns nothing); `--variant V600E` without a gene is a loud usage refusal |
| iCite | PubMed-indexed papers only | `biomcp article citations PMID`; `biomcp article references PMID`; `--section citations` | `biomcp article citations 19282848 --limit 3` total 959 = iCite (Europe PMC 953); preprint PMID `36656775` 3 = iCite; references 50 | DOI and PPR inputs are refused (usage) |
| LitSense | sentence search | `--source litsense` | One timeout seen (flagged partial) | |

Honest behaviour confirmed: LitVar truncation is stated; a source failure makes the merged search partial; iCite totals match.

Not checked: `--source pubtator` entity search reach; cell-line mentions live.

### Cellosaurus

Release 57.0 (2026-10-22), 169,026 cell lines, from the registry's release information.

| Record type or entry point | Source holds | BioMCP route | Measured reach | Failure modes found |
|---|---|---|---|---|
| Cell lines | 169,026 | `biomcp search cell-line TEXT`; `biomcp get cell-line CVCL_XXXX` | `HeLa` total 2,355; `iPSC` 1,219; species filter works (`--species "Mus musculus"`); a nonsense name returns 0 correctly | Exact name not ranked first: `biomcp search cell-line HeLa` puts "HeLa Kyoto" first and CVCL_0030 is not in the top five |
| Problem flags | `CVCL_1906` (HEp-2): contaminated, shown to be a HeLa derivative; discontinued by a vendor | `biomcp get cell-line CVCL_1906` | Contamination warning shown on get | `biomcp search cell-line "HEp-2"` rows carry no warning, so a contaminated line looks ordinary in a list. The Discontinued field is surfaced nowhere |
| Category and species | n/a | row fields | Surfaced (category "Induced pluripotent stem cell", "Cancer cell line") | `NIH-3T3` shows diseases total 0 |
| Assays | n/a | `biomcp cell-line assays` (ChEMBL) | Not probed | |

Honest behaviour confirmed: the problematic-line warning on get; species and category on each row; a correct zero.

Not checked: counts by category (the API gave none in the form tried); the full release file (about 1 GB uncompressed).

### NIH RePORTER, ORCID and Semantic Scholar

| Source | Record type or entry point | Measured reach | Failure modes found |
|---|---|---|---|
| NIH RePORTER | Grants: `biomcp search grant "KRAS G12C"` | Total 173 = RePORTER's 173 (fiscal years 2023 to 2026); `--fiscal-year 2020` 9 = 9; `--fiscal-year 2005` 0 = 0; `--investigator "Wei Zhang"` 414 rows | No hidden fiscal-year or active-only default; the total counts award rows (one per award and fiscal year, plus subprojects), and the hint says so |
| ORCID | `biomcp get author 0000-0002-1825-0097`; `biomcp author papers ...` | Usage error: needs a provider-prefixed ID | The `orcid:` prefix form was not tested |
| Semantic Scholar | `biomcp search author "Wei Zhang"` (auto source); `biomcp get article 19282848 --section tldr`; `biomcp article recommendations 19282848`; `biomcp article citation-evidence 33123537 17611497` | Author: 24,158 matching records, ordered not by prominence (ambiguity visible in `matching_records`); `--source pubmed` gives no rows and an "unresolved" note with 29,645 papers; "Pasi Janne" 4 records (top "P. Janne", 1,126 papers); a nonexistent name gives 0 with a notice. TLDR is machine-written and labelled. Recommendations gave 0 with a notice (S2 recommends only from a recent-paper pool). Citation evidence 6 context sentences | None found beyond the ordering |

Honest behaviour confirmed: author ambiguity is visible; the grant total is explained; a nonexistent name gives an honest 0.

Not checked: `orcid:`-prefixed IDs; `author papers` for an S2 ID; Semantic Scholar 429 behaviour.

### Census test ideas

Sizes and times are the findings' own wording. "Gate or scheduled" is given only where a design opinion states it; otherwise it reads "not stated".

| Source | What a repeatable census would read | Size | Rough run time | Gate or scheduled |
|---|---|---|---|---|
| ClinVar | `einfo` and `esearch` counts by variation type, review status, origin, chromosome and classification filter; 1,200 random IDs through `esummary`; 2 IDs per awkward class through `biomcp get variant clinvar:N clinvar`. Fail when any record with submissions returns empty, absent or failed. The code read adds one `efetch` per variation type through the decoder to assert none decodes to a bad reply, and a ratchet on "could not compare" rows | A few MB of JSON; the full XML is too large | About 3 minutes for NCBI calls plus about 70 binary calls (about 1 minute), under 5 minutes in all; code read: about 20 calls | Not stated; code read: under a minute from cached captures, a few minutes live |
| ClinGen (validity, dosage, regions, erepo, actionability) | The validity, dosage and region downloads, the actionability summaries, the paged erepo summary (14 pages), the GenCC export | 1.1 MB, 0.3 MB, 0.25 MB, 0.1 MB; erepo 4.4 MB per page, 61 MB in total; GenCC 27 MB | About 10 minutes in all (erepo about 1 minute at 1 request per second; binary replay about 120 calls, 3 to 5 minutes). Code read: the region TSV (about 1 MB) whole | Not stated; code read: region TSV in a quick run |
| GenCC | `https://thegencc.org` submissions export in the form the adapter uses | 27 MB, 30,388 rows | Included in the ClinGen run above | Not stated |
| CPIC | Per table `Prefer: count=exact` (big tables need only the count); fetch `pair` (about 0.2 MB), `pair_view`, `recommendation` (about 3 MB), guideline view, `gene`, `drug`; compare a sample of 25 genes. Code read: every `/gene` symbol's counts plus 200 random diplotypes looked up in reversed order | About 3 MB | About 30 requests, under 3 minutes at 2 requests per second; code read: about 8 minutes (about 150 genes by 3 counts plus 400 lookups) | Scheduled job (code read) |
| ClinPGx | `clinicalAnnotation` per level (6 levels), `labelAnnotation` and `guidelineAnnotation` per agency or source; check for empty gene lists. Code read: the clinical annotations download classified by gene count and level | About 6 MB (level 3 about 4,479 rows); code read: a few MB | About 20 requests, roughly 2 minutes at 1 request per second; code read: about 2 minutes | Scheduled; noncommercial terms keep real rows out of the repository (code read) |
| GWAS Catalog | Metadata and totals for studies, unpublished studies, associations, SNPs, genes, traits, publications (about 12 requests); a random page sample of associations (30 pages of 500) and 200 random studies; 20 fixed awkward IDs. Code read: totals plus 300 random association IDs and 50 trait labels in mixed case | The full associations file is over the limit and was not downloaded | About 12 requests (seconds); associations sample about 20 minutes; studies about 8 minutes; the fixed-ID check about 40 calls under 2 minutes; code read: about 400 requests, about 6 minutes | Scheduled only (code read) |
| Reactome | `ReactomePathways.txt`, `ReactomePathwaysRelation.txt`, `NCBI2Reactome.txt`; 25 random human IDs per stratum against `containedEvents` and `referenceEntities`, plus 30 UniProt and 10 miRBase inputs. Code read: `UniProt2Reactome`, `Ensembl2Reactome` and `miRBase2Reactome` joined to the HGNC complete set | 1.6 MB, 0.6 MB, 35 MB (chunked); code read: tens of MB plus about 15 MB | About 10 to 15 minutes at 2 requests per second; code read: parse-only work takes seconds | Both: a pinned slice in the landing gate and a scheduled live sample of 50 genes (code read) |
| WikiPathways | Human GPML zip, GMT file and pathway list JSON; compare distinct genes with the GMT row count for every pathway with gene nodes. Code read: `getPathwayInfo.json` and `findPathwaysByText.json` by species | 11 MB, 342 KB, 651 KB; code read: 1.2 MB | GPML parse about 1 minute; about 1,000 binary calls or a 100-pathway sample in 5 minutes at 2 calls per second | Both: a pinned copy in the landing gate and a weekly live refresh (code read) |
| ClinicalTrials.gov | `/api/v2/studies?countTotal=true&pageSize=1` with status and `AREA[...]` filters, compared with `--count-only`; 1,000 random studies by page-token jumps for alias and secondary-ID shares. Code read: `/api/v2/stats/field/values` for status, phase, type and intervention type | One small reply for the stats; do not use the 100+ MB dump | About 60 registry and 60 binary calls, about 3 minutes at 2 requests per second; code read: about 30 requests, under 1 minute | Enum parse in the landing gate (pinned stats file); live counts nightly (code read) |
| NCI Clinical Trials Search | `biomcp search trial --source nci --count-only` per status and per phase (sums must equal 27,775 and 27,221), 10 pages of 50 for missing NCT IDs, biomarker name searches | n/a | About 40 binary calls, under 2 minutes; code read: about 2 minutes plus a 500-row sample | Scheduled only (needs a key; code read) |
| gnomAD | GraphQL gene query for variant IDs, rsIDs and filters (3,340 TP53 rows in one reply; 20,660 for three genes); for each sampled ID the VCF, rsID and NC_ forms through `variant population`. Code read: a per-chromosome sample including M through the join, with a live r4 check | The per-chromosome sites file is far larger than 100 MB | 50 genes in 2 minutes at 1 request per 2 seconds; 200 IDs by 3 forms is 600 calls, about 15 minutes; code read: seconds | Not stated |
| MyVariant.info | Random ClinVar-carrying and gnomAD-exome-carrying hg38 records per chromosome, then a batch POST for membership. Also CAR REST on sampled ClinVar HGVS | About 400 rows | About 25 requests, 1 minute; full CAR run 2,000 rows at 2 requests per second, about 17 minutes | Not stated |
| Cancer Hotspots | `/api/hotspots/single/byGene/GENE` for every hotspot gene, comparing row counts and the type split with the tool's records | Each reply small; about 1,000 genes | About 15 minutes at 2 requests per second | Not stated |
| cBioPortal | `/api/studies` and `/api/molecular-profiles?projection=SUMMARY` for type counts; mutation rows for 3 studies by genes on X, Y and MT, run through the matcher | 554 rows (about 300 KB) and 915 KB; code read: about 12 calls | Two requests for counts; 548 per-study calls about 5 minutes at 2 requests per second (large studies are tens of MB); code read: under 30 seconds | Not stated |
| HGNC | `hgnc_complete_set.json` and `withdrawn.json`; classify by locus type, then sample symbols per stratum through `get gene`; add the 20 no-UniProt symbols as a fixed set | 33 MB and 0.7 MB | Parsing under 10 seconds; a 70-symbol sample with three sections about 1 minute | Not stated |
| MyGene.info | Per-gene calls for a 70-symbol sample | n/a | Seconds per call | Not stated |
| UniProt | `x-total-results` queries (about 25) plus 70 accessions from a reviewed human stream; fixed awkward set (A0A0A0MS99, P02340, P03366, P00722, P38398-2, Q13290, A0A008APQ8, Q8WZ42, P04637) | About 2 MB | About 15 seconds for counts; about 2 minutes in all | Not stated |
| InterPro | `entry/interpro/protein/uniprot/ACC` and `entry/unintegrated/...` for 70 reviewed human accessions, compared with `protein domains` | n/a | 2 requests per accession, about 2 minutes | Not stated |
| Complex Portal | Search `*` per species (5 calls), then 30 sampled complex IDs through `protein complex-members` | n/a | About 3 minutes | Not stated |
| HPA | Per-gene JSON for the 70 sampled Ensembl IDs, counting populated top-level keys against the tool's five | Bulk TSV about 10 MB zipped, not fetched | About 2 minutes (0.5 s each) | Not stated |
| STRING | `interaction_partners` for 30 sampled accessions at `limit=0` against `total` | n/a | About 1 minute | Not stated |
| gnomAD constraint | The constraint metrics file compared with HGNC coding genes | About 30 MB, not fetched | Under 2 minutes | Not stated |
| openFDA label | Bulk label download (about 12 files, about 1.5 GB, so sample) or DailyMed `spls.json`; 300 random set IDs through `get drug SETID label` | about 1.5 GB (sample, do not scan) | About 1 hour | Not stated; code read: the FDA harness rule means only a product-side census is feasible |
| Drugs@FDA | A name list (WHO INN list, 300 names) through `search drug NAME --source drugsfda` and `get drug NAME approvals`, run through the tool only | n/a | About 40 minutes at 2 calls per second | Not stated |
| Purple Book | Every product name and BLA in the seed CSV through `search drug NAME --source purple-book`, counting zero-row results (code read: about 2,270 rows, runs locally with no network) | Trivial (about 2,000 lines per the first file) | Minutes; code read: seconds, offline | Landing gate (code read) |
| FDA drug recall, Drug Safety Communications, withdrawn approvals | Loops through the tool only: classification by status by year windows (3 by 3 by 27 calls); one DSC page; years 2011 to 2026 for withdrawals (16 calls) | n/a | Minutes; trivial | Not stated |
| FDA companion diagnostics and PMA | The full CDx list (244 rows, 5 calls, done) and a pivot test per row (244 calls); 100 sampled P numbers through `get diagnostic` | n/a | About 5 minutes; about 100 calls, 1 minute at 2 per second | Not stated |
| FAERS | `--aggregate received-year` per drug for 30 drugs (no count census is applicable) | n/a | n/a | Not stated |
| ChEMBL | Full `mechanism.json` (8 pages of 1,000), `drug_indication.json` (61 pages), `molecule.json?max_phase__gte=0.5` (about 340 pages); then `get drug CHEMBLID targets or indications --source chembl`. Code read: a fixed sample of 50 parents whose rows sit only on children | About 2 MB (code read) | About 20 minutes of API time at 2 requests per second and about 34,000 binary calls for all phase 1 to 4 molecules, or 500 sampled for about 15 minutes; code read: 2 to 3 minutes at 1 request per second | Scheduled weekly, with a recorded 3-case replay in the gate (code read) |
| Open Targets | GraphQL `drugs` or `search` pages of 50 (19,170 drugs, about 400 requests), recording null mechanisms; then 500 random drugs through `get drug CHEMBLID indications --source opentargets` | n/a | About 15 minutes for the API half and 30 minutes for the binary half | Not stated |
| MyChem.info and drug identity | The 41-molecule sample plus the 16,809 phase 1 to 4 names; `get drug NAME` for 500 of them, classified by failure reason | n/a | About 25 minutes, pure binary | Not stated |
| EMA | The two JSON files; `get drug EMEA/H/C/... regulatory --source ema` on a 300-row sample | 7.5 MB | 5 seconds to read; about 10 minutes | Not stated |
| DDInter | The 15 CSVs, a 1,972-name list, 200 sampled names plus 200 `--partner` calls using brand names | 34 MB | About 25 seconds to download; about 10 minutes | Not stated |
| WHO Essential Medicines and prequalification | eEML search page (650 names, one request); the prequalification CSV (657 rows) through `search drug INN --source who-pq` | 1.3 MB; 129 KB | eEML: 100 names about 15 minutes (one page per second); prequalification under 5 minutes | Not stated |
| Monarch, MyDisease, OLS, HPO | `phenotype.hpoa` (about 10 seconds to read) against `get disease X phenotypes` for 200 sampled MONDO IDs and `get phenotype HP:X diseases` for 100 HP IDs; `mondo.obo` for 300 sampled classes and each xref family. Code read: direct against closure counts for a 100-MONDO sample | 35.8 MB; 53 MB | About 15 minutes; about 20 minutes; about 10 minutes; code read: about 200 calls, about 4 minutes. OLS paging is too slow (18 s per page of 200), so use the MONDO file | Scheduled (code read for Monarch) |
| Orphadata | The five XML files; 200 sampled codes through `get disease ORPHA:X prevalence --source orphadata`. Code read: product 9 prevalence plus MONDO's SSSOM file, to track the reachable share as a ratchet | 150 MB; code read: about 10 MB plus about 30 MB | About 2 minutes to parse and about 10 minutes of calls; code read: about 1 minute locally | Scheduled, the share as a ratchet number (code read) |
| SEER | The 73-site catalog (one call) crossed with datasets {1,3,4} and one period; code read: every published period code is accepted | One call | About 220 calls, 4 minutes; code read: 1 call | Scheduled, catches new releases (code read) |
| VAERS | CDC WONDER vaccine list, share of codes with a route | 1 or 2 calls | n/a | Scheduled (code read) |
| PubMed, Europe PMC and PMC | `esearch` counts per publication type, language and flag (about 30 requests at 3 per second); Europe PMC `hitCount` for sources and flags (about 15 requests); 200 PMIDs per class through `get article`; 100 PPR IDs and DOIs | n/a | Under a minute for PubMed counts; a few minutes for 200 `get article` calls | Not stated |
| PubTator and LitVar | 100 PMIDs against the PubTator export (about 5 minutes); 100 ClinVar variants with no rsID against LitVar autocomplete | n/a | About 5 minutes | Not stated |
| Cellosaurus | The API search with `fields=ac,cc` on 500 random accessions, comparing warnings, discontinued status, category and species with `get cell-line` | The release file (about 1 GB) is over the size cap | Not stated | Not stated |
| NIH RePORTER | v2 `projects/search` totals by fiscal year and text | No download needed | Not stated | Not stated |
| Design opinions that span sources | The code reads propose two layers: an offline landing-gate run on pinned slices replayed through the mappers (every source value parses; every record maps or is refused with a problem; no class of subject returns complete with zero rows when the slice holds rows for it), and a scheduled live job that fails when reach drops below a recorded floor. One audit asks that every row-filtering mapper return a "could not compare" count that reaches status | Small pinned slices (for example WikiPathways catalog JSON 1.2 MB) | Landing gate under a minute; scheduled jobs 5 to 15 minutes per source, polite | See the rows above |

