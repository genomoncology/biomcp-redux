# Architecture

BioMCP is a Rust command and a one-tool MCP server for biomedical search and get. Its user is an agent that supports cancer care decisions for clinicians. It asks providers such as PubMed, ClinicalTrials.gov, MyGene.info and ClinVar, and local data bundles, and returns typed JSON in biodata's shapes. It reports what sources say and never classifies a variant or scores a clinical match.

Every choice here follows `decisions/2026-10-23-least-code-easy-upkeep-fast.md`: the least code we own, easy upkeep, and fast. Where this design picks a lighter option over a heavier one, it says what it rejected and why. Where it changes earlier planning text, it marks the change **Overrides**, and section 13 lists every override in one place. Decisions in `decisions/` are settled, and this design works inside them. Section 14 holds the questions that need Ian.

The flags, sections and field names in the sketches below are placeholders. Each entity's slice settles its own, as `planning/slices.md` says. The shapes, the rules and the signatures are the design.

## 1. Goals and how the design meets them

| Goal | Mechanism |
| --- | --- |
| Far less code | One transport does every request step once. An adapter is a constant source description, serde raw types and pure mapping functions. One grammar serves the CLI and MCP. One generic view renders Markdown from JSON. |
| Easy upkeep | Five crates whose direction Cargo enforces. Lints and a ratchet test enforce size, state and dependency rules from the first commit. |
| Very fast | Blocking I/O on threads with no async runtime. One leg per source, each with its own deadline and limits. A SQLite reply cache, stored pools and stored bundle rows make warm calls local. A budget per command, measured in replay. |
| Right information in context | Search returns headers. Get returns one card with requested sections. Every response names the sections it left out and the next commands that go deeper. Nulls, empty lists and unrequested sections are omitted. |
| JSON by default, shaped by biodata | Every response is one envelope type from `biodata`, with a generated, checked-in and versioned JSON Schema. |
| MCP as one tool over the CLI | The tool takes the CLI argument list. The same grammar parses it and the same service call runs. |
| Search and get scope | The request type has search, get and drill-down commands only. |
| Data bundles as local APIs | A bundle is an endpoint kind in the transport. Its rows sit in SQLite and its adapter maps them like any reply. |
| Cache that later backs a wiki | One SQLite file. Its tables already hold typed results with their sources and times. |
| Outside-in tests with record and replay | Record and replay live in the transport. One small runner drives the binary and the MCP tool from YAML cases. |
| Lint ratchets | Workspace lints, per-crate `clippy.toml` files and one ratchet test, all in `make landing-gate`. |

## 2. Workspace and crates

### 2.1 The crates

Five crates. Each one exists because a rule needs a crate boundary to enforce it.

| Crate | Job | May import | May not import |
| --- | --- | --- | --- |
| `biodata` | Typed values for every entity, identifier, closed set, query, request and response. JSON Schemas. | `serde`, `winnow`; `schemars` behind the `schema` feature | Any workspace crate, any I/O crate, `serde_json` outside tests |
| `biomcp-fetch` | The transport: HTTP, deadlines, rate limits, retry, body limits, decoding, the reply cache, offline, record and replay, bundle download and bundle rows | `biodata`, `ureq`, `rusqlite`, `serde`, `serde_json`, `quick-xml`, `sha2` | `biomcp-adapters`, `biomcp-query`, `clap` |
| `biomcp-adapters` | One module per source: a source description, endpoints, raw reply types and mapping into biodata | `biodata`, `biomcp-fetch`, `serde` | `ureq`, `rusqlite`, `serde_json`, `biomcp-query`, `clap` |
| `biomcp-query` | The services and the entity store: resolve, fan out, merge, rank, pool, page, sections and next steps | `biodata`, `biomcp-fetch`, `biomcp-adapters`, `rusqlite`, `serde_json`, `sha2` | `ureq`, `clap` |
| `biomcp` | The binary: CLI grammar, MCP server, config file, Markdown view, the runner's test target, later the web mode | every crate above, `clap`, `serde_json` | `ureq`, `rusqlite` |

Why each crate is its own:

- `biodata` does no I/O and may later merge with another biodata crate, as `decisions/2026-10-23-biodata-is-the-base-data-layer.md` says. Only a crate with its own `Cargo.toml` and `clippy.toml` can hold that line.
- `biomcp-fetch` is the only crate that names an HTTP client, as `decisions/2026-10-23-one-transport.md` says. A separate crate makes that a dependency fact.
- `biomcp-adapters` must not reach the store, the services or the interface. It sits below `biomcp-query`, so Cargo refuses any such import. It also cannot name `ureq`, so it cannot build a client.
- `biomcp-query` holds the logic that the CLI, MCP and the later web mode share. Splitting it from the binary keeps clap and the config file out of it.
- `biomcp` is the only binary.

**Overrides** the earlier seven-crate table. Two planned crates fold into others:

- `biomcp-store` becomes the `store` module of `biomcp-query`. The store holds pools and later entities and links that only the services write. A separate crate would add a `Cargo.toml`, an error enum and a public API whose only caller is `biomcp-query`. The ratchet test confines `rusqlite` inside `biomcp-query` to `src/store/`. The adapters still cannot reach the store, because they sit below `biomcp-query`.
- `biomcp-render` becomes the `view` module of the binary. The view takes the serialized `serde_json::Value` of a response and returns text. It receives no fetch handle, no store and no typed record. It therefore cannot fetch, read a file or show content the JSON lacks. That property held only by review in the earlier plan. Here the function signature holds it.

Rejected: a crate per entity, a crate per source family, a separate `biomcp-types` crate beside `biodata`, and a separate MCP crate. Each adds a manifest and a public surface without enforcing a rule the five crates miss.

### 2.2 The workspace manifest

```toml
[workspace]
members = ["crates/biodata", "crates/biomcp-fetch", "crates/biomcp-adapters", "crates/biomcp-query", "crates/biomcp"]
resolver = "3"

[workspace.package]
edition = "2024"
rust-version = "1.95.0"          # pinned exactly in rust-toolchain.toml
license = "MIT"

[workspace.dependencies]
serde = { version = "1", default-features = false, features = ["std", "derive"] }
serde_json = "1"
schemars = "1"                   # JSON Schema 2020-12
winnow = { version = "1", default-features = false, features = ["ascii", "alloc"] }
thiserror = "2"
ureq = { version = "3", default-features = false, features = ["rustls", "gzip"] }
rusqlite = { version = "0.37", features = ["bundled"] }
quick-xml = { version = "0.38", features = ["serialize"] }
sha2 = { version = "0.10", default-features = false }
clap = { version = "4", default-features = false, features = ["std", "help", "usage", "error-context", "suggestions"] }

[workspace.lints.rust]
unsafe_code = "forbid"
unreachable_pub = "forbid"
missing_debug_implementations = "forbid"
missing_docs = "deny"

[workspace.lints.clippy]
allow_attributes_without_reason = "deny"
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
indexing_slicing = "deny"
todo = "deny"
unimplemented = "deny"
dbg_macro = "deny"
print_stdout = "deny"
print_stderr = "deny"
too_many_lines = "deny"          # threshold 80 in every clippy.toml
disallowed_methods = "deny"
disallowed_types = "deny"
```

The version numbers above are the stage 1 starting point. The stage that adds each dependency confirms its current release and records the code it saves, as `planning/rust-standards.md` says.

### 2.3 How tools enforce the boundaries

Three tools carry every boundary rule, and `make landing-gate` runs all three.

1. **Cargo.** A crate can import only what its manifest lists. Transitive crates cannot be named. The adapters depend on `biomcp-fetch`, which depends on `ureq`, and still cannot name `ureq`.
2. **clippy, per crate.** clippy reads the nearest `clippy.toml`, so each crate carries its own, and each repeats the test allowances and `too-many-lines-threshold = 80`.
   - `biodata`: disallows file, environment, network, process, thread and clock items one by one, and the interior mutability types `Cell`, `RefCell`, `Mutex`, `RwLock`, `OnceLock`, `LazyLock` and the atomics. Without `unsafe` and without those types, no global state can exist.
   - `biomcp-adapters`: the same list. An adapter has no reason to touch a file, the clock, the environment, a thread or shared state.
   - `biomcp-query`: disallows environment, file and process items, `std::thread::sleep`, `std::thread::spawn` and the interior mutability types. `std::thread::scope` is allowed only in `fanout.rs`, through one `#[expect(clippy::disallowed_methods, reason = "...")]`.
   - `biomcp-fetch`: disallows environment and process items. Its `Mutex`, `Condvar` and `Cell` uses each carry an `expect` with a written reason.
   - `biomcp`: disallows environment reads everywhere except `config.rs`, which carries one `expect` with a reason.
3. **The ratchet test.** `crates/biomcp/tests/ratchets.rs` is an ordinary integration test. It reads the workspace files and fails on any breach of a rule a compiler cannot see:
   - Each crate's direct dependencies are a subset of its allowed list, which the test holds as one table that mirrors section 2.1.
   - `rusqlite` appears in `biomcp-query` only under `src/store/`.
   - No product file exceeds 600 lines. No file in an adapter module exceeds 300.
   - `#[test]` appears in product crates only in the listed exception files.
   - Every `Quirk:` comment names a case file that exists.

`cargo deny check licenses advisories` also runs in the gate. It refuses GPL, LGPL and AGPL code and known advisories.

Rejected: a separate script for the ratchets. A Rust test needs no second language, runs under `cargo test` and fails with a normal test report.

### 2.4 File layout

```
Cargo.toml  rust-toolchain.toml  rustfmt.toml  deny.toml  Makefile
crates/
  biodata/
    clippy.toml  README.md
    src/lib.rs  macros.rs  error.rs  text.rs  date.rs  coord.rs  source.rs
    src/id/        pmid.rs  pmcid.rs  doi.rs  nct.rs  hgnc.rs  entrez.rs  symbol.rs  ...
    src/gene.rs  article.rs  trial.rs  ...            one file per entity
    src/query/     request.rs  page.rs  gene.rs  article.rs  trial.rs  ...
    src/response/  envelope.rs  section.rs  outcome.rs  problem.rs  gene.rs  article.rs  ...
    schemas/       generated *.schema.json, checked in
    tests/         schemas.rs  roundtrip.rs  ids.rs  (public API tables)
  biomcp-fetch/
    clippy.toml
    src/lib.rs  spec.rs  endpoint.rs  leg.rs  deadline.rs  rate.rs  send.rs  retry.rs
    src/decode.rs  cache.rs  record.rs  bundle.rs  secret.rs  error.rs
  biomcp-adapters/
    clippy.toml
    src/lib.rs  raw.rs  mapped.rs
    src/<source>/mod.rs  raw.rs  map.rs
  biomcp-query/
    clippy.toml
    src/lib.rs  ctx.rs  run.rs  fanout.rs  status.rs  error.rs
    src/<entity>/search.rs  get.rs  next.rs
    src/article/pool.rs  rank.rs
    src/store/mod.rs  schema.sql  pool.rs
  biomcp/
    clippy.toml
    src/main.rs  config.rs  grammar.rs  spell.rs  output.rs  exit.rs
    src/mcp.rs  src/view/mod.rs  hints.rs
    tests/cases.rs  ratchets.rs  runner/
tests/
  cases/<feature>/*.yaml
  replies/<recording>/<source>/<key>.json
```

## 3. biodata

`biodata` holds every value that crosses a layer or appears in output. Values that stay inside one layer, such as an adapter's raw reply types or a pool's internal features, stay in that layer. `biodata` names no other project, depends on no other workspace crate and has no command.

