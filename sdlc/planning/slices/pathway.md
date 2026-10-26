# Pathway slice investigation and proposed design

This proposal applies steps 1–4 of [the slice process](../slices.md). It starts from `85e6ba864c3f13ec50432a58670ec2af13e12857`. It proposes entity access to human pathways. It does not infer treatment, calculate enrichment or run analysis tools. The PM reviews the proposed command and source choices before implementation tickets exist.

## Source boundaries

Reactome and WikiPathways remain distinct sources. Their identifiers define separate entities. A common title, overlapping members or a literature citation does not establish pathway equivalence. Human records must carry source-stated species identity. Source release and retrieval time accompany observations. A record with an unexpected identity fails admission and cannot populate the requested card.

The [entity boundary ruling](../../decisions/2026-10-25-entity-boundary-rulings.md) drops g:Profiler and `enrich`. KEGG stays dropped under the existing gene ruling and its reuse terms. This investigation makes no request to either service. Reactome database data use CC0; Reactome images and software have different licenses. The proposed adapter requests database JSON only. WikiPathways content uses CC0; publisher terms and exact current response fields must be verified from the retained capture before types are approved. Both proposed sources belong to the open group, require no key and obey source disabling before network or bundle access.

## Questions and acceptable answers

The question bank separates a source's membership statement from a clinical conclusion. A complete acceptable answer names the source, exact pathway identifier, human species, source version, requested relation, and any limits or failures. Membership never means that every member is actionable, that a drug acts on the whole pathway or that a patient will respond.

The source capture asks 24 literal pathway queries. These are adapted research prompts, not quotations from patients or publications. The clinical reviewer supplies or validates the publication anchors before this bank can count as the final harvest. The final bank also includes source failure and empty-result cases.

<!-- question-bank -->
<!-- /question-bank -->

## Direct source evidence

All live calls run through `pm job` at the pushed starting revision. Each request is one GET with a deadline, an eight-MiB response admission bound and no automatic retry or redirect. The evidence retains raw bytes, status, headers, elapsed wall time and SHA-256. An HTTP failure, truncated body or malformed reply remains a failed observation rather than a valid empty result. No investigator keys enter the request scripts or evidence.

<!-- source-measurements -->
<!-- /source-measurements -->

### Public fields and identity joins

Reactome's query endpoint must prove the requested `stId`, `displayName`, `schemaClass`, species, summary and version before their types are accepted. Its participant endpoint describes physical entities, including complexes and modified proteins. A participant display name is not a gene symbol. The implementation must follow the publisher's reference entities and identifier namespaces. It must not extract symbols with regular expressions or treat each physical participant as one gene. Events may contain both objects and integer database references; every such reference needs either a checked hydration or an explicit unresolved record. Event records do not inherit the pathway's stable identifier.

The landed gene section already joins HGNC's UniProt accessions to `/data/mapping/UniProt/{accession}/pathways?species=9606`. The reverse pathway section must use the same identifier namespaces. Proposed joins resolve a captured UniProt accession through the existing checked HGNC map. A reference lacking an exact join remains a source reference with a problem and no fabricated gene action. Several proteins mapping to one gene do not erase their source membership or modification context.

WikiPathways search and get are catalog resources in 0.9. A requested WP identifier must match a returned catalog row exactly. Its xref resource carries Entrez references, so genes join through HGNC's exact Entrez map. Identifier namespaces remain explicit. A GPML `DataNode` might represent a metabolite, gene product, label or group; neither its text label nor its visual node count is a gene count. A WikiPathways gene section is proposed only when a direct current capture proves the public resource carries the relation.

A pathway-to-drug relation requires an exact gene identity followed by a captured source's explicit target identifier. No candidate source establishes that relation solely from pathway membership. The initial design exposes exact gene actions. It does not fan out across every gene, deduplicate drugs by name or label a drug as effective against a pathway. Existing drug identity and target sections remain the route from a selected drug to its exact gene context.

### Bundles

Prefer the existing Reactome online endpoint for exact cards and the existing UniProt-to-pathway join. Consider a local WikiPathways catalog only if the retained responses prove a complete public human inventory and useful exact identifiers. A data bundle requires an explicit `sync`, an atomic validated replacement, a revision-bound snapshot, no network on reads, preserved old data after a failed replacement, source disabling before local reads, and absent/stale/syncing outcomes. The implementation cannot silently turn an unsuccessful API query into a different endpoint or a bundle download.

