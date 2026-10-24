# Architecture

BioMCP is a Rust command and a one-tool MCP server for biomedical search and get. Its user is an agent that supports cancer care decisions for clinicians. It asks providers such as PubMed, ClinicalTrials.gov, MyGene.info and ClinVar, and local data bundles, and returns typed JSON in biodata's shapes. It reports what sources say and never classifies a variant or scores a clinical match.

Every choice here follows `decisions/2026-10-23-least-code-easy-upkeep-fast.md`: the least code we own, easy upkeep, and fast.

## Scope

1.0 covers search and get for the entities BioMCP 0.9 serves: gene, variant, article, trial, drug, disease, protein, pathway, phenotype, adverse event, diagnostic, pharmacogenomics, GWAS, cell line, author and study. A get may return a study. Article get includes supplementary material. Study analysis, charts and VCF work are out of scope, as `decisions/2026-10-23-search-and-get-is-the-1-0-scope.md` says.

## Layers

Each layer has one job and depends only downward.

| Layer | Job |
| --- | --- |
| biodata | Typed values for every entity, query and response, and their JSON Schemas. No I/O. |
| Transport | The one HTTP client and the bundle reader: deadlines, rate limits, retry, body limits, the raw reply cache, record and replay. |
| Adapters | One module per source. Declares endpoints and maps replies into biodata values. |
| Store | The SQLite entity store: entities, aliases, links and query pools. |
| Services | Resolve input, fan out to adapters, merge, rank, pool and page. |
| Rendering | Views of response types for people, such as Markdown. |
| Interfaces | The CLI grammar, the MCP tool, the config file and, later, the web wiki. |

## Workspace

| Crate | Owns | May depend on |
| --- | --- | --- |
| `biodata` | Entity types, ID newtypes, closed-set enums, coordinates, query types, the response envelope, JSON Schemas | `serde`, `winnow` for identifier and notation text; `schemars` behind the `schema` feature |
| `biomcp-fetch` | The transport | `biodata`, `reqwest`, `rusqlite`, `serde_json` |
| `biomcp-adapters` | One module per source | `biodata`, `biomcp-fetch`, `serde`, `serde_json` |
| `biomcp-store` | The entity store | `biodata`, `rusqlite`, `serde_json` |
| `biomcp-query` | The services | `biodata`, `biomcp-adapters`, `biomcp-store` |
| `biomcp-render` | People's views of response types | `biodata` |
| `biomcp` | The binary: CLI, MCP tool, config, the runner's test target | every crate above, `clap`, the MCP library |

A crate exists only once a stage needs it. `biomcp-store` arrives with the article pool. `biomcp-render` arrives with the first people's view.

Rules, enforced by Cargo, clippy and the ratchet check where a tool can:

- `biodata` does no I/O. Its own `clippy.toml` disallows file, environment, network, process and clock items one by one.
- Only `biomcp-fetch` depends on `reqwest`. Only `biomcp-fetch` and `biomcp-store` depend on `rusqlite`.
- Adapters never depend on the store, the services, rendering or the binary.
- Rendering depends only on `biodata`. It never fetches or reads a file.
- Services take their context, such as the deadline, the fetch handle and the store, as arguments. No crate except the transport and the store holds process state.
- Only the binary reads the config file or the environment, as `decisions/2026-10-23-settings-live-in-an-xdg-config-file.md` says.
- No layer builds command strings. Services return typed next steps, and the interface spells them.

The transport stage chooses blocking or async I/O and records the choice as a decision. Only `biomcp-fetch` and the fan-out in `biomcp-query` may name a runtime.

## biodata

`biodata` is the base data layer for every entity, as `decisions/2026-10-23-biodata-is-the-base-data-layer.md` says. It has no command of its own. It may later merge with another biodata crate into one open-source crate, so it names no other project and depends on no other crate in this workspace.

Layout:

```
crates/biodata/
  src/id/         one file per identifier family: pmid.rs, nct.rs, hgnc.rs, entrez.rs, rxcui.rs, mondo.rs, ...
  src/coord.rs    Build, Contig, Position1, Span0, GenomicInterval
  src/gene.rs  article.rs  trial.rs  ...   one file per entity
  src/query/      one file per entity query type
  src/response/   envelope.rs, status.rs, problem.rs, next.rs, one file per card or page type
  schemas/        generated *.schema.json, checked in
```

A value is built once through a checked constructor and never changes. Identifiers, closed sets, dates and coordinates are types, never strings. Free text such as an abstract or eligibility criteria stays text. Display text never lives in a value. The rules are in `planning/rust-standards.md`.

Variant parsing has one implementation. The variant slice decides whether this crate parses HGVS and alteration shorthand itself or takes a shared parser crate. Two copies are never allowed.

