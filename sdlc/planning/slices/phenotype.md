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
