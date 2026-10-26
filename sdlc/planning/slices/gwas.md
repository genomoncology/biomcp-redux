# GWAS slice

This investigation proposes the GWAS slice through steps 1 to 4 of [the slice method](../slices.md). It starts from `85e6ba864c3f13ec50432a58670ec2af13e12857`. It changes no product code. Design acceptance does not mark a capability ported.

## Scope and evidence

The entity is a source association or study. Its output answers which source reported an association, for which allele and trait, in which study context, with which reported statistics. It does not calculate personal risk, infer a causal variant, diagnose disease, or recommend treatment. Human health research questions are in scope. Running an association analysis, installing another tool, computing a polygenic score, fine-mapping and local ontology or gene-name inference are outside this design.

The current variant slice already owns `get variant ID gwas` and its exact-rsID adapter. This slice extends that adapter rather than creating a second GWAS transport, terms declaration, association decoder or effect representation. The variant section remains owned by the variant slice. Study and association identities add context without silently resolving a variant allele from an rsID.

## Public questions

The bank paraphrases retrieval questions anchored to primary papers. It does not claim the authors asked these questions verbatim. The bank preserves the publication's finding separately from later catalog curation. A source-local empty result and an unavailable source are investigator controls. They establish retrieval behavior only.

## Source contracts

