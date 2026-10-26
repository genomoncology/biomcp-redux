# Phenotype slice investigation and proposed design

This investigation starts from `85e6ba864c3f13ec50432a58670ec2af13e12857`. It proposes an entity interface for human phenotype terms and their source associations. It changes no product code. The disease slice already keeps Monarch disease-to-phenotype annotations. This slice must reuse those rows and their component terms rather than create a competing disease annotation model.

## Harvested questions and accepted source answers

The questions below adapt explicit source features and source workflows into user questions. They are not patient cases or quotations of questions submitted by those authors. A term lookup answers terminology. An association lookup answers what its publisher asserts. Neither establishes a diagnosis, causation, penetrance or an individual risk. Clinical acceptance of the question bank is separate from native transport measurements.

Sources: [NF1 GeneReviews](https://www.ncbi.nlm.nih.gov/books/NBK1109/), [PTEN GeneReviews](https://www.ncbi.nlm.nih.gov/books/NBK1488/), [VHL GeneReviews](https://www.ncbi.nlm.nih.gov/books/NBK1463/), [Birt-Hogg-Dube GeneReviews](https://www.ncbi.nlm.nih.gov/books/NBK1522/), [HPO 2024, PMID 37953324](https://pubmed.ncbi.nlm.nih.gov/37953324/), and [Monarch 2024, PMID 38000386](https://pubmed.ncbi.nlm.nih.gov/38000386/).

| Case | Source-derived question | Source answer to retain | Source |
| --- | --- | --- | --- |
| P01 | Which HPO terms describe cafe-au-lait macules? | Publisher term candidates, their exact IDs and definitions; NF1 source names the feature. No syndrome inference. | NF1 |
| P02 | What does neurofibroma mean in HPO? | Exact returned term and definition, with synonyms as a separate disclosure layer. | NF1 |
| P03 | Which term describes plexiform neurofibroma? | Exact term candidates; retain a more specific term separately from its ancestors. | NF1 |
| P04 | Which term describes axillary freckling? | Source term candidates and exact IDs. A missing literal hit is not an absent clinical feature. | NF1 |
| P05 | What does macrocephaly mean? | Source definition, not a locally chosen head-circumference threshold. | PTEN |
| P06 | Which HPO term describes gastrointestinal polyps? | Source candidates; no automatic equivalence between generic and hamartomatous polyps. | PTEN |
| P07 | Which HPO term describes lipoma? | Source term identity and definition. | PTEN |
| P08 | What term describes developmental delay? | Source candidates and definition; no automatic substitution of intellectual disability. | PTEN; HPO |
| P09 | What term describes retinal hemangioblastoma? | Source term identity and definition, retaining exact spelling. | VHL |
| P10 | Which term describes renal cysts? | Source candidates and exact IDs; no assumed subtype. | VHL |
| P11 | What does pheochromocytoma mean? | Source definition and term identifier. | VHL |
| P12 | Which term describes pancreatic cysts? | Exact source term and definition. | VHL |
| P13 | What term describes fibrofolliculoma? | Source identity and definition. | Birt-Hogg-Dube |
| P14 | Which term describes pulmonary cysts? | Source candidates and definitions. | Birt-Hogg-Dube |
| P15 | What does spontaneous pneumothorax mean? | Source definition; no inference that any observed pneumothorax is spontaneous. | Birt-Hogg-Dube |
| P16 | What does HP:0001250 mean? | Exact HPO concept identity, definition and native synonyms where supplied. | HPO; legacy documented example |
| P17 | Which native disease candidates resemble HP:0001250 and HP:0001263? | Native similarity ordering and score semantics only. A score is not evidence that every requested feature is annotated to the candidate. | Monarch; legacy documented example |
| P18 | What phenotype annotations does von Hippel-Lindau syndrome have? | Existing disease section, actual edge endpoints, provenance, evidence and qualifiers. No newly synthesized syndrome summary. | VHL; kept disease design D14 |
| P19 | What phenotype annotations does Li-Fraumeni syndrome have? | Existing disease section, association rows and native count. A source association does not establish a patient diagnosis. | Kept disease design D13; Monarch |
| P20 | What phenotype annotations does multiple endocrine neoplasia type 2 have? | Existing disease section preserves subtype edges and closure proof. A parent request must not relabel a subtype edge. | Kept disease design D18; Monarch |
| P21 | Which phenotype annotations are connected to the human VHL gene? | Source gene-to-phenotype associations, actual subject/object IDs, predicates and provenance, if the public tier returns them. | VHL; Monarch |
| P22 | Which phenotype annotations are connected to the human PTEN gene? | Same source association contract; no reverse inference from a disease name. | PTEN; Monarch |
| P23 | Which phenotype annotations are connected to the human NF1 gene? | Same source association contract; preserve many rows for one term when provenance differs. | NF1; Monarch |
| P24 | Does an unknown term or an unavailable source mean no matching syndrome exists? | No. An unknown exact term is absent; a successful empty search is empty; a source failure is failed or partial. These states make no clinical absence claim. | HPO/Monarch workflow; recorded failure case required |

## Investigation boundaries

Native captures run through ticket jobs. The first bounded metadata pass names six requests, a single attempt, no redirects or fallback, a per-response byte bound and a stop on rate limiting or server errors. The raw capture remains outside Git. No 0.9 binary has been executed by this investigation. Its output budgets remain unmeasured unless verified containment can supply the exact native request identities.

The disease slice retired scored symptom-to-diagnosis ranking. A new source-native similarity operation therefore needs an explicit PM ruling after source semantics and the harvested research question are reviewed. Term discovery must not silently choose the first HPO candidate and feed it into a ranker.

## What would delight the agent

1. **What does it ask first?** "What is this feature called?" A literal term search returns HPO candidates, not diseases. An agent selects an exact HP identifier before following any association.
2. **What is the smallest useful header?** Exact ID, publisher label, source, native match metadata and native collection count. The complete definition belongs on the default term card.
3. **Which details come next?** Source synonyms with their relationship classes, cross-references, hierarchy and source disease/gene association pages. The explicit disease section already answers a known syndrome's annotations.
4. **Which identities must survive?** Exact HP IDs for terms, MONDO IDs for disease concepts and returned HGNC/NCBIGene IDs for human genes. Original endpoints and provenance remain on every association. No symbol or disease-name join is permitted.
5. **How do source disagreements appear?** HPO's flat synonym list and Monarch's classified synonym groups remain separate source answers. For HP:0001250 the HPO response lists Epilepsy as a synonym while Monarch marks it as related, not exact. No local clinical equivalence follows.
6. **What lets the agent stop?** An exact term definition with source attribution answers a terminology question alone. A successful empty search answers only that literal source query. An association page answers only the captured publisher scope.
7. **What makes the answer trustworthy?** Publisher URL, retrieval time, ontology release when actually supplied, original assertion provenance, native predicate, evidence, negation and qualifier presence. A retrieval date is never invented as an ontology version.
8. **What does similarity add?** It answers P17's explicitly requested research similarity. It must be opt-in and source-native if approved. It cannot replace ontology search, choose a diagnosis or claim that every input term appears on every returned disease.

## Proposed types, schema and commands

The proposal separates term discovery, exact term identity and associations. The consumer layer owns checked requests, cards, pages and typed actions. The neutral layer reuses `Curie`, source concepts, attribution and the disease slice's association record. A new identifier wrapper accepts the published HP identifier grammar; a MONDO or gene identifier is a usage error for an exact phenotype card. No patient, clinical score, symptom parser or diagnosis type is introduced.

`PhenotypeHeader` holds HP ID and native label plus optional source match score, matched field and match type. Scores retain the publisher's meaning. `PhenotypeCard` holds requested ID, actual returned ID, label and complete native definition; optional comment and source concept metadata remain available as described below. `PhenotypeSynonym` retains a publisher relationship kind when provided; an unclassified HPO synonym must not acquire Monarch's classification. `PhenotypeAssociationPage` reuses native row identity, actual subject/object and original endpoints, predicate, evidence/publications, provenance, negation, frequency/onset/sex and other source qualifiers, and source totals. A gene association remains an association, not a causal clinical statement. Omitted, null and empty collections follow the established source-presence rules; no nonempty text invariant may silently discard a native empty definition or comment.

| Proposed operation | Default answer | Additional disclosure and cases |
| --- | --- | --- |
| `search phenotype TEXT --source monarch --limit N --offset N` | Literal source search restricted at the provider to `namespace=HP` and phenotype category; default five headers in native order. | P01–P16, P24. Validate the native limit, offset, total and every returned namespace. Retain native match metadata. An unrelated namespace is a source problem, never a local reclassification. |
| `get phenotype HP:ID --source hpo` | HPO term label and complete definition, native comment, attribution and published identity. | P01–P16. Direct HPO supplies the source comment and publication references absent from the earlier disease annotations. No follow-up metadata request is implicit. |
| `get phenotype HP:ID --source monarch` | Independent Monarch term card, complete description and native deprecation fact when supplied. | P16. No merged card or automatic fallback. Keep source hierarchy and association counts distinct from HPO descendant counts. |
| `get phenotype HP:ID synonyms --source hpo\|monarch` | Bounded native synonym rows. | P02, P05, P16; preserve Monarch exact/broad/narrow/related distinctions and HPO's unclassified list. |
| `get phenotype HP:ID xrefs --source hpo\|monarch` | Bounded native cross-reference rows and actual predicates where supplied. | P05, P16. Cross-reference presence alone proves no equivalence. |
| `get phenotype HP:ID hierarchy --source monarch` | Native direct parents/children with exact IDs and labels. | P03, P16. Descendant totals are explicitly publisher counts, not patient prevalence. No local graph traversal. |
| `get phenotype HP:ID diseases --source monarch` | Source disease-to-phenotype rows with requested HP beside actual endpoints, default one row. | P17–P20; `--section-limit`, `--section-offset`; preserve source order and native count. Exact/direct mode requires positive native proof. |
| `get phenotype HP:ID genes --source monarch` | Source human-gene-to-phenotype rows, default one row. | P21–P23; provider-side human taxon restriction must be positively proved before inclusion. No model-organism or ortholog expansion. |
| Existing `get disease MONDO:ID phenotypes --source monarch` | The already implemented disease annotation page. | P18–P20 reuse its command, schema and qualifiers. This slice adds no second route to synthesize syndrome annotations. |
| Proposed optional `search disease --phenotype HP:ID` or separate explicit similarity mode | Source-native disease candidates only if PM approves their exact semantics. | P17. Naming and scope remain a product decision; the ordinary term-search positional argument must never silently switch to ranking. |

Requests use the established closed source and section choices. Schemas are generated from types and versioned. Source/entity applicability errors return useful usage recovery before I/O. A mixed disabled component is skipped before cache or network access, using the existing Monarch admission rules. No disabled upstream payload is fetched and filtered afterward.

Typed next actions follow architecture 6.10. Each selected term header gets one exact get action, bounded by the checked page size, followed by native paging and at most three other actions. Each distinct linked disease/gene in an association page gets one exact pivot in native order. No action promotes an HGNC symbol to identity, relabels an NCBIGene ID or assumes every synonym is equivalent. An exact existing gene command must accept the returned identifier before its action is advertised. A failed source supplies a fresh action; a disabled source supplies no workaround provider.

## Legacy accounting

The inspected 0.9 source revision is `4050fdc7dbdf4d713d94cf02a39a147b863665be`. Its `src/cli/phenotype/mod.rs`, `dispatch.rs`, `src/entities/disease/search.rs`, `src/sources/hpo.rs`, `src/sources/monarch.rs` and user guide establish the following static inventory. Static inspection proves behavior ownership and request construction, not live output quality or token cost.

| 0.9 command, option or section | Proposed disposition | Reason and acceptance case |
| --- | --- | --- |
| `search phenotype TERMS` with space/comma HP ID sets | Separate term identity from source-native similarity; similarity needs ruling. | P16/P17. Do not silently turn one entity query into a patient diagnosis. |
| One symptom phrase or comma-separated phrases | Literal term discovery returns all native candidates, with explicit selection. | P01–P16; ambiguous/unknown phrase must never silently become the first candidate. No hand-built vocabulary or phrase rewriting. |
| Automatic HPO resolution before similarity | Replace with explicit source candidate selection. | Preserve original literal query and exact selected HP identifiers. No hidden phrase loss or expansion. |
| `-l`, `--limit` | Keep bounded pages; changed source operation has its own proved limits. | Positive first/middle/terminal page and invalid zero/out-of-range cases. Old accepted range 1–50 applies to similarity, not an invented term-search ceiling. |
| `--offset` | Keep for native term/association paging. Similarity offset remains limited to a captured provider window if kept. | P01/P16/P17; no fabricated continuation beyond the provider window. |
| Fixed first 50 similarity rows, local disease deduplication and slicing | Keep explicit window metadata only in an approved similarity operation. | No total beyond what the publisher supplies; raw-row count differs from distinct disease count. |
| `resolved_query` phrase, HPO ID and label | Keep input identity on the explicit operation. | P01/P16/P17. Distinguish a selected candidate from an inferred clinical synonym. |
| Native semantic score and ordering | Retain only in explicit approved similarity. | No probability, causal claim, local rerank or score blend. |
| Per-term direct-support states supported/not_supported/indeterminate/unavailable | Do not attach these automatically to every similarity row. Keep explicit association inspection; seek ruling before any retained enrichment. | Native exact positive assertion is different from similarity; an absent/incomplete lookup proves no clinical absence. |
| Automatic first fully supported disease next step | Replace with typed candidate pivots and clear source semantics. | No recommendation disguised as selection. PM decides any retained convenience. |
| `list phenotype` | Rename to generated `help phenotype`, with exact usage recovery. | Source choices, disclosure and examples come from the shared command schema. |
| `discover` phenotype suggestion | Use explicit entity term discovery; no symptom intent detector. | Shared retirement rules govern discovery. |
| `get phenotype` and sections | New surface; 0.9 has none. | Exact term definitions, synonyms, xrefs and explicit source associations answer this bank. |
| CLI JSON/Markdown and MCP invocation | Preserve through the shared checked request and envelope. | Assert matching source facts and typed next arguments. Shared global flags keep their existing contracts. |

The complete closed public-issue inventory returned 57 issues. No issue title or body contained phenotype, Monarch or HPO. The investigation must preserve the inventory and filter as external evidence; this means zero entity-specific closed issues were found, not that general transport, cache or MCP issues cease to apply.

## Clinical review of question boundaries

The clinical reviewer checked GeneReviews source sections: NF1 Suggestive Findings (revision 2025-04-03); PTEN Summary, Clinical Description and Suggestive Findings (update 2026-07-28); VHL Summary (revision 2025-05-01); and Birt-Hogg-Dube Summary/Clinical Description (update 2024-12-05). These support the features used in P01–P15. They establish retrieval questions, not literal HPO equivalence or diagnostic sufficiency. NF1 and Legius syndrome can share pigmentary findings; a shared term does not select a diagnosis. Exact native HP IDs and source association interpretation require the capture review below.