A publisher directory's compressed file length does not measure decompressed bytes, record count or a build's memory use. No full bundle download is authorized merely to fill those blanks. Capture metadata first. A later bounded whole-bundle job measures full size and validates the complete file independently of code landing when its source design needs one.

## Delight questions

1. **Which human pathways does this exact gene belong to?** Start with the landed `get gene HGNC:1097 pathways`, then follow the exact returned Reactome stable ID. A proposed `search pathway --gene HGNC:1097` supplies a revision-bound full walk when the gene card's bounded list is insufficient.
2. **Which pathway did the search find?** A header contains its source, stable identifier, native title and source-stated species. It never adds an actionability label or a guessed synonym.
3. **Which genes can I inspect next?** A paged gene relation holds exact source references, optional resolved HGNC IDs and source-provided labels. Actions name resolved genes only.
4. **What happened inside this pathway?** Reactome's events section supplies source events and their native identifiers. WikiPathways does not advertise an events section without a captured public field.
5. **How do I compare sources?** Request each source explicitly. Two records retain their native identity and provenance even when their titles match. No synthetic merged pathway is created.
6. **How can I stop early?** A short header and optional whole summary answer identity and source description. Members and events remain behind explicit disclosure. Counts describe the specific retrieved relation rather than clinical significance.
7. **What is empty and what failed?** A successfully parsed, identity-checked empty source result is empty. HTTP, timeout, malformed content and identity errors are failed source outcomes. A partial multi-source result preserves the successful leg and names the failed leg.
8. **What can I trust clinically?** Membership comes from a named source and release. An agent must obtain independent clinical evidence for therapy, response or actionability. The pathway record makes no such claim.
9. **What does a warm call cost?** A warm page reads its retained snapshot. A cold call's source latency and bytes come from this capture, not from an invented performance estimate. No whole-source fanout occurs during an ordinary read.

## Proposed types and schema

`biodata` owns checked `PathwayId` as a tagged Reactome stable ID or WikiPathways ID, `Pathway`, `PathwayRef`, `PathwayMember`, `PathwayEventRef` and namespaced source identifiers. Reuse the current `PathwayRef` contract; version any changed wire shape. A Reactome stable ID's optional version is a separate source field. A WikiPathways revision stays separate from its WP identity. No generic unchecked string replaces existing checked identifier types.

The neutral entity contains identity, native name, human species and source description. Consumer types own `PathwayHeader`, card, page and source-specific sections. Optional absent scalar fields are omitted. Requested source-supported empty sections carry the established empty state. Unsupported sections fail usage validation before I/O. An object with an unresolved source identifier is not dropped silently. The schema defines all closed source and section sets and versioned page continuations.

The service retains native search order for each named source. It follows the established multi-source round-robin order without score blending or title reranking. Equal native pathway keys may deduplicate inside one source; cross-source names never merge. A pool stores a fixed ordered window, source outcomes, source revision, request identity and expiry. `next` never repeats a live request to reconstruct a page.

## Proposed commands, headers and disclosure

These spellings are proposals for the PM, not shipped behavior.

| Command | Default answer | Deeper data and next steps |
| --- | --- | --- |
| `search pathway TEXT --source reactome` | Five exact source headers, matched literal query, source outcome and total when supplied | One get per header, then page; no guessed total when source omits it |
| `search pathway TEXT --source wikipathways` | Five human catalog headers from the captured public relation | Get per header, then revision-bound page |
| `search pathway --gene HGNC:1097 --source reactome` | Five pathway headers from exact gene-to-UniProt mappings | Get actions then page; several accessions retain source scope |
| `search pathway --top-level --source reactome` | Five source-stated human top-level pathway headers | Get actions then page |
| `get pathway R-HSA-5673001` | Exact identity, title, species, release, available sections and source terms | Section actions for captured source-supported relations |
| `get pathway WP:ID` | Checked WP identity and native human metadata | Only captured available sections |
| `get pathway ID summary` | Whole source summary with source scope and citations if present | No sentence clipping or extraction of clinical claims |
| `get pathway ID genes` | First five source membership references and totals | Exact gene pivots, then section continuation |
| `get pathway ID events` | First five native Reactome events and totals | Page before at most three other actions |
| `get pathway ID all` | All supported sections, each with its own bounded initial page | Continuations retain source, revision and section |

