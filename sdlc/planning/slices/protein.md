# Protein slice proposal

This investigation starts from `85e6ba864c3f13ec50432a58670ec2af13e12857` and follows steps 1 to 4 of the [slice method](../slices.md). It proposes entity access to source records. It adds no sequence analysis, clinical classification, network inference or external bioinformatics execution.

## Evidence recipe

The bounded native investigation asks UniProt, InterPro, Complex Portal and STRING for the same human accession set: P00533 (EGFR), P15056 (BRAF), P38398 (BRCA1), P04637 (TP53), P13569 (CFTR), Q9UIF7 (MUTYH), P31749 (AKT1), P42771 and Q8N726 (the two CDKN2A protein products). Reused replies answer multiple questions; reuse is recorded rather than counted as another measurement. Each source receives one request per accession, at most 36 physical GETs, with no retries, redirects, fallback or additional pages. Each reply has a 30 second deadline and a 4 MiB admission bound. Requests are sequential with at least one second between them. A source failure consumes its slot and remains a failure.

UniProt exact record JSON proves accession, taxon, sequence, features, isoform descriptions and cross-references. InterPro uses its accession-scoped entry endpoint with page_size=25; a next pointer proves only that this first page is incomplete. Complex Portal uses its accession search with human taxon filter and number=25; search text alone never proves participant membership. STRING uses the public network endpoint with species=9606 and limit=5; returned stable protein identifiers and scores remain source statements, and a network association never becomes a physical interaction.

Record the exact URL, request count, status, headers, elapsed time, raw bytes, SHA-256, returned identifiers, native count and continuation. Preserve failures unchanged. No raw source bodies enter Git. Archive the evidence on Yellow and independently verify every archive member before deleting unique scratch. Bundle downloads are deferred unless the API evidence proves a need. No whole proteome, STRING network or InterProScan installation is part of this investigation.

The 0.9 executable remains unmeasured until a denied-network replay is verified. Source inspection and existing fixtures establish leads only. Proposed command budgets remain proposals until a measured projection over retained native replies establishes their size. The PM reviews the completed proposal on this branch before product tickets exist.

## Bounded identity follow-up

The initial captures expose missing search, explicit isoform-sequence and STRING mapping evidence. A separate follow-up permits at most 15 GETs: reviewed human UniProt search for `gene_exact:BRAF`, `gene_exact:CDKN2A` and the literal absence control `accession:NONEXISTENT0190`, each size5; exact UniProt FASTA for P38398-2 and Q9UIF7-2; STRING `get_string_ids` for each of the nine original accessions with species9606 and limit1; and Complex Portal exact `complex/CPX-715`. The last endpoint is a hypothesis, and a non-success consumes its slot without URL repair. Every call retains the same deadline, body bound, request spacing and archive rules. No further network leg is implied.

## Harvested questions and accepted source answers

These are source-derived research questions, not quotations or claims that a named user asked these exact words. The paper supplies the human-health problem; the captured publisher record supplies the accepted entity fact. A historical paper's residue spelling stays literal. No question asks BioMCP to classify a variant, predict a structure, infer an interaction or recommend treatment. Coordinates below use one-based inclusive source numbering. Source records are the nine accession captures in the evidence archive. Clinical review checks the interpretation boundary separately.