## Adapter shape

### The shared path

The transport gives adapters one call per body kind: `json`, `xml`, `text` and `bytes`. Each call takes an endpoint declaration and request parameters and applies, in order:

1. The caller's deadline.
2. The source's rate limit, keyed by source and credential.
3. The raw reply cache: serve a fresh reply, or fetch and store.
4. Replay mode serves only recorded replies and fails on a miss. Record mode writes each reply with method, URL, body hash, status, headers and delay.
5. The send, with retry on 429, 5xx and dropped connections, honoring `Retry-After`, capped by the deadline.
6. The body size limit and content-type check.
7. Status mapping into one `SourceError`: `NotFound`, `RateLimited`, `Unavailable`, `Rejected`, `BadReply` or `DeadlineExceeded`.

An endpoint is a constant:

```rust
pub(crate) const ESEARCH: Endpoint = Endpoint::get(Source::PubMed, "esearch.fcgi")
    .optional_key(Secret::NcbiApiKey, "api_key")
    .fresh_for(Freshness::Hours(24));
```

An adapter call is a short function:

```rust
pub fn search(fx: &Fetch, q: &ArticleQuery, at: Cursor) -> Result<Hits<Pmid>, SourceError> {
    let raw: raw::ESearch = fx.json(&ESEARCH, &[
        ("db", "pubmed"), ("term", &term(q)), ("sort", "relevance"),
        ("retstart", &at.offset_str()), ("retmax", &at.size_str()),
    ])?;
    Ok(map::hits(raw, at))
}
```

### One module per source

```
crates/biomcp-adapters/src/<source>/
  mod.rs   endpoints and public calls
  raw.rs   serde types that mirror only the fields the mapping reads
  map.rs   raw to biodata, with per-field problems
```

`map.rs` converts raw types into biodata values through checked constructors. A failed check becomes a typed per-field problem, such as "PubMed returned a PMID of 0", and the rest of the record survives. An adapter never constructs a client, retries, sleeps, reads the environment or builds an error string. A source quirk, such as PubMed returning newest first without `sort=relevance`, sits in one visible line of one adapter with a doc comment and a case that fails without it.

Sources that need more than one GET fit the same path. A GraphQL source sends a query constant as the POST body. A paged source maps its cursor into the shared `Cursor`.

### Data bundles are local sources

A local data bundle, such as EMA or WHO data, is a source like any remote API, as `decisions/2026-10-23-data-bundles-are-local-apis.md` says. It uses a `Bundle` endpoint kind. The transport downloads it, revalidates it by ETag or date, and stores it. The adapter reads rows from the stored file through the same `mod.rs`, `raw.rs` and `map.rs` shape. `biomcp sync <bundle>` refreshes one. Each bundle is judged on its own merits in its slice.

### Size

BioMCP 0.9 spends about 55,800 product lines on source code and reply mapping. The 1.0 estimate is 13,500 to 27,850 lines, because the transport does once what each 0.9 adapter repeats. The article slice tests the estimate first. If its adapters average over 300 lines, this section is revised.

## Output contract

JSON is the default output, shaped by biodata's schemas, as `decisions/2026-10-23-json-is-the-default-output.md` says.

Every command returns one envelope:

```json
{
  "schema": "biomcp.gene_card/1",
  "status": "partial",
  "query": { "schema": "biomcp.gene_get/1", "id": "BRAF", "sections": ["civic"] },
  "result": { "gene": { "schema": "biodata.gene/1", "symbol": "BRAF" }, "sections": { "civic": {} } },
  "sections": { "civic": { "state": "unavailable", "source": "civic", "reason": "rate_limited" } },
  "problems": [],
  "next": [ { "command": ["get", "variant", "BRAF V600E"] } ]
}
```

The values above are illustrative. Rules:

- `status` is `complete`, `partial` or `failed`. A source failure never becomes an empty success.
- A requested section reports one of four states: data, confirmed empty, unavailable or not applicable. Sections the caller did not request are absent.
- `next` holds typed steps. The CLI prints them as commands and MCP as tool arguments.
- Nulls and empty lists are omitted. Long free text carries a `truncated` flag and the step that fetches the rest.
- A coordinate is a `GenomicInterval` with a stated or explicitly unstated build, never prose.
- The CLI prints compact JSON. `--format md` prints a Markdown view for people, rendered from the same type. A view never holds content the JSON lacks.

### Schemas

`schemars` derives the schemas from the Rust types behind the `schema` feature. The generated files sit in `crates/biodata/schemas/`. A test regenerates them and fails on any difference. Each schema name carries a major version. Adding an optional field keeps the version. Removing, renaming or retyping a field raises it. The same types generate the CLI arguments, the MCP input schema and the output schema, so the three cannot drift.