`--source` is explicit for source-specific sections and defaults to Reactome for plain search. Repeated sources can use the established union mechanism only after both source legs are proved. `--limit` accepts the existing checked page-size type; `--offset`, `--pool` and `--fresh` follow existing snapshot contracts. The exact genes/events continuation spelling follows the existing section-page mechanism. No free-form URL or source endpoint enters the command tree. `--top-level` and text are mutually exclusive. `--gene`, text and top-level are mutually exclusive. No `--type` is needed when the entity is already pathway. Non-human pathway IDs are rejected before network I/O.

Next actions follow architecture 6.10: selectable headers receive get actions in source order, then page, then at most three others. Membership pages emit a pivot per distinct resolved gene, then page. Other responses have at most five actions. Missing bundles offer sync; expired snapshots offer restart; disabled sources do not offer a network action.

### Measured disclosure costs

<!-- disclosure-costs -->
<!-- /disclosure-costs -->

Bytes divided by four are a planning estimate of tokens and are labeled as such. They are not a tokenizer measurement. Whole observed raw source bodies are not proposed default outputs. Before adapters land, actual CLI and MCP output must be retained and measured at the released revision, with full output and the shared source-terms comparable formula reported separately. The product cases enforce measured bytes plus 15 percent for new budgets. An existing entity-help or MCP description cap increase requires a PM ruling. Whole-tree help and source enum allowances follow their standing measured rules.

## Complete 0.9 accounting

The inspected legacy source is `4050fdc7dbdf4d713d94cf02a39a147b863665be`. No legacy command runs against the network in this investigation. Verified network containment has not yet been established for a retained executable, so all 0.9 latency and output comparisons are **unmeasured**, rather than presumed equal to current fixtures. The source inventory below accounts for behavior without executing KEGG, g:Profiler, hidden trial fallback or drug fanout.

| 0.9 surface | Every option or section | Proposed outcome and reason |
| --- | --- | --- |
| `search pathway` | Positional query; `-q/--query` | Keep positional query. Retire query alias with `use: search pathway <text>`. |
| `search pathway` | `--type` | Drop redundant type filtering; exact accepted pathway class is enforced. Return fixed retirement guidance. |
| `search pathway` | `--top-level` | Keep native human Reactome top-level discovery, with explicit query conflict validation. |
| `search pathway` | `-l/--limit`, `--offset` | Keep bounded pages with snapshot-bound continuation. |
| `get pathway` | ID and default card | Keep Reactome and captured human WikiPathways identity. KEGG IDs return fixed retirement without a request. |
| `get pathway` | `genes` | Keep exact source identifier relations after captures prove them. Remove participant-name regex extraction. |
| `get pathway` | `events` | Reactome only, preserve integer/object reference distinction. Unsupported source requests fail before I/O. |
| `get pathway` | `enrichment` | Drop under entity boundary ruling; fixed message points to external g:Profiler use without calling it. |
| `get pathway` | `all` | All supported kept sections, bounded individually; never invokes enrichment. |
| `pathway drugs` | ID; `-l/--limit`; `--offset` | Retire automatic whole-pathway gene fanout. Guide to selecting an exact gene and the existing explicit drug workflow. Requires PM confirmation because this is a shipped behavior. |
| `pathway articles` | ID; `-l/--limit`; `--offset` | Retire implicit title-to-keyword search. Guide to explicit `search article <text>`; never claim article linkage from a matching title. Requires PM confirmation. |
| `pathway trials` | ID; `-l/--limit`; `--offset`; `--source ctgov\|nci` | Retire pathway-name condition search and first-successful-gene fallback. Guide to explicit `search trial --gene <gene>`. Requires PM confirmation. |
| `gene pathways` | Gene; `--limit`; `--offset` | Keep landed `get gene <input> pathways`; full membership paging belongs to proposed `search pathway --gene`. |
| `enrich` | Comma-separated genes; `-l/--limit` | Drop whole command under Ian's entity boundary ruling. Fixed retirement message; no provider and no local calculation. |
| global output flags | JSON, Markdown, CLI/MCP transport | Preserve shared versioned envelope and identical request interpretation. Do not create a separate pathway MCP tool. |