The ROADMAP names GWAS Catalog as the GWAS source. The public v2 API is the proposed initial source. The [official API guide](https://www.ebi.ac.uk/gwas/docs/programmatic-access/rest-api/) and [migration guide](https://www.ebi.ac.uk/gwas/docs/news/rest-api-v2-migration-guide/) distinguish curated association data from complete genome-wide summary statistics. V1 is retired. No request falls back to v1.

The source exposes studies, associations, traits, genes, publications and study ancestry metadata. Source filters, their combinations and sort order need captured known-positive checks before implementation enables them. Public documentation does not by itself prove that a particular response carries a field. The implementation must compare returned study, association, rsID and ontology identities with its checked request.

### Identity and joins

| Identifier | Meaning and allowed join | Forbidden inference |
| --- | --- | --- |
| `GCST…` | Exact source study accession; links study metadata and its association collection. | A publication is not one study; never collapse distinct accessions by PMID. |
| Association ID | Positive source record identifier qualified as a GWAS Catalog association. | Equal rsID, trait or effect values do not make two association records equal. |
| rsID | Exact source rsID, passed to the existing variant lookup. | An rsID need not name one genomic allele. Never select the first allele or invent a build. |
| EFO or other source ontology ID | Exact mapped trait identifier plus the source label and reported trait text. | Do not equate a trait label with a disease ID or match labels locally. |
| Source gene identifier | Exact returned stable gene identity, with mapping origin retained. | A reported or nearest mapped gene is not established as the causal gene. A symbol alone does not authorize a cross-source identity join. |
| PMID | Exact source publication identity, passed to the article entity. | Do not copy a paper's abstract into every association or treat one PMID as one analysis population. |

Coordinates, if later exposed, carry the build actually stated by the source. This first design needs rsID joins and does not add interval search, remapping or a local build assumption.

## Proposed commands and disclosure

The proposed grammar follows the existing `search`, `get` and entity relation forms. Search returns association headers. A selected association returns its source record. A selected study returns study metadata. Source order and source sort remain explicit.

| Form | Default header or card | Further disclosure |
| --- | --- | --- |
| `search gwas --variant rsID` | One bounded association page with association ID, study accession, reported and mapped traits, effect allele, p value and reported effect. | Each exact association get, continuation, and bounded source-qualified pivots. |
| `search gwas --trait TEXT` | Literal publisher trait search, echoed unchanged; no local expansion. | Caller can choose exact ontology IDs from source results. |
| `search gwas --trait-id ID` | Associations with the exact mapped ontology ID. | Explicit child-trait inclusion changes the source request and is echoed. |
| `search gwas --gene GENE` | Associations selected by the publisher's documented mapped-gene definition. | Explicit extended gene-set choice preserves legacy v1 meaning where needed. |
| `search gwas --study GCST…` | Associations belonging to the exact study. | Source-native association continuation. |
| `search gwas --pubmed PMID` | Associations from the exact publication, retaining all study IDs. | Study gets and article pivot. |
| `get gwas association:ID` | Exact association identity, all source effect alleles and traits, reported effect and interval, source study and publication IDs. | Study card, exact-rsID variant actions and publication. |
| `get gwas GCST…` | Exact study accession, publication, reported and mapped traits, sample descriptions, summary-statistics availability and source date where supplied. | `associations` and separately paged `ancestries` when proved by capture. |
| `get gwas GCST… associations` | One bounded native association page belonging to this study. | Continuation with the original checked study and source query. |
| `get gwas GCST… ancestries` | One bounded source ancestry/sample metadata page. | Source pagination; never a local patient-level denominator. |

All search filters are explicit. Empty search input is a usage error, rather than a scan of the whole catalog. The command-specific page limit and offset use the existing page/pool contract. A continuation preserves every original selector and sort choice. Reads never refresh a bundle or fan out into summary-statistics files.

The initial page candidate is three associations. The measured native projection, complete envelope and action bytes determine whether that remains useful; no unmeasured hard token ceiling is proposed as an observation. A caller may request a larger bounded page. Study metadata is fetched only by its explicit get or selected section, not once per search hit.

Each response has one pure next-step function under architecture 6.10. Search selects one association get per row before continuation. Source-order distinct linked study, rsID and publication pivots follow the architecture's bound. A source error yields the existing fresh action; an expired pool yields restart. An association get never emits its own get as a step. Steps use the current source-given identity rather than the caller's spelling or a locally guessed symbol.

## Proposed types and schemas

The shared biodata layer owns checked study accessions, qualified association IDs, source ontology IDs, exact rsIDs, text and finite/exact numeric representations. The query layer owns request selectors and response records. The source adapter owns source field spelling and validation. Each fact has one definition; generated schemas and help derive from those definitions.

`GwasSearch` contains explicit optional variant, trait text, trait ID, mapped-gene, study and PMID selectors, explicit child-trait and extended-gene-set choices, page window and any reviewed p-value selector. `GwasGet` is an explicit study-or-association sum. Study-only sections reject association input before I/O. Every selector requires a captured positive case and an invalid-input case.

An association record preserves its association ID, study accession, reported-trait array, mapped-trait ID/label array, all rsID/effect-allele pairs, p-value representation, odds ratio, beta text/number/units where supplied, interval, frequency text including unknown markers, PMID, author and source mapping context. Missing values are optional; an unknown frequency is not zero. Odds ratio and beta are separate optional fields. Neither overrides the other, and an effect cannot be interpreted without its allele, unit and study context. A zero caused by floating-point underflow cannot replace the source's exact mantissa/exponent representation.

A study card preserves source accession, publication and source-native sample descriptions. Discovery and replication populations remain separate. Ancestry rows preserve source categories, stage, counts and recruitment metadata where supplied; they never sum independently described samples into unique people. Summary-statistics availability is a source flag or link, not proof that full data were downloaded, validated or licensed for every use.

The new schemas are versioned response and request schemas. Extension of the existing shared association representation must preserve variant-section compatibility. A captured public field absent from some records remains optional. An unproved public field stays outside the initial schema. No dictionary of local risk labels, inferred disease names or gene heuristics is proposed.

## Source and bundle disposition

Keep the existing keyless GWAS Catalog API and source policy. API metadata and complete summary-statistics datasets have distinct rights. The [publisher FAQ](https://www.ebi.ac.uk/gwas/docs/faq/) places non-summary catalog data under EBI service terms and describes study-specific exceptions for summary statistics. Preserve the current restricted source category until the PM reviews any change. Enabling the source does not authorize every reuse of every linked dataset. No key is required and no key value enters the design or evidence.

Keep the original source captures outside Git. Prefer bounded API reads for the harvested questions. Investigate the association, study, ancestry and trait-mapping download inventories before deciding whether a local bundle buys a demonstrated retrieval capability. A download endpoint being public does not justify downloading the whole catalog. No summary-statistics bundle or tool execution belongs to this first slice. Do not add GWAS Central, Enrichr or another source without a harvested unanswered question and a separate source decision.

## Agent questions

1. **What do I ask first?** Ask for associations by the exact variant, trait, study, publication or source-defined gene selector. A literal trait query remains literal. It is never expanded into a diagnosis or inferred disease hierarchy.
2. **What lets me choose?** A search row carries the source association ID, study, trait, allele and reported statistic. It carries enough information to choose one source assertion without another identity call.
3. **What comes next?** Get the association or study, then select ancestry/sample context, a variant, the publication or the next association page. The output names those actions explicitly.
4. **Which IDs must survive?** All association IDs, study accessions, rsIDs, mapped ontology IDs and PMIDs survive unchanged. A gene pivot needs an exact stable source join, not a locally matched label.
5. **How does disagreement appear?** Preserve separate source records and their context. A later study or a different ancestry may report another effect. Do not combine them or choose the smaller p value as clinical truth.
6. **What does zero mean?** A successful source page with zero records means that this source query returned no rows. A failed request is failed. Neither establishes no association in nature, benignity or lack of risk.
7. **What controls latency?** One bounded API page controls a search. Warm reply reuse follows the existing source cache. Native request timings and output projections are measured separately from product latency; no product latency is claimed by a design projection.
8. **What changes from 0.9?** Preserve the ability to retrieve mapped-gene and trait associations, bound disclosure and retain effect context. Add exact association/study identity. Replace incomplete prefix intersections with a reviewed source query or an honest unsupported combination.
9. **When may I stop?** Stop after the selected source assertion answers the retrieval question with its allele, effect type, study and publication. Do not fetch ancestry, every association or full summary statistics unless the question needs them.
10. **What makes the assertion trustworthy?** Show publisher identity, original study/publication, checked identifiers, retrieval date, source status and terms. Report the source's statistic without a clinical interpretation.

## Legacy surface and public issues

The static inventory reads 0.9 source revision `4050fdc7dbdf4d713d94cf02a39a147b863665be`. Command declarations, dispatch, source adapter, entity mapper and user guide were read together. The guide's suggestion that gene and trait can be combined does not prove the old prefix intersection was complete.

| 0.9 surface | Options and sections | Proposed disposition |
| --- | --- | --- |
| `search gwas` | Positional query alias, `-g`/`--gene`, `--trait`, `--p-value`, `-l`/`--limit`, `--offset`. | Keep mapped-gene and literal trait access, bounded pages and the finite p-value selector's useful capability. Review exact source-supported combinations and threshold semantics before implementation; do not claim local first-50-row filtering as a complete answer. |
| `get variant ID gwas` | `gwas` section, also selected by `all`; shared variant get options. | Already owned by variant. Retain exact-rsID behavior and source status. Reuse its adapter; do not duplicate ownership or count this as a new GWAS leaf. |
| Generic `batch` | Variant batch with shared section selector. | Shared ownership. No new GWAS batch command or hidden multi-ID fan-out. |
| JSON/Markdown and MCP | Shared `--json`, cache/source policy and transport options. | Shared envelope and existing Markdown/MCP path. They do not multiply the command-specific option count. |

The old search maps either gene or trait to one v2 request sorted by p value. A combined gene/trait search fetches two independent 50-row prefixes and intersects them locally. That bounded-prefix result cannot establish completeness for the source intersection. The old p-value filter applies locally to fetched rows. Exact p-value parts are preserved by the old mapper when supplied. New types must retain that precision and cannot replace underflow with a literal probability of zero.

All 57 closed public issues were enumerated through GitHub. The GWAS-relevant bodies were read. [Issue 283](https://github.com/genomoncology/biomcp/issues/283) uses a GWAS call to expose Windows cache-write stdout corruption. Keep the shared clean JSON-RPC stream behavior; add GWAS coverage to the shared Windows/MCP proof when this source writes cache. This is not a source-specific new subprocess mechanism. [Issue 78](https://github.com/genomoncology/biomcp/issues/78) includes GWAS disease associations as an enrichment-library use. Source association retrieval is kept; local enrichment analysis, other organisms and installation/execution of another tool are outside the amended entity-access scope. Neither issue authorizes a new analytic command.

No 0.9 binary is executed until operating-system containment and a local delivery witness establish that it cannot contact a provider. Without that proof, its timings and byte costs remain unmeasured. Static accounting is not runtime comparison. The proposed design must not claim clinical or output parity from static code reading.

## Outside-in case plan

Every command above needs a known-positive case anchored to a direct request and original publication. CLI and MCP share the same expected envelope and next actions. Cases cover exact study and association identity, multiple studies per PMID, multiple rsIDs/effect alleles per association, multiple ontology mappings, distinct reported/mapped genes, unknown frequency text, optional effect values, exact p-value underflow, default and explicit page sizes, continuation with every selector, terminal pages and source ordering.

Every filter needs its own positive and negative boundary. Combined filters require a source-known intersection outside both independent early prefixes. Trait-child inclusion and extended gene sets require distinct captures that prove their different semantics. Numeric thresholds require rows on both sides, missing values and underflow, with the searched population and truncation made explicit. Invalid limits/offsets/IDs fail before I/O. There is no fuzzy query repair or name-based disease join.

Failure cases cover 429, 500, timeout, malformed JSON, HTTP 200 wrong identity, duplicate association IDs, incorrect page metadata, missing required arrays, valid empty, expired continuation, source disabled and restricted sources disabled. Synthetic failures prove handling; an operator must not trigger provider failure deliberately. Terms/source policy apply before cache or network. Warm reuse, explicit fresh, stale partial status and unchanged source-bound continuation are separate stories. No absent catalog entry becomes a negative clinical finding.

## Ian's questions

The PM decides the product forms and scope through design review. The source contracts and measurements below supply the trade-offs. No clinical or configuration question needs Ian. A material expansion beyond entity access, a new paid service or publication still needs the existing approval path.
