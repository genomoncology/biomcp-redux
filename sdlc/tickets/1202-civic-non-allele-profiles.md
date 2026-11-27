# 1202 — civic-non-allele-profiles

Status: OPEN.

Milestone: 7

## Outcome

An agent can read CIViC evidence and assertions for any CIViC molecular profile, not only profiles whose members carry a ClinGen allele ID, and without naming a therapy. The witnesses are a Factor profile (MSI High, molecular profile 5118), a Region profile (17p Deletion, molecular profile 5877), a Fusion profile with no therapy (ETV6::NTRK3 Fusion in congenital mesoblastic nephroma, EID10841) and a categorical gene profile (CDKN2A Loss OR CDKN2A Mutation, EID11654). Each profile member also states its CIViC feature type (Gene, Fusion, Factor or Region), with the Factor's NCIt ID and the Region variant's ISCN name when CIViC gives them.

The BioMCP PM chooses how the lookup is addressed. Options: (a) `get variant` accepts a CIViC molecular profile ID or exact profile name when `civic` is the only section; (b) a CIViC profile form on `search variant` that returns profile IDs for `get variant --civic-profile`; (c) gene-level evidence on `get gene ... civic` for features that are genes, with Factors and Regions under (a) or (b). Recommendation: (a), because it keeps one command for "evidence about this alteration" and reuses the existing `--disease`, `--drug`, significance and status filters.

Two further requirements (BioMCP PM, 2026-11-27):

- Empty versus can't look: when BioMCP cannot confirm a CIViC match, for example a gene variant with no ClinGen allele ID, the reply never says `complete` with nothing. It either returns the candidate profile, labelled as a name match, or reports the CIViC source as incomplete with the reason.
- Reach census: a test, run as a pm job, reads CIViC's own nightly data release and counts the share of evidence items each BioMCP route reaches (variant, drug, profile lookup), broken down by profile type: gene variant with a ClinGen allele ID, gene variant without one, factor (such as TMB or MSI), fusion, region, and with or without a therapy. It records today's numbers before the fix and the numbers after it, and fails when reach drops below the recorded after-fix figures.

## Why

CIViC 2026 (Nucleic Acids Research, doi:10.1093/nar/gkag1156) made Features the top-level entity, with Fusions, Factors for genome-wide biomarkers such as MSI and TMB, and Regions for cytogenetic changes. Today `get variant` reaches CIViC only through a CAid, `get gene ... civic` gives a count and a description, and `get drug ... civic` reaches any profile only through a therapy. So prognostic, diagnostic, oncogenic and predisposing evidence for a Factor, a Region, a Fusion or a categorical variant has no route at all. Every Region evidence item in CIViC on 2026-11-27 was prognostic with no therapy, and MSI High's prognostic items (EID1940, EID12039, the paper's own example) were unreachable.

## Evidence

Checked 2026-11-27 against live CIViC GraphQL and the 1.0.0 build at d1c6afa3def3.

- `get variant "MSI High" civic`, `"TMB High"`, `"EML4::ALK Fusion"` and `"5q deletion"` are refused as usage errors; `get variant "ABL1 D276G" civic` returns `civic` unavailable, `no-join-id`, until the caller converts to a CAid.
- `get gene ALK civic` gives `variant_count` 45 and a description; ALK's fusions are separate Fusion features and are not mentioned.
- `get drug pembrolizumab civic` returns TMB High profiles with member `feature: "TMB"` and `feature_record: "civic-feature:61745"`, but no feature type, so an agent cannot tell a Factor from a gene symbol.
- CIViC's root queries take `molecularProfileName`, `molecularProfileId`, `variantId`, `diseaseName`, `therapyName`, `status` and `significance` (`evidenceItems`), and `molecularProfiles(name:, featureId:, variantId:)`. `Factor` has `ncitId`; `RegionVariant` has `iscnName`; `Fusion` has `fivePrimeGene` and `threePrimeGene`.
- CIViC had 4 Regions, 11 Factors and its Fusions as separate features on that date.
- CIViC's nightly release (accepted items) held 5,024 evidence items. 2,291 (46%) sit on a profile with at least one member that has a CAid, which `get variant` can reach; 1,574 (31%) have no such member but name a therapy, so only `get drug` reaches them; 1,159 (23%) have neither and no BioMCP route reaches them. Of 1,775 gene variants in that release, 882 have no CAid.
- A gene variant without a CAid reads as `empty`, not as unreachable. `get variant CA397842035 civic` (TP53 R158G) returns `civic` state `empty` with problem `hgvs_p value.unknown`, yet CIViC holds TP53 R158G (variant 4115, EID10560) with no allele registry ID and no coordinates. In the CIViC benchmark pilot, a chatbot agent with only BioMCP's tools tried 30 calls and gave no answer for this item. The section must say the CIViC record may exist but could not be joined, and the profile lookup above must reach it.

## Proof

- Proof: the reach census job's before and after numbers, with the after numbers kept as its floor; an outside-in case where TP53 R158G (no ClinGen allele ID) returns a labelled name match or an incomplete CIViC source with its reason, never `complete` with no rows.
- Proof: outside-in cases that fail today: the four witnesses above return their evidence items with disease and drug filters applied, recorded once from live CIViC; a member of the TMB High profile states feature type Factor and its NCIt ID; a 17p Deletion member states Region and its ISCN name; the drug and variant routes return the same statuses for one profile with submitted items.

## Starts from

- Starts from: `crates/biomcp-adapters/src/civic/` (`variant.rs`, `variant_filter.rs`, `drug_request.rs`, `variant_subject.rs`), the profile model from 0473, and the CIViC data model at https://docs.civicdb.org.

## Keeps

- Keeps: CAid-based variant lookups and their replies unchanged; rejected items stay excluded by default; no allele identity is derived from a profile label.

## Defers

- Defers: CIViC source retraction (`source.retracted`) and open flags (`flagged`) on evidence items, which CIViC uses for curation review; and organization approvals on assertions. File separately if a persona case needs them.

## Changes

- Changes: `crates/biomcp-adapters/src/civic/`, the CIViC parts of `crates/biomcp-query/src/variant/`, the drug CIViC section, schemas, help text and cases.