Its modules group what a later merge would keep together. Identifiers, coordinates, dates, text and the entity types form the shareable core. `query` and `response` hold BioMCP's request and response contract, and a later split can lift them out without touching the core.

### 3.1 Identifiers

Every identifier is a newtype with a private field, one checked way in, `FromStr` and `Display` that round-trip, and serde in both directions through the same check. Every identifier serializes as its canonical text, so an agent can pass any ID back exactly as it received it.

```rust
/// A PubMed identifier, such as `17611497`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pmid(NonZeroU32);

impl Pmid {
    pub fn new(n: u32) -> Result<Self, IdError>;
    pub fn get(self) -> u32;
}
impl FromStr for Pmid { type Err = IdError; /* digits only, nonzero, at most nine */ }
impl fmt::Display for Pmid { /* plain digits */ }
text_value!(Pmid, "Pmid", r"^[1-9][0-9]{0,8}$");

/// A ClinicalTrials.gov identifier, such as `NCT02576665`.
pub struct NctId(u32);                       // renders as NCT plus eight digits

/// An HGNC gene identifier, such as `HGNC:1097`.
pub struct HgncId(NonZeroU32);               // FromStr requires the prefix; new() takes the number

/// A gene symbol as written, compared and hashed without case.
pub struct GeneSymbol(Box<str>);
impl PartialEq for GeneSymbol { /* eq_ignore_ascii_case */ }
impl Hash for GeneSymbol { /* hashes the ASCII-lowercased bytes, so Hash agrees with Eq */ }

/// A DOI, kept as written and compared without case.
pub struct Doi(Box<str>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum IdError {
    #[error("id.empty")] Empty,
    #[error("id.zero")] Zero,
    #[error("id.prefix")] Prefix,
    #[error("id.width")] Width,
    #[error("id.character")] Character,
}
```

One macro removes the serde and schema boilerplate. Any type with `FromStr` and `Display` gets serialization through `Display`, deserialization through `FromStr` and a string schema with its pattern:

```rust
macro_rules! text_value {
    ($ty:ty, $schema_name:literal, $pattern:literal) => {
        impl serde::Serialize for $ty { /* serializer.collect_str(self) */ }
        impl<'de> serde::Deserialize<'de> for $ty { /* deserialize_str with a visitor that calls FromStr */ }
        #[cfg(feature = "schema")]
        impl schemars::JsonSchema for $ty { /* {"type":"string","pattern":$pattern} */ }
    };
}
```

The visitor accepts borrowed and owned strings, so every serde format works. Error messages carry the stable code and never the input text, because inputs may hold patient data.

### 3.2 Closed sets, dates, text and coordinates

A closed set is an enum with snake_case names. A set that may grow is `#[non_exhaustive]`.

```rust
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum TrialPhase { EarlyPhase1, Phase1, Phase1Phase2, Phase2, Phase2Phase3, Phase3, Phase4, NotApplicable }
```

The enum parses only its own canonical names. Each provider's spelling, such as `PHASE1` or `Phase 1/Phase 2`, maps to the enum in that adapter's `map.rs`. A provider value outside the set becomes a problem on the record and leaves the field absent. No enum carries an `Other(String)` escape.

Dates from literature are often partial, so one type holds what the source stated:

```rust
/// A calendar date to the precision the source gave: a year, a month or a day.
pub struct PartialDate { year: u16, month: Option<NonZeroU8>, day: Option<NonZeroU8> }
impl PartialDate { pub fn new(year: u16, month: Option<u8>, day: Option<u8>) -> Result<Self, DateError>; }
// FromStr and Display use 2007, 2007-07 and 2007-07-15.
```

Free text is `Text(Box<str>)`, checked nonempty and trimmed. An abstract, a summary or eligibility criteria stay text. Display text, such as a formatted date, never lives in a value.

Coordinates carry their assembly and counting rule in the type: `Position1(NonZeroU32)`, `Span0 { start: u32, end: u32 }` and `GenomicInterval { build: Build, contig: Contig, span: Span0 }`. `Build` has an explicit `Unstated` case for sources that do not say. Nothing defaults to one build. Variant parsing has one implementation, and the variant slice decides whether it lives here or in a shared parser crate.

### 3.3 Entities, headers and cards

Each entity has two levels, so progressive disclosure is a type fact.

- A **header** is what a search returns: the identifiers and the few fields an agent needs to choose its next step.
- The **entity** is the full record a get returns. It contains its header.
- A **card** is the get response. It holds the entity, the requested sections and the names of the sections left out.

```rust
/// Enough to pick a gene and chain the next command.
pub struct GeneHeader {
    pub symbol: GeneSymbol,
    pub hgnc: Option<HgncId>,
    pub entrez: Option<EntrezId>,
    pub name: Option<Text>,
    pub gene_type: Option<GeneType>,
}

/// One human gene as its primary source describes it.
pub struct Gene {
    #[serde(flatten)]
    pub header: GeneHeader,
    pub aliases: Box<[GeneSymbol]>,
    pub summary: Option<Text>,
    pub location: Option<GenomicInterval>,
    pub ensembl: Option<EnsemblGeneId>,
    pub uniprot: Box<[UniProtAccession]>,
}

/// The get gene response.
pub struct GeneCard {
    pub gene: Gene,
    pub sources: Box<[Source]>,          // where the core fields came from
    pub sections: GeneSections,          // requested sections only
    pub more: Box<[GeneSection]>,        // sections that exist and were not requested
}

#[derive(Default)]
pub struct GeneSections {
    pub civic: Option<Section<CivicGeneSummary>>,
    pub pathways: Option<Section<Box<[PathwayRef]>>>,
    // one field per GeneSection variant; None means not requested
}

#[non_exhaustive]
pub enum GeneSection { Civic, Pathways, /* the gene slice names the rest */ }
```

Every field skips serialization when it is `None` or empty. A header and an entity are biodata core types. A card and its sections are BioMCP response types, so sections stay out of entity types, as rule 15 of `planning/rust-standards.md` says.

### 3.4 Sections and their four states

A requested section is always present in the card and always in one of four states. One generic type holds the state and the data together, so a section can never carry data and an error at once.

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum Section<T> {
    Data { source: Source, data: T },
    Empty { source: Source },                     // the source answered and had nothing
    Unavailable { source: Source, reason: Reason },
    NotApplicable,                                // the section cannot apply to this record
}
```

**Overrides** the earlier envelope, which kept section states in a top-level `sections` map beside a `result.sections` map. One object per section now carries both. The card's `more` list replaces any listing of unrequested sections, so 0.9's sixteen `not_requested` entries become one short list of names.

### 3.5 Provenance and problems

Source identity is one enum. Adding a source adds one variant.

```rust
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "mygene")] MyGene,
    #[serde(rename = "pubmed")] PubMed,
    #[serde(rename = "europepmc")] EuropePmc,
    #[serde(rename = "pubtator")] PubTator,
    #[serde(rename = "ctgov")] ClinicalTrialsGov,
    #[serde(rename = "civic")] Civic,
    #[serde(rename = "ema")] Ema,
    // one variant per source; each carries the short name 0.9 users already know
}
```

Provenance appears at three levels, each as small as its question needs:

- A card lists the sources of its core fields in `sources`.
- Each section names its one source in its state.
- A pooled search item lists every source that found it, with the native rank:

```rust
pub struct Found { pub source: Source, pub rank: NonZeroU32 }
pub struct ArticleItem { pub header: ArticleHeader, pub found_by: Box<[Found]> }
```

Field-level provenance does not exist, because no field is ever merged from two sources (section 6.4).

A problem is a typed note about one field of one record. It lets a record survive one bad field.

```rust
pub struct Problem {
    pub source: Source,
    pub record: Option<Box<str>>,     // the record's canonical ID text, when known
    pub field: &'static str,          // the biodata field name, such as "pmid"
    pub code: ProblemCode,            // such as id.zero, date.invalid, value.unknown
}
```

`field` serializes as text and deserializes into a checked field name. A required field that fails drops the record and leaves a problem. An optional field that fails leaves the field absent and a problem.

### 3.6 Queries and the request type

Every command is one query type. Its doc comments become the CLI help, the MCP description and the reference docs, through `schemars`. Two extension keys carry what a schema cannot say about argument spelling: `x-positional` for a positional argument's index and `x-short` for a one-letter flag.

```rust
/// Search human genes by symbol, name or alias.
#[derive(Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct GeneSearch {
    /// Free text matched against symbols, names and aliases.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    pub query: Option<Text>,
    /// Return headers for these exact genes.
    pub id: Box<[GeneId]>,
    #[serde(flatten)]
    pub page: Page,
}

/// The paging window every search shares.
pub struct Page { pub limit: Limit, pub offset: u32 }    // Limit is 1 to 50, default 10

/// One command. The tag is the CLI path joined by underscores.
#[non_exhaustive]
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Request {
    SearchGene(GeneSearch),
    GetGene(GeneGet),
    SearchArticle(ArticleSearch),
    GetArticle(ArticleGet),
    ArticleCitations(ArticleEdges),
    SearchTrial(TrialSearch),
    GetTrial(TrialGet),
    TrialLocations(TrialLocations),
    // one variant per command; housekeeping commands are not requests
}
```

The CLI path maps to the tag mechanically: `search gene` is `search_gene` and `trial locations` is `trial_locations`. A next step is a `Request` value, so the services never build command text.

### 3.7 The envelope

Every command returns one envelope. Its body type changes by command.

```rust
pub struct Envelope<T> {
    pub schema: SchemaId,                  // such as biomcp.gene_card/1
    pub status: Status,                    // complete, partial or failed
    pub query: Request,                    // the request as parsed and resolved
    pub result: Option<T>,
    pub page: Option<PageInfo>,            // searches only
    pub sources: Box<[SourceOutcome]>,     // one per source asked
    pub problems: Box<[Problem]>,
    pub next: Box<[Step]>,                 // at most five
    pub explain: Option<Explain>,          // only with --explain
}