| Case | Question and harvest source | Accepted answer and command coverage |
| --- | --- | --- |
| P01 | Which feature contains displayed EGFR residue858? [Paez, PMID15118125](https://pubmed.ncbi.nlm.nih.gov/15118125/) | P00533 displayed sequence, Protein kinase712–979; `protein features P00533 --position 858`. No efficacy conclusion. |
| P02 | Which feature contains EGFR residue790? [Kobayashi2005](https://doi.org/10.1056/NEJMoa044238) | Same source kinase range; source active-site837 remains a different annotation. Position filter uses exact checked coordinates. |
| P03 | Which accession and sequence version qualify that EGFR numbering? Same paper | P00533, source entry sequenceVersion2, length1210; `get protein P00533`. The sequence audit is not an independently supplied isoform version. The capture explicitly lists displayed P00533-1; do not synthesize -1 on other entries. |
| P04 | Which EGFR alternative products does the source describe? Same paper and [UniProt P00533](https://www.uniprot.org/uniprotkb/P00533/entry) | P00533-1, P00533-2, P00533-3 and P00533-4; Displayed versus Described status preserved; `protein isoforms P00533`. A list is not an alignment. |
| P05 | Which source domain contains current displayed BRAF residue600? [Davies, PMID12068308](https://pubmed.ncbi.nlm.nih.gov/12068308/) | P15056 Protein kinase457–717; historical paper V599 is not translated to V600. `protein features P15056 --position 600`. |
| P06 | What other BRAF domains are supplied? Same paper | UniProt RBD155–227 and kinase457–717, zinc-finger234–280 as a distinct type; InterPro has11 entries across different types. `protein domains P15056`. |
| P07 | Which BRAF record should an agent get after a gene search? Same paper | Native human reviewed UniProt candidate accession, with an exact get action; `search protein --gene BRAF`. Captured search returned exactly P15056. |
| P08 | Can a description mentioning BRAF prove complex membership? [Complex Portal2025](https://pmc.ncbi.nlm.nih.gov/articles/PMC11701666/) | No. CPX-26353's participants omit P15056, so exclude it even though its description mentions BRAF. `protein complexes P15056`. |
| P09 | Does a predicted complex mean experimentally curated membership? Same source | Keep `predictedComplex` literally; BRAF native search has18 hits,12 exact protein-participant admissions. Never label all12 curated. |
| P10 | Where are BRCA1's two BRCT features? [PMID9000507](https://pubmed.ncbi.nlm.nih.gov/9000507/) | P38398 BRCT1 1642–1736 and BRCT2 1756–1855, separately; the intervening gap stays outside either feature. `protein features P38398 --type domain`. |
| P11 | Which source annotation identifies the BRCA1 RING region? Same paper and [UniProt P38398](https://www.uniprot.org/uniprotkb/P38398/entry) | Source type Zinc finger, RING-type24–65; do not rename it a Domain feature. The type filter covers an exact documented native feature type. |
| P12 | Which BRCA1 complex has BARD1 as a participant? [Complex Portal2022](https://pmc.ncbi.nlm.nih.gov/articles/PMC8689886/) | Native CPX-715, BRCA1-BARD1 complex, predictedComplex=false, exact source participant IDs. `protein complex-members P38398 --complex CPX-715`. |
| P13 | Which BRCA1 alternate accession can qualify a sequence? [PMID14578343](https://pubmed.ncbi.nlm.nih.gov/14578343/) supplies BRCT research context only; [UniProt P38398 alternative products](https://www.uniprot.org/uniprotkb/P38398/entry) supplies the isoform identity | Native P38398-2 remains distinct from displayed P38398. Exact sequence retrieval must name and validate the returned isoform; it cannot inherit displayed coordinates. `protein sequence P38398-2`. |
| P14 | Which TP53 sequence owns the displayed DNA-binding range? [Bourdon, PMID16131611](https://pubmed.ncbi.nlm.nih.gov/16131611/) | P04637 displayed sequence length393; DNA binding102–292. `protein features P04637 --type dna_binding`. |
| P15 | What TP53 alternative products are explicitly described? Same paper | P04637-1 through P04637-9 in the captured alternative-products record; list all through bounded pages without making a residue translation. `protein isoforms P04637 --limit 5`. |
| P16 | Can TP53 family and tetramerisation annotations all be called domains? Same paper and [InterPro FAQ](https://interpro-documentation.readthedocs.io/en/latest/faq.html) | No. IPR002117 is family; IPR010991 is domain. Preserve native types and match fragments. `protein domains P04637 --type domain`. |
| P17 | Which source region contains CFTR displayed F508? [Riordan, PMID2475911](https://pubmed.ncbi.nlm.nih.gov/2475911/) | P13569 ABC transporter1 423–646. Overlap does not label F508 itself ATP-binding. `protein features P13569 --position 508`. |
| P18 | Is absence of a CFTR complex a source failure? Same paper and Complex Portal accession query | The captured human search is HTTP200 with total0. This is source-local confirmed empty; it says nothing about every biological complex. `protein complexes P13569`. |
| P19 | Can historical MUTYH residue labels be transferred to another isoform? [PMID15987719](https://pubmed.ncbi.nlm.nih.gov/15987719/) | No transfer is proved. Captured Q9UIF7 has S368 and G393; preserve the paper's G382D literally. `protein sequence Q9UIF7` supplies source sequence, not an alias conversion. |
| P20 | Which distinct protein records represent CDKN2A products? [UniProt-cited PMID11314011](https://pubmed.ncbi.nlm.nih.gov/11314011/) and [UniProt P42771](https://www.uniprot.org/uniprotkb/P42771/entry) | P42771 length156 and Q8N726 length132. Alternative-product links with status External keep their own accession. `search protein --gene CDKN2A`, then exact gets. |
| P21 | Does ARF have an InterPro Domain entry on this snapshot? Same native source question | Q8N726 has one captured entry, family IPR010868; a domain-type selection is empty, not a missing source or a universal claim about ARF structure. |
| P22 | Does STRING's EGFR network consist only of EGFR edges? [STRING score documentation](https://string-db.org/help/scores/) | No. Network response includes CSK-HBEGF. Admit endpoint-qualified associations only after an explicit STRING mapping; preserve confidence and evidence channels. `protein associations P00533`. |
| P23 | Can a corporate CA failure be shown as no protein? [Public issue250](https://github.com/genomoncology/biomcp/issues/250) | No. Identity-source failure is failed, optional-source failure partial, each with problem and fresh action. Synthetic failure cases preserve the distinction; no natural failure occurred in36 successful probes. |
| P24 | Should protein access install and execute another analysis package? [Public issue78](https://github.com/genomoncology/biomcp/issues/78) | No. Retrieve existing source records and native sequences; never run alignment, prediction, enrichment, InterProScan or another bioinformatics tool. |
| P25 | Which domain contains displayed AKT1 E17? [Carpten, PMID17611497](https://pubmed.ncbi.nlm.nih.gov/17611497/) | P31749 PH5–108, separate from kinase150–408. `protein features P31749 --position 17`; source overlap, no oncogenicity classification. |
| P26 | Which current source feature contains displayed EGFR C797? [Thress, PMID25939061](https://pubmed.ncbi.nlm.nih.gov/25939061/) | P00533 Protein kinase712–979. `protein features P00533 --position 797`; the paper motivates retrieval and the command makes no resistance prediction. |
| P27 | Which protein identifier and length qualify the MUTYH record? [PMID15987719](https://pubmed.ncbi.nlm.nih.gov/15987719/) | Q9UIF7, length546, Alpha-1 displayed. Its alternative IDs Q9UIF7-2 through Q9UIF7-6 remain explicit alternatives. `get protein Q9UIF7`, `protein isoforms Q9UIF7`. |
| P28 | Which BRCA1 candidate complexes are source-reported rather than text mentions? [Complex Portal2025](https://pmc.ncbi.nlm.nih.gov/articles/PMC11701666/) | Six exact-member admissions among nine search hits; CPX-715, CPX-845 and CPX-955 have predictedComplex=false, three others true. Preserve IDs, flags and snapshot scope rather than claiming a universal total. |

P01–P21 reuse the same nine records across all four candidate sources; the ledger records one request per source/accession, not one per question. Source-specific evidence can answer only its own fields. InterPro does not answer complex membership; Complex Portal and STRING do not establish residue coordinates; UniProt does not make an arbitrary transcript-to-isoform mapping. P23/P24 are failure and boundary requirements, so the corresponding native source answer is not applicable rather than fabricated. The test proposal adds authored429/500/timeout/malformed-body cases.

## Source findings and terms

All36 initial requests returned HTTP200 with no retry or redirect. These are first-observation network calls from M5 under shared load, with unknown upstream/CDN warmth. The first source job reported launch load6.89 on18 cores beside three other jobs, with a1GiB memory and1GiB disk booking. They are not quiet-machine speed baselines or provider throughput guarantees. No warm-call timing is claimed.

| Source | Requests | Raw bytes min–max | Seconds min / median / max | Public-tier findings |
| --- | --- | --- | --- | --- |
| UniProt | 9 | 55,086–874,741 | 0.601 /0.822 /1.837 | Exact primary accession, human taxon9606, entry audit, displayed sequence, features and evidence, alternative products and cross-references. Raw TP53 is nearly875KB, which rules out default whole-record output. |
| InterPro | 9 | 919–9,975 | 0.407 /2.326 /3.578 | Native count1–15, next/previous, metadata accession/name/type, protein accession/taxon/length and per-location fragment arrays. A family, site, repeat or superfamily is not relabeled a domain. |
| Complex Portal | 9 | 63–186,368 | 0.403 /0.518 /0.738 | Search totals0–18, actual participant IDs, predicted flag and native stoichiometry string. Largest admitted complex has93 participants, so member lists require separate pages. |
| STRING | 9 | 1,644–4,152 | 0.196 /0.215 /0.314 | Native network edges6–15 despite limit5; limit controls added nodes, not edge count. Both stable endpoint IDs, taxon and distinct evidence-channel scores are available. An explicit query mapping is required before association admission. |

| Source | Use and group | Terms and decision |
| --- | --- | --- |
| UniProt | attribution; no restricted-use group | [CC BY4.0](https://www.uniprot.org/help/license); preserve attribution and source record URL. Keep identity, source features, sequences, isoform descriptions and structural cross-references. |
| InterPro | open; no restricted-use group | [InterPro data CC0](https://interpro-documentation.readthedocs.io/en/latest/license.html). Member-database-specific material keeps upstream rights; request integrated InterPro entries. Keep source match locations, not a local scan. |
| Complex Portal | open; no restricted-use group | [Publisher's2025 resource paper](https://pmc.ncbi.nlm.nih.gov/articles/PMC11701666/) states data CC0. Keep exact-member relations, native predicted status and bounded participants. |
| STRING | attribution; no restricted-use group | [Current licensing](https://string-db.org/cgi/access) states CC BY4.0 including commercial reuse, with original rights on incorporated material. Do not apply the old version10 academic-only license to current outputs. Defer: explicit mapping collapses distinct CDKN2A protein products; no sequence-specific association claim is proved. |

The identity follow-up proves a lossy STRING boundary: P42771 and Q8N726 both map to 9606.ENSP00000418915 and return identical networks. The source accepts both queryItems, but this does not prove sequence-specific protein identity. Defer the STRING adapter for the first protein slice; retain this finding and its source association option for PM review. P22 records why a universal direct-protein interpretation is invalid. No hidden symbol or gene-name repair is permitted.

No candidate source requires a key for these public endpoints. All obey open-by-default source policy and direct source disables. The existing noncommercial/restricted groups do not contain these open or attribution sources. Output includes registry-owned use/terms/terms_url rather than copied text in each type. Source group names follow the existing registry when implemented; this table proposes no new shared group.

[UniProt's publisher training](https://www.ebi.ac.uk/training/online/courses/uniprot-exploring-protein-sequence-and-functional-info/why-do-we-need-uniprot/) and the [InterPro2025 paper](https://pmc.ncbi.nlm.nih.gov/articles/PMC11701551/) describe eight-week releases. Complex Portal changes through curated and predicted releases; a fixed cadence is not established here. STRING access reports release12.5 since2026-09-29; this is an observed release, not a promised cadence. Bind source evidence to its returned audit/release metadata and retrieval date. No body provides permission to invent a sequence version absent from that record.

### Bundles and rejected work

Prefer the measured accession APIs. UniProt bulk human proteome, InterPro match bundles, ComplexTab and whole-human STRING files are not downloaded or measured here. Their full sizes remain unmeasured; no local bundle or sync command is proposed because each harvested question names an entity and the bounded replies already answer it. Record-specific cached replies do not imply a whole-source mirror. A later batch-use question must establish bundle size, update rules and need before adding sync. InterProScan execution, STRING enrichment, sequence alignment and structure prediction are excluded by the entity boundary. PDB/AlphaFold cross-references are links, not downloaded structure files or coverage claims.

## Proposed identity, types and schema

The shared model remains [biodata's agreement](../biodata-variants-and-annotations.md). The type ticket adopts `UniProtIsoform`, `AminoAcids`, `UniProtFeature` and `ProteinSite` exactly as that agreement defines them. These are shared-model commitments, not claims that these types already exist in this checkout. `UniProtIsoform` keeps P15056 or P15056-2 as supplied; it never appends -1. `AminoAcids` holds nonempty uppercase standard residues plus U/O/X. `Position1` names queried residue positions; source-inclusive start/end become `UncertainSpan0` with their uncertainty retained. InterPro fragments remain arrays, including discontinuity, rather than a single min/max hull. No feature lacking exact bounds participates in an exact position-overlap claim unless the uncertainty contract can prove it.

| Proposed consumer type | Fields and source contract |
| --- | --- |
| `ProteinHeader` | exact accession, entry ID, source protein name, source gene names/cross-reference IDs, taxon, reviewed status, length when supplied, publisher URL. Names are display fields, not join keys. |
| `ProteinCard` | header, source entry/sequence audit, requested sections, `more`, next actions. No default sequence, reference list, variant list or whole network. |
| `ProteinIsoform` | every stated isoform ID, name, synonyms, Displayed/Described/External status, source VSP sequence IDs. Missing direct sequence is explicit and never built from a different entry by name. |
| `ProteinFeature` | shared UniProtFeature plus source attribution, source coordinate convention and evidence codes/references. Source Domain, Region, Zinc finger and Repeat remain distinct labels. |
| `ProteinDomainMatch` | IPR accession, native name/type, source protein accession, source protein length/taxon, all native location groups and fragments. Member signatures stay distinct from the integrated IPR record. |
| `ProteinComplexHeader` | CPX accession, source name, organism, native predicted flag, actual total participants, exact query-participant identity. No description-based admission. |
| `ProteinComplexMember` | native participant identifier, participant type, taxon, name, native stoichiometry text and supplied cross-references. Preserve non-protein members as typed source facts; they do not receive protein get actions. |
| `ProteinAssociation` | explicit mapped query STRING ID, both endpoint STRING IDs, taxon, source preferred names, combined confidence and separate channel scores, source network mode and release when actually supplied. No name-based UniProt conversion or physical-binding assertion. |
| `ProteinStructureRef` | UniProt PDB cross-reference ID and its source properties (method, resolution, chains/ranges when present). A source link is not a claim that the structure resolves the requested residue. |

All new public request/page/card schemas are generated and versioned by the existing schema owner; field names above are proposals. Array cardinality and absence follow captured records, not convenient optional defaults. A page carries native total only for that collection. Complex search total is not admitted membership total until the complete query collection is scanned. STRING limit5 produced up to15 edges, so no edge total is inferred from that argument. Caches retain native replies and source versions; each local pool binds query, source settings, filtering, revision and continuation. Expired/mismatched pools return restart, never silently refetch.

Gene pivots use the landed gene entity's exact HGNC/UniProt cross-references. A gene with two proteins returns candidates, never a first-hit guess. Protein search passes text unchanged to UniProt; an explicit `--gene` uses its exact gene field under taxon9606. Default human/reviewed scope stays explicit. Gene names in returned protein records do not manufacture HGNC IDs. Variant-to-protein residue work needs the source's explicit protein/transcript identifier and sequence identity. Gene plus protein-change spelling alone does not join coordinate systems. `ProteinSite` remains a site, not an allele. No genome build, MANE status or clinical significance is inferred from a protein record.

## Proposed command tree and progressive disclosure

| Command | Default header or page | Drill-down and exact next steps |
| --- | --- | --- |
| `search protein TEXT` or `search protein --gene SYMBOL` | Five human reviewed ProteinHeaders, source-native order, count and retained continuation | One canonical get per returned candidate, then page, then at most three other actions. Text and --gene are mutually exclusive. |
| `get protein ACCESSION... [SECTION...]` | ProteinCard with source name, taxon, length and entry/sequence version; no unrequested bulk section | Five section/pivot actions maximum. Section values: features, domains, isoforms, complexes, structures, function, sequence; `all` requests each under the same per-section bound. |
| `protein features ACCESSION [--position N] [--type TYPE]` | Five source feature rows, coordinates, exact feature type and evidence | Filter exact checked positions and native documented type before paging. A position requires an exact sequence identity. No mutation effect inference. |
| `protein domains ACCESSION [--type TYPE]` | Five InterPro matches with native types and location fragments | Source-reported next page or a locally retained page; no flattening or selecting the first overlapping fragment. |
| `protein isoforms ACCESSION` | Five native alternative-product rows | Exact get or sequence actions only when the captured endpoint proves that identity can be retrieved. External alternatives retain their own base accession. |
| `protein complexes ACCESSION` | Five admitted complex headers | Member action per displayed complex, then page; admission requires exact native protein participant and human scope. |
| `protein complex-members ACCESSION --complex CPX-ID` | Five native members of one proven containing complex | Page through all participant types; protein pivots only for source-supplied supported exact protein IDs. The anchor accession must be an actual participant. This route is a PM choice below. |
| `protein associations ACCESSION` (deferred option) | Five query-incident STRING edges after explicit ID mapping | Preserve source order and all source confidence channels; no inferred partner accession, reranking, enrichment or neighbor-neighbor edges. |
| `protein structures ACCESSION` | Five UniProt structural cross-reference rows | Link to publisher PDB record; never fetch/install a structural analysis tool. |
| `protein sequence ISOFORM` | One exact native sequence with returned accession and audit metadata | Deep disclosure only; named isoform must match the source response. Default get omits it. |

All page commands share `--limit` (default5, proposed maximum25), `--pool` and `--offset`; emitted arguments use the existing continuation grammar. `get` uses existing `--section-limit`, source selection, freshness and policy behavior. Default search reviewed-only can be widened with `--include-unreviewed`; `--reviewed` is an explicit default spelling with a conflict check. The existing generic global options retain one owner. No `--all-species` is proposed under human-health entity scope. No source retry rewrites the input.

### Delight questions

1. The agent starts with a source accession or a gene question. An exact card answers identity; source-native search returns competing protein headers when the gene has more than one product.
2. The smallest useful header identifies the accession, name, taxon, reviewed state, sequence length and audit. It lets the agent choose a sequence without reading the full record.
3. A residue question goes straight to features with a checked position; the result names the feature, bounds and evidence. It does not load interactions or infer disease meaning.
4. The exact IDs needed next are UniProt accessions/isoforms, IPR IDs, CPX IDs, native member IDs and STRING endpoint IDs. Display symbols never substitute for them.
5. UniProt and InterPro label different source annotations. Output keeps source-specific rows and fragments; no merged consensus boundary is created.
6. Empty is a successful native collection with no admitted records. Failed identity is failed; a failed optional source makes the card partial. Policy-disabled sources are skipped, not empty.
7. Native network latency is measured above; cold product and warm product latency remain unmeasured because the implementation does not exist. A source reply is not a product benchmark.
8. 0.9's unbounded full feature/interaction contexts are not a useful default. The investigation adds explicit isoform identity and fragmented source locations that its compact domain rows lose. Static accounting below is separate from executable evidence.
9. The agent can stop after an exact source feature overlap or identity card. More data remains behind a named relation or section, never a command suggestion duplicated in several fields.
10. A research answer needs the publisher URL, retrieval date, source entry/release and sequence version when provided, source evidence and explicit scope. A protein feature alone never justifies a clinical recommendation.

## 0.9 command, option and issue accounting

Static audit uses the legacy checkout at `4050fdc7dbdf4d713d94cf02a39a147b863665be`, package version0.9.4. The inspected owners are `src/cli/protein/mod.rs`, `dispatch.rs`, `src/entities/protein.rs`, the four source clients, protein templates and the user guide. The executable was not run because a verified network containment recipe for this exact baseline was not established in this lane. Every 0.9 timing/token comparison is therefore **unmeasured**, not estimated from fixture size or current source JSON. No unrestricted baseline call is authorized by this proposal.

| 0.9 surface | Proposed disposition and reason |
| --- | --- |
| `search protein -q/--query TEXT` and positional query | Keep one positional spelling; raw text passes unchanged to UniProt. The duplicate query spelling receives retirement guidance. |
| human reviewed default | Keep; source-native query and returned taxon/review state checked. |
| `--all-species` | Drop under current human-health scope; no organism research examples justify it. |
| `--reviewed` | Keep explicit default spelling with conflict validation. |
| `--include-unreviewed` | Keep opt-in reviewed-scope widening; first implementation needs an exact positive capture before claiming coverage. The investigation's two gene searches used reviewed-only. |
| `--disease TEXT` | Defer until a source-native known-positive human question and exact query capture establish value; do not implement disease-name heuristics. |
| `--existence 1..5` | Defer: none of the harvested questions needs this search selector. Return source evidence where supplied. |
| `--limit` | Keep checked page size; proposed default5/max25 follows measured projection below, subject to PM review. |
| `--offset` | Keep over the retained source-bound pool, never offset-as-source-refetch. |
| `--next-page` | Rename to existing opaque `--pool` continuation and emitted offset. A source cursor stays inside retained state. |
| `get protein ACCESSION` or symbol | Keep exact accession and explicit source isoforms. A gene symbol resolves through gene candidates; ambiguity never chooses the first protein. |
| `domains` | Keep; preserve InterPro types and every location fragment rather than accession/name/type alone. |
| `interactions` | Defer STRING; its network limit is not edge pagination and distinct CDKN2A products collapse to one STRING ID. Do not relabel association confidence as physical interaction. |
| `complexes` | Keep source-attributed headers and exact-member admission. Add bounded member drill-down as a proposed route. |
| `structures` | Keep UniProt PDB cross-references with truthful provider credit, native properties and paging; no inferred residue mapping. |
| `all` | Keep explicit opt-in over kept sections with bounded independent pages. No unbounded arrays. |
| `protein structures ACCESSION --limit --offset` | Keep relation spelling with the existing pool convention; exact source identity and schema apply equally to JSON/Markdown/MCP. |
| Global `--json`, freshness/cache/policy/settings and MCP transport | Reuse landed global behavior and source registry. This slice adds no parallel environment/key parser or rendering path. |

A complete audit of57 closed public issues used title and body, with the terms protein, UniProt, InterPro, Complex Portal and STRING. Eight issues matched lexically; none is silently omitted below. Direct protein feature/identity regressions in private legacy tickets remain separate evidence and are not presented as public reports.

| Closed public issue | Accounting for this entity |
| --- | --- |
| [250: corporate CA](https://github.com/genomoncology/biomcp/issues/250) | Direct gene-to-protein failure. Reuse the landed shared trust/transport; assert source failure never becomes absent. No special protein CA client. |
| [113: data mapping](https://github.com/genomoncology/biomcp/issues/113) | Protein-change labels from wrong genes motivate checked identity admission; variant aggregation itself belongs to variant. No cross-gene label joins. |
| [78: gget integration](https://github.com/genomoncology/biomcp/issues/78) | Retrieve source entities only. External analysis installation/execution and enrichment stay outside BioMCP. |
| [249: emitted repeat ID rejected](https://github.com/genomoncology/biomcp/issues/249) | Variant fix already landed. Apply its contract to every emitted protein/isoform/IPR/CPX action; a source ID must round-trip or no action is offered. |
| [37: tools and ChatGPT compatibility](https://github.com/genomoncology/biomcp/issues/37) | Inherited shared CLI/MCP/search-fetch implementation, no protein-specific tool routing. |
| [21: top-level search/fetch](https://github.com/genomoncology/biomcp/issues/21) | Same inherited wrapper contract; canonical protein publisher URLs are needed for citable results. |
| [20: trial semantic fields](https://github.com/genomoncology/biomcp/issues/20) | Incidental protein/marker text. No protein feature change; trial selectors remain owned by trial. |
| [289: ORCID key branch](https://github.com/genomoncology/biomcp/issues/289) | Incidental matching text, not protein access. No protein-specific change. |

Legacy private regressions provide additional design controls: 2114 prevents a residue mismatching its sequence from producing structural placement; 2036/2042 preserve canonical versus isoform identity even when MANE facts are missing; 2027 preserves InterPro source location/order scope; 2287 fixes protein structures provider-credit panic. Source inspection establishes the lessons; this investigation claims no new executable reproduction.

## Proposed outside-in cases by command and option

Every command uses the same CLI/JSON/Markdown/MCP request model. Tests reuse exact captures where requests match; invented fault bodies remain labeled synthetic. They never inherit credentials, config, a warm cache or home directory. The current native evidence does not contain raw malformed/timeout/429/500 replies or an unreviewed positive search. Those controls are authored or captured in the implementation ticket before admission claims.

| Surface | Required cases and boundary |
| --- | --- |
| Search positional text / `--gene` | P07 BRAF single and P20 CDKN2A two candidates; literal text query needs an exact native capture before implementation proof. Mutually exclusive input forms and absent input are usage errors. |
| `--reviewed` / `--include-unreviewed` | Reviewed human positives, widened-scope native positive, flag conflict, taxon/identity contamination refused. No invented unreviewed positive. |
| `--limit`, `--pool`, `--offset` | Default5, one-row page, full captured collection, first/middle/terminal pages, duplicate/no-gap order, expired/mismatched pool, zero/max+1 limits and invalid offset. Query/filter/source changes invalidate a pool. |
| Get exact accession, multi-get, symbol/isoform | P03/P13/P19/P20/P27; returned accession/taxon mismatch, absent versus failed, cross-entry External isoform, stable per-item ordering, exact echoed query. |
| Get section list / `--section-limit` / `all` | Each kept section individually, repeated section handling under shared grammar, bounded union, unsupported section usage error, more/next recovery, source disable before cache/network. |
| Features `--position` / `--type` | P01/P02/P05/P10/P11/P14/P17/P25/P26; exact endpoints, out-of-range residue, position0 usage error, wrong reference sequence, uncertainty, inclusive-to-interbase conversion, type/position conjunction, no clinical inference. |
| Domains `--type` | P06/P16/P21; family versus domain, repeated/discontinuous fragments, complete location groups, native count vs filtered count and source pagination. |
| Isoforms | P04/P13/P15/P20/P27; Displayed/Described/External, source suffix preserved, no synthetic -1, external product accession preserved. |
| Complexes | P08/P09/P12/P18/P28; description-only exclusion, exact participant match, non-protein IDs, predicted flag, complete admitted count only after full capture, no93-member default. |
| Members `--complex` | P12; CPX-715 exact membership and both IDs, wrong anchor/complex rejected, source detail identity mismatch, source stoichiometry text retained, page5 over large predicted complex. |
| Structures | Source PDB IDs/properties from retained UniProt records, no provider-credit panic, no inferred residue/chain placement, pages and no requested section failure disguised as empty. |
| Sequence | Exact P38398-2 and Q9UIF7-2 FASTA identities, sequence-only data, no inherited canonical version, newline/FASTA parsing, mismatched ID, invalid residues and source failure. |
| Deferred STRING option | P22 and CDKN2A collapsed mapping remain recorded reasons, not a shipped case claim. A future ticket needs exact mapping/network request equality, query-endpoint checks, source confidence fields and explicit loss-of-specificity semantics. |
| Global source/freshness/policy | Open/default, source disabled before I/O, transport error/stale/fresh actions, deterministic offline replay, proper source credits, source URL and terms in both views. |

Source policy and terms come from the existing source owner. Shared types define validation once. No separate Markdown command registry or duplicate source/provider-credit table is added by implementation. Retained test pools are private per case, and concurrency tests wait for events rather than fixed sleeps.

## Ian's questions for PM review

1. Approve UniProt identity/features/isoforms/sequence/structures, InterPro location matches and Complex Portal exact-member access; defer STRING because its source mapping merges distinct CDKN2A products. Keeping STRING would add one mapping request and a network request per protein while still requiring an explicit source-level identity warning. Recommendation: defer it until a research question requires that source identity.
2. Approve `protein complex-members ACCESSION --complex CPX-ID` with bounded participants. Alternative: headers and publisher links only. The bounded relation costs one source detail request when absent from the retained search reply and avoids a93-member default; it answers P12 directly. Recommendation: keep the relation under exact anchor membership. No new complex search entity is proposed.
3. Approve the proposed search/get and section ceilings after the native projection measurements below. The alternative is five generic unbounded source records, which already cost up to875KB each. These are implementation acceptance ceilings, not claims about a shipped command.
4. Accept source-inspected0.9 accounting with executable timings and token comparison explicitly unmeasured, or assign a separate contained baseline. A baseline must prove denied external networking before running and must use exact current request recordings. Recommendation: do not delay source/type design on an unrestricted legacy invocation.

## Verified evidence and identity follow-up

The evidence store holds these immutable archives under `data/biomcp-redux/0190-protein-investigation/`. Each archive passed a full archive SHA-256 check, exact member inventory and each content SHA-256 check independently on the coordination host and on Yellow. These are bounded investigation results, not product acceptance.

| Archive | Bound revision and job | SHA-256 and contents |
| --- | --- | --- |
| `j14202-native-sources.tar.gz` | `ecbbfb02876ef49eb317730c796c9c50396b931f`, j14202 | `25d2cd2bb2a1236e6aee9fe84ffc7879723df92bd031396da3e41b0c6ff989c8`;75 members,36 raw source bodies/headers, receipts and manifest. |
| `j14211-j14221-identity-followup.tar.gz` | `692d389bee1bd796c2cf80c5c5213653cd22e652`, j14211/j14221 | `2558f595fd554a648f6477fb1fcd2e6e3645b425f64d8df0ffd6375575dbdafe`;33 members,15 request bodies/headers and retained receipts. |
| `j14236-native-row-projection.tar.gz` | `b8c2bb0bb4c29dc9d5ec5169d4f10fc8fb8afda8`, j14236 | `411ba037bf120a998ac461a206a4ad2877f8b54bb387d160a325acb995ee1024`;1,058 members, offline recipe,1,055 projected responses, measurements and manifest. |

The follow-up sent exactly15 new requests: BRAF search returned P15056; CDKN2A search returned P42771 and Q8N726. The deliberately invalid accession query returned HTTP400, not an absent result. That body is preserved as an input-validation observation. P38398-2 FASTA returned exact isoform identity and63 residues; Q9UIF7-2 returned536 residues. A FASTA body is expected non-JSON; the generic capture helper's JSON parse note does not classify that native sequence as a failed source. CPX-715 detail returned exact complex identity and participants. All nine STRING mapping replies returned HTTP200 and matched the queryItem, while two distinct CDKN2A protein products collapsed to the same source STRING ID. That is a genuine source granularity limitation.

The first follow-up attempt stopped after delivering only the BRAF request because the throwaway helper lacked its receipts-list initialization. The continuation reused the untouched body/headers and sent only the14 undelivered requests. BRAF search elapsed time is unmeasured because it was not serialized before the setup failure; neither a fabricated timing nor a repeated request fills the gap. Total native traffic was51 requests, of which50 returned200 and one returned400. No retries, redirects, alternate URLs or queries occurred.

Clinical review accepted the corrected P01–P28 bank and meaning contracts. P23/P24 are controls rather than published-research examples. The reviewer confirmed the historical MUTYH citation against the [publisher abstract](https://academic.oup.com/carcin/article-abstract/26/11/2010/2476061), corrected EGFR790 to Kobayashi2005 and restricted PMID14578343 to BRCT research context. Source facts do not imply classification, efficacy, binding or coordinate translation.

## Measured disclosure budget proposal

The offline projection uses all captured native rows, with page size5 and a representative envelope including source terms. It preserves raw row fields rather than pretending to be the generated future schema. It measures compact UTF-8 JSON and defines token units as `ceil(bytes/4)`; the stdout column adds one newline. It executes no product and performs no provider requests. Thus these are measured source-row projections, not measurements of shipped CLI/MCP output or a semantic tokenizer. Every implementation ticket must measure its actual complete envelopes and next actions through the existing runner, then set the accepted ceilings against that output.

The projected responses omit typed next actions. The proposed ceiling adds a1,000-byte explicit reserve for those actions and request fields, then15% headroom. This reserve is a design allowance, not a measured fact. Search has only one- and two-header native results; its proposed five-header ceiling is a conservative6,500 bytes and cannot be described as a measured full five-hit page. A positive five-hit search capture remains an implementation admission requirement.

| Layer / command | Captured projected pages | Largest stdout bytes / token units | Proposed full stdout ceiling / token units |
| --- | --- | --- | --- |
| complex-members | 157 | 2,237 /559 | 3,723 /931 |
| complexes | 15 | 2,110 /528 | 3,577 /894 |
| domains | 19 | 4,790 /1,198 | 6,659 /1,665 |
| features | 623 | 7,878 /1,970 | 10,210 /2,553 |
| function | 9 | 9,590 /2,398 | 12,179 /3,045 |
| get | 9 | 897 /224 | 2,182 /546 |
| isoforms | 14 | 1,261 /315 | 2,601 /650 |
| search | 2 | 1,191 /298 | 6,500 /1,625 |
| sequence | 9 | 2,242 /561 | 3,729 /932 |
| structures | 198 | 1,120 /280 | 2,438 /610 |

The sequence maximum is the complete1863-residue BRCA1 sequence with metadata; no sequence truncation is proposed. Function text and evidence-rich feature rows remain deeper options because their measured pages are costly. The default card is below900 bytes in this projection, while raw UniProt replies span55KB–875KB. The section pages are independent: `all` adds each requested section's bounded first page and does not claim to fit the default card ceiling. Its composite ceiling must be derived from actual complete implementation output before admission. No arbitrary shared help or discovery ceiling is raised here.

0.9 default/section output tokens, comparable output after registry terms removal, product cold/warm latency and complete future `all` envelopes remain unmeasured. This proposal never compares source JSON bytes with0.9 rendered output. A PM-approved implementation must verify ceilings on the actual public schema and all emitted actions.

## Bounded admission capture recipe

The admission follow-up uses the landed source-capture helper, with no product code changes. It permits at most twelve physical requests, one second between requests, a thirty second deadline and a four MiB body bound. Each row runs once. There are no retries, redirects, query changes or extra pages. A 401,403,429 or5xx stops that source. No keys, cookies, STRING, bundles or analysis tools are used.

The literal EGFR text comes from P01, and CDKN2A comes from P20. The search fields retain accession, entry identity, protein/gene names, taxon, reviewed status and length. The original j14202 ledger identifies P00533 as the largest InterPro collection, with fifteen entries. P00000 is the well-formed absent-accession control; the response must establish absence rather than this recipe assuming it. The exact first requests are:

| Witness | Exact URL |
| --- | --- |
| reviewed | `https://rest.uniprot.org/uniprotkb/search?query=%28EGFR%29+AND+%28organism_id%3A9606%29+AND+%28reviewed%3Atrue%29&format=json&size=5&fields=accession%2Cid%2Cprotein_name%2Cgene_names%2Corganism_id%2Creviewed%2Clength` |
| unreviewed | `https://rest.uniprot.org/uniprotkb/search?query=%28CDKN2A%29+AND+%28organism_id%3A9606%29&format=json&size=5&fields=accession%2Cid%2Cprotein_name%2Cgene_names%2Corganism_id%2Creviewed%2Clength` |
| interpro-largest | `https://www.ebi.ac.uk/interpro/api/entry/interpro/protein/uniprot/P00533/?page_size=5` |
| interpro-domain | `https://www.ebi.ac.uk/interpro/api/entry/interpro/protein/uniprot/P04637/?page_size=5&type=domain` |
| complex | `https://www.ebi.ac.uk/intact/complex-ws/search/P15056?first=0&number=5&filters=species_f%3A%28%22Homo+sapiens%22%29` |
| isoform | `https://rest.uniprot.org/uniprotkb/P38398-2.json` |
| uniprot-absent | `https://rest.uniprot.org/uniprotkb/P00000.json` |
| interpro-absent | `https://www.ebi.ac.uk/interpro/api/entry/interpro/protein/uniprot/P00000/?page_size=5` |

UniProt gets exactly one second page through the first reply's `Link` with `rel="next"`. InterPro gets exactly one second page through its returned `next` URL. Complex Portal gets exactly one second page with `first=5`, preserving its query, human filter and `number=5`, only when the first total exceeds five. [InterPro documents page size and type selection](https://interpro-documentation.readthedocs.io/en/latest/api.html). [Complex Portal documents first/number offsets](https://www.ebi.ac.uk/intact/complex-ws/search/). No continuation is inferred for a failed or empty first reply.

Each missing witness retains the ticket's fallback: search ceilings return to the PM without a five-hit page; widened search remains unsupported without an unreviewed positive; an unproved source page returns only its first native window with an incomplete label when more exists; an unproved InterPro type selector filters a complete retained collection locally; an unproved isoform entry get accepts only base accessions and names isoforms/sequence guidance; absent-versus-failed cases remain explicitly synthetic without native absence replies. These captures establish source behavior, not biological answers.

## Admission capture results

Job j14362 ran the declared recipe from landed revision `6e20aa069bbb981f9c9c6f0da3fd4ba31e556ceb`. It sent exactly eleven physical requests, without keys, retries, redirects, alternate queries or extra pages. Recorded start times are at least one second apart. Ten requests returned HTTP200. The last InterPro request retained a status0, `truncated` outcome, zero body bytes and zero header bytes; no HTTP absence status was observed. Job success means the bounded recipe and archive completed, not that every witness succeeded.

Every row below cites the same immutable evidence archive: Yellow `data/biomcp-redux/0229-protein-admission/j14362-native-witnesses.tar.gz`, SHA-256 `2808c280a38e69e533562269c2376590fff28e4661fbb4a61dc5705bf46a3047`. It contains the predeclared recipe, runner, eleven one-request plans and logs, ledger, eleven helper archives and an outer manifest. Both the coordination host and Yellow independently verified the exact38 outer members, all37 outer content hashes, all66 inner members and all55 inner content hashes. Outer and inner packaging suppress AppleDouble files. No raw source reply enters Git.

| Witness and ledger rows | Native result | Capability unlocked or deferred |
| --- | --- | --- |
| Reviewed human text search, `reviewed` and `reviewed-second` | Literal EGFR search reports399 results. Both pages contain exactly five reviewed human records, with disjoint accession sets. The second request equals the first reply's exact `rel="next"` URL. Bodies are4,650 and3,732 bytes. | Proves a full five-hit text-search window and one native continuation, so the search ticket can measure its actual five-header envelope against the accepted starting ceiling. This does not prove the remaining389 records or a complete collection. |
| Unrestricted human search, `unreviewed` | CDKN2A search omits the reviewed restriction and reports155 results. All five returned records are human and reviewed. The returned continuation was not requested. | No unreviewed positive was captured. `--include-unreviewed` stays deferred with a not-yet-supported message. No query change or additional page fills this gap. |
| Largest InterPro collection, `interpro-largest` and `interpro-largest-second` | P00533 reports15 entries. Each page returns five distinct IPR IDs scoped to native protein `p00533`; the windows do not overlap. The second request equals the first body's exact `next` URL. The second body still has a next pointer. | Proves one source-native page step. These ten rows remain an incomplete collection of fifteen; no terminal page or complete collection is claimed. |
| InterPro native type selection, `interpro-domain` | Documented `type=domain` for P04637 returns HTTP200, count4, four native domain entries and no next pointer. All four rows name native protein `p04637`. | Proves source-side domain selection for this exact request. Keep source entry types and fragments; no family or site becomes a domain. |
| Complex Portal human search, `complex` and `complex-second` | P15056 with the human filter and `number=5` reports18 results. The documented `first=0` and `first=5` requests each return five disjoint CPX IDs. Bodies are13,272 and81,334 bytes. | Proves the documented offset page step. Search candidates still need exact participant admission; text matching never proves membership. Ten captured candidates do not establish all18 candidates or an admitted total. |
| Exact isoform entry, `isoform` | Exact P38398-2 JSON returns HTTP200, primary accession P38398-2, human taxon9606 and sequence length63. | Proves exact isoform entry retrieval for this accession. It does not inherit the base entry's sequence or residue coordinates. |
| UniProt absent control, `uniprot-absent` | P00000 returns HTTP200 with exact accession P00000 and a nonhuman taxon. The candidate was well formed but was not absent. No interpretation of that record is made. | Native absence remains unproved. Human identity admission must reject this nonhuman record. Absent-versus-failed acceptance cases remain labeled synthetic; no replacement accession was requested. |
| InterPro absent control, `interpro-absent` | The P00000 request records status0, outcome `truncated`, empty headers and empty body. The helper exits2 and retains its receipt unchanged. There is no admitted HTTP404 or native empty collection. | Native absence remains unproved. Treat this capture as a failed bounded request, never as absent. Absent-versus-failed acceptance cases remain labeled synthetic. No retry or repaired URL was attempted. |

These source windows remove the paging, five-hit search, domain-selector and exact-isoform evidence gaps only. They do not measure product output or expand the approved source set. The unreviewed and native-absence capabilities keep their stated fallbacks. All retained timing values describe single shared-machine source calls rather than a throughput or quiet-machine benchmark.