## The CLI tree

The user is an agent, as `decisions/2026-10-23-the-user-is-an-agent.md` says. Speed and progressive disclosure come first: return just enough, and say which subcommand or option goes deeper.

- `biomcp search <entity> [filters]` returns headers: the identifier, a title line and the few fields an agent needs to choose. It never returns whole records.
- `biomcp get <entity> <id> [sections]` returns one card. Sections are a closed enum per entity. Several IDs may be given at once.
- `biomcp <entity> <relation> <id>` drills down or follows a link, such as `trial locations NCT...`, `trial outcomes NCT...` or `gene trials BRAF`. Trials set the pattern: a search returns trial headers, and subcommands drill into locations, outcomes and other sections.
- Every response's `next` names the drill-downs that apply to what it returned.
- One spelling per idea across entities. Help text lives as doc comments on the query type fields, so the CLI help, the MCP schema and the reference docs come from one place.
- Global flags: `--format md`, `--fresh` to refetch, `--disable-sources` and `--config`.
- Housekeeping commands: `schema`, `sync`, `store`, `serve`, `mcp-config`, `health`, `version` and `skill`.

Every command and option has at least one harvested case, as `planning/slices.md` says.

## MCP

MCP is one main tool, as `decisions/2026-10-23-mcp-is-one-tool-over-the-cli.md` says. Its input is the command path and arguments as the CLI tree spells them, as a list and never a shell string. The same grammar parses them and the same command functions run. It returns the same envelope as the CLI, in MCP's structured output where the client supports it and as JSON text otherwise. The tool description and `schema` paths tell the agent the tree. `biomcp serve` runs MCP over standard input and output. `biomcp serve --http` serves it over HTTP on 127.0.0.1 by default.

## Cache and store

The cache is a local SQLite database, as `decisions/2026-10-23-the-cache-is-a-local-sqlite-database.md` says. One file holds two kinds of data with separate owners.

| Table | Owner | Holds |
| --- | --- | --- |
| `reply` | transport | source, request key, status, headers, body, fetched time, fresh-until time |
| `entity` | store | kind, canonical ID, schema version, JSON value, contributing sources, fetched and fresh-until times |
| `alias` | store | kind, normalized text, canonical ID, source |
| `link` | store | from kind and ID, relation, to kind and ID, source, evidence, fetched time |
| `pool` | store | query key, request, per-source status, created time |
| `pool_item` | store | query key, rank, entity ID, ranking features |

- Each endpoint declares how long its replies stay fresh. A stale value is served only with a `stale` note in the envelope.
- A pool is a snapshot. It is built once per query key and never changed. Paging slices the stored order, and a continuation token holds the pool key and offset. A rerun after a partial source builds a new pool.
- A warm page makes no network request.
- All SQL lives in `biomcp-fetch` for replies and `biomcp-store` for the rest. No store trait exists until a second engine is needed.
- Later, `biomcp serve --http` also serves a browsable wiki over the store: one page per stored entity, links from the `link` table, each fact's source and fetched time, and a button that runs the same query service to pull in more.

## Errors

- Each library crate defines one error enum with `thiserror` and stable codes. Source identity is an enum, never a string.
- The binary maps each error family to an exit code in one function.
- A source failure inside a command becomes a typed partial status, not an error exit.
- No error or diagnostic prints the user's input text, because inputs may hold patient data.

## Testing

Outside-in tests only, through the built binary and the MCP tool, over recorded replies, as `decisions/2026-10-23-outside-in-tests-and-one-small-runner.md` says. A case is one YAML file:

```yaml
feature: article-search
covers: [search article, --gene, --variant]
story: AKT1 E17K leads to the 2007 discovery paper and keeps every plain PubMed hit
harvest: paper PMID 17611497
verified: 2026-10-23 live PubMed ESearch, sort=relevance
run: [search, article, --gene, AKT1, --variant, E17K, --all]
replay: akt1-e17k-2026-10-23
expect:
  /status: complete
  /result/items/*/pmid:
    includes_all_of: floor/pubmed-akt1-e17k.json
    rank_of: { "17611497": "<= 10", "33123537": "<= 2" }
    unique: true
  schema: biomcp.article_page/1
```

The flags above are placeholders for whatever the article slice names. Cases name stable facts: set inclusion, rank ranges, ordering between key items, counts within a range, statuses and schema validity. Full snapshots are allowed only for help text and schema files. No case touches the network or waits on a real clock. A live run reports every reply whose key fields changed, and a person refreshes recordings on purpose. The runner takes `--cases DIR`, so case sets that cannot be published run against a built binary from outside the repo.