pub struct SchemaId { pub name: &'static str, pub major: u16 }   // text form name/major
pub enum Status { Complete, Partial, Failed }
pub struct PageInfo { pub offset: u32, pub returned: u32, pub total: Option<u64>, pub pool: Option<PoolInfo> }
pub struct SourceOutcome { pub source: Source, pub state: SourceState, pub reason: Option<Reason>, pub count: Option<u64> }
pub enum SourceState { Ok, Empty, Partial, Stale, Failed, Disabled }
pub struct Step { pub args: Box<[Box<str>]> }     // the argument list after the program name

#[non_exhaustive]
pub enum Reason { RateLimited, Unavailable, Rejected, BadReply, DeadlineExceeded, Offline, MissingKey, NotInBundle, ReplayMiss }

/// Every response type names its schema.
pub trait Body: Serialize { const SCHEMA: SchemaId; }
```

`Step` holds an argument list because the CLI and MCP print the same JSON. The binary spells each typed `Request` into a `Step` through the grammar (section 8.3). The view prints a `Step` as a quoted shell command.

### 3.8 Schemas and versions

- `schemars` derives every schema behind the `schema` feature. The schema dialect is JSON Schema 2020-12, which `schemars` 1.x emits by default.
- The generated files sit in `crates/biodata/schemas/`, one per response type, per query type and for `Request`. Names follow `biodata.<entity>/<major>` for core types and `biomcp.<response>/<major>` for responses.
- `crates/biodata/tests/schemas.rs` regenerates every schema and fails on any difference from the checked-in copy. Run with `UPDATE_SCHEMAS=1`, it writes the new copies instead.
- The same test runs a compatibility check. Every property path and type in the checked-in schema must still exist with the same type in the new one, unless the major version rose. Adding an optional field keeps the version. Removing, renaming or retyping a field raises it. The check is about a hundred lines and catches what a reviewer misses.
- The binary embeds the checked-in schema files with `include_str!`. `biomcp schema <name>` prints one, the grammar reads them, and the MCP tool list cites them. The checked-in files are therefore the one authority, and the drift test keeps them equal to the types.
- `crates/biodata/tests/roundtrip.rs` round-trips each case value through JSON, MessagePack, CBOR, YAML and TOML. Those format crates are test dependencies only.

## 4. The transport

`biomcp-fetch` sends every request, as `decisions/2026-10-23-one-transport.md` says.

### 4.1 Blocking I/O on threads

The transport uses blocking I/O through `ureq` with rustls, and the services fan out on scoped standard threads. No crate depends on an async runtime.

- A command makes at most a few dozen requests to at most a dozen sources. One thread per source leg costs nothing measurable at that scale.
- Blocking code is shorter. Deadlines are timeouts, rate waits are sleeps, and a leg is a plain function call. No future, no `Send` bound puzzle, no runtime handle threads through the layers.
- The dependency tree shrinks by the async runtime, the async HTTP stack and their transitive crates. Builds get faster and the binary smaller.

Rejected: `reqwest` with `tokio`. It is the heavier stack for no gain at this concurrency. `reqwest`'s blocking client also starts a hidden runtime. **Overrides** the earlier text that named `reqwest` and left blocking against async open. Section 14 asks Ian to confirm, because the one-transport decision names `reqwest`.

### 4.2 The API

```rust
/// One per process. Shared by reference across legs and threads.
pub struct Fetch { /* ureq agent, reply cache, mode, per-source limiters, secrets, clock */ }

pub struct FetchConfig {
    pub mode: Mode,
    pub cache_path: PathBuf,
    pub secrets: Secrets,
    pub max_body: u64,
    pub source_deadlines: BTreeMap<Source, Duration>,   // config overrides of each spec's default
}

pub enum Mode {
    Live { fresh: bool },              // fresh refetches and rewrites the cache
    Offline,                           // cache only; a miss fails the leg with reason offline
    Record { dir: PathBuf },           // live, and writes each exchange to dir
    Replay { dir: PathBuf },           // recordings only; a miss fails with reason replay_miss
}

impl Fetch {
    pub fn open(config: FetchConfig) -> Result<Self, FetchError>;
    pub fn deadline(&self, budget: Duration) -> Deadline;
    pub fn leg(&self, spec: &'static SourceSpec, deadline: Deadline) -> Leg<'_>;
    pub fn sync<R: DeserializeOwned>(&self, bundle: &'static Bundle<R>, deadline: Deadline) -> Result<BundleInfo, SourceError>;
}

/// One source's work inside one command. Used on one thread.
pub struct Leg<'a> { /* fetch, spec, deadline, trace */ }

impl Leg<'_> {
    pub fn json<T: DeserializeOwned>(&self, ep: &Endpoint, params: &[(&str, Arg<'_>)]) -> Result<T, SourceError>;
    pub fn xml<T: DeserializeOwned>(&self, ep: &Endpoint, params: &[(&str, Arg<'_>)]) -> Result<T, SourceError>;
    pub fn text(&self, ep: &Endpoint, params: &[(&str, Arg<'_>)]) -> Result<Box<str>, SourceError>;
    pub fn post_json<B: Serialize, T: DeserializeOwned>(&self, ep: &Endpoint, body: &B) -> Result<T, SourceError>;
    pub fn rows<R: DeserializeOwned>(&self, bundle: &'static Bundle<R>, find: Find<'_>) -> Result<Vec<R>, SourceError>;
    pub fn finish(self) -> LegReport;      // state, requests, cache hits, stale flag, elapsed, trace
}

pub enum Arg<'a> { Text(&'a str), Num(u64) }   // From<&str>, From<u32>, From<u64>

pub trait Found<T> { fn found(self) -> Result<Option<T>, SourceError>; }   // NotFound becomes Ok(None)
```

A source and its endpoints are constants built with `const fn`:

```rust
pub struct SourceSpec {
    pub source: Source,
    pub base: &'static str,
    pub auth: Auth,
    pub rate: Rate,                    // without a key
    pub keyed_rate: Option<Rate>,      // with a key, when the source allows more
    pub in_flight: u8,                 // concurrent requests to this source
    pub deadline: Duration,            // default leg budget
}

pub enum Auth {
    None,
    Query { secret: SecretName, param: &'static str, required: bool },
    Header { secret: SecretName, name: &'static str, required: bool },
}

pub struct Rate { per_second: u16, burst: u16 }
pub enum Freshness { Never, Hours(u16), Days(u16) }
pub enum Method { Get, PostForm, PostJson }

pub struct Endpoint { pub spec: &'static SourceSpec, pub method: Method, pub path: &'static str, pub fresh: Freshness, pub idempotent: bool }

impl SourceSpec {
    pub const fn get(&'static self, path: &'static str) -> Endpoint;
    pub const fn post_form(&'static self, path: &'static str) -> Endpoint;
    pub const fn post_json(&'static self, path: &'static str) -> Endpoint;
}
impl Endpoint {
    pub const fn fresh(self, fresh: Freshness) -> Self;
}
```

### 4.3 The request path

Every call on a `Leg` runs the same steps in this order:

1. Build the URL from the base, the path and the parameters. Add the credential from `Secrets` when the spec names one. A required key that is missing fails the leg with `missing_key` before any send.
2. Compute the reply key: SHA-256 over the method, the URL without the credential parameter and the body. Parameter order is the adapter's order, which is fixed.
3. In replay mode, serve the recording for the key, or fail with `replay_miss`.
4. Look up the reply cache. A fresh reply returns at once. In offline mode, a stale reply returns marked stale, and a miss fails with `offline`.
5. Wait for the source's rate slot and an in-flight permit. Both waits are bounded by the leg's deadline.
6. Send with a timeout equal to the deadline's remaining time.
7. Retry as section 4.5 says.
8. Check the content type and read the body up to the size limit.
9. Store the reply in the cache with its fresh-until time. In record mode, also write the recording file.
10. Map the status into a `SourceError` or decode the body into the caller's raw type. A decode failure is `bad_reply`.

When steps 5 to 8 fail and the cache holds a stale reply for the key, the leg serves the stale reply and reports `stale` with the failure's reason. A source failure never becomes an empty success.

### 4.4 Rate limits and concurrency

- Each source has one limiter keyed by source and by whether a key is present. NCBI allows more requests with a key, so PubMed's spec has a `keyed_rate`. PubTator stays at the same rate either way, so its spec has none.
- A limiter holds the next free slot time under a `Mutex`. A caller takes the slot, releases the lock and sleeps outside it. `in_flight` caps concurrent requests to one source with a `Mutex` and `Condvar` permit count.
- Limits are per source, and each leg runs on its own thread. A slow or rate-limited source therefore delays only its own leg. No queue, permit or deadline is shared across sources.
- The limiter is per process. Several BioMCP processes sharing a key can exceed a source's limit together. The source then answers 429, and step 7 retries within the deadline. A cross-process limiter waits until a recorded failure shows the need.

### 4.5 Retries

- Retry on 429, 500, 502, 503, 504, a reset connection and a connect timeout, when the endpoint is idempotent. All GET endpoints are. A POST that only reads, such as ESummary, declares itself idempotent.
- At most three attempts. Backoff starts at 250 ms and doubles. A deterministic jitter from the reply key spreads retries without a random number crate.
- `Retry-After` in seconds or as an HTTP date replaces the backoff. When the wait would pass the deadline, the leg fails at once with `rate_limited` and the stated wait. One function in `retry.rs` parses the header.
- A 4xx other than 429 is `rejected` and never retried. A 404 is `NotFound`, which an adapter turns into `None` with `.found()`.

### 4.6 Deadlines and time

The command has one budget, 20 seconds by default and set in config. Each leg's deadline is the smaller of the command budget and the source's own budget from its spec or the config. A leg that runs out fails with `deadline_exceeded` and the others continue. The scope in section 6.3 ends when the last leg ends, and every leg ends by its deadline, so a command ends within its budget plus a small margin.

`Deadline` reads a `Clock` owned by `Fetch`. In live and record modes the clock is real time. In replay mode the clock is virtual: a recorded delay, a backoff or a `Retry-After` wait advances the leg's virtual time instead of sleeping. A recording with a 30-second delay therefore produces `deadline_exceeded` at once, and no case ever waits on a real clock.

```rust
pub struct Deadline { /* end instant or virtual budget */ }
impl Deadline {
    pub fn remaining(&self) -> Duration;
    pub fn expired(&self) -> bool;
    pub fn child(&self, cap: Duration) -> Deadline;     // the smaller of the two
}
```

### 4.7 The reply cache

The cache stores raw reply bodies keyed by reply key, in the `reply` table of the one SQLite file (section 7). Each endpoint declares how long its replies stay fresh. A literature summary may stay fresh for thirty days, and a trial's status for a day. `Freshness::Never` skips the cache, for the rare endpoint that must always be live.

`--fresh` sets `Mode::Live { fresh: true }`. The leg skips step 4 and rewrites the entry. Decoding runs on every read, so a raw type change never needs a cache migration.

### 4.8 Record, replay and offline

Offline, record and replay are user features, so no test hook exists in product code. `--offline` answers from the cache. `--record DIR` lets a user capture a failing command and attach the folder to a bug report. `--replay DIR` reruns it anywhere. The runner uses the same two flags.

A recording is one JSON file per exchange, at `<dir>/<source>/<key>.json`, so a diff shows exactly what changed:

```json
{
  "request": { "method": "GET", "url": "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=pubmed&term=AKT1+E17K&sort=relevance&retmode=json&retstart=0&retmax=10000" },
  "replies": [
    { "status": 429, "retry_after_s": 1, "delay_ms": 120 },
    { "status": 200, "content_type": "application/json", "delay_ms": 412, "body": "{\"esearchresult\":{...}}" }
  ]
}
```

- `replies` serves in order within one run, so a retry sequence is one file. A single reply is the common case.
- A failure case is an ordinary file: a status, a large `delay_ms` for a timeout, or a cut `body` for a broken reply.
- A binary body sits beside the file as `<key>.bin`.
- Recorded URLs never hold a credential, because step 2 removes it before the key and the file exist.
- A recorded bundle holds a slice of the real published file with the rows its cases need.

### 4.9 Credentials

`Secrets` maps a `SecretName` enum to a `Secret(Box<str>)` whose `Debug` and `Display` print `***`. Only the binary's config module builds `Secrets`, from the config file and the documented environment variables. The secret enters only the outgoing request. It never enters a cache key, a recording, a trace, an error or the envelope.

### 4.10 Bundles

A bundle is a published file that BioMCP downloads and queries locally, as `decisions/2026-10-23-data-bundles-are-local-apis.md` says. The transport owns its download, its revalidation and its rows.

```rust
pub struct Bundle<R: 'static> {
    pub spec: &'static SourceSpec,
    pub name: &'static str,                      // the name `biomcp sync` takes
    pub path: &'static str,                      // the published file under the spec's base
    pub format: RowFormat,                       // JsonArray { pointer }, JsonLines, Tsv, Csv
    pub fresh: Freshness,
    pub keys: fn(&R) -> Keys,                    // pure, from the adapter's map.rs
}

pub struct Keys(Vec<(KeyKind, Box<str>)>);       // normalized lookup keys for one row
pub enum KeyKind { Id, Name, Synonym }
pub enum Find<'a> { Id(&'a str), Name(&'a str), Prefix(&'a str) }
```

`Fetch::sync` downloads the file with an ETag or date check, splits it into rows, runs `keys` on each row and stores each row's original JSON text with its keys in one transaction. `Leg::rows` looks rows up by key, deserializes them into `R` and returns them. The adapter maps them like any reply. A query therefore reads a few indexed rows and never parses the whole file.

When a bundle is absent, the first query syncs it under the bundle's own deadline and reports the sync in its source outcome. Offline mode never syncs. A stale bundle answers with `stale` and a `sync` next step. `biomcp sync <name>` refreshes one bundle on request.

### 4.11 Errors

```rust
#[derive(Debug, thiserror::Error)]
#[error("{source}: {reason}")]
pub struct SourceError { pub source: Source, pub reason: Reason, pub status: Option<u16>, pub retry_after: Option<Duration> }

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum FetchError {                 // opening the transport only
    #[error("fetch.cache_open")] CacheOpen(rusqlite::Error),
    #[error("fetch.tls")] Tls,
    #[error("fetch.recordings")] Recordings(std::io::Error),
}
```

`SourceError` reuses `biodata::Reason`, so a failure travels from the transport into the envelope without a second enum.

## 5. Source adapters

### 5.1 The shape

An adapter is one folder with three files:

- `mod.rs` holds the `SourceSpec` constant, the endpoint constants and one short public function per question the source answers. Each function builds parameters, makes one or two `Leg` calls and hands the raw reply to `map`.
- `raw.rs` holds serde types that mirror only the fields the mapping reads.
- `map.rs` holds pure functions from raw types to biodata values. Each failed check becomes a `Problem`.

Shared helpers live once in `biomcp-adapters/src/`:

```rust
// raw.rs: shapes many providers share
#[derive(Deserialize)] #[serde(untagged)] pub(crate) enum OneOrMany<T> { One(T), Many(Vec<T>) }
#[derive(Deserialize)] #[serde(untagged)] pub(crate) enum NumOrText { Num(u64), Text(String) }

// mapped.rs: a value with the problems found while building it
pub struct Mapped<T> { pub value: T, pub problems: Vec<Problem> }
pub struct Problems { source: Source, record: Option<Box<str>>, list: Vec<Problem> }
impl Problems {
    pub(crate) fn required<T, E: Into<ProblemCode>>(&mut self, field: &'static str, r: Result<T, E>) -> Option<T>;
    pub(crate) fn optional<T, E: Into<ProblemCode>>(&mut self, field: &'static str, r: Option<Result<T, E>>) -> Option<T>;
}

// one page from one source, before any merge
pub struct SourcePage<T> { pub items: Vec<Mapped<T>>, pub total: Option<u64> }
```

`SourcePage` and `Mapped` stay in the adapters crate, because no output ever shows them.

### 5.2 Worked example: MyGene.info

```rust
// crates/biomcp-adapters/src/mygene/mod.rs
//! MyGene.info: human gene identity, names, aliases and location.

use biodata::{GeneHeader, Gene, GeneSymbol, Source, query::GeneSearch};
use biomcp_fetch::{Auth, Endpoint, Found, Freshness, Leg, Rate, SourceError, SourceSpec};
use std::time::Duration;

pub(crate) static SPEC: SourceSpec = SourceSpec {
    source: Source::MyGene,
    base: "https://mygene.info/v3/",
    auth: Auth::None,
    rate: Rate::per_second(10),
    keyed_rate: None,
    in_flight: 4,
    deadline: Duration::from_secs(8),
};

const QUERY: Endpoint = SPEC.get("query").fresh(Freshness::Days(7));

const HEADER_FIELDS: &str = "symbol,name,entrezgene,HGNC,type_of_gene";
const GENE_FIELDS: &str = "symbol,name,entrezgene,HGNC,type_of_gene,alias,summary,genomic_pos,ensembl.gene,uniprot";

/// Search genes by free text.
pub fn search(leg: &Leg<'_>, q: &GeneSearch) -> Result<SourcePage<GeneHeader>, SourceError> {
    let text = q.query.as_ref().map_or("", |t| t.as_str());
    let raw: raw::Query = leg.json(&QUERY, &[
        ("q", text.into()),
        // Quirk: without species MyGene returns every organism. Case: gene-search/human-only.yaml
        ("species", "human".into()),
        ("fields", HEADER_FIELDS.into()),
        ("size", q.page.limit.get().into()),
        // Quirk: from plus size may not pass 10,000. Case: gene-search/deep-offset.yaml
        ("from", q.page.offset.into()),
    ])?;
    Ok(map::header_page(raw))
}

/// Get one gene by its official symbol.
pub fn get(leg: &Leg<'_>, symbol: &GeneSymbol) -> Result<Option<Mapped<Gene>>, SourceError> {
    let q = format!("symbol:\"{symbol}\"");
    let raw: Option<raw::Query> = leg.json(&QUERY, &[
        ("q", q.as_str().into()),
        ("species", "human".into()),
        ("fields", GENE_FIELDS.into()),
        ("size", 1u32.into()),
    ]).found()?;
    Ok(raw.and_then(|r| r.hits.into_iter().next()).and_then(map::gene))
}
```

The deep-offset limit is enforced once in the query type's `Page` checks for this source, so the adapter never sends an offset MyGene refuses.

```rust
// crates/biomcp-adapters/src/mygene/raw.rs
use crate::raw::{NumOrText, OneOrMany};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Query { pub total: Option<u64>, #[serde(default)] pub hits: Vec<Hit> }

#[derive(Deserialize)]
pub(super) struct Hit {
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub entrezgene: Option<NumOrText>,            // Quirk: number or string
    #[serde(rename = "HGNC")]
    pub hgnc: Option<OneOrMany<NumOrText>>,       // Quirk: scalar or list
    pub type_of_gene: Option<String>,
    pub alias: Option<OneOrMany<String>>,         // Quirk: string or list
    pub summary: Option<String>,
    pub genomic_pos: Option<OneOrMany<GenomicPos>>,
    pub ensembl: Option<OneOrMany<Ensembl>>,
    pub uniprot: Option<Uniprot>,
}

#[derive(Deserialize)]
pub(super) struct GenomicPos { pub chr: String, pub start: u32, pub end: u32 }
#[derive(Deserialize)]
pub(super) struct Ensembl { pub gene: String }
#[derive(Deserialize)]
pub(super) struct Uniprot { #[serde(rename = "Swiss-Prot")] pub swiss_prot: Option<OneOrMany<String>> }
```

```rust
// crates/biomcp-adapters/src/mygene/map.rs
use super::raw;
use crate::mapped::{Mapped, Problems};
use biodata::{Gene, GeneHeader, Source};

pub(super) fn header_page(raw: raw::Query) -> SourcePage<GeneHeader> {
    SourcePage { total: raw.total, items: raw.hits.into_iter().filter_map(header).collect() }
}

pub(super) fn header(hit: raw::Hit) -> Option<Mapped<GeneHeader>>;
pub(super) fn gene(hit: raw::Hit) -> Option<Mapped<Gene>>;

// The body of header(), in full, to show the size:
//     let mut p = Problems::new(Source::MyGene);
//     let symbol = p.required("symbol", hit.symbol.as_deref().map_or(Err(IdError::Empty), str::parse))?;
//     p.record(&symbol);
//     let value = GeneHeader {
//         hgnc: p.optional("hgnc", hit.hgnc.and_then(first).map(hgnc_id)),
//         entrez: p.optional("entrez", hit.entrezgene.map(entrez_id)),
//         name: hit.name.and_then(Text::new),
//         gene_type: p.optional("gene_type", hit.type_of_gene.as_deref().map(gene_type)),
//         symbol,
//     };
//     Some(p.finish(value))
//
// gene() reuses header() and adds aliases, summary, location, Ensembl and Swiss-Prot.
// Quirk: genomic_pos is a list when a gene has several placements; the first is primary.
// The gene slice confirms the assembly MyGene states for genomic_pos.
// gene_type() is the one table from MyGene's type_of_gene spellings to GeneType.
```

The whole adapter is about 150 lines. 0.9's MyGene adapter is 888 lines, most of it client setup, request plans, decoders and inline tests that the transport and the cases replace.

### 5.3 Worked example: PubMed ESearch and ESummary

PubMed answers a search in two steps. ESearch returns PMIDs in relevance order, and ESummary returns a header for each PMID.

```rust
// crates/biomcp-adapters/src/pubmed/mod.rs
//! PubMed through NCBI E-utilities: relevance-ordered PMIDs and article headers.

pub(crate) static SPEC: SourceSpec = SourceSpec {
    source: Source::PubMed,
    base: "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/",
    auth: Auth::Query { secret: SecretName::NcbiApiKey, param: "api_key", required: false },
    rate: Rate::per_second(3),
    keyed_rate: Some(Rate::per_second(10)),     // NCBI allows more with a key
    in_flight: 3,
    deadline: Duration::from_secs(10),
};

const ESEARCH: Endpoint = SPEC.get("esearch.fcgi").fresh(Freshness::Hours(24));
const ESUMMARY: Endpoint = SPEC.post_form("esummary.fcgi").fresh(Freshness::Days(30));

/// ESearch returns at most the first 10,000 records of any search.
pub const MAX_WINDOW: u32 = 10_000;
const SUMMARY_BATCH: usize = 200;

/// PMIDs for a PubMed term, in PubMed's relevance order.
pub fn search_ids(leg: &Leg<'_>, term: &PubMedTerm, window: Window) -> Result<SourcePage<Pmid>, SourceError> {
    let raw: raw::ESearch = leg.json(&ESEARCH, &[
        ("db", "pubmed".into()),
        ("term", term.as_str().into()),
        // Quirk: without sort=relevance NCBI returns newest first. Case: article-search/akt1-e17k-floor.yaml
        ("sort", "relevance".into()),
        ("retmode", "json".into()),
        ("retstart", window.offset.into()),
        ("retmax", window.size.into()),
    ])?;
    Ok(map::ids(raw))
}

/// Headers for PMIDs, in the order given. Missing PMIDs become problems.
pub fn summaries(leg: &Leg<'_>, ids: &[Pmid]) -> Result<Vec<Mapped<ArticleHeader>>, SourceError> {
    let mut out = Vec::with_capacity(ids.len());
    for batch in ids.chunks(SUMMARY_BATCH) {
        let joined = join_ids(batch);
        let raw: raw::ESummary = leg.json(&ESUMMARY, &[
            ("db", "pubmed".into()),
            ("retmode", "json".into()),
            ("id", joined.as_str().into()),
        ])?;
        out.extend(map::summaries(raw, batch));
    }
    Ok(out)
}
```

`PubMedTerm` is built in `map.rs` from the typed article query, so the term grammar has one owner. Paging is the service's choice: the pool build asks for one window of up to `MAX_WINDOW` PMIDs in one request, because a PMID list is small.

```rust
// crates/biomcp-adapters/src/pubmed/raw.rs
#[derive(Deserialize)]
pub(super) struct ESearch { pub esearchresult: ESearchResult }
#[derive(Deserialize)]
pub(super) struct ESearchResult { pub count: NumOrText, #[serde(default)] pub idlist: Vec<String> }

/// Quirk: ESummary keys each record by its PMID and lists the order in `uids`.
#[derive(Deserialize)]
pub(super) struct ESummary { pub result: SummaryResult }
#[derive(Deserialize)]
pub(super) struct SummaryResult {
    #[serde(default)] pub uids: Vec<String>,
    #[serde(flatten)] pub docs: HashMap<String, DocSum>,
}
#[derive(Deserialize)]
pub(super) struct DocSum {
    pub title: Option<String>,
    pub fulljournalname: Option<String>,
    pub sortpubdate: Option<String>,              // Quirk: "2007/07/05 00:00"
    #[serde(default)] pub authors: Vec<Author>,
    #[serde(default)] pub pubtype: Vec<String>,
    #[serde(default)] pub articleids: Vec<ArticleIdRow>,   // carries doi and pmc
}
```

```rust
// crates/biomcp-adapters/src/pubmed/map.rs
pub(super) fn ids(raw: raw::ESearch) -> SourcePage<Pmid>;           // native rank is list position
pub(super) fn summaries(raw: raw::ESummary, asked: &[Pmid]) -> Vec<Mapped<ArticleHeader>>;
pub(super) fn term(q: &ArticleQuery) -> PubMedTerm;                // the one term grammar
fn pub_date(text: &str) -> Result<PartialDate, DateError>;
fn pub_type(text: &str) -> Result<PublicationType, ProblemCode>;    // PubMed spellings to the enum
```

### 5.4 Worked example: a local data bundle

EMA publishes its medicine list as a JSON report. The adapter declares the bundle and maps its rows.

```rust
// crates/biomcp-adapters/src/ema/mod.rs
//! EMA medicines: European authorisation status, read from EMA's published JSON report.

pub(crate) static SPEC: SourceSpec = SourceSpec {
    source: Source::Ema,
    base: "https://www.ema.europa.eu/en/documents/report/",
    auth: Auth::None,
    rate: Rate::per_second(1),
    keyed_rate: None,
    in_flight: 1,
    deadline: Duration::from_secs(60),          // the sync budget; lookups are local
};

pub(crate) static MEDICINES: Bundle<raw::Medicine> = Bundle {
    spec: &SPEC,
    name: "ema",
    path: "medicines-output-medicines_json-report_en.json",
    format: RowFormat::JsonArray { pointer: "/data" },
    fresh: Freshness::Hours(72),
    keys: map::medicine_keys,
};

/// EMA authorisations whose medicine name or active substance matches.
pub fn approvals(leg: &Leg<'_>, name: &DrugName) -> Result<Vec<Mapped<Approval>>, SourceError> {
    let rows = leg.rows(&MEDICINES, Find::Name(name.normalized()))?;
    Ok(rows.into_iter().filter_map(map::approval).collect())
}
```

`raw::Medicine` mirrors the report's row fields the mapping reads. `map::medicine_keys` returns the normalized medicine name and each active substance. `map::approval` maps a row into `biodata::Approval` with region `Eu`. The JSON pointer and the feed name above follow 0.9's EMA adapter, and the drug slice verifies both against the live file. 0.9 reads six EMA feeds. The drug slice keeps only the feeds its examples need, and each kept feed is one more `Bundle` constant.

### 5.5 How adapters are tested

Adapters have no tests of their own. Cases drive the binary over recordings, and each adapter's lines run because a case needs them.

- Each adapter has at least one success case per public function, recorded live with `--record`.
- Each adapter has four failure cases built from small hand-edited recordings: a 429 with `Retry-After`, a 500, a delay past the source deadline and a cut body. Each asserts `status: partial` and the source's outcome and reason.
- Each quirk comment names the case that fails without it. The ratchet test checks that the case file exists.
- A nightly live run replays every case live and reports each reply whose key fields changed. A person refreshes recordings on purpose with `--record`. A test run never writes a recording.

### 5.6 What an adapter may never do

- Build a client, open a socket, sleep, retry or read the clock. The transport does each.
- Read the environment, a file or the config.
- Hold or share state across calls.
- Name `ureq`, `rusqlite` or `serde_json`, or keep a `serde_json::Value` field. Raw types name the fields they read.
- Call another adapter, the store or a service. Chaining across sources belongs to the services.
- Merge, rank, page across sources or decide a status. It returns what its source said.
- Build an error string, a command string or display text.
- Interpret clinical meaning.

### 5.7 Size

| Kind | Lines each |
| --- | --- |
| Simple JSON or XML source | 100 to 250 |
| Multi-step, paged or GraphQL source | 250 to 500 |
| Bundle source | 150 to 400 |
| The transport | 1,500 to 2,500 |

These are estimates. The article slice measures its adapters first. If they average over 300 lines, this section is revised. Each file in an adapter folder stays under 300 lines.

## 6. The query layer

`biomcp-query` turns a `Request` into an outcome. Every interface calls one function.

### 6.1 Context and the one entry point

```rust
/// What a command needs. Built by the binary from config.
pub struct Ctx<'a> {
    pub fetch: &'a Fetch,
    pub store: &'a Store,
    pub budget: Duration,
    pub disabled: &'a BTreeSet<Source>,
    pub explain: bool,
}

/// A response before the interface spells its next steps.
pub struct Outcome<T> {
    pub status: Status,
    pub result: Option<T>,
    pub page: Option<PageInfo>,
    pub sources: Vec<SourceOutcome>,
    pub problems: Vec<Problem>,
    pub next: Vec<Request>,
    pub explain: Option<Explain>,
}

/// Run one request. Source failures become statuses. Errors are input or store failures.
pub fn run(cx: &Ctx<'_>, req: &Request) -> Result<Outcome<Body>, QueryError>;

#[derive(Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Body { GenePage(GenePage), GeneCard(GeneCard), ArticlePage(ArticlePage), ArticleCard(ArticleCard), TrialPage(TrialPage), TrialCard(TrialCard), TrialLocations(TrialLocationPage) }
```

`run` is one `match` from each `Request` variant to its entity function, such as `gene::search(cx, q)`. That match is the routing table, and it stays one line per command. `Body` lives in `biodata::response` beside the types it wraps.

### 6.2 Resolve

Each entity function first turns loose input into canonical identifiers. `get gene PD-L1` resolves the alias to `CD274` through the gene source, and the envelope's `query` shows the resolved form. Resolution uses the same adapters and the same cache. A resolution that matches several records returns `Failed` with a `next` search for the ambiguous text. Resolution never guesses.

### 6.3 Fan-out

```rust
/// One source's work, ready to run on its own thread.
pub(crate) struct Job<'a, T> {
    pub spec: &'static SourceSpec,
    pub run: Box<dyn FnOnce(&Leg<'_>) -> Result<T, SourceError> + Send + 'a>,
}

/// Run each job on its own scoped thread with its own deadline. Returns in job order.
pub(crate) fn fan_out<'a, T: Send>(cx: &Ctx<'a>, jobs: Vec<Job<'a, T>>) -> Vec<LegResult<T>>;

pub(crate) struct LegResult<T> { pub source: Source, pub result: Result<T, SourceError>, pub report: LegReport }
```

- A disabled source returns `Disabled` without a thread.
- Each thread opens its `Leg` with `cx.fetch.deadline(cx.budget).child(spec.deadline)`.
- A panicking leg becomes a failed leg with reason `unavailable` and an `explain` note, so one bad source cannot take down the command.
- A job with one source runs on the calling thread. Fan-out costs nothing for a single-source get.

Rejected: a worker pool, a channel-based scheduler and an async executor. A dozen scoped threads per command need none of them.

### 6.4 Merge

Merge is a pure union. Records are keyed by their canonical identifier. Equal keys merge and distinct keys never do, even when two records share a DOI. Each merged item keeps every source that found it in `found_by`.

```rust
pub(crate) fn union<K: Ord + Clone, T>(legs: Vec<(Source, SourcePage<T>)>, key: impl Fn(&T) -> K) -> Vec<Merged<K, T>>;
pub(crate) struct Merged<K, T> { pub key: K, pub value: T, pub found_by: Vec<Found> }
```

Field values are never merged. For each entity the service declares one primary source for headers, such as PubMed ESummary for any item with a PMID. An item the primary source cannot describe keeps the header from the first source that found it, in the order the service lists its sources. Fields from secondary sources reach the agent as sections, each under its own source. Two sources that disagree therefore both show, and BioMCP never picks a winner. Rejected: per-field merge rules with conflict records. They add a rule per field and per source pair and an output shape agents would need to learn.

### 6.5 Rank

Ranking is a separate pure stage over features the pool already holds.

```rust
pub(crate) struct Features { pub in_title: bool, pub in_abstract: bool, pub curated: bool, pub families: u8, pub best_rank: u32, pub year: Option<u16> }
pub(crate) fn rank(items: &mut [PoolItem], q: &RankQuery);   // stable lexicographic sort, then best native rank, then ID
```

One ranker serves every article command. The article slice chooses the features and their order against its question kernel. The ranker's tie-break table is one of the listed unit test exceptions. Other entities use the primary source's order until a slice shows a need for more.

### 6.6 Pools and paging

A search whose answer comes from several sources builds a pool once and pages over it.

1. **Key.** The pool key is SHA-256 over the schema version, the normalized request without its page window and the set of enabled sources.
2. **Hit.** A fresh stored pool serves the page from the store. No request goes out.
3. **Build.** Otherwise, fan out to every source for IDs and native ranks, union them, fetch headers for the pool in batches from the primary source, compute features, rank, and store the pool with each item's header and features and each source's outcome.
4. **Page.** Slice the stored order by `offset` and `limit`. The page carries the pool's ID, size, creation time and the stored source outcomes.

Paging needs no token. The same request with a new `--offset` computes the same key and reads the same pool, so two walks give the same order without gaps or duplicates. A pool stays fresh for a time its entity declares. A pool built while a source failed stays partial until it expires or the agent passes `--fresh`, and its `next` includes the `--fresh` rerun. **Overrides** the earlier continuation token, which carried the same pool key and offset that the request already carries.

A pool holds every item's header, so a warm page is a store read. A pool's size is capped per entity, and a source that could return more reports `partial` with its total.

### 6.7 Get and sections

`get` fetches the entity from its primary source, then fans out one job per requested section. Each section job returns its `Section<T>` state. A section is `NotApplicable` when the record's type rules it out, such as a gene section that needs a protein the gene lacks. The card's `more` lists the sections the caller did not request.

A part of one record is a section when it fits in one reply. It becomes an `<entity> <relation> <id>` drill-down when it pages, such as a trial's locations. One rule decides every entity.

### 6.8 Links across entities

A link from one entity to another is a search on the target entity with a filter taken from the source entity. The gene card's next steps are searches:

```rust
pub(crate) fn gene_next(card: &GeneCard) -> Vec<Request> {
    let symbol = card.gene.header.symbol.clone();
    vec![
        Request::SearchTrial(TrialSearch { gene: Some(symbol.clone()), ..Default::default() }),
        Request::SearchArticle(ArticleSearch { gene: Some(symbol.clone()), ..Default::default() }),
        Request::SearchDrug(DrugSearch { target: Some(symbol), ..Default::default() }),
    ]
}
```

A drug card's next steps search trials and articles by that drug. A trial card's next steps get its conditions' diseases and its interventions' drugs. An article card's next steps follow its citations, its references and its annotated genes. The chain from gene to drug to trial to article is therefore a walk over typed next steps, each with exact identifiers.

**Overrides** the earlier `gene trials BRAF` style commands. A relation that equals a filter on the target's search is a next step, so `search trial --gene BRAF` is the one spelling. Drill-down commands remain for a record's paged parts, such as `trial locations`, and for graph edges no search can express, such as `article citations`. This follows "one spelling per idea" in `decisions/2026-10-23-the-user-is-an-agent.md`, and section 14 asks Ian to confirm it.

The store's `link` table records these edges once the wiki needs them (section 7.3).

### 6.9 Next steps

Each response type has one pure `next` function in its entity's `next.rs`. It returns typed requests, at most five, ordered by what the slice's investigation found agents need first. Searches include the next page when more remain. Partial results include the `--fresh` rerun. Errors include the search that would find the input. The binary spells each request into a `Step`.

### 6.10 Status

```rust
pub(crate) fn status(sources: &[SourceOutcome], has_result: bool, sections_unavailable: bool) -> Status;
```

- `Complete`: every asked source answered from live or fresh cache, and no requested section is unavailable.
- `Partial`: a result exists, and some source failed, came from a stale cache, or reported more than the pool holds, or a requested section is unavailable.
- `Failed`: no result exists and at least one source failed. A confirmed absence is not a failure (section 8.5).

## 7. The store

### 7.1 One file, two owners

The cache is one SQLite file at `$XDG_CACHE_HOME/biomcp/biomcp.sqlite`, as `decisions/2026-10-23-the-cache-is-a-local-sqlite-database.md` says. The transport owns the reply and bundle tables. The store module of `biomcp-query` owns the rest. Both open the file in WAL mode with a busy timeout, so several BioMCP processes can share it.

The file is a cache. Every row can be rebuilt from the sources. Each owner records its schema version in `meta`, and on a mismatch it drops and recreates its own tables. No migration code exists.

```sql
-- owner: biomcp-fetch
CREATE TABLE IF NOT EXISTS meta (owner TEXT PRIMARY KEY, version INTEGER NOT NULL);

CREATE TABLE IF NOT EXISTS reply (
  key          BLOB PRIMARY KEY,        -- SHA-256 of method, URL without credential, body
  source       TEXT NOT NULL,
  url          TEXT NOT NULL,           -- without credential
  status       INTEGER NOT NULL,
  content_type TEXT,
  body         BLOB NOT NULL,
  fetched_at   INTEGER NOT NULL,        -- Unix seconds
  fresh_until  INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS reply_fresh ON reply (fresh_until);

CREATE TABLE IF NOT EXISTS bundle (
  name TEXT PRIMARY KEY, source TEXT NOT NULL, etag TEXT, modified TEXT,
  rows INTEGER NOT NULL, fetched_at INTEGER NOT NULL, fresh_until INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS bundle_row (bundle TEXT NOT NULL, n INTEGER NOT NULL, row TEXT NOT NULL, PRIMARY KEY (bundle, n));
CREATE TABLE IF NOT EXISTS bundle_key (bundle TEXT NOT NULL, kind TEXT NOT NULL, key TEXT NOT NULL, n INTEGER NOT NULL);
CREATE INDEX IF NOT EXISTS bundle_key_find ON bundle_key (bundle, kind, key);

-- owner: biomcp-query, store module
CREATE TABLE IF NOT EXISTS pool (
  key         BLOB PRIMARY KEY,         -- see section 6.6
  schema      TEXT NOT NULL,
  request     TEXT NOT NULL,            -- the normalized request as JSON
  sources     TEXT NOT NULL,            -- each source's outcome as JSON
  size        INTEGER NOT NULL,
  created_at  INTEGER NOT NULL,
  fresh_until INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS pool_item (
  pool     BLOB NOT NULL REFERENCES pool (key) ON DELETE CASCADE,
  rank     INTEGER NOT NULL,
  kind     TEXT NOT NULL,
  id       TEXT NOT NULL,
  header   TEXT NOT NULL,               -- the item's header and found_by as JSON
  features TEXT NOT NULL,
  PRIMARY KEY (pool, rank)
);

-- arrive with their first reader (section 7.3)
CREATE TABLE IF NOT EXISTS entity (
  kind TEXT NOT NULL, id TEXT NOT NULL, schema TEXT NOT NULL, value TEXT NOT NULL,
  sources TEXT NOT NULL, fetched_at INTEGER NOT NULL, fresh_until INTEGER NOT NULL,
  PRIMARY KEY (kind, id)
);
CREATE TABLE IF NOT EXISTS alias (kind TEXT NOT NULL, text TEXT NOT NULL, id TEXT NOT NULL, source TEXT NOT NULL, PRIMARY KEY (kind, text, id));
CREATE TABLE IF NOT EXISTS link (
  from_kind TEXT NOT NULL, from_id TEXT NOT NULL, relation TEXT NOT NULL,
  to_kind TEXT NOT NULL, to_id TEXT NOT NULL, source TEXT NOT NULL, fetched_at INTEGER NOT NULL,
  PRIMARY KEY (from_kind, from_id, relation, to_kind, to_id, source)
);
CREATE INDEX IF NOT EXISTS link_to ON link (to_kind, to_id);
```

```rust
pub struct Store { /* rusqlite::Connection */ }
impl Store {
    pub fn open(path: &Path) -> Result<Self, StoreError>;
    pub fn stats(&self) -> Result<StoreStats, StoreError>;
    pub fn clean(&self, now: Timestamp) -> Result<Cleaned, StoreError>;     // drops expired pools and replies past their grace
    pub(crate) fn pool(&self, key: &PoolKey, now: Timestamp) -> Result<Option<Pool>, StoreError>;
    pub(crate) fn put_pool(&self, pool: &Pool) -> Result<(), StoreError>;
}
```

The store needs no `Mutex`. Pool writes happen on the calling thread after the fan-out ends. The transport's cache uses one `Mutex<Connection>` because legs write from many threads.

### 7.2 Freshness

- A reply is fresh until its endpoint's freshness ends.
- A pool is fresh until its entity's pool freshness ends or any source's stored outcome says partial and the caller passed `--fresh`.
- An entity row, once it exists, is fresh until the earliest fresh-until time of the replies it came from.
- A stale value is served only when its source fails, and then only with `stale` in its source outcome. Offline mode serves stale values the same way.
- `biomcp store clean` removes expired rows. A size cap in config removes the oldest replies first.

### 7.3 When each table arrives

- `reply`, `meta`: stage 3, the transport.
- `bundle`, `bundle_row`, `bundle_key`: the first slice that keeps a bundle.
- `pool`, `pool_item`: the article slice.
- `entity`, `alias`, `link`: the first slice or feature that reads them. Alias lookups may arrive early if a slice shows resolution latency matters. Otherwise the wiki brings all three.

Until a reader exists, no code writes these rows. Writing data nothing reads is code the cases cannot check.

### 7.4 The wiki later

`biomcp serve --http` will serve a browsable wiki over the store. Each stored entity gets a page at `/<kind>/<id>`, rendered from its JSON through the same generic view the Markdown output uses, in HTML. Links come from the `link` table, so a reader clicks from disease to gene to drug to article. Each page shows each fact's source and fetched time from the row. A "fetch more" button runs the page's next steps through the same `run` call and stores the results. When the wiki arrives, the services start writing `entity` and `link` rows at the end of each `run`, in the same transaction as any pool.

### 7.5 Postgres later

All SQL sits in two modules: `biomcp-fetch/src/cache.rs` and `bundle.rs`, and `biomcp-query/src/store/`. The SQL uses only forms both engines accept, such as `INSERT ... ON CONFLICT`. JSON stays text in SQLite and becomes `jsonb` in Postgres. A Postgres backend arrives when a shared deployment needs concurrent writers. Until then no store trait exists.

## 8. Interfaces

### 8.1 The CLI grammar

The grammar stays as `decisions/2026-10-23-the-user-is-an-agent.md` sets it: `search <entity>` for headers, `get <entity> <id>` for a card and `<entity> <relation> <id>` for drill-downs.

This design recommends that hybrid over a pure entity-first tree such as `biomcp gene search`:

- `search` and `get` are the two moves every question starts with. `biomcp search` and `biomcp get` each list every entity, which is the map an agent needs first.
- 0.9 users, 0.9 skills and models trained on 0.9 docs already use this tree. Keeping it costs nothing and saves every one of them a relearning step.
- Drill-downs are entity-specific, so `biomcp trial` listing its drill-downs reads naturally.
- Internally all three forms map to one `Request` tag, so the hybrid adds no code.

Rejected: entity-first everywhere. It gains one uniform shape and costs every 0.9 user and skill a change. Rejected: verb-first everywhere, such as `biomcp locations trial NCT...`, which reads poorly and hides drill-downs under verbs no agent would guess.

`get` takes one identifier followed by section names, as 0.9 does. A list of exact records goes through `search <entity> --id A --id B`, which returns their headers. **Overrides** the earlier "several IDs may be given at once" for `get`. A mix of several IDs and positional section names cannot parse without guessing.

### 8.2 The trees for gene, article and trial

```
biomcp search gene [QUERY] [--id ID]... [--limit N] [--offset N]
biomcp get gene <SYMBOL | HGNC:N | ENTREZ> [SECTION]...
    sections: the gene slice names them, such as civic, pathways, protein, expression

biomcp search article [QUERY] [--gene SYMBOL] [--variant TEXT] [--disease TEXT] [--drug TEXT]
                      [--author TEXT] [--since DATE] [--until DATE] [--type TYPE]
                      [--id PMID]... [--limit N] [--offset N]
biomcp get article <PMID | PMCID | DOI> [SECTION]...
    sections: such as abstract, annotations, fulltext, supplements
biomcp article citations <PMID> [--limit N] [--offset N]
biomcp article references <PMID> [--limit N] [--offset N]

biomcp search trial [QUERY] [--condition TEXT] [--gene SYMBOL] [--variant TEXT] [--drug TEXT]
                    [--status STATUS]... [--phase PHASE]... [--sponsor TEXT]
                    [--near LAT,LON] [--radius KM] [--id NCT]... [--limit N] [--offset N]
biomcp get trial <NCT> [SECTION]...
    sections: such as eligibility, arms, outcomes, contacts, references
biomcp trial locations <NCT> [--near LAT,LON] [--radius KM] [--limit N] [--offset N]

global: --format md  --fresh  --offline  --explain  --disable-sources LIST  --config PATH
        --record DIR  --replay DIR
housekeeping: help, schema, sync, store, serve, mcp-config, health, version, skill
```

Every flag above is a placeholder for what its slice names. Each slice keeps only the options its harvested examples need. Ranking weights, source pickers and session labels from 0.9 return only if an example needs them.

### 8.3 Parsing from the schemas

The binary builds the grammar at run time from the checked-in schemas, with clap's builder API.

```rust
// crates/biomcp/src/grammar.rs
/// Build the clap command for one CLI path from its request schema.
fn command_for(path: &[&str], schema: &Schema) -> clap::Command;

/// Parse an argument list into a request. The same function serves the CLI and MCP.
pub(crate) fn parse(args: &[String]) -> Result<Parsed, UsageError>;
pub(crate) enum Parsed { Request(Request, Globals), Housekeeping(Housekeeping, Globals), Help(HelpTopic) }

// crates/biomcp/src/spell.rs
/// Spell a request back into the argument list that parses to it.
pub(crate) fn spell(req: &Request) -> Step;
```

1. The first one or two words pick the request schema from the embedded set. Only that schema is read, so parsing stays well under a millisecond.
2. Each schema property becomes a clap argument. The property's type sets its parser: a string, an integer, a boolean flag, or a repeatable option for an array. An enum's names become the allowed values. `x-positional` makes it positional, `x-short` adds its letter, and the doc comment becomes its help.
3. clap's matches become a JSON object, and serde deserializes it into the `Request` variant. Deserialization runs the checked constructors, so a bad identifier fails at the edge with its field and code.
4. `spell` walks the same schema in reverse: positionals first, then flags in schema order, omitting defaults.

The CLI, the MCP input and the schemas therefore cannot drift, because one set of files feeds all three. clap still supplies help formatting, suggestions for misspelled flags and standard error forms.

Rejected: clap derive structs that mirror each query type, which duplicate every field. Rejected: clap derive inside `biodata`, which puts a CLI library in the data crate. Rejected: a hand-written argument parser, which would rebuild what clap's builder already does.

### 8.4 The JSON envelope

The CLI prints the envelope as compact JSON on standard output.

```json
{
  "schema": "biomcp.gene_card/1",
  "status": "partial",
  "query": { "command": "get_gene", "id": "BRAF", "sections": ["civic", "pathways"] },
  "result": {
    "gene": { "symbol": "BRAF", "hgnc": "HGNC:1097", "entrez": "673", "name": "B-Raf proto-oncogene, serine/threonine kinase", "gene_type": "protein_coding" },
    "sources": ["mygene"],
    "sections": {
      "civic": { "state": "unavailable", "source": "civic", "reason": "rate_limited" },
      "pathways": { "state": "data", "source": "reactome", "data": [ { "id": "R-HSA-5673001", "name": "RAF/MAP kinase cascade" } ] }
    },
    "more": ["protein", "expression"]
  },
  "sources": [
    { "source": "mygene", "state": "ok" },
    { "source": "civic", "state": "failed", "reason": "rate_limited" },
    { "source": "reactome", "state": "ok", "count": 1 }
  ],
  "next": [
    { "args": ["get", "gene", "BRAF", "civic", "--fresh"] },
    { "args": ["search", "trial", "--gene", "BRAF"] },
    { "args": ["search", "article", "--gene", "BRAF"] },
    { "args": ["search", "drug", "--target", "BRAF"] }
  ]
}
```

The values above are illustrative. Rules:

- Nulls, empty lists, empty objects, `false` flags and unrequested sections are omitted.
- Identifiers are canonical text.
- A header cuts long text to a length its type states and sets `truncated: true`. The get returns it whole.
- `query` echoes the resolved request, so the agent sees how BioMCP read its input.
- `next` holds at most five steps and never repeats one.
- Every output validates against its schema in every case run.

### 8.5 Errors and exit codes

An error also prints one envelope, with schema `biomcp.error/1`:

```json
{ "schema": "biomcp.error/1", "status": "failed", "error": { "code": "query.not_found", "message": "no human gene has this symbol" }, "next": [ { "args": ["search", "gene", "BRAFF"] } ] }
```

| Exit | Meaning |
| --- | --- |
| 0 | A result, complete or partial |
| 1 | Confirmed absent: the source answered that the record does not exist |
| 2 | Usage: the arguments do not parse |
| 4 | Failed: every source that could answer failed |
| 5 | Local: config, cache file, recordings, or a replay miss |

The binary maps each error family to an exit code in one function in `exit.rs`. Messages come from the library's error and never contain input text, although the `next` step may repeat the caller's own input as an argument. A replay miss exits 5 even when the envelope says partial, so a case with a missing recording always fails.

### 8.6 The Markdown view

`--format md` prints a Markdown view for people. One generic function walks the response JSON:

```rust
// crates/biomcp/src/view/mod.rs
pub(crate) fn markdown(envelope: &serde_json::Value, hints: &Hints) -> String;

// crates/biomcp/src/view/hints.rs: one entry per schema, data only
pub(crate) struct Hint { pub schema: &'static str, pub title: &'static str, pub columns: &'static [&'static str] }
```

A hint names the JSON pointer of a card's title and the columns of a page's table. Everything else renders by structure: objects as definition lists, arrays of objects as tables, sections under headings with their state. Next steps print as quoted shell commands. The view reads only the serialized JSON, so it cannot show anything the JSON lacks, and it cannot fetch. It replaces 0.9's 34 templates and 102 Markdown functions.

### 8.7 The MCP tool

MCP offers one tool, as `decisions/2026-10-23-mcp-is-one-tool-over-the-cli.md` says.

```json
{
  "name": "biomcp",
  "description": "Search and get biomedical records. Pass CLI arguments as a list. Start with [\"help\"] for the command tree, [\"search\", \"<entity>\", ...] for headers, [\"get\", \"<entity>\", \"<id>\", ...] for one card. Every response lists next steps as argument lists you can pass back.",
  "inputSchema": {
    "type": "object",
    "properties": { "args": { "type": "array", "items": { "type": "string" }, "description": "The argument list after the program name, such as [\"get\", \"gene\", \"BRAF\"]." } },
    "required": ["args"],
    "additionalProperties": false
  },
  "outputSchema": { "$comment": "the generic envelope schema; its schema field names the exact response schema" }
}
```

- A call parses `args` with the same `grammar::parse` and runs the same `run`. Global flags that make no sense in a server, such as `--config`, `--record` and `--replay`, are refused.
- The result carries the envelope as `structuredContent` and the same compact JSON as text content. MCP added structured tool output in its 2025-06-18 revision. Clients that predate it read the text.
- The server keeps one `Fetch` and one `Store` open across calls, so later calls reuse connections and the in-process limiter.

`biomcp serve` speaks MCP over standard input and output with a small hand-written JSON-RPC loop. It answers `initialize`, `notifications/initialized`, `ping`, `tools/list` and `tools/call`, and refuses every other method with a standard error. Each `tools/call` runs on its own thread, and responses go out through one locked writer. The loop is a few hundred lines.

Rejected: an MCP library. The available Rust MCP libraries pull in an async runtime, which section 4.1 avoids, and one tool needs five methods. **Overrides** the earlier plan to use "the MCP library". Section 14 asks Ian to confirm.

### 8.8 Help and discovery

Help is JSON by default, like every other output, and comes from the schemas. `biomcp help` lists the entities and the verbs that apply to each. `biomcp help get gene` lists the arguments, the sections and two examples drawn from cases. `biomcp help --all` lists every command and option, and the runner's surface check reads it. An agent can therefore learn the whole tree through the one tool with `["help"]`, and spend no context on parts it never uses.

### 8.9 The web mode later

`biomcp serve --http` will serve MCP's streamable HTTP transport at `/mcp` and the wiki at `/`, bound to 127.0.0.1 by default. A small blocking HTTP server fits the thread model and arrives with the wiki. Its handlers call the same `grammar::parse` and `run`.

## 9. Configuration, observability and performance

### 9.1 Configuration

`config.rs` is the only code that reads a file or the environment for settings, as `decisions/2026-10-23-settings-live-in-an-xdg-config-file.md` says.

```json
{
  "budget_ms": 20000,
  "format": "json",
  "disable_sources": ["semanticscholar"],
  "cache": { "path": "/home/user/.cache/biomcp/biomcp.sqlite", "max_mb": 1024 },
  "keys": { "ncbi_api_key": "..." },
  "sources": { "pubmed": { "deadline_ms": 10000 } }
}
```

```rust
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfigFile { budget_ms: Option<u32>, format: Option<Format>, disable_sources: Option<Vec<Source>>, cache: Option<CacheConfig>, keys: Option<KeysConfig>, sources: Option<BTreeMap<Source, SourceConfig>> }

pub(crate) struct Settings { pub fetch: FetchConfig, pub budget: Duration, pub format: Format, pub disabled: BTreeSet<Source>, pub explain: bool }
pub(crate) fn load(globals: &Globals) -> Result<Settings, ConfigError>;
```

- Precedence: the flag, then the config file, then the built-in default. A key comes from the config file, then from its documented environment variable, such as `NCBI_API_KEY` or `S2_API_KEY`.
- An unknown key, a wrong type or a bad value fails with a named error that gives the key path.
- The cache path defaults to `$XDG_CACHE_HOME/biomcp/biomcp.sqlite`.
- Tests set `XDG_CONFIG_HOME` and `XDG_CACHE_HOME` to empty temporary folders per case.

### 9.2 Observability

BioMCP writes nothing to standard error during a successful command. `--explain` adds an `explain` object to the envelope:

```json
"explain": {
  "total_ms": 1840,
  "stages": [ { "stage": "resolve", "ms": 3 }, { "stage": "fan_out", "ms": 1790 }, { "stage": "rank", "ms": 2 }, { "stage": "store", "ms": 9 } ],
  "requests": [ { "source": "pubmed", "endpoint": "esearch.fcgi", "cache": "miss", "status": 200, "attempts": 1, "ms": 412, "bytes": 98211 } ]
}
```

A trace line names the source and the endpoint path, never the full URL, a credential or the query text. The agent asked for its own query and already holds it. The binary prints the plan for a command without sending anything when the caller combines `--explain` with `--offline` and an empty cache. No logging crate exists.

### 9.3 Performance budget

Speed is measured against a floor worked out from first principles: the requests an answer needs, the bytes it must read and the work the CPU must do. Each gap between the floor and the measurement has a stated cause.

| Operation | Budget |
| --- | --- |
| Process start to output for `help` or `schema` | under 10 ms |
| BioMCP's own work for any command in replay with zero recorded delay | under 30 ms at the 95th percentile |
| A command whose replies are all fresh in the cache | under 50 ms |
| A warm page of a stored pool | under 30 ms and zero requests |
| Building an article pool of 2,000 items in replay with zero delay | under 150 ms |
| Peak memory for the largest pool | under 100 MB |
| A cold command | the slowest source it needs plus under 50 ms, within the budget |

Measurement:

- The runner records each case's wall time. `make perf` runs a tagged case set in replay with zero delays, compares each case with its budget and prints the slowest stages from `--explain`. It runs on transport, query and grammar changes and at each milestone's end. The landing gate does not run it, because timing on a shared machine varies.
- The nightly live run records cold timings and request counts per source, so a source that slows down shows up as a source fact.
- `biomcp health` reports each source's reachability and latency on demand.

## 10. Testing

Tests are outside-in, as `decisions/2026-10-23-outside-in-tests-and-one-small-runner.md` says.

### 10.1 The runner

`crates/biomcp/tests/cases.rs` is one integration test target with `harness = false`. It uses `libtest-mimic` to turn each YAML file under `tests/cases/` into a named test, so Cargo's filter and parallel run work per case. It builds nothing itself. It runs the binary Cargo built for the test.

```yaml
feature: article-search
covers: [search article, --gene, --variant, --offset]
story: AKT1 E17K leads to the 2007 discovery paper and keeps every plain PubMed hit
harvest: paper PMID 17611497
verified: 2026-10-23 live PubMed ESearch with sort=relevance
replay: akt1-e17k-2026-10-23
steps:
  - run: [search, article, --gene, AKT1, --variant, E17K, --limit, "50"]
    expect:
      /status: complete
      /result/items/*/header/pmid:
        rank_of: { "17611497": "<= 10", "33123537": "<= 2" }
  - run: [search, article, --gene, AKT1, --variant, E17K, --limit, "50", --offset, "50", --offline]
    expect:
      /status: complete
  - walk: [search, article, --gene, AKT1, --variant, E17K]
    expect:
      /result/items/*/header/pmid:
        includes_all_of: floor/pubmed-akt1-e17k.json
        unique: true
```

The case keys:

- `feature`, `covers`, `story`, `harvest` and `verified` describe the case. `covers` names the commands and options it exercises.
- `replay` names the recording folder. Each step runs with `--replay` on it, in a fresh temporary config and cache shared by the case's steps.
- `run` sends one CLI command. `mcp` sends the same arguments through `biomcp serve` as a `tools/call`. A step with both runs each and asserts the two envelopes are equal.
- `walk` pages through every result by following `next` until no next page remains, and asserts over the concatenated items.
- `follow: N` runs the Nth next step of the previous output, which tests chaining.
- `exit` sets the expected exit code, 0 by default.
- `expect` maps JSON pointers to assertions. A `*` segment maps over a list.

The assertion set stays small: equality, subset, `includes`, `includes_all_of` a floor file, count range, `rank_of` ranges, `before` between two items, `unique` and regular expression. Every output is also validated against the schema it names, without any key in the case. A failure prints the pointer, the expected value and the actual value.

The warm-page rule is a step with `--offline` after a step that built the pool. If the offline step succeeds, the page made no request.

The runner takes `--cases DIR`, so case sets that cannot be published run against a built binary from outside the repo. A later `score:` block in place of `expect:` writes metrics against a stored baseline for nightly scoring runs.

The runner is a test, so it may read its own environment and write temporary files. Its YAML reader is a test dependency. The runner stage picks a maintained serde YAML crate, because the original `serde_yaml` is archived.

### 10.2 Checks the runner adds

- **Surface.** One test reads `biomcp help --all` and fails when any command or option appears in no case's `covers`.
- **Schema.** Every output validates against its schema through the `jsonschema` test dependency, with remote resolution off.
- **Parity.** Every step with both `run` and `mcp` compares the two envelopes.

### 10.3 Coverage

`make coverage` runs the case suite under `cargo llvm-cov` and compares line coverage with the floor in `coverage.floor`. The landing gate runs it. The floor starts at the first measured value and rises whenever a landing exceeds it. `make unreached` lists the lines no case runs, for the reviewer. Each listed line is either deleted or given a case.

### 10.4 Where unit tests are allowed

The ratchet test holds the list. Each entry names a file and its reason.

- `crates/biodata/tests/`: identifier, date and notation tables through the public API, the schema drift and compatibility test, and the format round trip. These are outside-in for the library.
- `crates/biomcp-query/src/article/rank.rs`: the ranker's tie-break table, because a ranking rule is easier to state as rows than as recorded pools.
- `crates/biomcp-fetch/src/retry.rs`: the `Retry-After` parse table and the backoff schedule, because the forms are many and each case would need its own recording.

Everything else goes through the runner.

## 11. Ratchets and size rules

Every ratchet starts at its 1.0 target, because the code starts empty. `make landing-gate` runs `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` with the case suite, the schema test and the ratchet test, `cargo deny check licenses advisories`, and `make coverage`.

| Rule | Target | Mechanism |
| --- | --- | --- |
| Dependency direction between crates | the table in 2.1 | Cargo manifests, and the ratchet test's allowed list per crate |
| One crate names the HTTP client | `biomcp-fetch` only | the ratchet test's allowed lists |
| SQL owners | `biomcp-fetch`, and `biomcp-query/src/store/` | allowed lists and the ratchet test's path check |
| No I/O in `biodata` and adapters | none | per-crate `clippy.toml` `disallowed_methods` and `disallowed_types` |
| No global or shared mutable state outside the transport | none | per-crate `clippy.toml` `disallowed_types`, and `unsafe_code = "forbid"` |
| Threads | `fanout.rs`, the transport and the MCP loop only | `disallowed_methods` with one reasoned `expect` each |
| Environment reads | `config.rs` only | `disallowed_methods` with one reasoned `expect` |
| File length | 600 lines; 300 in an adapter folder | the ratchet test |
| Function length | 80 lines | `clippy::too_many_lines` at 80 |
| Suppressions without a reason | none | `allow_attributes_without_reason`; `dead_code` and `unused_imports` never allowed |
| Panics, unwraps, indexing, prints | none in product code | workspace clippy lints |
| Unit tests in product crates | the listed files | the ratchet test counts `#[test]` |
| Outputs with a checked-in, versioned schema | every command | the schema test and the runner's validation |
| Breaking schema changes without a major bump | none | the compatibility check in the schema test |
| Commands and options without a case | none | the runner's surface check |
| Quirks without a case | none | the ratchet test checks each named case exists |
| Format branches in interface code | one dispatch in `output.rs` | review; the view takes JSON only |
| `String` fields holding IDs, closed sets, dates or coordinates | none | review, visible in each schema |
| `serde_json::Value` in adapters | none | adapters cannot name `serde_json` |
| Test hooks in product code | none | record, replay and offline are user features |
| Feature set tested against feature set shipped | the same | one build profile in the gate |
| Licenses | no GPL, LGPL or AGPL | `cargo deny` |
| Outside-in coverage | never falls | `make coverage` with `coverage.floor` |
| Documented commands that fail | none | a docs check runs each documented command over recordings |

## 12. Porting 0.9 knowledge

1.0 ports 0.9's knowledge and copies none of its code, as `decisions/2026-10-23-a-clean-repo-whose-history-replaces-biomcp.md` says.

### 12.1 The quirks list

Each slice's investigation reads the 0.9 adapters for its sources and writes a quirks table in its design file, `planning/<entity>.md`:

| Source | Quirk | Evidence | 1.0 handling | Case |
| --- | --- | --- | --- | --- |
| PubMed | Without `sort=relevance`, ESearch returns newest first | 0.9 `src/sources/pubmed.rs`; live check 2026-10-23 | the `sort` parameter in `search_ids` | `article-search/akt1-e17k-floor.yaml` |

In the code, each kept quirk is one line with a `// Quirk:` comment that names its case. A quirk the slice drops gets a row that says why.

The seed list below comes from reading the 0.9 sources for this design. Each slice verifies its rows against the live source before relying on them.

| Source | Quirk |
| --- | --- |
| PubMed | Without `sort=relevance`, results come newest first. |
| PubMed | ESearch returns at most the first 10,000 records. |
| PubMed | ESummary JSON keys records by PMID and gives the order in `result.uids`. |
| NCBI | A key raises the allowed rate from 3 to 10 requests per second. |
| PubTator | The rate stays at 3 per second with or without the NCBI key. |
| MyGene | `species=human` is required, or every organism returns. |
| MyGene | `from` plus `size` may not pass 10,000. |
| MyGene | `HGNC` is a scalar or a list, `entrezgene` a number or a string, `alias` a string or a list. |
| MyGene | `uniprot` separates Swiss-Prot from TrEMBL, and 0.9 prefers Swiss-Prot. |
| MyGene | A symbol query needs the `symbol:"..."` form with quoting to match exactly. |
| ClinicalTrials.gov | Pages by `pageToken`, and `countTotal=true` gives the total. |
| Europe PMC | Pages by `cursorMark` starting from `*`, with `resultType=core` for full records. |
| Semantic Scholar | A 429 without a key comes from a shared pool, and a key header `x-api-key` lifts it. |
| openFDA | Some count queries ask for an exact retry through their error body. |
| EMA | The medicine data is several JSON report feeds of about 11 MB in all, refreshed by EMA about every three days. |

### 12.2 The rest of the port

- **Recordings.** 0.9's replay recordings convert into the transport's format by a one-time script that lives in the converting ticket and is deleted after. Only recordings a 1.0 case uses are kept.
- **Spec assertions.** Each slice reads 0.9's executable specs for its entity and turns each clinician-visible fact into a case. Wording checks are dropped.
- **Help text.** 0.9 help text moves into query type doc comments where a 1.0 option keeps it.
- **Skills.** The agent guide is rewritten against the 1.0 tree and its examples come from cases.
- **Release pipeline.** The five-platform release, signing and installers carry over in the parity milestone.
- **Accounting.** Every 0.9 command, section and option for an entity ends as ported, renamed or dropped by a named ruling in its slice's tickets.

## 13. Changes from the earlier architecture text

Each item replaces earlier text in this file. `planning/rust-standards.md` and `plan.md` still name `biomcp-store`, `biomcp-render` and `reqwest` in places. Those lines follow this file once Ian accepts the changes.

1. **Five crates.** `biomcp-store` folds into `biomcp-query` as its `store` module. `biomcp-render` folds into the binary as a JSON-fed `view` module.
2. **Blocking transport.** `ureq` on standard threads replaces `reqwest`. No async runtime exists.
3. **Hand-written MCP loop.** A small JSON-RPC loop replaces an MCP library.
4. **Formats in the transport.** The transport decodes JSON, XML and rows, so adapters never name `serde_json` or `quick-xml`.
5. **Bundle rows in SQLite.** Sync stores each bundle row with its lookup keys, so a query reads a few rows and never parses the whole file.
6. **Sections inside the card.** One object per requested section carries its state and data. The card's `more` lists the rest. The envelope's top-level `sections` map is gone.
7. **Next steps as argument lists.** `next` holds argument lists that the CLI and MCP print alike. The view prints them as shell commands.
8. **One ID per get.** `get` takes one identifier and positional sections. `search <entity> --id` returns headers for a list.
9. **Links as next steps.** A relation equal to a filter on another entity's search becomes a next step. Drill-down commands remain for paged parts and for graph edges.
10. **Paging by offset over a pool.** The request carries the pool key, so no continuation token exists.
11. **Error envelope and exit codes.** Errors print a `biomcp.error/1` envelope, and a confirmed absence exits 1.
12. **Grammar from schemas.** clap's builder reads the checked-in schemas at run time. No clap derive exists.
13. **Ratchet test.** The ratchet check is a Rust integration test. No script holds it.
14. **Store tables on demand.** `entity`, `alias` and `link` arrive with their first reader. The article slice brings only `pool` and `pool_item`.
15. **Coverage floor in the gate.** The landing gate fails when outside-in coverage falls below its floor.

## 14. Open questions for Ian

1. **Blocking I/O with `ureq` in place of `reqwest`.** The one-transport decision names `reqwest`. Recommendation: accept `ureq` with standard threads. It removes the async runtime, shortens every adapter and service function, and shrinks the build. The cost is a one-line amendment to that decision.
2. **A hand-written MCP loop.** Recommendation: accept. One tool needs five JSON-RPC methods, and the Rust MCP libraries bring the async runtime that question 1 removes. The cost is a few hundred lines we own and must track against MCP revisions.
3. **Five crates.** Recommendation: accept the store inside `biomcp-query` and the view inside the binary. Every boundary rule still holds through Cargo, clippy or the ratchet test, and two manifests and two public APIs disappear.
4. **Links as next steps.** Recommendation: drop 0.9's link commands that equal a search filter, such as `gene trials`, and have the gene card point at `search trial --gene`. This keeps one spelling per idea and removes about two dozen commands and their cases. The parity run maps each dropped 0.9 command to its 1.0 search. The cost is that 0.9 users of those commands change their habit.
5. **One ID per get.** Recommendation: `get` takes one identifier with positional sections, as 0.9 does, and a list of exact records goes through `search --id`. Several IDs and positional sections together cannot parse without guessing.
6. **Exit code 1 for a confirmed absence.** Recommendation: accept. An agent can tell "no such gene" from "the source failed" from the exit code alone, and the error envelope's next step offers the search.
7. **Bundles sync on first use.** Recommendation: a query that needs an absent bundle downloads it once under the bundle's own deadline and reports it, and offline mode never downloads. The alternative fails the query with a `sync` next step, which costs every new user one extra round trip.
8. **Coverage floor in the landing gate.** Recommendation: gate on it, with the floor rising only. It catches deleted cases and untested code at the landing that causes them. The cost is a second instrumented build in each gate run.
9. **Entity and link rows wait for the wiki.** Recommendation: write no `entity`, `alias` or `link` rows until the wiki or a measured resolution need reads them. The cost is that the wiki starts with an empty graph and fills as it is used.