### Closed public issues

The GitHub closed-issue inventory contains 57 issues on the investigation date. A case-insensitive scan of every title and full body for `pathway`, `Reactome`, `WikiPathways`, `g:Profiler` and `enrich` finds [issue 78](https://github.com/genomoncology/biomcp/issues/78). Its enrichment request is dropped by the entity boundary ruling. Its Open Targets associations and other entities remain with their owning slices; its request does not authorize importing analysis tools. This inventory is separate from private development tickets.

The legacy private history also records useful failure classes: 0016 truthful source-supported sections, 0017 default gene walls and ranking, 0163/0169 WikiPathways HTML/404 sanitization, 0346 isolated WikiPathways tests, 0390 routine/live separation, 0422 volatile live top-N, 0443 wrong-entity guidance, 0669 deterministic ranking, 1063 scoped search-all results and 2282 recording fidelity. Keep truthful errors, isolated fixtures, deterministic retained evidence and exact identity guidance. Remove title heuristics and unsupported sections. Never pin live mutable source counts as timeless expectations.

## Outside-in case matrix

| Contract | Required cases |
| --- | --- |
| Plain search and source choice | Known-positive exact native row for each source; literal multiword query; each requested source only; retained native order; source conflict validation; empty query; actual captured no-match |
| Gene selector | Exact HGNC and UniProt join; several accessions; missing exact map; no symbol extraction; incompatible text/top-level selectors; disabled source before lookup |
| Top-level | Native human positive and terminal page; non-human rejection; text conflict |
| Paging options | Default, one and maximum limit; invalid limits; offset zero and last page; no duplicates/gaps; pool mismatch; expired/missing pool restart; retained snapshot after new sync |
| Get identity | Reactome and WP positive; wrong returned ID; malformed/non-human ID; unknown genuine ID; missing species; wrong species; wrong-entity fixed guidance |
| Summary | Whole native text; absent summary distinct from source failure; citations remain native references |
| Genes | Exact namespaced references; duplicates and one-to-many protein/gene mappings; unresolved ID preserved without action; mixed physical-entity kinds; actual relation count; source-specific availability |
| Events | Native object plus integer references; no guessed stable IDs; absent public field; unsupported WikiPathways request before I/O |
| All and section paging | Supported sections only; each page bounded; source state per requested section; no hidden enrichment leg; genuine continuation arguments round-trip through CLI/MCP |
| Faults | Recorded 429, 500, timeout, invalid JSON, HTML success body, oversized body, foreign identity; no failure converted to empty |
| Terms and policy | Exact use/name/URL; disabled-before-cache/network; open/restricted groups; no credential discovery; source terms comparable helper removes all three metadata fields |
| Retirement | Every legacy spelling/option above; fixed exact message; zero provider requests; no invalid parser fallback |
| Costs | Default, maximum, terminal and largest observed section; CLI newline and compact JSON bases explicit; full and comparable output; entity help, whole help, MCP description and executable examples |

Captured success cases carry their observation date, request and exact expected identifiers. Authored failures are labeled synthetic. A fixture projection preserves every union variant and fields that determine identity, absence, multiplicity and collection scope. Native bodies stay in the evidence archive. Tests use isolated configuration and deny network on replay.

## Structural risks

One source declaration owns terms, quota, deadlines and admission limits. Existing checked gene/pathway identities own joins. Generated schemas and help derive from the request types. A single next-action function owns actions. The inventory spans Rust cases and YAML. Tests preserve required OS environment while isolating user state. Exact source records and snapshots prevent live rereads from changing a page. Native capture precedes types, and every consumed field is tested against actual public-tier evidence. No new generic graph framework, pathway-equivalence engine or analysis backend is proposed.

## Ian's questions

No new decision is requested from Ian. Existing entity and source rulings apply. The PM should decide the exact source defaults, the proposed retirement of the three heuristic/fanout helpers, the useful bundle scope and any missing public-field proof before filing product work. Clinical review must approve the question bank and wording. Any new cost, paid source, licensed content or external commitment returns through the coordinator and PM.
