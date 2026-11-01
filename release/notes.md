# BioMCP 1.0

BioMCP 1.0 retrieves source records for cancer care and human health research through the CLI and MCP. Upgrade existing `biomcp-cli` installations with their package manager. FHIR patient reading is planned for 1.1 and is unavailable in 1.0.

## Breaking changes for 0.9 users

JSON replaces Markdown as the default CLI output. Select `--format md` or set `"format": "md"` in configuration for Markdown. MCP always returns JSON. Scripts must read the new response envelopes and entity schemas instead of parsing 0.9 output.

MCP clients must replace the separate typed tools with the `biomcp` tool and its CLI argument array. Run `biomcp mcp-config` to generate stdio wiring for the installed executable. The client still starts `biomcp serve`. HTTP uses `biomcp serve --http [ADDR]` with `/mcp`; it refuses browser origins and requires allowed hosts for non-loopback listeners. Hosted deployments need caller credential isolation and source pacing configuration. See the [MCP setup](https://github.com/genomoncology/biomcp/blob/main/README.md#connect-an-mcp-client).

The [upgrade table](https://github.com/genomoncology/biomcp/blob/main/sdlc/planning/upgrade.md#commands) lists every 0.9 leaf command and each removed or deferred option, with its replacement or retirement ruling. Read the linked entity sections for positional sections and aliases. The [1.0 command reference](https://github.com/genomoncology/biomcp/blob/main/docs/reference/commands.md) gives accepted arguments. Major changes include:

- Use `get ENTITY` with repeated IDs instead of batch wrappers. Use trailing section words instead of batch `--sections`.
- Use `sync SOURCE` for stored bundles and `store clean` for expired replies and collections. `cache clean --dry-run`, `--max-age` and `--max-size` have no maintenance-policy replacement.
- Use returned `--next-page` and `--pool` commands for continuation. Keep the returned argument arrays and policy settings together. Search offsets and hidden sessions do not identify the new collections.
- Use explicit entity search and get commands. Discover, chart generation, local study analysis, self-update and automatic text-based classification or identity merging retire. Patient reading moves to 1.1.
- Trial discovery uses literal `--variant` and `--criteria`; mutation and biomarker aliases and semantic therapy-line filters retire. Discovery never determines eligibility.
- Article downloads use exact file selections and `--save PATH`. Text readers, PDFs and supplements have separate commands. HTTP MCP refuses file saves.
- Variant normalization uses `--source`, `--hgvs` and explicit builds. Variant source assertions, population and recurrence use their named commands. No source score becomes a clinical classification.

## Configuration migration

Create a JSON configuration file at the platform path described in the [upgrade guide](https://github.com/genomoncology/biomcp/blob/main/sdlc/planning/upgrade.md#configuration-and-keys). Unknown fields fail. Replace the old `cache.toml` and its `[cache].max_age_secs`, cache mode, size and disk policies. The following 0.9 operator variables no longer configure 1.0:

- `BIOMCP_CACHE_DIR`, `BIOMCP_CACHE_MODE`, `BIOMCP_CACHE_MAX_AGE`, `BIOMCP_CACHE_MAX_SIZE`, `BIOMCP_CACHE_MIN_DISK_FREE`: use the platform store and explicit `store clean`; there is no user cache eviction policy.
- `BIOMCP_DEFAULT_ASSEMBLY`: pass the command's explicit `--build` when required.
- `BIOMCP_STUDY_DIR`, `BIOMCP_DDINTER_DIR`, `BIOMCP_EMA_DIR`, `BIOMCP_WHO_DIR`, `BIOMCP_CVX_DIR`, `BIOMCP_GTR_DIR`, `BIOMCP_GENCC_DIR`, `BIOMCP_WHO_IVD_DIR`: use supported `sync SOURCE` bundles. Unsupported bundles and local study analysis have no 1.0 replacement.
- `BIOMCP_FHIR_BASE`: patient reading is planned for 1.1.
- `BIOMCP_CA_BUNDLE` and its `SSL_CERT_FILE` fallback: set `ca_bundle` to add PEM roots to platform trust.
- `BIOMCP_VARIANT_QUERY_GENE_ROUTING`: use explicit variant selectors.
- `DISGENET_API_KEY`: the DisGeNET gene section retires.

The source credential names `ONCOKB_TOKEN`, `NCBI_API_KEY`, `S2_API_KEY`, `NCI_API_KEY`, `OPENFDA_API_KEY`, `ORCID_ACCESS_TOKEN`, `UMLS_API_KEY` and `ALPHAGENOME_API_KEY` remain recognized. Use `biomcp key set SOURCE` to keep values out of shell history. Stored keys override environment values. Use JSON `format`, `disable_sources`, `keys`, `ncbi_email`, `ca_bundle`, `proxy` and `http.allowed_hosts` for the corresponding settings. Proxy environment variables remain supported. Every source starts enabled; commercial users must disable the `noncommercial` and `restricted` groups and review publisher terms.

## Output migration

Every JSON reply advertises its schema identity. Read `result` for records, `sources` for source outcomes and terms, `problems` for typed failures, `status` for completeness, `page` for paging and `next` for argument arrays. Commands have their own typed result shapes; 0.9 fields are not a compatibility interface. Missing optional values stay absent. Source-native identifiers and assertions remain separate when sources disagree. Counts preserve their source scope and never establish clinical eligibility or an assayed denominator.

The 0.9 metadata and error fields change as follows:

| 0.9 field | 1.0 migration |
| --- | --- |
| Flattened entity fields at the reply root | Read the command's `result` and its typed item or section shape. Gene cards, for example, use `result.items[].answer.gene`; article and trial cards have different schemas. |
| `_meta.evidence_urls[].label`, `_meta.evidence_urls[].url` | Read record URLs and source provenance from the result's schema. There is no universal evidence URL list. |
| `_meta.next_commands[]` | Read `next[].args` arrays with typed actions; do not split shell command strings. |
| `_meta.suggestions[]` | Read explicit next actions and source problems. No generated suggestion text is a compatibility field. |
| `_meta.section_sources[]` | Read `sources[]` and the source-native identities in each result. |
| `_meta.workflow`, `_meta.workflow_rationale`, `_meta.workflow_playbook` | Removed with automatic workflow selection. |
| `_meta.discovery_sources[]` | Removed with discover. Use explicit entity and source queries. |
| `_meta.not_found` | Read the typed problem and status for the requested command. |
| `_meta.alias_resolution.kind`, `requested_entity`, `query`, `canonical`, `canonical_id`, `confidence`, `match_tier`, `sources`, `candidates` | Read each command's explicit resolution and candidates. The old confidence tiers and cross-entity alias inference retire. |
| `error.code`, `error.message`, `error.source`, `error.recovery`, `error.limit_bytes`, `error.limit`, `error.unit` | Read `problems[]` and returned next actions. Error codes and limit fields use the advertised command schema. |
| Variant `filter_evaluation`, `requested_variant`, `resolution` at the reply root | Read the variant result's checked request, source outcomes and native candidates. Do not reuse old top-level projections. |
| Preview `values`, `returned`, `total`, `has_more`, `omitted_oversized` | Read the section or page schema and follow its continuation. A preview count never supplies a population denominator. |

The seven 0.9 public entity schemas also change their entire root contract: gene, variant, article, trial, drug, disease and pathway records now sit inside command-specific envelopes. Their [1.0 schemas](https://github.com/genomoncology/biomcp/tree/main/crates/biomcp-types/schemas) define every field. Source-specific sections no longer flatten into those seven shared records. Rebuild integrations against their command schema rather than treating a field of the same name as the same contract.

The candidate's schema identity changes appear below when these notes are staged. The [schema directory](https://github.com/genomoncology/biomcp/tree/main/crates/biomcp-types/schemas) defines output fields and required values. In particular, adverse-event snapshot fields become optional when a cache write fails; callers must handle their absence. A major bump permits an intentional breaking schema change. The schema check does not prohibit all breaking changes.
