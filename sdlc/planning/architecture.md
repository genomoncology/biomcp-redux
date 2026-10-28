# Architecture

BioMCP is a Rust command with an MCP server for biomedical search and get. Its user is an agent that answers biomedical research questions about any human health issue, including clinical questions such as cancer care and rare disease. It gives entity-driven access to data sources and APIs (download, search and get) and never downloads or runs other bioinformatics tools. It asks providers such as PubMed, ClinicalTrials.gov, MyGene.info and ClinVar, and local data bundles such as HGNC, and returns typed JSON in biodata's shapes. It reports what sources say and never classifies a variant or scores a clinical match.

Every choice here follows `decisions/2026-10-23-least-code-easy-upkeep-fast.md`: the least code we own, easy upkeep, and fast. Where this design picks a lighter option over a heavier one, it says what it rejected and why. Decisions in `decisions/` are settled, and this design works inside them. Section 13 records the PM's rulings on this design's earlier open questions, and section 14 lists what remains open.

The gene sketches follow `planning/slices/gene.md` and `decisions/2026-10-24-gene-slice-rulings.md`. Flags, sections and field names for other entities are placeholders that each slice settles, as `planning/slices.md` says. The shapes, the rules and the signatures are the design.

## 1. Goals and how the design meets them

| Goal | Mechanism |
| --- | --- |
| Far less code | One transport does every request step once. An adapter is a constant source description, serde raw types with one validity check and pure mapping functions. One grammar serves the CLI and MCP. One generic view renders Markdown from JSON. |
| Easy upkeep | Six crates whose direction Cargo enforces. Lints and a ratchet test enforce size, state and dependency rules from the first commit. |
| Very fast | Blocking I/O on threads with no async runtime. One leg per source, each with its own deadline, under a limiter keyed by the source key its adapter declares. A SQLite reply cache, pinned pools and indexed bundle rows make warm calls local. A budget per command, measured in replay. |
| Right information in context | The densest facts sit at the top of the tree, as `decisions/2026-10-24-dense-facts-first-with-a-token-budget.md` says. Search returns capped headers. Get returns cards with requested sections. Every response names the sections it left out and typed next steps that go deeper. Nulls, empty lists and unrequested sections are omitted. Cases hold size ceilings per command and per section. |
| JSON by default, shaped by biodata | Every response is one envelope type from `biomcp-types`, with a generated, checked-in and versioned JSON Schema. |
| MCP over the CLI | The main tool takes the CLI argument list. Research search and fetch wrappers use the same grammar and command functions. |
| Search and get scope | The request type holds search, get and drill-down commands only. |
| Data bundles as local APIs | A bundle is an endpoint kind in the transport. Its rows sit in SQLite with lookup keys and a word index, and its adapter maps them like any reply. |
| Cache that later backs a wiki | One SQLite file holds typed results with their sources and times, and the wiki tables join it when the wiki arrives. |
| Outside-in tests with record and replay | Record and replay live in the transport as user features. One small runner drives the binary and the MCP tool from YAML cases. |
| Lint ratchets | Workspace lints, per-crate `clippy.toml` files and one ratchet test, all in `make landing-gate`. |

## 2. Workspace and crates

### 2.1 The crates

Six crates. Each exists because a rule needs a crate boundary to enforce it.

| Crate | Job | May import | May not import |
| --- | --- | --- | --- |
| `biodata` | Neutral entity values, identifiers, coordinates, notation and closed vocabularies. JSON Schemas. | `serde`, `winnow`; `schemars` behind the `schema` feature | Any workspace crate, any I/O crate, `serde_json` outside tests |
| `biomcp-types` | BioMCP queries, requests, source identities and response disclosure; generated contract schemas | `biodata`, `serde`; `schemars` behind the `schema` feature | Any I/O crate, transport, adapters or services |
| `biomcp-fetch` | The transport: HTTP, quotas, deadlines, retry, body limits, decoding, reply checks, the reply cache, offline, record and replay, bundle sync and bundle rows | `biodata`, `biomcp-types`, `ureq`, `rusqlite`, `serde`, `serde_json`, `quick-xml`, `sha2`, `url`, `rustls`, `rustls-platform-verifier`, `httpdate`, `tempfile`, `csv`, `flate2`, `thiserror`, `prost`, `zstd` | `biomcp-adapters`, `biomcp-query`, `clap` |
| `biomcp-adapters` | One module per source: a source description, endpoints, raw reply types with their checks and mapping into biodata | `biodata`, `biomcp-types`, `biomcp-fetch`, `serde`, `quick-xml`, `calamine`, `csv`, `zip`, `thiserror` | `ureq`, `rusqlite`, `serde_json`, `biomcp-query`, `clap` |
| `biomcp-query` | The services and the store: resolve, fan out, merge, rank, pool, page, sections and next steps | `biodata`, `biomcp-types`, `biomcp-fetch`, `biomcp-adapters`, `rusqlite`, `serde_json`, `sha2`, `thiserror` | `ureq`, `clap` |
| `biomcp` | The binary: CLI grammar, MCP server, config file, Markdown view, the runner's test target, later the web mode | every crate above, `clap`, `serde`, `serde_json`, `httparse`, `base64`, `ctrlc`, `rpassword`, `tempfile`, `sha2`, `shlex` | `ureq`, `rusqlite` |

Why each crate is its own:

- `biodata` does no I/O and may later merge with another biodata crate, as `decisions/2026-10-23-biodata-is-the-base-data-layer.md` says. Only a crate with its own manifest and `clippy.toml` can hold that line.
- `biomcp-fetch` is the only crate that names an HTTP client, as `decisions/2026-10-23-one-transport.md` says.
- `biomcp-adapters` sits below `biomcp-query`, so Cargo refuses any import of the store or the services. It cannot name `ureq`, so it cannot build a client.
- `biomcp-query` holds the logic that the CLI, MCP and the later web mode share. Splitting it from the binary keeps clap and the config file out of it.
- `biomcp` is the only binary.

`biomcp-types` separates consumer query and disclosure choices from reusable neutral values. Biodata never imports it. `Attribution.source` and `Representation.source` are neutral text; BioMCP writes canonical `Source` spellings through one checked conversion. That conversion has one reasoned lint exception in `crates/biomcp-types/src/source.rs` because every enum spelling is nonempty. Schema majors change for this enum-to-text boundary; emitted source values stay the same.

The store is the `store` module of `biomcp-query`. The ratchet test confines `rusqlite` inside `biomcp-query` to `src/store/`. The Markdown view is the `view` module of the binary. It takes the serialized `serde_json::Value` of a response and returns text, so it cannot fetch, read a file or show content the JSON lacks.

Rejected: a crate per entity, a crate per source family, a separate store crate, a separate render crate, a separate MCP crate. Each adds a manifest and a public surface without enforcing an additional boundary.

`rpassword` owns the terminal no-echo credential prompt. Its maintained prompt function replaces platform-specific terminal control code.

### 2.2 The workspace manifest

```toml
[workspace]
members = ["crates/biodata", "crates/biomcp-types", "crates/biomcp-fetch", "crates/biomcp-adapters", "crates/biomcp-query", "crates/biomcp"]
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
ureq = { version = "3", default-features = false, features = ["rustls", "platform-verifier", "gzip", "json"] }
rusqlite = { version = "0.37", features = ["bundled"] }
quick-xml = { version = "0.42", default-features = false, features = ["serialize"] }
sha2 = { version = "0.10", default-features = false }
httparse = "=1.10.1"  # bounded HTTP request headers; socket ownership stays in biomcp
ctrlc = { version = "=3.5.2", features = ["termination"] } # safe native process interrupts
base64 = "0.23"     # MCP HTTP metadata sentinel decoding
clap = { version = "4", default-features = false, features = ["std", "help", "usage", "error-context", "suggestions", "string"] }

[workspace.lints.rust]
unsafe_code = "deny"
unreachable_pub = "deny"
missing_debug_implementations = "deny"
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

Lints are `deny`, never `forbid`, so a rare exception can carry `#[expect(lint, reason = "...")]` where a reviewer sees it. `allow_attributes_without_reason` makes every such exception state its reason, and the ratchet test lists the files allowed to carry one.

`ureq` uses rustls with the ring provider and platform certificate verification. `ca_bundle` adds checked PEM roots through rustls-platform-verifier without replacing platform trust. The transport keeps ureq's TCP and CONNECT connectors and a narrow TLS connector because ureq's root selector cannot express additive platform roots. `config.rs` owns PEM I/O and proxy environment reads; the transport receives checked, owned settings. `clap`'s `string` feature lets the grammar build arguments from owned names read out of the schemas.

The version numbers above are the stage 1 starting point. The stage that adds each dependency confirms its current release and feature names and records the code it saves, as `planning/rust-standards.md` says.

`rpassword` reads terminal credentials without echo in one maintained prompt call. `tempfile` publishes key configuration atomically from a sibling file with Unix owner-only permissions. The binary uses the standard file lock to serialize writers. Server calls clone the shared Fetch transport with an immutable credential snapshot reloaded from the selected config; quotas, connections, recordings, clocks, caches and source restrictions remain shared.

### 2.3 How tools enforce the boundaries

Three tools carry every boundary rule, and `make landing-gate` runs all three.

1. **Cargo.** A crate can import only what its manifest lists. Transitive crates cannot be named. The adapters depend on `biomcp-fetch`, which depends on `ureq`, and still cannot name `ureq`.
2. **clippy, per crate.** clippy reads the nearest `clippy.toml`, so each crate carries its own, and each repeats the test allowances and `too-many-lines-threshold = 80`.
   - `biodata`: disallows file, environment, network, process, thread and clock items one by one, and the interior mutability types `Cell`, `RefCell`, `Mutex`, `RwLock`, `OnceLock`, `LazyLock` and the atomics. Without `unsafe` and without those types, no global state can exist.
   - `biomcp-adapters`: the same list.
   - `biomcp-query`: disallows environment, file and process items, `std::thread::sleep`, `std::thread::spawn` and the interior mutability types. `std::thread::scope` is allowed only in `fan_out.rs`, through one reasoned `expect`.
   - `biomcp-fetch`: disallows environment and process items. Its `Mutex`, `Condvar`, `Cell` and `std::thread::scope` uses each carry a reasoned `expect`.
   - `biomcp`: disallows environment reads everywhere except `config.rs`, and thread spawns everywhere except `mcp.rs` and its HTTP socket owner, each through one reasoned `expect`.
3. **The ratchet test.** `crates/biomcp/tests/ratchets.rs` is an ordinary integration test. It reads the workspace files and fails on any breach of a rule a compiler cannot see:
   - Each crate's direct dependencies are a subset of its allowed list, which the test holds as one table that mirrors section 2.1.
   - `rusqlite` appears in `biomcp-query` only under `src/store/`.
   - No product or test file exceeds 600 lines. No file in an adapter folder exceeds 300.
   - `#[test]` appears in product crates only in the listed exception files.
   - `#[serde(default)]` never appears in an adapter's `raw.rs` (section 5.1).
   - `#[expect(` appears only in the listed files.
   - Every `Quirk:` comment names a case file that exists.
   - `SourceKey::declare` and `Quota` statics appear only in `biomcp-adapters/src/quotas.rs`, and no two quotas declare the same key.

`cargo deny check licenses advisories` also runs in the gate. It refuses GPL, LGPL and AGPL code and known advisories.

Rejected: a separate script for the ratchets. A Rust test needs no second language, runs under `cargo test` and fails with a normal test report.

### 2.4 File layout

```
Cargo.toml  rust-toolchain.toml  rustfmt.toml  deny.toml  Makefile  .gitattributes
crates/
  biodata/
    clippy.toml  README.md
    src/lib.rs  macros.rs  error.rs  text.rs  date.rs  coord.rs  number.rs
    src/id/        hgnc.rs  entrez.rs  ensembl.rs  symbol.rs  uniprot.rs  pmid.rs  nct.rs  ...
    src/gene.rs  variant/  trial.rs  ...             neutral entity values
  biomcp-types/
    clippy.toml  README.md
    src/lib.rs  source.rs  predictor.rs  article.rs
    src/query/     request.rs  gene.rs  article.rs  trial.rs  ...
    src/response/  envelope.rs  section.rs  step.rs  problem.rs  gene.rs  article.rs  ...
    schemas/       generated *.schema.json, checked in
    tests/         schemas.rs  roundtrip.rs  ids.rs  (public API tables)
  biomcp-fetch/
    clippy.toml
    src/lib.rs  spec.rs  quota.rs  endpoint.rs  leg.rs  deadline.rs  clock.rs  send.rs  retry.rs
    src/decode.rs  xml.rs  cache.rs  record.rs  bundle.rs  secret.rs  error.rs
  biomcp-adapters/
    clippy.toml
    src/lib.rs  raw.rs  mapped.rs  quotas.rs
    src/<source>/mod.rs  raw.rs  map.rs
  biomcp-query/
    clippy.toml
    src/lib.rs  ctx.rs  run.rs  fan_out.rs  status.rs  error.rs
    src/<entity>/search.rs  get.rs  next.rs
    src/gene/resolve.rs
    src/article/pool.rs  order.rs
    src/store/mod.rs  schema.sql  pool.rs
  biomcp/
    clippy.toml
    src/main.rs  config.rs  paths.rs  grammar.rs  retired.rs  spell.rs  output.rs  exit.rs  mcp.rs
    src/view/mod.rs  hints.rs
    tests/cases.rs  ratchets.rs  runner/
tests/
  cases/<feature>/*.yaml
  replies/<recording>/<source>/<key>.json
```

`.gitattributes` sets `eol=lf` for every text file, so recordings, schemas and cases hash and diff the same on Windows.

## 3. Neutral values and consumer types

`biodata` holds reusable neutral entity values. `biomcp-types` holds the query, response, source and disclosure values that cross BioMCP layers. Values that stay inside one layer, such as an adapter's raw reply types or a pool's ranking features, stay in that layer. `biodata` names no other project, depends on no other workspace crate and has no command.

Its modules group what a later merge would keep together. Identifiers, coordinates, dates, text and the entity types form the shareable core. `biomcp-types/query` and `biomcp-types/response` hold BioMCP's request and response contract beside its source enum. The consumer crate imports neutral values without a reverse edge.

A minimal set of the consumer types lands before the transport stage, because the transport's signatures name `Source`, `Reason` and the problem type.

### 3.1 Identifiers

Every identifier is a newtype with a private field, one checked way in, `FromStr` and `Display` that round-trip, and serde in both directions through the same check. Every identifier serializes as its canonical text, so an agent can pass any ID back exactly as it received it.

```rust
/// An HGNC gene identifier, such as `HGNC:1097`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HgncId(NonZeroU32);

impl HgncId {
    pub fn new(n: u32) -> Result<Self, IdError>;   // where the caller knows the namespace is HGNC
    pub fn get(self) -> u32;
}
impl FromStr for HgncId { type Err = IdError; /* requires the HGNC: prefix */ }
impl fmt::Display for HgncId { /* HGNC:1097 */ }
text_value!(HgncId, "HgncId", r"^HGNC:[1-9][0-9]*$");

pub struct EntrezGeneId(NonZeroU32);         // 673
pub struct EnsemblGeneId { number: u64, version: Option<NonZeroU16> }   // ENSG00000157764, version kept apart
pub struct Cytoband(Box<str>);               // 7q34, 9p21.3; checked chromosome, arm and band
pub struct Pmid(NonZeroU32);                 // 17611497
pub struct NctId(u32);                       // NCT plus eight digits

/// A gene symbol as written, compared and hashed without case.
pub struct GeneSymbol(Box<str>);
impl PartialEq for GeneSymbol { /* eq_ignore_ascii_case */ }
impl Hash for GeneSymbol { /* hashes the ASCII-lowercased bytes, so Hash agrees with Eq */ }

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

The visitor accepts borrowed and owned strings. Error messages carry the stable code and never the input text, because inputs may hold patient data.

### 3.2 Closed sets, dates, text and coordinates

A checked string enum uses [`wire_enum!`](../../crates/biodata/src/macros.rs). Each variant declares its wire value once. The helper derives checked serde conversion and a string schema with an `enum` list. A plain schema derive can emit unit-enum `oneOf` branches that the compatibility checker rejects. A set that may grow is `#[non_exhaustive]`.

```rust
crate::macros::wire_enum! {
    /// A closed set of checked choices.
    Choice {
        /// The canonical choice.
        Canonical => "canonical",
    }
}
```

The enum parses only its own canonical names. Each provider's spelling, such as HGNC's `protein-coding gene` or ClinicalTrials.gov's `PHASE1`, maps to the enum in that adapter's `map.rs`. A provider value outside the set becomes a problem on the record and leaves the field absent. No enum carries an `Other(String)` escape.

Dates from literature are often partial, so one type holds what the source stated:

```rust
/// A calendar date to the precision the source gave: a year, a month or a day.
pub struct PartialDate { year: u16, month: Option<NonZeroU8>, day: Option<NonZeroU8> }
impl PartialDate { pub fn new(year: u16, month: Option<u8>, day: Option<u8>) -> Result<Self, DateError>; }
// FromStr and Display use 2007, 2007-07 and 2007-07-15.
```

Free text is `Text(Box<str>)`, checked nonempty and trimmed. A summary, an abstract or eligibility criteria stay text. Alias text stays text, because aliases hold spaces, slashes and parentheses, such as `HER-2/neu`. Display text, such as a formatted date, never lives in a value.

Coordinates carry their assembly and counting rule in the type: `Position1(NonZeroU32)`, `Span0 { start: u32, end: u32 }` and `GenomicInterval { build: Build, contig: Contig, span: Span0, strand: Option<Strand> }`. `Build` has an explicit `Unstated` case for sources that do not say. Nothing defaults to one build. Variant parsing has one implementation, and the variant slice decides whether it lives here or in a shared parser crate.

### 3.3 Entities, headers and cards

Each entity has two levels, so progressive disclosure is a type fact.

- A **header** is what a search returns: the identifiers and the few fields an agent needs to choose its next step. Every list inside a header is capped by its type, such as the first three authors and a count.
- The **entity** is the full record a get returns.
- A **card** is one get result. It holds the entity, the requested sections and the names of the sections left out.

Section 3.10 gives the rules that decide which layer each fact belongs to.

The gene types follow `planning/slices/gene.md`:

```rust
/// How an input reached a gene.
pub struct Match { pub kind: MatchKind, pub text: Text }
#[non_exhaustive]
pub enum MatchKind { Symbol, PreviousSymbol, Alias, NcbiAlias, HgncId, EntrezId, EnsemblId }

/// Enough to pick a gene and chain the next command.
pub struct GeneHeader {
    pub hgnc_id: HgncId,
    pub symbol: GeneSymbol,
    pub name: Text,
    pub locus_group: LocusGroup,
    pub location: Option<Cytoband>,
    pub status: GeneStatus,
    pub r#match: Option<Match>,          // present when the header answers an input
}

/// One human gene: HGNC identity with MyGene.info's summary and spans.
pub struct Gene {
    pub hgnc_id: HgncId,
    pub symbol: GeneSymbol,
    pub name: Text,
    pub status: GeneStatus,
    pub locus_group: LocusGroup,
    pub locus_type: Option<LocusType>,
    pub location: Option<Cytoband>,
    pub previous_symbols: Box<[GeneSymbol]>,
    pub aliases: Box<[Text]>,
    pub ncbi_aliases: Box<[Text]>,       // aliases NCBI holds and HGNC lacks
    pub entrez_id: Option<EntrezGeneId>,
    pub ensembl_id: Option<EnsemblGeneId>,
    pub uniprot: Box<[UniProtAcc]>,
    pub omim: Box<[OmimId]>,
    pub mane_select: Option<ManeSelect>,
    pub spans: Box<[GenomicInterval]>,   // one per build, each with strand
    pub summary: Option<Summary>,        // whole RefSeq text with its source and date
}

/// Requested landed sections, each with its own source state.
pub struct GeneSections {
    pub constraint: Option<Section<Constraint>>,
}

#[non_exhaustive]
pub enum GeneSection { Constraint }
#[non_exhaustive]
pub enum GeneSectionWord { Constraint, All }

```

No struct uses `#[serde(flatten)]`. A header is built from its entity by a method, such as `Gene::header(&self, matched: Option<Match>) -> GeneHeader`. Every field skips serialization when it is `None` or empty. Complete neutral entities are biodata types. Headers, cards and sections are BioMCP consumer types, so sections stay out of entity types, as rule 15 of `planning/rust-standards.md` says.

### 3.4 Resolution

A get answers each input with a resolution. Ambiguous and withdrawn inputs are answers, never failures.

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "resolution", rename_all = "snake_case")]
pub enum GeneResolution {
    Resolved { gene: Box<Gene>, r#match: Match },
    Ambiguous { candidates: Box<[GeneHeader]> },        // ranked; each carries how it matched
    Withdrawn { hgnc_id: HgncId, symbol: GeneSymbol, hgnc_status: HgncStatus, merged_into: Box<[HgncId]> },
    NotFound {},
}

/// One input of a get and its answer, in input order.
pub struct GeneItem {
    pub input: GeneInput,
    pub status: ItemStatus,
    pub answer: Option<GeneResolution>,
    pub sections: Option<GeneSections>,
    pub more: Box<[GeneSection]>,
}
pub struct GeneCards { pub items: Box<[GeneItem]> }
```

Only resolved gene items receive `sections` or `more`. Requested sections have a state even when they do not apply. Unrequested sections appear in `more` only when the HGNC facts permit them. Both fields are omitted when empty. The envelope names each source asked once; a gene item adds no duplicate source list. Later section tickets add their own enum word, response field and source job.

Each later entity defines its resolution the same way, with the variants its sources can give.

### 3.5 Sections and their five states

A requested section is always present and always in one of five states. One generic type holds the state and the data together, so a section can never carry data and an error at once.

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum Section<T> {
    Data { source: Source, data: T },
    Empty { source: Source, record: Option<SourceRecord> },
    Unavailable { source: Source, reason: Reason },
    Skipped { source: Source, reason: Reason },     // no_key or disabled
    NotApplicable,
}

/// The source's own entry for this entity: its ID and a link.
pub struct SourceRecord { pub id: Text, pub url: Option<Text> }
```

`Empty` with a `record` means the source holds an entry for this entity with nothing in it. CIViC holds VSIR with no variants. `Empty` without a `record` means the source holds no entry at all. CIViC lacks ABCC8. Both are answers, and the agent can tell them apart. `Skipped` means BioMCP did not ask the source: the source needs the user's own key and none is configured, or the user disabled it. A skip is the user's setting, never a failure. `NotApplicable` means the record's own facts rule the section out, such as `protein` for MIR21, which HGNC lists with no UniProt accession.

A card's `more` lists the sections the caller did not request, so 0.9's sixteen `not_requested` entries become one short list of names.

### 3.6 Provenance and problems

Source identity is one enum. Each variant carries the short name 0.9 users already know.

```rust
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "hgnc")] Hgnc,
    #[serde(rename = "mygene")] MyGene,
    #[serde(rename = "pubmed")] PubMed,
    #[serde(rename = "europepmc")] EuropePmc,
    #[serde(rename = "pubtator")] PubTator,
    #[serde(rename = "ctgov")] ClinicalTrialsGov,
    #[serde(rename = "civic")] Civic,
    #[serde(rename = "litvar")] LitVar,
    #[serde(rename = "semantic-scholar")] SemanticScholar,   // the one spelling everywhere
    #[serde(rename = "icite")] Icite,
    #[serde(rename = "pmc-oa")] PmcOa,
    // one variant per source
}
```

Provenance appears at three levels, each as small as its question needs:

- The envelope lists every source asked in `sources`.
- Each section names its one source in its state.
- A pooled search item lists every source that found it, with the native rank. `found_by` prints as a map from source to rank, such as `{"pubmed": 2, "europepmc": 13}`, and appears only on pages built from several sources, because on a one-source page an item's rank is its position:

```rust
pub struct ArticleItem { pub pmid: Option<Pmid>, pub header: Option<ArticleHeader>, pub found_by: BTreeMap<Source, NonZeroU32>, pub cited_by: Option<u32> }
```

`cited_by` comes from iCite and appears only under `--sort citations`. Both fields sit on the item and never on the biodata header, because they come from other sources.

No field is merged from two sources (section 6.5), so field-level provenance needs no type.

A problem is a typed note about one field of one record. It lets a record survive one bad field.

```rust
pub struct Problem {
    pub source: Source,
    pub record: Option<Text>,         // the record's canonical ID text, when known
    pub field: Box<str>,              // the biodata field name, such as "pmid"
    pub code: ProblemCode,            // such as id.zero, date.invalid, value.unknown
}
```

A required field that fails drops the record and leaves a problem. An optional field that fails leaves the field absent and a problem.

### 3.7 Queries and the request type

Every command is one query type. Its doc comments become the CLI help, the MCP description and the reference docs, through `schemars`. The schema carries argument spelling through extension keys: `x-positional` gives a positional argument's index and `x-trailing-section` marks the section-word suffix. A future short flag can use `x-short` when one is consumed.

```rust
/// Get gene cards for symbols, previous symbols, aliases, HGNC IDs, Entrez IDs or Ensembl IDs.
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct GeneGet {
    /// One to twenty gene inputs. Cards return in input order.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    pub input: Box<[GeneInput]>,
    /// Sections to add to each card.
    #[cfg_attr(feature = "schema", schemars(extend("x-trailing-section" = true)))]
    pub section: Box<[GeneSectionWord]>,
}

/// Search human genes by text, band or locus group.
#[derive(Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct GeneSearch {
    /// Words matched against symbols, previous symbols, aliases and names.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    pub query: Option<Text>,
    /// Genes in this cytoband or any band inside it, such as 9p21.
    pub band: Option<Cytoband>,
    pub locus_group: Option<LocusGroup>,
    /// Results per page, 1 to 50.
    #[serde(default)]
    pub limit: Limit,
    /// Results to skip.
    #[serde(default)]
    pub offset: u32,
    /// The pool to page, from a previous page's next step.
    pub pool: Option<PoolId>,
}

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

Query types never use `#[serde(flatten)]`, because serde refuses `deny_unknown_fields` beside it. Each search names its own `limit`, `offset` and `pool` fields. `GeneInput` is free text that the edge parses into an HGNC ID, an Ensembl ID, an Entrez ID or symbol-like text, as `planning/slices/gene.md` says.

The CLI path maps to the tag mechanically: `search gene` is `search_gene` and `trial locations` is `trial_locations`. A next step is a `Request` value, so the services never build command text.

### 3.8 The envelope

Every command returns one envelope. Its body type changes by command, and each body names its own query type, so `query` echoes only the command that ran.

The envelope omits empty collections, including source identity lists and empty search pages. The case runner checks product envelopes from recorded CLI and MCP calls. Raw provider recordings retain their native arrays. Query echoes omit inactive false flags and retain values that shape the request, including limits, filters, source selections and continuations. Trial search results omit the raw provider parameter lists. Help echoes its path as a `command` list inside the query object; root help echoes an empty query object.

Variant identity lists serialize through one namespace owner. Their entries use `rs:`, `clingen:`, `clinvar:`, `cosmic:` or the source's `ga4gh:` prefix. Published identifiers and the accepted command input grammar stay unchanged. A ClinVar accession retains its `VCV` spelling after the namespace prefix, and ClinGen identifiers retain `CA` or `PA`.

Trial `source_updated` contains the native `value` and a typed `timezone` marker. The recorded ClinicalTrials.gov version replies provide timestamps without timezone text, so these replies say `source_gave_none`. A native timestamp with `Z` or an offset retains that text and says `in_value`. BioMCP infers no timezone. Date-only source update fields retain their `PartialDate` type. The reviewed source evidence is the version response in `tests/replies/trial/condition/ctgov/`; the publisher documents the version endpoint in [its API reference](https://clinicaltrials.gov/data-api/api).

```rust
/// Every response body names its schema and the query that produced it.
pub trait ResponseBody: Serialize {
    const SCHEMA: &'static str;
    type Query: Serialize;
}

#[derive(Serialize)]
#[serde(bound = "T: ResponseBody")]
pub struct Envelope<T: ResponseBody> {
    schema: &'static str,                  // such as biomcp.gene_cards/1
    pub status: Status,
    pub query: T::Query,                   // the query as parsed and resolved
    pub result: Option<T>,
    pub page: Option<PageInfo>,            // searches only
    pub sources: Box<[SourceOutcome]>,     // one per source asked
    pub problems: Box<[Problem]>,
    pub next: Box<[Step]>,                 // see 6.10 for the bound
}

pub struct SchemaId { pub name: &'static str, pub major: u16 }   // text form name/major
pub enum Status { Complete, Partial, Absent, Failed }
pub enum ItemStatus { Complete, Partial, Absent, Failed }
pub struct PageInfo { pub offset: u32, pub returned: u32, pub total: u64 } // HGNC totals are known; pool metadata waits for a pooled source
pub struct PoolInfo { pub id: PoolId, pub size: u32, pub created: Timestamp, pub expires: Timestamp }
pub struct SourceOutcome {
    pub source: Source, pub state: SourceState, pub reason: Option<Reason>,
    pub term: Option<Text>,            // the exact query this source was sent
    pub count: Option<u64>,            // the source's own total
    pub fetched: Option<u32>,          // the depth the pool holds from this source
    pub notices: Box<[Notice]>,        // such as PubMed's dropped words
}
pub enum SourceState { Ok, Stale, Syncing, Failed }

/// One next command, with what it does.
pub struct Step { pub kind: StepKind, pub args: Box<[Box<str>]> }
#[non_exhaustive]
pub enum StepKind { Page, Fresh, Disambiguate, Sync, Section }

#[non_exhaustive]
pub enum Reason { NotFound, RateLimited, Unavailable, Rejected, BadReply, DeadlineExceeded, Offline, NoKey, Disabled, ReplayMiss, Syncing, NoJoinId, NotOpen }
```

`NotOpen` marks a PMC article outside the PMC OA bucket or a preprint without full text. A filter or sort that a named source cannot apply is a usage error, code `usage.unsupported_filter`, naming the source and the filter before any request. A notice on a source outcome never changes its state: PubMed answered the words it kept.

`Step` holds an argument list because the CLI and MCP print the same JSON. The binary spells each typed `Action` into a `Step` through the grammar (section 8.3). `Action::Run { request, fresh }` retains request policy; `Action::Sync { bundle }` names source-level housekeeping without inventing an entity request. The view prints a `Step` as a quoted shell command. An error prints the separate envelope in section 8.5.

### 3.9 Schemas and versions

- `schemars` derives every schema behind the `schema` feature. The dialect is JSON Schema 2020-12, which `schemars` 1.x emits by default.
- The binary depends on `biomcp-types` with the `schema` feature on, which enables `biodata/schema`. The gate's tests and the shipped binary therefore build the same feature set. The feature stays optional for other users of `biodata`.
- The generated files sit in `crates/biomcp-types/schemas/`, one per response type, per query type and for `Request`. Names follow `biodata.<entity>/<major>` for core types and `biomcp.<response>/<major>` for responses.
- `crates/biomcp-types/tests/schemas.rs` regenerates every schema and fails on any difference from the checked-in copy. Run with `UPDATE_SCHEMAS=1`, it writes the new copies instead.
- Generation must stay inside the compatibility checker's [supported subset](../../crates/biomcp-types/tests/schemas/compatibility.rs). Use `wire_enum!` for checked string enums. Optional unions contain one value and null; `oneOf` supports internally tagged object branches, not unit-enum branches. Unsupported keywords such as `uniqueItems` fail even when generating an initial schema. Enforce source uniqueness at runtime and keep its existing proof. Do not expand the checker to accommodate a new derive form or keyword.
- The same test runs a compatibility check. It resolves every `$ref` through `$defs` and lists each schema as JSON pointer paths with their types. Every path and type in the checked-in schema must still exist in the new one unless the major version rose. Adding an optional field keeps the version. Removing, renaming or retyping a field raises it.
- The same test checks that every query schema stays inside the grammar's subset: a flat object whose properties are strings, integers, booleans, string enums, or arrays of those, with no nested objects, `oneOf` or `anyOf` other than an optional value, and one allowed exception: a root `anyOf` of `required` lists that says at least one selector must be present, such as `query` or `band` on a gene search. A query type outside the subset fails here, before the grammar sees it.
- The binary embeds the checked-in schema files with `include_str!`. `biomcp schema <name>` prints one, and the grammar reads them. The checked-in files are the one authority, and the drift test keeps them equal to the types.
- MCP advertises compact reply envelopes with schema IDs selected from the help catalog. The result has object type. Error, research projection and raw schema-document forms remain available. Clients retrieve full contracts through `biomcp schema <id>`, including calls through the MCP tool. Discovery embeds no full reply graph and requires no schema resolver.
- The public contract tests in `crates/biomcp-types/tests/` check JSON round trips, and `crates/biomcp-types/tests/schemas/values.rs` checks canonical enum spellings against their schemas. BioMCP writes and reads only JSON, so other formats wait for a consumer.

### 3.10 How types carry progressive disclosure

Progressive disclosure is a rule about where each fact sits in the types, so every slice places facts the same way. `decisions/2026-10-24-progressive-disclosure-lives-in-response-types.md` records it.

**The entity is complete and knows nothing about disclosure.** A biodata entity holds every fact its sources give, for any consumer. `Option` means the source has no value. It never means "not fetched" or "not requested". Disclosure lives in BioMCP's response types, which pick from the entity.

**Four layers, each its own type with its own schema:**

| Layer | Type | Returned by | Cost |
| --- | --- | --- | --- |
| Header | `<Entity>Header`, a consumer projection of the entity | search, ambiguous inputs | No request beyond the search |
| Card | `<Entity>Card` holding the entity's core fields | get, by default | The identity source's request only |
| Section | `Section<T>`, named in a closed enum | get with a section name | One source, one request |
| Pivot or page | A `Step` to another search or the next page | `next` | A new command |

**Where a fact goes.** A slice applies these rules in order to every fact it keeps:

1. A scalar or small fixed subobject that the identity source returns and the common question needs goes on the card, such as the gene's band or its MANE Select pair.
2. A fact from another source, or one that needs another request, goes in a section. One section has one source and one request, so asking for more data and paying for more data are the same act. An article's `fulltext` and `supplements` sections each have one source, the PMC OA bucket, with Europe PMC full text for a preprint's `fulltext`. Reading a part of the body or rows of a supplementary file is a step, `article text` or `article supplement`, never a section.
3. Bulky prose goes in a section unless the common question needs it. The gene's RefSeq summary is on the card by Ian's ruling, and UniProt function text is behind `protein`.
4. A short list that cannot grow far, such as previous symbols, sits inline whole.
5. A list that can grow, such as CIViC variants or Reactome pathways, sits in a section as `Capped<T> { items, total }` with a `next` step to the rest. A header holds only capped lists. A list is never cut without its total.
6. A list of other entities, such as a gene's trials, articles or variants, is never embedded. The card offers a pivot step to that entity's search.
7. Another entity appears only as a reference that holds its ID and label, such as `PathwayRef`. No entity type contains another entity type, so depth comes from following IDs, and the types never nest without bound.

**What absence means.** A missing field means the source has no value. A missing section means it was not requested, and the card's `more` names it. A requested section always appears with one of its five states. JSON never prints `null` or an empty list.

**Order is priority.** Struct fields are declared in priority order: identifiers first, closed values next, then lists, then prose. Serde keeps that order, so an agent that reads only the start of a record reads the densest part.

**What the design rejects.** A `--fields` selector, a depth parameter or a query language would make each response's shape depend on the request, so one schema could not describe it and an agent could not predict it. They also cost code. Sections are a closed enum, so every response shape is known ahead.

**Checks.** The schema test fails when an entity type contains another entity type, when a header holds a list that is not `Capped`, or when a card type holds a list of another entity. Each slice's cases hold the byte or token ceiling per layer, as `planning/slices.md` says.

## 4. The transport

`biomcp-fetch` sends every request, as `decisions/2026-10-23-one-transport.md` says.

### 4.1 Blocking I/O on threads

The transport uses blocking I/O through `ureq`, and the services fan out on scoped standard threads. No crate depends on an async runtime.

- A command makes at most a few dozen requests to at most a dozen sources. One thread per source leg costs nothing measurable at that scale.
- Blocking code is shorter. Deadlines are timeouts, quota waits are sleeps, and a leg is a plain function call.
- The dependency tree loses the async runtime, the async HTTP stack and their transitive crates. Builds get faster and the binary smaller.

Rejected: `reqwest` with `tokio`. It is the heavier stack for no gain at this concurrency. `reqwest`'s blocking client also starts a hidden runtime.

### 4.2 The API

```rust
/// One per process. Shared by reference across legs and threads.
pub struct Fetch { /* ureq agent, reply cache, mode, quota state, secrets, clock */ }

pub struct FetchConfig {
    pub mode: Mode,
    pub cache_path: PathBuf,
    pub secrets: Secrets,
    pub proxy: Option<ProxyUrl>,
    pub max_body: u64,
    pub source_deadlines: BTreeMap<Source, Duration>,
    pub sync_budget: Duration,
    pub auto_sync_max_bytes: u64,
}

pub enum Mode {
    Live { fresh: bool },              // fresh refetches and rewrites the cache
    Offline,                           // cache and bundles only; a miss fails the leg with reason offline
    Record { dir: PathBuf },           // live, and writes each exchange to dir
    Replay { dir: PathBuf },           // recordings only; a miss fails with reason replay_miss
}

impl Fetch {
    pub fn open(config: FetchConfig) -> Result<Self, FetchError>;
    pub fn deadline(&self, budget: Duration) -> Deadline;
    pub fn leg(&self, spec: &'static SourceSpec, deadline: Deadline) -> Leg<'_>;
    pub fn sync<R: Row>(&self, bundle: &'static Bundle<R>) -> Result<BundleInfo, SourceError>;
}

/// A raw reply type that knows whether a decoded reply is a real answer.
pub trait Reply: DeserializeOwned {
    /// Refuse a 2xx reply that carries an error or lacks the fields an answer needs.
    fn check(&self) -> Result<(), BadReply>;
}

/// One source's work inside one command.
pub struct Leg<'a> { /* fetch, spec, deadline, trace */ }

impl Leg<'_> {
    pub fn json<T: Reply>(&self, ep: &Endpoint, params: &[(&str, Arg<'_>)]) -> Result<T, SourceError>;
    pub fn json_opt<T: Reply>(&self, ep: &Endpoint, params: &[(&str, Arg<'_>)]) -> Result<Option<T>, SourceError>;   // 404 is None
    pub fn xml<T: Reply>(&self, ep: &Endpoint, params: &[(&str, Arg<'_>)]) -> Result<T, SourceError>;
    pub fn post_json<B: Serialize, T: Reply>(&self, ep: &Endpoint, body: &B) -> Result<T, SourceError>;
    pub fn rows<R: Row>(&self, bundle: &'static Bundle<R>, find: Find<'_>) -> Result<Vec<R>, SourceError>;
    /// Run one call per chunk on scoped threads, each under the source's quota. Returns in chunk order.
    pub fn batch<C: Sync, T: Send>(&self, chunks: &[C], call: impl Fn(&Leg<'_>, &C) -> Result<T, RequestError> + Sync) -> Vec<Result<T, RequestError>>;
    pub fn finish(self) -> LegReport;      // state, requests, cache hits, stale flag, elapsed, trace
}

pub enum Arg<'a> { Text(&'a str), Num(u64), Ids(&'a [Box<str>]) }   // Ids are sorted and joined with commas
```

`Leg::batch` bounds scoped workers with `MAX_BATCH_WORKERS`, independently of provider quota admission. Each worker takes successive chunks in input order by its assigned stride; results return in input order. A callback leg runs a nested batch inline. Each outer invocation owns its worker bound, and independent invocations share the provider quota. Empty input returns empty. Otherwise missing credentials, a foreign clock domain and an exhausted budget refuse each chunk in that order before its callback runs. A source failure remains that chunk's normal `RequestError` result. A callback panic propagates after all scoped workers finish. Transport deadlines cannot interrupt arbitrary callback computation or blocking.

Callback legs share the parent's retained stale report and absolute deadline. Replay workers fork at the parent's current instant, retain their clock across assigned chunks and join their greatest completion into the parent, including progress before a callback panic. Provider permits retain released virtual slot completion times. A later replay request cannot reuse a slot before that completion; it also observes token spacing and FIFO admission. Thread arrival can conservatively delay virtual work. No deterministic event scheduler or ideal parallel schedule is promised.

A source and its endpoints are statics and constants built with `const fn`. A quota is one provider limit, named by a source key and shared by every source that draws on it.

```rust
/// The name of one provider limit, such as `ncbi-eutils`. The adapter author picks it from the provider's documented quota.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceKey(&'static str);
impl SourceKey {
    /// Accepts lowercase letters, digits and hyphens. Called only in quotas.rs, in a const.
    pub const fn declare(name: &'static str) -> Self;
}

/// One provider limit with its unkeyed and keyed rates.
pub struct Quota {
    pub key: SourceKey,
    pub rate: Rate,                    // without the user's key
    pub keyed_rate: Option<Rate>,      // with it
    pub in_flight: u8,
}

pub struct SourceSpec {
    pub terms: Terms, // publisher name, terms page and reviewed Use category
    pub source: Source,
    pub base: &'static str,
    pub key: KeyUse,
    pub quota: &'static Quota,
    pub deadline: Duration,            // default leg budget
}

/// Whether a source takes the user's key, and whether it runs without one.
pub enum KeyUse {
    None,
    Optional(Credential),              // runs without the key at the unkeyed rate
    Required(Credential),              // never called without the key
}
pub struct Credential { pub secret: SecretName, pub place: KeyPlace }
pub enum KeyPlace { Query(&'static str), Header(&'static str) }

pub struct Rate { per_second: u16, burst: u16 }
pub enum Freshness { Never, Minutes(u16), Hours(u16), Days(u16) }
pub enum Method { Get, PostForm, PostJson }

pub struct Endpoint { pub spec: &'static SourceSpec, pub method: Method, pub path: &'static str, pub fresh: Freshness, pub idempotent: bool }

impl SourceSpec {
    pub const fn get(&'static self, path: &'static str) -> Endpoint;
    pub const fn post_form(&'static self, path: &'static str) -> Endpoint;
    pub const fn post_json(&'static self, path: &'static str) -> Endpoint;
}
impl Endpoint { pub const fn fresh(self, fresh: Freshness) -> Self; }
```

The quotas live in one file, `biomcp-adapters/src/quotas.rs`, so two sources on one provider can never declare two limits:

```rust
pub(crate) static NCBI_EUTILS: Quota = Quota { key: SourceKey::declare("ncbi-eutils"), rate: Rate::per_second(3), keyed_rate: Some(Rate::per_second(10)), in_flight: 3 };
pub(crate) static PUBTATOR: Quota = Quota { key: SourceKey::declare("pubtator"), rate: Rate::per_second(3), keyed_rate: None, in_flight: 3 };
```

### 4.3 Keyed sources

A source whose terms require the user's own license or account, such as OncoKB, declares `KeyUse::Required`, as `decisions/2026-10-24-keyed-sources-run-only-with-the-user-s-key.md` says. Each slice names its keyed sources. A free source whose key only raises the limit, such as NCBI E-utilities or Semantic Scholar, declares `KeyUse::Optional`: it runs without a key at its unkeyed rate, and a 429 that outlasts the retries adds one `key` step to the response.

- `Fetch::leg` checks the key before anything else. When the key is not configured, the leg is skipped. Every call on it returns `SourceError` with reason `no_key` and touches no network, cache entry or recording. This holds for the source's public endpoints too.
- The fan-out sees the same check first and records the source as `skipped` with reason `no_key` without starting a thread.
- A requested section from that source reports `Skipped` with reason `no_key`. The source outcome reports `skipped`, once per envelope. The response's next steps include one `key` step, `biomcp help key <source>`, whose help leads with `biomcp key set <source>` and also names the config key and the environment variable.
- A skip changes no status. A card whose only gap is a skipped keyed source is complete.
- Replay cases for a keyed source set a placeholder key in the case's temporary config. The reply key never includes a credential, so the recording matches.


### 4.4 The request path

Every call on a `Leg` runs the same steps in this order:

1. Build the URL from the base, the path and the parameters. An `Arg::Ids` list is sorted before joining, so the same set of IDs always makes the same request. Add the user's key from `Secrets` when the spec takes one and it is configured. A leg whose required key is missing never reaches this step (section 4.3).
2. Compute the reply key: SHA-256 over the method, the URL without the credential parameter and the body.
3. In replay mode, take the recording for the key, or fail with `replay_miss`. Skip to step 8.
4. Look up the reply cache. A fresh entry skips to step 10. In offline mode, a stale entry skips to step 10 marked stale, and a miss fails with `offline`.
5. Wait for a FIFO permit from the source's quota: a rate slot and an in-flight place. The wait is bounded by the leg's deadline.
6. Send. The request's timeouts bound DNS resolution, connect, receiving the response head and receiving the body, and the overall timeout equals the deadline's remaining time.
7. Retry as section 4.6 says.
8. Check the content type and read the body up to the size limit.
9. Decode the body into the caller's raw type and run its `check`. A decode failure or a failed check is `bad_reply`.
10. Store the reply. Only a 2xx reply that decoded and passed its check is stored, with its endpoint's freshness. A 404 is stored with a fixed short freshness of ten minutes, so a repeated miss costs no request and a new record appears soon. No other status is ever stored. In record mode, write the recording file whatever the status.
11. Map the status into a `SourceError`, or return the decoded value.

A 200 reply that carries an error, such as an NCBI reply with an `ERROR` field and no ID list, therefore fails its check, is never cached and reaches the agent as `bad_reply`. A recorded case holds exactly that reply.

When steps 5 to 9 fail and the cache holds a stale entry for the key, the leg serves the stale entry and reports `stale` with the failure's reason. A source failure never becomes an empty success.

### 4.5 Quotas and concurrency

Limiters are keyed by a source key and never by a URL, a host name or an endpoint.

- Each adapter's `SourceSpec` names the `Quota` that matches the provider's documented limit, and each `Quota` names its `SourceKey`. Sources that share one provider limit declare the same quota. PubMed, NCBI Gene and ClinVar all draw on E-utilities, so all three use `ncbi-eutils`, and a gene leg and an article leg in one command cannot together exceed NCBI's limit. Sources on one host with separate limits declare separate keys. PubTator sits on an NCBI host with its own limit, so it uses `pubtator`.
- `Fetch` holds one limiter per `SourceKey`, built from its `Quota` on first use. The limiter's constructor takes only a `&'static Quota`, so nothing can derive a limiter from a URL or host. The ratchet test checks that `SourceKey::declare` and `Quota` statics appear only in `quotas.rs` and that no two quotas declare the same key.
- Each limiter applies the keyed rate when the leg's source has its key configured, and the unkeyed rate otherwise.
- Permits are served in arrival order. A waiter takes a ticket under a `Mutex`, and a `Condvar` wakes tickets in order. A caller sleeps for its rate slot outside the lock.
- Each leg runs on its own thread, and only sources on the same quota wait for each other. A slow or rate-limited source therefore delays only the legs that share its provider.
- The article slice declares `ncbi-eutils` (3 per second, 10 with a key), `europepmc` (assumed 10 per second), `pubtator` (3 per second), `litvar` (assumed 3 per second), `semantic-scholar` (1 request every 2 seconds without a key, 1 per second with the optional key), `icite` (assumed 5 per second) and `pmc-oa` (assumed 10 per second). Each assumed rate is confirmed against the provider before its adapter lands.
- A recorded case sends eight parallel calls to one E-utilities quota without a key and asserts from `--explain` that their start times keep the quota's rate. 0.9 drew seven 429 replies from the same pattern.
- The limiter is per process. Several BioMCP processes sharing a key can exceed a provider's limit together. The provider then answers 429, and step 7 retries within the deadline.

### 4.6 Retries

- Retry on 429, 500, 502, 503, 504, a reset connection and a connect timeout, when the endpoint is idempotent. All GET endpoints are. A POST that only reads, such as ESummary, declares itself idempotent.
- At most three attempts. Backoff starts at 250 ms and doubles. A deterministic jitter from the reply key spreads retries without a random number crate.
- `Retry-After` in seconds or as an HTTP date replaces the backoff. When the wait would pass the deadline, the leg fails at once with `rate_limited` and the stated wait. One function in `retry.rs` parses the header.
- A 4xx other than 404 and 429 is `rejected` and never retried. A 404 is `not_found`, which `json_opt` turns into `None`.

### 4.7 Deadlines and time

The command has one budget, 20 seconds by default and set in config. Each leg's deadline is the smaller of the command budget and the source's own budget. A leg that runs out fails with `deadline_exceeded`, and the others continue. Every leg ends by its deadline, so a command ends within its budget plus a small margin. Bundle syncs run under their own `sync_budget` (section 4.11).

`Deadline` reads a `Clock` owned by `Fetch`. In live and record modes the clock is real time. In replay mode the clock is virtual, and nothing sleeps:

- Each leg carries its own virtual time, starting at zero.
- A request asks its quota for a slot at the leg's virtual time. The quota grants the later of that time and its next free slot, and moves its next free slot forward by one rate interval.
- The recorded `delay_ms`, any backoff and any `Retry-After` wait advance the leg's virtual time.
- Deadlines compare against the leg's virtual time.

A recording with a 30-second delay therefore produces `deadline_exceeded` at once, and quota spacing shows in `--explain` exactly as it would live. Thread scheduling can change which parallel leg gets the earlier slot, so cases assert spacing and outcomes and never assert which of two parallel legs started first.

```rust
pub struct Deadline { /* end instant or virtual budget */ }
impl Deadline {
    pub fn remaining(&self) -> Duration;
    pub fn expired(&self) -> bool;
    pub fn child(&self, cap: Duration) -> Deadline;     // the smaller of the two
}
```

### 4.8 The reply cache

The cache stores checked reply bodies keyed by reply key, in the `reply` table of the cache file (section 7). Each endpoint declares how long its replies stay fresh. A literature summary may stay fresh for thirty days and a trial's status for a day. `Freshness::Never` skips the cache.

`--fresh` sets `Mode::Live { fresh: true }` for a CLI process. Shared MCP calls set `Leg::fresh` on their own source legs. Both policies skip step 4 and rewrite the entry; a batch child inherits its leg policy. Decoding and the check run on every read, so a raw type change never needs a cache migration.

### 4.9 Record, replay and offline

Offline, record and replay are user features, so no test hook exists in product code. `--offline` answers from the cache and bundles. `--record DIR` lets a user capture a failing command and attach the folder to a bug report. `--replay DIR` reruns it anywhere. The runner uses the same flags, and `biomcp serve --replay DIR` serves MCP over recordings.

Replay uses a temporary owned cache by default. `--case-cache DIR` is an explicit process setting available only with replay or offline. `ReplayCache::create_case` exclusively creates a marked directory with its fixed cache filename; `open_case` refuses unmarked, foreign and symlinked paths or files. It never accepts an arbitrary SQLite filename or the ordinary user cache. Later offline processes may open the same marked directory. Tool arguments cannot select record, replay or case-cache paths.

A recording is one JSON file per exchange, at `<dir>/<source>/<key>.json`:

```json
{
  "request": { "method": "GET", "url": "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=pubmed&term=AKT1+E17K&sort=relevance&retmode=json&retstart=0&retmax=10000" },
  "replies": [
    { "status": 429, "retry_after_s": 1, "delay_ms": 120 },
    { "status": 200, "content_type": "application/json", "delay_ms": 412, "json": { "esearchresult": { "count": "192", "idlist": ["33123537", "17611497"] } } }
  ]
}
```

- A JSON body is embedded as JSON under `json`, so a diff shows the changed field. An XML or text body sits under `text`. A binary body sits in a bounded relative per-exchange `.bin` sidecar, published atomically; repeated exchanges never overwrite earlier bytes, and replay refuses missing, linked or escaping sidecars.
- `replies` serves in order within one run, so a retry sequence is one file.
- A failure case is an ordinary file: a status, a large `delay_ms` for a timeout, a cut `text` for a broken body, or a 200 whose `json` carries the provider's error field.
- Recorded URLs never hold a credential, because step 2 removes it before the key and the file exist. Batch ID lists are sorted before keying, so a recording matches whatever order the caller asked in.
- A recorded bundle holds a slice of the real published file with the rows its cases need.

### 4.10 Credentials and proxies

`Secrets` maps a `SecretName` enum to a `Secret(Box<str>)` whose `Debug` and `Display` print `***`. Only the binary's config module builds `Secrets`, from the config file and the documented environment variables. Every declared key uses the same per-command deferred loader, which resolves the config value before its environment fallback when the source is admitted, including raw-cache and replay validation. Disabled sources and stored query pages do not resolve keys. The secret enters only the outgoing request. It never enters a cache key, a recording, a trace, an error or the envelope. The config file's `proxy` key overrides all proxy environment settings. Otherwise `config.rs` reads `HTTP_PROXY`, `HTTPS_PROXY` and `NO_PROXY` and passes checked routes to Fetch. Proxy credentials and CA paths never enter reply identities or recordings.

### 4.11 Bundles

A bundle is a published file that BioMCP downloads and queries locally, as `decisions/2026-10-23-data-bundles-are-local-apis.md` says. The transport owns its download, its revalidation and its rows.

```rust
/// A bundle row type: its raw shape and the keys that find it.
pub trait Row: DeserializeOwned {
    fn keys(&self) -> Keys;
}

pub struct Bundle<R: Row + 'static> {
    pub spec: &'static SourceSpec,
    pub name: &'static str,                      // the name `biomcp sync` takes
    pub path: &'static str,                      // the published file under the spec's base
    pub format: RowFormat,                       // JsonArray { pointer }, JsonLines, Tsv { list_separator }, Csv
    pub fresh: Freshness,
    pub row: PhantomData<R>,
}

pub struct Keys { pub exact: Vec<(KeyKind, Box<str>)>, pub words: Vec<Box<str>> }
pub enum KeyKind { Id, Symbol, PreviousSymbol, Alias, Xref, Band }
pub enum Find<'a> { Exact(KeyKind, &'a str), Prefix(KeyKind, &'a str), Words(&'a str) }
```

`Fetch::sync` downloads the file with an ETag or date check, splits it into rows, calls `keys` on each row, and stores each row's original text, its exact keys and its words in one transaction. `Leg::rows` finds rows by an exact key, by a key prefix such as `9p21` for every band inside it, or by words through an SQLite FTS5 index. The rows deserialize into `R`, and the adapter maps them like any reply. A query therefore reads a few indexed rows and never parses the whole file. The bundled SQLite build includes FTS5, and the first bundle stage confirms it with a case.

Sync rules:

- `biomcp sync` with no name syncs every bundle the enabled sources use. The install guide recommends running it once after install.
- Explicitly synchronized bundle reads, including EMA, use `Leg::stored_rows` or the complete-set stored reader. They never synchronize, including under fresh or replay settings. Missing bundles return `syncing` with an explicit sync step; stale rows remain readable with a stale marker.
- The older `Leg::rows` path still synchronizes absent or stale data. HGNC and ClinGen call this path. `auto_sync_max_bytes` remains active in `bundle/download.rs`: it bounds those automatic transfers and rejects a known oversized prior bundle before download. It does not control stored-bundle reads or explicit sync. Offline mode never syncs.
- A sync takes a lock row in `bundle_sync`. Another process that finds the lock held answers `syncing` with a retry step, and a lock older than the sync budget is taken over.
- An explicit `biomcp sync` runs under `sync_budget`, separate from the command budget. An automatic sync on first use runs inside the remaining budget of the leg that needs the rows, so a command never outlives its deadline; a first download that does not finish answers `syncing` with a retry step and resumes on the next call. The download writes to a part file in the cache folder and resumes with a range request when the server supports one. A sync that runs out of budget leaves the part file, answers `syncing` with a retry step, and the next sync resumes it.
- The older automatic read path may return stale stored rows when its refresh fails. Stored-bundle reads do not attempt that refresh.

### 4.12 Errors

```rust
#[derive(Debug, thiserror::Error)]
#[error("{source}: {reason}")]
pub struct SourceError { pub source: Source, pub reason: Reason, pub status: Option<u16>, pub retry_after: Option<Duration> }

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum FetchError {                 // opening the transport only
    #[error("fetch.cache_open")] CacheOpen(rusqlite::Error),
    #[error("fetch.cache_version")] CacheVersion,
    #[error("fetch.tls")] Tls,
    #[error("fetch.recordings")] Recordings(std::io::Error),
}
```

`SourceError` reuses `biomcp_types::Reason`, so a failure travels from the transport into the envelope without a second enum.

### 4.13 AlphaGenome streaming transport

Ian approved this design in [AlphaGenome joins 1.0 through a streaming gRPC transport](../decisions/2026-10-27-alphagenome-streaming-transport.md). That decision authorizes the shared streaming transport and candidate dependencies below as an explicit exception to the blocking transport and no-runtime rule. It replaces the earlier recommendation to drop AlphaGenome. The decision governs the work order and release timing.

Transport implementation and cost and target proof use zero provider calls. The variant section remains separate work, and a clinical reviewer must choose the public variant and outputs before any separately authorized capture. Released 0.9.3 includes AlphaGenome by default, so shipping 1.0 without it loses released behavior. The release must disclose that loss. This design makes no claim about clinical value or measured performance.

#### Protocol evidence

The official repository was read at commit `55943112b83439e0c2512110f1ae607d4b5b492a`. The root [client](https://github.com/google-deepmind/alphagenome/blob/55943112b83439e0c2512110f1ae607d4b5b492a/src/alphagenome/models/dna_client.py) and [service proto](https://github.com/google-deepmind/alphagenome/blob/55943112b83439e0c2512110f1ae607d4b5b492a/src/alphagenome/protos/dna_model_service.proto) forward to `v1`. Those forwarding files cannot establish the transport contract alone.

The substantive [v1 client](https://github.com/google-deepmind/alphagenome/blob/55943112b83439e0c2512110f1ae607d4b5b492a/src/alphagenome/models/v1/dna_client.py) opens TLS to `gdmscience.googleapis.com:443` and sends `x-goog-api-key` metadata. It sends one request message on each prediction stream. Its unlimited message settings and retry decorator are provider-client policies that BioMCP must not copy. The [v1 service](https://github.com/google-deepmind/alphagenome/blob/55943112b83439e0c2512110f1ae607d4b5b492a/src/alphagenome/protos/v1/dna_model_service.proto) declares bidirectional `PredictVariant` and `ScoreVariant` streams. Responses interleave output descriptors and tensor chunks. The method for a future section remains a separate product and clinical review choice.

The [tensor schema](https://github.com/google-deepmind/alphagenome/blob/55943112b83439e0c2512110f1ae607d4b5b492a/src/alphagenome/protos/tensor.proto) supports inline bytes and counted chunks with per-chunk ZSTD compression. The [tensor decoder](https://github.com/google-deepmind/alphagenome/blob/55943112b83439e0c2512110f1ae607d4b5b492a/src/alphagenome/tensor_utils.py) supplies dtype and byte-layout behavior. A protobuf decoder alone cannot produce the requested numbers. These facts establish protocol requirements without choosing a variant, assembly, interval, output or scorer.

The source reads produced these SHA-256 values. They identify reviewed public files, not captured API replies.

| File under `src/alphagenome/` | SHA-256 |
| --- | --- |
| `models/v1/dna_client.py` | `63b67596e4e62cc3aecb59f427965919185136c304d1b41d0e9a0a8ac3f81a7f` |
| `protos/v1/dna_model_service.proto` | `d487d76600e7e25413780e8dfb63c0129ca8449e3b3baee10ef6d3caf0927f86` |
| `protos/v1/dna_model.proto` | `cce623263fe102712673d79ed861a07388b9841ef8bf310edc9f0d9f1154f6d9` |
| `protos/tensor.proto` | `07779023b2868377cbfc3c2ce96cd266ae425a0a1116aea755691c263d6238f7` |
| `tensor_utils.py` | `aa9f278730e96c4dee7381491aa02749c6dc7dba229f7e238b4931060584b25f` |

The [gRPC HTTP/2 protocol](https://github.com/grpc/grpc/blob/276738ec84e8e11de4b2044949061821cc697738/doc/PROTOCOL-HTTP2.md) requires message framing and final gRPC status. HTTP 200 alone is insufficient. JSON over the existing HTTP client is not a substitute. A handwritten HTTP/2, HPACK and gRPC implementation would create a protocol maintenance burden. A provider SDK and a subprocess running the Python client both violate the shared transport boundary.

#### Released baseline and candidate cost

The [0.9.3 release](https://github.com/genomoncology/biomcp/releases/tag/v0.9.3) points to commit `5641de7d7b79b53872f8599dbb4dba22e611dcaf`. Its pinned [manifest](https://github.com/genomoncology/biomcp/blob/5641de7d7b79b53872f8599dbb4dba22e611dcaf/Cargo.toml) enables `alphagenome` in default features, brings `tonic`, `prost`, `zstd` and `tokio-stream` through that feature, and includes Tokio independently. Its [lockfile](https://github.com/genomoncology/biomcp/blob/5641de7d7b79b53872f8599dbb4dba22e611dcaf/Cargo.lock) resolves tonic 0.12.3, prost 0.13.5, zstd 0.13.3 and tokio 1.50.0. That released stack is an existing implementation and the first cost baseline to inspect. The newer candidates below are not required upgrades.

The pinned [release workflow](https://github.com/genomoncology/biomcp/blob/5641de7d7b79b53872f8599dbb4dba22e611dcaf/.github/workflows/release.yml) builds with default features for Linux x86_64 and arm64, macOS x86_64 and arm64, and Windows x86_64. The release publishes native archives for all five targets. This establishes that the older AlphaGenome stack packaged across the promised native targets. The published archive sizes describe the entire executable package; they do not measure AlphaGenome's incremental size or build time. Prior packaging reduces uncertainty about feasibility. It does not prove the new shared transport's platform trust, proxy, deadline, replay or recording contracts.

The decision approves these candidate dependencies. This design makes no manifest changes and does not establish a resolved dependency tree. Only `biomcp-fetch` would depend on the client stack. The adapters would receive checked messages through the shared `Leg` interface. Generated provider message types would remain private to the source adapter and its transport codec boundary.

| Component | Reviewed primary source and proposed role | License evidence | Size and build time |
| --- | --- | --- | --- |
| `tonic` and `tonic-prost` 0.14.7 | [tonic manifest](https://github.com/hyperium/tonic/blob/818532da3abfa6068b0855f483e3c627d9e14640/tonic/Cargo.toml), [codec manifest](https://github.com/hyperium/tonic/blob/818532da3abfa6068b0855f483e3c627d9e14640/tonic-prost/Cargo.toml); gRPC client and protobuf codec | Both declare MIT | Not measured |
| `h2` 0.4, `hyper` 1 and `hyper-util` 0.1 | Required by the reviewed tonic `channel` feature; HTTP/2 framing and connection management | Resolved versions and their license files need the prototype audit | Not measured |
| `prost` 0.14.3 | [workspace manifest](https://github.com/tokio-rs/prost/blob/fafa97f3e05b9ffd84769c7c606499243a6fa614/Cargo.toml); generate and decode protobuf messages | Apache-2.0 | Not measured |
| `tokio` 1.48.0 reference | [manifest](https://github.com/tokio-rs/tokio/blob/556820ff84030b37e74e11b86b7733f5795770ea/tokio/Cargo.toml); runtime, network, time and bounded channel support | MIT | Not measured; this reference is not a compatibility-tested selection |
| `tokio-rustls` 0.26 with existing rustls and platform verifier | Tonic manifest declares the TLS integration; preserve ring, platform trust and additive CA behavior | Full resolved license audit remains required | Not measured |
| `zstd` 0.14.3 without default features | [wrapper manifest](https://github.com/gyscos/zstd-rs/blob/cdfeaad8cea22ede46b23a146798e8ce0f5d8f6b/Cargo.toml); decode tensor chunk compression | BSD-3-Clause wrapper; [upstream ZSTD license](https://github.com/facebook/zstd/blob/f8745da6ff1ad1e7bab384bd1f9d742439278e99/LICENSE) offers BSD terms | Not measured; C compilation and static linkage need proof |

Tonic would disable default features and request client `channel`, `codegen` and `tls-ring` support. Server, router, webpki-only roots and gRPC compression features are unnecessary. Tensor ZSTD is separate from gRPC compression. A custom connector must reuse checked proxy routes and the existing platform verifier plus additive CA roots. Tonic's ordinary native-root option alone does not prove that behavior. Compatibility of the connector, verifier and runtime remains a prototype question.

The prototype would resolve the complete target-specific dependency tree into a lockfile, review every license and advisory, and pin the generation toolchain. A reviewed generation step would use the complete transitive proto import closure from the pinned provider commit and preserve Apache notices. Generated code would be committed so package builds need neither `protoc` nor network access. Regeneration would compare against those pinned inputs. No generated messages or provider code are added by this proposal.

Compressed crate downloads, source expansion, transitive crate counts, incremental executable size, installed package size, cold and warm build time, and peak build memory are all **not yet measured**. Source-file byte counts are not dependency costs. No published benchmark establishes these costs for this workspace. The first authorized measurement should compare pinned 0.9.3 with and without its `alphagenome` feature under the same toolchain, profile, host and build-cache conditions. It must preserve every other feature, including charts, and make no provider calls. Since Tokio serves other 0.9.3 functions, this comparison cannot isolate the cost of adding a runtime to the blocking 1.0 transport. The next comparison must measure the unchanged 1.0 baseline against its candidate under the same conditions. It must retain raw build timings and size output through the ordinary job evidence. The PM must review those results as part of the approved cost and target proof.

#### Shared transport semantics

`Fetch` would retain ownership of admission, settings, clocks, quotas, checked cache, recordings and errors. A lazy worker inside `biomcp-fetch` would own one runtime and its connection state. Source legs would remain blocking callers. A bounded command channel and response channel would prevent unbounded work. Shared clones would reuse the same worker, quota and replay cursors. Worker shutdown would cancel pending streams and join its thread. It must never detach work beyond the caller's deadline. Replacing all existing HTTP calls with the new stack is outside this proposal.

Source disable checks precede deferred credential resolution. The shared loader reads config before `ALPHAGENOME_API_KEY` only when AlphaGenome is admitted. A missing key skips live, cached, offline and replay reads under the existing keyed-source rule. Tests use a placeholder key in isolated config. The key goes only into outgoing metadata. Request identities, errors, traces and recordings exclude it. Provider text and trailers receive allowlist filtering because they can echo request metadata. AlphaGenome retains the approved restricted terms label and source switches.

One declared provider quota in `quotas.rs` would cover every AlphaGenome RPC and metadata call. The shared permit covers the entire stream through terminal status or cancellation. Chunks consume no separate permits. No metadata discovery, readiness probe, health call or reconnect may silently issue another RPC. The provider's key-specific quota is not established by this research; a future implementation must cite the documented applicable quota before selecting its declaration. Other processes sharing a key can still exhaust it.

The transport would make one application attempt. It would disable retries, hedging, reconnect-and-resend and provider service-config retries for this protocol. HTTP retry handling in section 4.6 would not wrap an RPC. `RESOURCE_EXHAUSTED` becomes `rate_limited`; authentication failures become `rejected`; `UNAVAILABLE` becomes a source failure; deadline and cancellation retain their own reasons. Retry metadata can inform the caller but cannot schedule another attempt. An interrupted stream yields no successful partial result.

The existing absolute leg deadline would cover queue admission, quota wait, DNS, proxy CONNECT, TLS, stream establishment, upload, every received message, decompression, validation and final trailers. `grpc-timeout` communicates the remaining budget; a local absolute timeout also cancels the stream and releases the permit. Each read must use the remaining total budget. Receiving a chunk must not reset it. Bounded decoding must check the deadline between chunks. Blocking DNS, C decompression and worker shutdown need explicit deadline proof; a timeout around a caller waiting on an uncancellable worker is insufficient.

Reply identity would include protocol version, authority, full service/method path, pinned schema identity and the exact credential-free request bytes. Repeated fields keep their declared order unless the source contract proves order irrelevant. Request serialization must be stable and map fields must be canonicalized. Output selectors, model version and all contextual request fields participate. A changed schema or method must never reuse an old HTTP or RPC key.

Only a complete stream with successful terminal gRPC status, valid message ordering, exact declared chunk counts, checked tensor dimensions and a passing source identity check can enter the reply cache. HTTP 200, an early output message or clean TCP EOF does not establish success. A gRPC `NOT_FOUND` must not inherit HTTP negative-cache behavior without a source contract. Provider-default model requests use `Freshness::Never` until their reproducibility and freshness contract is reviewed. Other failures are uncached. Existing checked stale fallback may apply only to a complete prior compatible entry and must disclose the new failure. Offline mode never starts the worker's network path.

Recordings would extend the existing exchange format with a versioned gRPC protocol tag, credential-free request identity, selected safe HTTP metadata, ordered bounded protobuf sidecars, message delays, terminal gRPC status and a typed interruption reason. They would use the existing safe relative-sidecar and atomic-publication rules. An interrupted capture remains a failed exchange with its bounded prefix; it cannot masquerade as a complete cached stream. Failure to record remains a local error. Replay would drive the same decoder, message-order checks and cache-admission predicate, advance the shared virtual clock per recorded event, require terminal status and fail closed on missing or invalid sidecars. It would make no connection or SDK call. Recording order and terminal markers must survive concurrent calls without sharing mutable sequence state between streams.

#### Bounds and release proof

Streaming does not establish a memory bound. Before implementation, one transport-owned limits type must define request bytes, header and trailer bytes, individual protobuf bytes before decoding, total wire bytes, message count, metadata rows, tensor rank and dimensions, chunk count, total expanded tensor bytes, decoder window memory, result rows, serialized output bytes, queued calls and in-flight streams. Each allocation must check its relevant limit first. Tensor shape multiplication and all byte sums use checked arithmetic. Dtype width, chunk order and final expanded byte count must agree. Unknown dtypes, compression, payload variants and inconsistent counts fail as `bad_reply`. Limits must also apply on cache and replay reads.

The current `max_body` supplies a total wire ceiling; it cannot substitute for expanded bytes or decoder window limits. Decompression must stream into a capped destination and enforce a ZSTD window ceiling. The implementation must avoid holding an entire raw stream, all expanded tensors and all JSON values together. A bounded spool can preserve raw replies for recording and cache admission while a single bounded tensor is decoded. Temporary disk growth needs the same total-wire ceiling and cleanup on every error. Cache publication must be atomic after final validation. Concurrency limits must multiply into a documented process memory ceiling that includes HTTP/2 windows, protobuf expansion, decoder workspace and output buffers.

Numeric settings remain unselected because there is no authorized representative capture or prototype measurement. No bound in this proposal is claimed to be enforced today. The prototype would use synthetic protocol fixtures to prove pre-allocation rejection, decompression limits, cancellation, cache admission and replay behavior. A clinical reviewer must select any later public variant, assembly, interval and outputs from cited sources before a separately authorized provider capture. A small reply chosen by an engineer cannot justify production bounds. The measured-plus-margin rule applies only after representative approved evidence exists.

The bounded prototype would first measure the released feature baseline. It would then add only the selected client and generation dependencies in an isolated branch, use a local TLS HTTP/2 test service with synthetic messages, and make zero AlphaGenome calls or key reads. It would stop after dependency measurements and the installed-package checks below. It would not implement a variant section, configure external services or publish packages. Any failure of platform trust, proxy parity, cancellation, bounds or a promised release target must return to the PM for a scope decision before product work.

Every target promised in [the release plan](release.md) must run the installed candidate: Linux x86_64 and arm64 at the declared manylinux floor, macOS x86_64 and arm64, and Windows x86_64. Tests must cover local TLS and HTTP/2 negotiation, platform roots plus an added CA, invalid-certificate rejection, explicit and environment proxy routing including exclusions, total deadlines during each stream phase, one attempt on refusal or interruption, quota permit lifetime, malformed and oversized streams, tensor decompression limits, safe recording, replay and offline behavior. Two processes must exercise the ordinary shared cache. The local fixture chooses no biological data.

Proof must install archives and wheels outside the checkout and exercise both command entry points, stdio MCP and the existing HTTP MCP path. It must also execute both container architectures, macOS Homebrew installs, the promised desktop bundles and registry-selected wheels. Package checks must establish the candidate executable's identity and required dynamic libraries, including any ZSTD C linkage, and refuse a runtime Python, SDK, `protoc` or system-ZSTD requirement. Cross-compilation and archive inspection alone cannot pass a native execution requirement. Missing hosts remain unproved release targets. The existing release rehearsal owns these checks; this proposal adds no separate evidence system.

The structural risks are duplicated protocol truth, unshared transport state and testing source builds instead of installed artifacts. Pinned proto generation, a single Fetch owner, one cache-admission predicate for live/cache/replay, and the installed-platform rehearsal address those risks. No product code, dependencies, runtime, credentials or provider calls belong to this design change.


#### AlphaGenome scoring section

The variant section uses the ruled legacy `ScoreRequest` without exposing request selectors. Mitochondrial inputs remain unsupported before RPC because the integration lacks a verified chromosome mapping and interval contract. The publisher [FAQ](https://www.alphagenomedocs.com/faqs.html) identifies the human assembly; it does not establish that mitochondrial contract. Source resolution establishes the GRCh38 allele before the scoring leg. MyVariant chromosome notation receives its existing assembly-qualified accession before CAR resolves the unchanged genomic expression. A missing key skips both resolution and scoring when AlphaGenome is the only requested section.

The pinned publisher [client conversion](https://github.com/google-deepmind/alphagenome/blob/55943112b83439e0c2512110f1ae607d4b5b492a/src/alphagenome/models/v1/dna_client.py#L332-L384) and [scorer conversion](https://github.com/google-deepmind/alphagenome/blob/55943112b83439e0c2512110f1ae607d4b5b492a/src/alphagenome/models/variant_scorers.py#L732-L807) document plane, gene row and track axis order. The response retains every native plane and metadata row. A nongene aggregate has no invented gene. Requested scorer identity and interval remain request provenance because the response has no native scorer label or interval field. The response does not add units or a returned model version.

Each scorer pages scalar rows independently through the existing section offset and limit. Every reply includes all scorer groups, their complete native shapes and dtypes, and each group's own next request. Tensor coordinates preserve native row-major order. Bounds derive from the complete approved scoring capture and apply before result publication. Licensed raw replies and numerical replay inputs remain outside Git. The CLI contracts reuse the shared local TLS peer and child environment.

## 5. Source adapters

### 5.1 The shape

An adapter is one folder with three files:

- `mod.rs` holds the `SourceSpec` static, the endpoint constants and one short public function per question the source answers. Each function builds parameters, makes one or two `Leg` calls and hands the raw reply to `map`.
- `raw.rs` holds serde types that mirror only the fields the mapping reads, and the `Reply` or `Row` impl for each top-level type.
- `map.rs` holds pure functions from raw types to biodata values. Each failed check becomes a `Problem`.

`#[serde(default)]` is banned in `raw.rs`. A field the provider may omit is an `Option`, and the top-level `check` decides whether its absence makes the reply an error. A missing result list can therefore never decode as an empty answer.

Every present source field that cannot map is omitted and produces a `Problem` naming that field. The adapter returns the remaining item through `Mapped { value, problems }`. A missing optional field produces no problem. An invalid required identity still fails the item. Mapping never guesses a value from prose. Optional collection members follow the same rule individually.

`mapped.rs` owns rejected-field construction, deduplication and text validation. Adapters retain source-specific parsing and pass its results to that owner. Required identity text uses the strict shared conversion. Optional text uses the shared problem context, so empty or invalid text cannot fail unrelated fields.

Shared helpers live once in `biomcp-adapters/src/`:

```rust
// raw.rs: shapes many providers share
#[derive(Deserialize)] #[serde(untagged)] pub(crate) enum OneOrMany<T> { One(T), Many(Vec<T>) }
#[derive(Deserialize)] #[serde(untagged)] pub(crate) enum NumOrText { Num(u64), Text(String) }

// mapped.rs: a value with the problems found while building it
pub struct Mapped<T> { pub value: T, pub problems: Vec<Problem> }
pub struct Problems { source: Source, record: Option<Text>, list: Vec<Problem> }
impl Problems {
    pub(crate) fn required<T>(&mut self, field: &str, value: Option<T>) -> Option<T>;
    pub(crate) fn optional<T, E>(&mut self, field: &str, value: Option<Result<T, E>>) -> Option<T>;
}

// one page from one source, before any merge
pub struct SourcePage<T> { pub items: Vec<Mapped<T>>, pub total: Option<u64> }
```

`SourcePage` and `Mapped` stay in the adapters crate, because no output shows them.

### 5.2 Worked example: HGNC, a local bundle that owns gene identity

HGNC owns gene identity and resolution, as `planning/slices/gene.md` says. It ships as two bundles: the CC0 complete set of about 17 MB and the withdrawn file. Both sit under the default threshold, so they sync on first use.

```rust
// crates/biomcp-adapters/src/hgnc/mod.rs
//! HGNC: approved human gene symbols, previous symbols, aliases, bands and cross-references.

pub(crate) static SPEC: SourceSpec = SourceSpec {
    source: Source::Hgnc,
    base: "https://storage.googleapis.com/public-download-files/hgnc/tsv/tsv/",
    key: KeyUse::None,
    quota: &quotas::HGNC,
    deadline: Duration::from_secs(5),
};

pub(crate) static COMPLETE: Bundle<raw::GeneRow> = Bundle {
    spec: &SPEC,
    name: "hgnc",
    path: "hgnc_complete_set.txt",
    format: RowFormat::Tsv { list_separator: '|' },
    fresh: Freshness::Days(7),
    row: PhantomData,
};

/// Every HGNC record an input can name, with how it matched.
pub fn lookup(leg: &Leg<'_>, input: &GeneInput) -> Result<Vec<(Mapped<Gene>, MatchKind)>, SourceError> {
    let finds = map::finds(input);                  // ID inputs give one exact find; text gives symbol, previous symbol, alias
    let mut out = Vec::new();
    for (find, kind) in finds {
        out.extend(leg.rows(&COMPLETE, find)?.into_iter().filter_map(map::gene).map(|g| (g, kind)));
    }
    Ok(out)
}

/// Genes in a band or any band inside it.
pub fn in_band(leg: &Leg<'_>, band: &Cytoband) -> Result<Vec<Mapped<GeneHeader>>, SourceError>;

/// Genes whose symbols, aliases or names hold these words.
pub fn by_words(leg: &Leg<'_>, words: &Text) -> Result<Vec<Mapped<GeneHeader>>, SourceError>;
```

```rust
// crates/biomcp-adapters/src/hgnc/raw.rs
#[derive(Deserialize)]
pub(super) struct GeneRow {
    pub hgnc_id: String,
    pub symbol: String,
    pub name: String,
    pub status: String,                         // Quirk: exact lookups include withdrawn entries. Case: gene-get/brca3-withdrawn.yaml
    pub locus_group: String,
    pub locus_type: Option<String>,
    pub location: Option<String>,
    pub alias_symbol: Option<String>,           // Quirk: a quoted, pipe-separated list
    pub prev_symbol: Option<String>,
    pub entrez_id: Option<String>,
    pub ensembl_gene_id: Option<String>,
    pub uniprot_ids: Option<String>,
    pub omim_id: Option<String>,
    pub mane_select: Option<String>,            // RefSeq and Ensembl transcripts, pipe-separated
}

impl Row for GeneRow {
    fn keys(&self) -> Keys;   // Id, Symbol, PreviousSymbol, Alias, Xref (Entrez and Ensembl), Band; words from symbol, aliases and name
}
```

`map.rs` holds `finds`, `gene`, `header` and the tables from HGNC's spellings to `GeneStatus`, `LocusGroup` and `LocusType`. A second `Bundle` constant, `WITHDRAWN`, reads HGNC's withdrawn file with its own row type, and `lookup` reads both. Resolution into resolved, ambiguous, withdrawn or not found happens in the query layer from these rows (section 6.2). The gene stage confirms both download URLs.

### 5.3 Worked example: MyGene.info, a simple remote source

MyGene.info supplies the RefSeq summary and the spans on both builds for genes HGNC has already resolved. It is called by Entrez ID, never by symbol, because its symbol queries rank aliases badly.

```rust
// crates/biomcp-adapters/src/mygene/mod.rs
//! MyGene.info: RefSeq summary and GRCh38 and GRCh37 spans for resolved genes.

pub(crate) static SPEC: SourceSpec = SourceSpec {
    source: Source::MyGene,
    base: "https://mygene.info/v3/",
    key: KeyUse::None,
    quota: &quotas::BIOTHINGS,
    deadline: Duration::from_secs(8),
};

const GENES: Endpoint = SPEC.post_form("gene").fresh(Freshness::Days(7));
const FIELDS: &str = "summary,genomic_pos,genomic_pos_hg19";
const BATCH: usize = 200;

/// Summary and spans for each Entrez ID, keyed by ID. IDs MyGene lacks are absent.
pub fn genes(leg: &Leg<'_>, ids: &[EntrezGeneId]) -> Result<Vec<Mapped<GeneExtras>>, SourceError> {
    let ids: Vec<Box<str>> = ids.iter().map(|id| id.to_string().into()).collect();
    let chunks: Vec<&[Box<str>]> = ids.chunks(BATCH).collect();
    let pages = leg.batch(&chunks, |leg, chunk| {
        leg.json::<raw::Genes>(&GENES, &[("ids", Arg::Ids(chunk)), ("fields", FIELDS.into()), ("species", "human".into())])
    });
    map::extras(pages)
}
```

```rust
// crates/biomcp-adapters/src/mygene/raw.rs
#[derive(Deserialize)]
#[serde(transparent)]
pub(super) struct Genes(pub Vec<Hit>);

impl Reply for Genes {
    fn check(&self) -> Result<(), BadReply> { /* a list; each hit has a query field */ }
}

#[derive(Deserialize)]
pub(super) struct Hit {
    pub query: String,
    pub notfound: Option<bool>,                     // Quirk: unknown IDs come back as {query, notfound: true}
    pub summary: Option<String>,
    pub genomic_pos: Option<OneOrMany<Pos>>,        // Quirk: an object or a list. GRCh38
    pub genomic_pos_hg19: Option<OneOrMany<Pos>>,   // GRCh37
}

#[derive(Deserialize)]
pub(super) struct Pos { pub chr: String, pub start: u32, pub end: u32, pub strand: Option<i8> }
```

`map::extras` builds a `GeneExtras { entrez_id, summary: Option<Summary>, spans: Box<[GenomicInterval]> }` per hit, with the build stated on each span and the summary's "provided by RefSeq" date split from its text. The query layer joins extras onto the HGNC record by Entrez ID. The adapter is about 120 lines.

### 5.4 Worked example: PubMed, a paged two-step source

PubMed answers a search in two steps. ESearch returns PMIDs in relevance order, and ESummary returns a header for each PMID.

```rust
// crates/biomcp-adapters/src/pubmed/mod.rs
//! PubMed through NCBI E-utilities: relevance-ordered PMIDs, headers and abstracts.

pub(crate) static SPEC: SourceSpec = SourceSpec {
    source: Source::PubMed,
    base: "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/",
    key: KeyUse::Optional(Credential { secret: SecretName::NcbiApiKey, place: KeyPlace::Query("api_key") }),
    quota: &quotas::NCBI_EUTILS,                // source key ncbi-eutils: 3 per second, 10 with a key
    deadline: Duration::from_secs(10),
};

const ESEARCH: Endpoint = SPEC.get("esearch.fcgi").fresh(Freshness::Hours(24));
const ESUMMARY: Endpoint = SPEC.post_form("esummary.fcgi").fresh(Freshness::Days(30));
const EFETCH: Endpoint = SPEC.post_form("efetch.fcgi").fresh(Freshness::Days(30));

/// ESearch returns at most the first 10,000 records of any search.
pub const MAX_WINDOW: u32 = 10_000;
const BATCH: usize = 200;

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

/// Headers for PMIDs. Missing PMIDs become problems.
pub fn summaries(leg: &Leg<'_>, ids: &[Pmid]) -> Result<Vec<Mapped<ArticleHeader>>, SourceError>;   // ESummary JSON in batches through leg.batch

/// Abstract text for PMIDs, with inline markup kept as text.
pub fn abstracts(leg: &Leg<'_>, ids: &[Pmid]) -> Result<Vec<(Pmid, Text)>, SourceError>;          // EFetch XML in batches
```

```rust
// crates/biomcp-adapters/src/pubmed/raw.rs
#[derive(Deserialize)]
pub(super) struct ESearch { pub esearchresult: ESearchResult }
#[derive(Deserialize)]
pub(super) struct ESearchResult {
    pub count: Option<NumOrText>,
    pub idlist: Option<Vec<String>>,
    #[serde(rename = "ERROR")]
    pub error: Option<String>,                  // Quirk: NCBI answers 200 with an ERROR field. Case: article-search/esearch-error-200.yaml
}
impl Reply for ESearch {
    fn check(&self) -> Result<(), BadReply> { /* refuse when error is present or idlist or count is absent */ }
}

/// Quirk: ESummary keys each record by its PMID and lists the order in `uids`.
#[derive(Deserialize)]
pub(super) struct ESummary { pub result: Option<SummaryResult>, pub error: Option<String> }
#[derive(Deserialize)]
pub(super) struct SummaryResult {
    pub uids: Option<Vec<String>>,
    #[serde(flatten)] pub docs: HashMap<String, DocSum>,
}

/// EFetch PubMed XML uses an explicit raw XML decoder for mixed abstracts.
#[derive(Deserialize)]
pub(super) struct ArticleSet { #[serde(rename = "PubmedArticle")] pub articles: Option<Vec<PubmedArticle>> }
#[derive(Deserialize)]
pub(super) struct AbstractLabel {
    #[serde(rename = "@Label")] pub label: Option<String>,
}
#[derive(Deserialize)]
pub(super) struct AbstractText { pub label: Option<String>, pub text: MixedText }
impl AbstractText {
    fn decode_element(xml: &str) -> Result<Self, BadReply> {
        let label: AbstractLabel = quick_xml::de::from_str(xml).map_err(|_| BadReply)?;
        Ok(Self { label: label.label, text: MixedText::from_xml(xml)? })
    }
}
```

`Leg::xml<T: XmlReply>` calls the raw type's `XmlReply::decode_xml` method. Ordinary XML replies use its bounded serde default. A raw type with mixed elements overrides this boundary, reads the element fragments with quick-xml's event Reader and calls `MixedText::from_xml` on each complete fragment. It uses serde for the remaining structured fields. The article adapter supplies this source-specific element selection when that source is built.

`MixedText::from_xml` preserves an element's text, whitespace and inline order with arbitrary nested markup removed. It keeps `BRAF<sup>V600E</sup>` as `BRAFV600E`. It ignores attributes and comments, decodes built-in and numeric references and CDATA, and returns empty text for an empty element. Its reader is iterative and bounded by the same XML nesting limit as the decoder. Generic quick-xml serde drops whitespace-only mixed events, so an arbitrary `$value` serde field cannot provide this contract. A recorded article case asserts that its abstract contains `V600E`. `#[serde(flatten)]` appears in `SummaryResult` because ESummary mixes one fixed key with a map, and raw types never use `deny_unknown_fields`.

### 5.5 How adapters are tested

Cases drive the binary over recordings, and each adapter's lines run because a case needs them. Section 10.4 lists the narrow public API and decoder boundaries that command cases cannot construct without test hooks.

- Each adapter has at least one success case per public function, recorded live with `--record`.
- Each adapter has five failure cases built from small hand-edited recordings. A synthetic recording starts as a copy of a real `--record` capture of the same command, and only its response is edited. No helper rebuilds a request's encoding, parameter order, ID order or hash, because the transport already defines them. The five cases are a 429 with `Retry-After`, a 500, a delay past the source deadline, a cut body and a 200 that carries the provider's error shape. Each asserts the item or envelope status, the source's outcome and its reason, and a later offline step shows nothing bad was cached.
- Each quirk comment names the case that fails without it. The ratchet test checks that the case file exists.
- A nightly live run replays every case live and reports each reply whose key fields changed. A person refreshes recordings on purpose with `--record`. A test run never writes a recording.

### 5.6 What an adapter may never do

- Build a client, open a socket, sleep, retry or read the clock. The transport does each.
- Read the environment, a file or the config.
- Hold or share state across calls.
- Name `ureq`, `rusqlite` or `serde_json`, keep a `serde_json::Value` field, or use `#[serde(default)]` in a raw type.
- Declare its own rate limit. Limits live in the shared quota table.
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
| The transport | 1,800 to 2,800 |

The article slice estimates PubMed at 400 lines, Europe PMC 350, PubTator3 300, LitVar2 150, LitSense2 120, Semantic Scholar 350, iCite 150, the PMC OA bucket 200, the JATS reader with its Markdown converter 400, the PDF link and download 80 and the supplement readers 250.

These are estimates. The article slice measures its adapters first. If they average over 300 lines, this section is revised. Each file in an adapter folder stays under 300 lines.

## 6. The query layer

`biomcp-query` turns a `Request` into an outcome. Every interface calls one function.

### 6.1 Context and the one entry point

```rust
/// What a command needs. The caller opens one Store per call.
pub struct Ctx<'a> {
    pub fetch: &'a Fetch,
    pub budget: Duration,
    pub fresh: bool,
}

/// A response before the interface spells its next steps.
pub struct Outcome<T> {
    pub status: Status,
    pub result: Option<T>,
    pub page: Option<PageInfo>,
    pub sources: Vec<SourceOutcome>,
    pub problems: Vec<Problem>,
    pub next: Vec<(StepKind, Action)>,
}

/// Run one request. Source failures become statuses. Errors are input, pool or store failures.
pub fn run(cx: &Ctx<'_>, req: &Request) -> Result<Outcome<QueryBody>, QueryError>;

#[derive(Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum QueryBody { GenePage(GenePage), GeneCards(GeneCards) }
```

`run` is one `match` from each `Request` variant to its entity function, such as `gene::get(cx, q)`. That match is the routing table, one line per command. `QueryBody` lives in `biodata::response` beside the types it wraps.

The first gene commands need no query store. A CLI process makes one call. MCP calls share one process-owned `Fetch`, including its provider quotas, cache and replay cursors. A per-leg fresh override reaches batch children and cannot weaken process policy. An offline tool call uses a separate cache-only client. Query store ownership arrives with its first consumer.

### 6.2 Resolve

Each entity function first turns loose input into canonical identifiers. For genes, `gene::resolve` reads HGNC rows from the bundle with no request:

1. An HGNC, Entrez or Ensembl ID input matches its exact key.
2. Text matches an approved symbol first, then previous symbols, then aliases, all without case.
3. One approved record resolves. A withdrawn record with no approved match answers withdrawn.
4. Several records answer ambiguous with every candidate as a header, ranked by match kind with previous symbols before aliases and protein-coding genes first, each carrying how it matched.
5. No record answers not found.

Resolution never guesses. The envelope's `query` echoes the inputs, and each item's answer shows how it matched. Other entities resolve the same way against their own identity source.

### 6.3 Get

`get` takes up to twenty inputs and returns one item per input, in input order. Each item has its own status and answer, so one bad input never hides the others. For genes:

1. Resolve every input (section 6.2).
2. Fetch the extras for every resolved gene in one batched leg, such as MyGene.info by Entrez ID. With an OncoKB key, one OncoKB leg runs beside it for the card's short `oncokb` block. Without the key, the envelope names OncoKB as skipped once.
3. Fan out one job per requested section and source, each covering every resolved gene the section applies to, and join the results back per card.

A section is `NotApplicable` when the record's own facts rule it out. The card's `more` lists the sections the caller did not request.

A part of one record is a section when it fits in one reply. It becomes an `<entity> <relation> <id>` drill-down when it pages, such as a trial's locations.

### 6.4 Fan-out

```rust
/// Join every scoped source worker and return results in declaration order.
pub(crate) fn run<'a, T: Send + 'a>(jobs: Vec<Box<dyn FnOnce() -> T + Send + 'a>>) -> Vec<T>;
```

The command creates its absolute deadline before identity resolution. Each source job opens one leg capped by its declared source deadline, retaining the command's absolute deadline. MyGene annotations and applicable requested sections run beside each other. The query deduplicates resolved HGNC identities before scheduling and joins each job's results back in caller order, including repeated inputs. A source failure affects only its own identities; local transport failures retain their typed local error.

A pure applicability function for each section serves both scheduling and `more`. Entity facts that rule out a requested section produce `NotApplicable` without a job. Jobs retain their leg reports and mapping problems; stale source failures remain visible. Panics propagate after every scoped worker has joined.

The shared SQLite mutex covers SQL only. Virtual-clock replay legs wait for the in-process SQL owner; external SQLite contention still returns immediately. Live SQL admission keeps its deadline-aware wait. Network work and decoding happen outside that mutex.

Rejected: a worker pool, a channel-based scheduler and an async executor. A dozen scoped threads per command need none of them.

### 6.5 Merge

Merge is a pure union. Records are keyed by their canonical identifier. Equal keys merge and distinct keys never do, even when two records share a DOI. Each merged item keeps every source that found it in `found_by`.

For articles, the pool resolves DOI-only rows to PMIDs at build, by the source's own PubMed ID or by ESearch `[doi]`, and keeps a resolved PMID only when its record carries the same DOI. The key is the PMID, then the PMCID, then the DOI, then the Europe PMC ID. Duplicates within one source keep that source's first rank. Articles run only the sources the agent named.

```rust
pub(crate) fn union<K: Ord + Clone, T>(legs: Vec<(Source, SourcePage<T>)>, key: impl Fn(&T) -> K) -> Vec<Merged<K, T>>;
pub(crate) struct Merged<K, T> { pub key: K, pub value: T, pub found_by: Vec<FoundBy> }
```

Field values are never merged. Each field has one owning source, as each slice's design states. HGNC owns gene identity and MyGene.info owns the summary and spans, so the gene card joins their fields by identifier and never chooses between two values. For a pool, the service declares one primary source for headers, such as PubMed ESummary for any item with a PMID. An item the primary source cannot describe keeps the header from the first source that found it, in the order the service lists its sources. Fields from secondary sources reach the agent as sections under their own source. Rejected: per-field merge rules with conflict records, which add a rule per field and per source pair and an output shape agents would need to learn.

### 6.6 Rank

Ranking is a separate pure stage over features the pool already holds.

```rust
pub(crate) struct Features { pub in_title: bool, pub in_abstract: bool, pub curated: bool, pub families: u8, pub best_rank: u32, pub year: Option<u16> }
pub(crate) fn rank(items: &mut [PoolItem], q: &RankQuery);   // stable lexicographic sort, then best native rank, then ID
```

Articles have no ranker. Their order is the closed enum `Order`: `rank` keeps each source's own order and, with several sources, takes them in turns in the order the agent listed them, skipping a paper already shown; `date` takes each named source's own newest-first order the same way; `citations` sorts the whole pool by iCite's count of citing PubMed papers, ties in `rank` order, items with no count last. `Features` shrinks to the iCite count, filled only for `citations`. BioMCP never blends sources into one score, as `planning/slices/article.md` records. Gene search ranks by match kind, then locus group, as `planning/slices/gene.md` says.

### 6.7 Pools and paging

A search whose answer comes from several sources builds a pool once and pages over it.

1. **Key.** The request key is SHA-256 over the schema version, the normalized request without `limit`, `offset` and `pool`, and the set of enabled sources. For articles the normalized request holds the ordered source list and the sort, because the turn order depends on them.
2. **Pick.** With `--pool ID`, read that pool. Without it, read the request key's current pool when it is fresh.
3. **Build.** Otherwise build a new pool (section 6.8), give it a new ID, store it and make it the request key's current pool. A replaced pool stays readable by ID until its own expiry.
4. **Page.** Slice the stored order by `offset` and `limit`. The page carries the pool's ID, size, creation and expiry times and the stored source outcomes.

Every next-page step carries `--pool <id>`, so an agent's walk reads one snapshot from start to end even if a `--fresh` rerun replaces the current pool midway. A `--pool` ID that no longer exists returns the error `pool.expired` with a `restart` step: the same search at offset zero without `--pool`. A pool built while a source failed is partial, and its next steps include a `fresh` rerun. A pool whose stored request differs from supplied selectors returns usage.pool_mismatch with a restart for the supplied checked request without that pool; it is not pool.expired. Pool expiry is fixed at 24 hours from publication, not renewed by reads.

A pool retains every item identity and its header hydration state (unattempted, checked missing or present), so a warm hydrated page is a store read with no request. A PubMed item with no valid summary keeps its PMID and omits the header, with a named problem, partial page and fresh step; ESummary request failure fails the page. Headers never determine the floor order or size. A pool never grows after build: each source is fetched to a fixed depth then. Article headers come from one ESummary call per page when the page is first read, and are stored with the pool, so a page read twice makes no second call.

A pool's size is capped per entity. A capped pool whose sources all answered is complete, with `page.truncated` set and `page.total` giving the sources' total. Truncation is a fact about the answer's size, and the status stays complete.

### 6.8 Building an article pool

The article pool runs only the sources the agent named. With no `--source`, it names PubMed alone, and the page is PubMed's own relevance order for the exact term echoed in the query.

1. **Acquire.** Run each named source in parallel with its own deadline. PubMed sends one ESearch for its whole window of up to 9,999 PMIDs. LitVar2 returns its whole list, capped at 9,999 with `page.truncated`. Europe PMC and Semantic Scholar return their first 100. Resolve DOI-only rows and fold as section 6.5 says, in the order section 6.6 names.
2. **Headers.** Each page's headers come from PubMed ESummary for its PMIDs, or from the first source that found an item with no PMID, and are stored with the pool.
3. **Citations.** Only `--sort citations` calls iCite, for every PMID the pool holds, in batches of 200, four at a time. Above 10,000 PMIDs the search refuses with a usage error that names the count.

The pool never drops an item PubMed returned. A named source that fails leaves the pool partial with that source named and a `fresh` step. When PubMed is named and fails, the status is failed, because the plain PubMed floor cannot be met. The article slice's bar that every plain PubMed hit appears in the full walk therefore holds by construction, or the response says why.

### 6.9 Links across entities

A link from one entity to another is a search on the target entity with a filter taken from the source entity, and the step carries the resolved identifier:

```rust
pub(crate) fn gene_next(gene: &Gene) -> Vec<(StepKind, Request)> {
    let id = GeneInput::from(gene.hgnc_id);
    vec![
        (StepKind::Pivot, Request::SearchTrial(TrialSearch { gene: Some(id.clone()), ..Default::default() })),
        (StepKind::Pivot, Request::SearchArticle(ArticleSearch { gene: Some(id.clone()), ..Default::default() })),
        (StepKind::Pivot, Request::SearchDrug(DrugSearch { target: Some(id), ..Default::default() })),
    ]
}
```

`search trial --gene HGNC:3430` resolves the ID to ERBB2 and its aliases before it queries trial sources, so the alias failure 0.9 showed with `gene drugs HER2` cannot recur. A drug card's next steps search trials and articles by that drug. A trial card's next steps get its conditions' diseases and its interventions' drugs. An article card's next steps follow its citations, its references and its annotated genes. The chain from gene to drug to trial to article is a walk over typed next steps with exact identifiers.

Link commands that equal a filter on the target's search do not exist. Drill-down commands remain for a record's paged parts, such as `trial locations`, and for graph edges no search can express, such as `article citations`. 0.9's retired spellings answer with a usage error that names the new command (section 8.3).

### 6.10 Next steps

Each response type has one pure `next` function in its entity's `next.rs`. It returns typed actions with their kinds, ordered by what the slice found agents need first. When a page holds selectable candidates, `next` holds one `disambiguate` get per candidate on that page, bounded by the checked page limit, then the `page` step, then at most three other steps. An association page treats each distinct linked record on the page, such as a gene, the same way, with one `pivot` per record in source order. Otherwise it holds at most five steps:

| Kind | When |
| --- | --- |
| `page` | More results remain. Carries `--pool` and the next offset. |
| `fresh` | A source failed or served stale. Reruns with `--fresh`. |
| `section` | A section in `more` has data behind it, such as `protein` when HGNC lists a UniProt accession. |
| `pivot` | A search on another entity with this record's resolved ID. |
| `disambiguate` | One `get` per candidate of an ambiguous input, by its canonical ID. |
| `restart` | A pool expired. The same search from the start. |
| `sync` | A bundle is absent, stale or syncing. |
| `key` | A keyed source was skipped because its key is not configured. Points to `biomcp help key <source>`. |

The binary spells each request into a `Step`.

### 6.11 Status

```rust
pub(crate) fn item_status(answer_found: bool, identity_failed: bool, partial_sections: bool) -> ItemStatus;
pub(crate) fn status(items: &[ItemStatus]) -> Status;                         // get
pub(crate) fn search_status(sources: &[SourceOutcome]) -> Status;            // search
```

- An item is `Complete` when its answer is resolved, ambiguous or withdrawn and nothing it asked for failed. It is `Partial` when a requested section is unavailable or a source served stale. A skipped section leaves the status unchanged, because the user's settings chose it. It is `Absent` when the identity source answered not found. It is `Failed` when the identity source failed.
- A get's status is `Absent` when every item is absent, `Failed` when every item failed, `Partial` when any item is partial or failed, and `Complete` otherwise.
- A search is `Complete` when every source answered, even with zero hits. It is `Partial` when some source failed or served stale and others answered, and `Failed` when every source failed. Skipped sources count toward none of these.

## 7. The store

### 7.1 One file, two owners

The cache is one SQLite file, as `decisions/2026-10-23-the-cache-is-a-local-sqlite-database.md` says. Its folder is `$XDG_CACHE_HOME/biomcp`, then `~/.cache/biomcp`, and on Windows `%LOCALAPPDATA%\biomcp\cache`. The transport owns the reply and bundle tables. The store module of `biomcp-query` owns the pool tables. Both open the file in WAL mode with a busy timeout.

The filename carries stable layout generations as `cache-<transport namespace>-<query namespace>.sqlite`. Query schema versions never select a new file. The default path retains an existing generation-5 file, or reuses the preceding generation-4 file when no generation-5 file exists. Cleanup retains both supported generations and their sidecars when they coexist. An allowed ordinary bundle read imports only a name missing from the selected current file. The predecessor is opened read-only and checked under both schema owners. Rows, keys, search words and metadata are staged through a streaming snapshot, published in one transaction and verified against the staged contents before commit. Legacy HGNC imports share the transport upgrade’s UniProt index backfill. Current published names take precedence without timestamp comparison. Both files remain after success or interruption. Stored-only reads also transfer in recording mode; recording auto-sync reads still capture a fresh provider exchange. Replay, health and disabled sources never trigger a transfer. Cleanup also retains any obsolete file with a bundle table, including an empty table that can receive a concurrent publication. Uncertain files remain untouched. An additive change keeps the filename and migrates the SQLite schema in one transaction, after both owners' metadata checks, so old bundles and state stay usable offline. Transport and query store versions are owned separately. The internal SQLite schema version can advance without changing the compatible filename namespace. The query store opens the active file through `Fetch::cache_access`, which retains the transport connection and any replay directory owner. Query code never reconstructs a path. Each command owns a query connection; MCP shares Fetch.

The binary supplies the query module's exported `SchemaOwner` to transport and replay-cache construction. Both owners' metadata is checked before journal or table mutation. Query operations use `CacheAccess::admit` to share the transport's existing SQL admission guard. Each command keeps its own connection, and the guard covers SQL only. No network operation or nested admission runs inside a store operation. Store errors remain typed local failures.

The authoritative tables live in [`biomcp-fetch/src/cache.rs`](../../crates/biomcp-fetch/src/cache.rs), [`biomcp-fetch/src/bundle/store.rs`](../../crates/biomcp-fetch/src/bundle/store.rs) and [`biomcp-query/src/store/schema.sql`](../../crates/biomcp-query/src/store/schema.sql). The first article consumer stores the pool, ordered checked PMIDs, original source outcomes and current pointer in one transaction. Pool IDs come from SQLite random bytes, so concurrent builds do not collide when wall timestamps match. The request key hashes the response schema and checked selectors without page size, offset or pool.

Each pool item distinguishes unattempted metadata, checked missing metadata and an accepted header. Hydration joins by checked PMID, preserves order and fixed lifetime, and never replaces an accepted header during a concurrent write. A stale fallback's source outcome stays with its item, so a warm page keeps its partial status and offers a replacement search. A summary hole keeps its PMID and offers a fresh read of the same pool. Pool creation and expiry use wall time.

The private store opens with `Store::open(CacheAccess, Deadline)` and exposes `find`, `publish`, `page` and `hydrate` to article queries. Public `store::CACHE_NAMESPACE_VERSION` selects the default layout; `store::VERSION` and `store::OWNER` validate and migrate the query schema. Version 4 query snapshots are discarded during migration because their source outcomes predate required publisher terms. Transport replies and bundles remain. Store statistics wait for their own consumer. Cleanup follows 7.2.

For sequential article stages, `Fetch::command_deadline` retains one fixed replay origin. The command debits each completed leg's virtual spend before starting another and caps real SQLite admission with its original wall budget. `CommandDeadline::finish` advances shared replay time to the completed absolute endpoint once, including early failures. Concurrent completions take the maximum endpoint, never the sum. Existing independent-leg consumers keep `Fetch::deadline`.

### 7.2 Freshness

- A reply is fresh until its endpoint's freshness ends. A 404 is fresh for ten minutes.
- A pool is readable by ID until it expires. It is the current pool for its request until it expires or a `--fresh` rerun replaces it.
- A stale reply or bundle is served only when its source fails, and then only with `stale` in its source outcome. Offline mode serves stale values the same way.
- Publishing a pool or snapshot deletes expired pools, their current pointers and items, and expired trial and citation snapshots in the same transaction. Each kind uses the bounded `PUBLISH_BATCH` in the store cleanup module. Live pools and snapshots remain readable. Ordinary reply writes remove expired replies in the same transaction before admitting the new checked reply. Replies remain available for stale serving until a write or explicit cleanup removes them. `biomcp store clean` removes expired replies, expired pools and snapshots, and validated older cache files with their SQLite WAL and shared-memory sidecars. 1.0 has no size cap.

### 7.3 When each table arrives

- `meta`, `reply`: the transport stage.
- `bundle`, `bundle_sync`, `bundle_row`, `bundle_key`, `bundle_words`: the first slice that keeps a bundle. HGNC makes that the gene slice.
- `pool`, `pool_current`, `pool_item`: the first slice that pools. The article slice pools, and gene search may if its design does.

Tables for entities, aliases and links are not designed here. They arrive with the wiki, which designs them against what it reads.

### 7.4 The wiki later

`biomcp serve --http` will serve a browsable wiki over the cache. Each entity gets a page rendered from its JSON through the same generic view the Markdown output uses, in HTML. Links follow the typed next steps. Each page shows each fact's source and fetched time. A "fetch more" button runs the page's next steps through the same `run` call. The wiki's own design adds the tables it needs and the code that writes them.

### 7.5 Postgres later

All SQL sits in two places: `biomcp-fetch/src/cache.rs` and `bundle.rs`, and `biomcp-query/src/store/`. The SQL uses forms both engines accept, such as `INSERT ... ON CONFLICT`, except the FTS5 table, which maps to a Postgres text search index. JSON stays text in SQLite and becomes `jsonb` in Postgres. A Postgres backend arrives when a shared deployment needs concurrent writers. Until then no store trait exists.

## 8. Interfaces

### 8.1 The CLI grammar

The grammar stays as `decisions/2026-10-23-the-user-is-an-agent.md` sets it: `search <entity>` for headers, `get <entity> <input>...` for cards and `<entity> <relation> <id>` for drill-downs.

This design keeps that hybrid over a pure entity-first tree such as `biomcp gene search`:

- `search` and `get` are the two moves every question starts with. `biomcp search` and `biomcp get` each list every entity, which is the map an agent needs first.
- 0.9 users, 0.9 skills and models trained on 0.9 docs already use this tree.
- Drill-downs are entity-specific, so `biomcp trial` listing its drill-downs reads naturally.
- All three forms map to one `Request` tag, so the hybrid adds no code.

Rejected: entity-first everywhere, which costs every 0.9 user and skill a change for one uniform shape. Rejected: verb-first everywhere, such as `biomcp locations trial NCT...`, which hides drill-downs under verbs no agent would guess.

### 8.2 The trees for gene, article and trial

```
biomcp search gene [TEXT] [--band BAND] [--locus-group GROUP] [--limit N] [--offset N] [--pool ID]
biomcp get gene <INPUT>... [SECTION]...
biomcp get gene <INPUT>... --section SECTION...
    inputs: symbols, previous symbols, aliases, HGNC:N, Entrez IDs, Ensembl IDs; up to 20
    sections: protein, civic, clingen, constraint, expression, pathways, oncokb, all

biomcp search article [TEXT] [--source LIST] [--sort RULE] [--gene INPUT] [--variant TEXT]
                      [--disease TEXT] [--drug TEXT] [--author TEXT] [--journal TEXT]
                      [--since DATE] [--until DATE] [--type TYPE]
                      [--open-access] [--no-preprints] [--exclude-retracted | --include-retracted]
                      [--limit N] [--offset N] [--pool ID]
biomcp get article <PMID | PMCID | DOI | Europe PMC ID>... [SECTION]...
    sections: fulltext, supplements, annotations, citations, indexing, tldr, pdf, all
biomcp article text <ID> (--part N | --find TEXT | --variant CHANGE | --save PATH [--replace])
biomcp article pdf <ID> --save PATH [--replace]
biomcp article supplement <ID> <FILE> [--rows A-B | --find TEXT | --variant CHANGE | --save PATH [--replace]]
biomcp article citations <ID> [--source icite|semantic-scholar] [--limit N] [--offset N] [--pool ID]
biomcp article references <ID> [--source icite|semantic-scholar] [--limit N] [--offset N] [--pool ID]
biomcp article citation-evidence <CITING> <CITED> [--fulltext]
biomcp article recommendations <ID>... [--negative ID]... [--limit N]

biomcp search trial [TEXT] [--condition TEXT] [--gene INPUT] [--variant TEXT] [--drug TEXT]
                    [--status STATUS]... [--phase PHASE]... [--sponsor TEXT]
                    [--near LAT,LON] [--radius KM] [--limit N] [--offset N] [--pool ID]
biomcp get trial <NCT>... [SECTION]...
biomcp trial locations <NCT> [--near LAT,LON] [--radius KM] [--limit N] [--offset N]

global: --format md  --fresh  --offline  --explain  --disable-sources LIST  --config PATH
        --record DIR  --replay DIR
housekeeping: help, schema, sync, store, serve, mcp-config, health, version, skill
```

The gene tree follows `planning/slices/gene.md` and the article tree follows `planning/slices/article.md`. The trial flags are placeholders for what its slice names.

Sections are a closed enum per entity. Without `--section`, the trailing positional words that exactly name a section are sections, and the words before them are inputs. With `--section`, every positional word is an input. An input that is also a section name therefore needs `--section`, and help says so.

### 8.3 Parsing from the schemas

The binary builds the grammar at run time from the checked-in schemas, with clap's builder API.

```rust
// crates/biomcp/src/grammar.rs
/// Build the clap command for one CLI path from its request schema.
fn command_for(path: &[&str], schema: &Schema) -> clap::Command;

/// Parse an argument list into a request. The same function serves the CLI and MCP.
pub(crate) fn parse(args: &[String]) -> Result<Parsed, UsageError>;
pub(crate) enum Parsed { Request(Request, Globals), Housekeeping(Housekeeping, Globals), Help(HelpTopic) }

// crates/biomcp/src/retired.rs
/// 0.9 spellings that 1.0 retired, with the command that replaces each.
pub(crate) const RETIRED: &[Retired] = &[
    Retired { path: &["gene", "trials"], use_instead: "search trial --gene <gene>" },
    Retired { path: &["gene", "articles"], use_instead: "search article --gene <gene>" },
    Retired { path: &["gene", "drugs"], use_instead: "search drug --target <gene>" },
    Retired { path: &["gene", "definition"], use_instead: "get gene <gene>" },
    // each slice adds its entity's rows from its 0.9 accounting
];

// crates/biomcp/src/spell.rs
/// Spell a request back into the argument list that parses to it.
pub(crate) fn step(kind: StepKind, action: &Action) -> Result<Step, serde_json::Error>;
```

1. The first one or two words pick the request schema from the embedded set. Only that schema is read.
2. A retired path returns a usage error whose message names its replacement. The message holds only the fixed replacement text, never the caller's input.
3. Each schema property becomes a clap argument. The property's type sets its parser: a string, an integer, a boolean flag, a repeatable option for an array, or allowed values for an enum. `x-positional` makes it positional, `x-short` adds its letter, and the doc comment becomes its help.
4. clap's matches become a JSON object, and serde deserializes it into the `Request` variant. Deserialization runs the checked constructors, so a bad identifier fails at the edge with its field and code.
5. `spell` walks the same schema in reverse: positionals first, then flags in schema order, omitting defaults. Sections always spell as `--section`, so a spelled step never depends on the collision rule.

The schema test keeps every query schema inside the subset step 3 handles (section 3.9). The CLI, the MCP input and the schemas therefore cannot drift, because one set of files feeds all three.

Rejected: clap derive structs that mirror each query type, which duplicate every field. Rejected: clap derive inside `biodata`, which puts a CLI library in the data crate. Rejected: a hand-written argument parser, which would rebuild what clap's builder does.

### 8.4 The JSON envelope

The CLI prints the envelope as compact JSON on standard output.

```json
{
  "schema": "biomcp.gene_cards/1",
  "status": "partial",
  "query": { "input": ["HER2", "MLL2"], "section": ["civic"] },
  "result": { "items": [
    { "input": "HER2", "status": "partial", "answer": { "resolution": "resolved", "card": {
      "gene": { "hgnc_id": "HGNC:3430", "symbol": "ERBB2", "name": "erb-b2 receptor tyrosine kinase 2", "status": "approved", "locus_group": "protein_coding", "location": "17q12", "entrez_id": "2064" },
      "match": { "kind": "alias", "text": "HER2" },
      "sources": ["hgnc", "mygene"],
      "sections": { "civic": { "state": "unavailable", "source": "civic", "reason": "rate_limited" } },
      "more": ["protein", "clingen", "constraint", "expression", "pathways"] } } },
    { "input": "MLL2", "status": "complete", "answer": { "resolution": "ambiguous", "candidates": [
      { "hgnc_id": "HGNC:7133", "symbol": "KMT2D", "locus_group": "protein_coding", "location": "12q13.12", "status": "approved", "match": { "kind": "previous_symbol", "text": "MLL2" } },
      { "hgnc_id": "HGNC:15840", "symbol": "KMT2B", "locus_group": "protein_coding", "location": "19q13.12", "status": "approved", "match": { "kind": "alias", "text": "MLL2" } } ] } }
  ] },
  "sources": [
    { "source": "hgnc", "state": "ok" },
    { "source": "mygene", "state": "ok" },
    { "source": "civic", "state": "failed", "reason": "rate_limited" }
  ],
  "next": [
    { "kind": "disambiguate", "args": ["get", "gene", "HGNC:7133"] },
    { "kind": "disambiguate", "args": ["get", "gene", "HGNC:15840"] },
    { "kind": "fresh", "args": ["get", "gene", "HGNC:3430", "--section", "civic", "--fresh"] },
    { "kind": "pivot", "args": ["search", "trial", "--gene", "HGNC:3430"] }
  ]
}
```

The values above are illustrative, and the names in them are omitted from some headers for space. Rules:

- Nulls, empty lists, empty objects, `false` flags and unrequested sections are omitted.
- Identifiers are canonical text.
- Header lists are capped by their types. A header cuts long text to a stated length and sets `truncated: true`. The get returns it whole.
- `query` echoes only the query that ran.
- `next` follows the bound in 6.10, gives each step its kind, and never repeats one.
- Every output validates against its schema in every case run, and cases cap the size of each command's output and of each section (section 10.1).

### 8.5 Errors and exit codes

An error prints one envelope with schema `biomcp.error/1`:

```json
{ "schema": "biomcp.error/1", "status": "usage", "error": { "code": "usage.retired", "message": "use: search trial --gene <gene>" } }
```

`main` returns `std::process::ExitCode` explicitly. Nothing calls `std::process::exit`. One function in `exit.rs` maps the outcome:

| Exit | Status | Meaning |
| --- | --- | --- |
| 0 | `complete`, `partial` | An answer. Resolved, ambiguous and withdrawn inputs are answers, and so is a search with zero hits |
| 1 | `absent` | Every input was confirmed absent by its identity source |
| 2 | `usage` | The arguments do not parse, a retired spelling was used, or a value failed its check |
| 3 | `expired` | A `--pool` ID no longer exists; the error carries a `restart` step |
| 4 | `failed` | Every source that could answer failed |
| 5 | `local` | Config, cache file, recordings, or a replay miss |

Messages come from the library's error and never contain input text. A next step may repeat the caller's input as an argument. A replay miss exits 5 even when the envelope says partial, so a case with a missing recording always fails.

### 8.6 The Markdown view

`--format md` prints a Markdown view for people. One generic function walks the response JSON:

```rust
// crates/biomcp/src/view/mod.rs
pub(crate) fn markdown(envelope: &serde_json::Value, hints: &Hints) -> String;

// crates/biomcp/src/view/hints.rs: one entry per schema, data only
pub(crate) struct Hint { pub schema: &'static str, pub title: &'static str, pub columns: &'static [&'static str] }
```

A hint names the JSON pointer of a card's title and the columns of a page's table. Everything else renders by structure: objects as definition lists, arrays of objects as tables, sections under headings with their state. A field whose type is biodata's `Markdown`, such as a part of an article's body, prints as it is, unescaped. Next steps print as quoted shell commands. The view reads only the serialized JSON, so it cannot show anything the JSON lacks, and it cannot fetch. It replaces 0.9's 34 templates and 102 Markdown functions.

### 8.7 The MCP tools

MCP offers the main `biomcp` tool and read-only research `search` and `fetch` wrappers, as the amended `decisions/2026-10-23-mcp-is-one-tool-over-the-cli.md` says.

```json
{
  "name": "biomcp",
  "description": "Search and get biomedical records for biomedical research questions. Pass CLI arguments as a list.\n<compact command tree generated from the schemas>",
  "inputSchema": {
    "type": "object",
    "properties": { "args": { "type": "array", "items": { "type": "string" }, "description": "The argument list after the program name, such as [\"get\", \"gene\", \"BRAF\"]." } },
    "required": ["args"],
    "additionalProperties": false
  },
  "outputSchema": { "type": "object", "description": "Compact reply envelope. Use biomcp schema <id> for its full contract.", "properties": { "schema": { "enum": ["biomcp.gene-cards/4"] }, "status": { "type": "string" }, "result": { "type": "object" } }, "required": ["schema", "status"] }
}
```

The description embeds the command tree generated from the embedded schemas: each entity, its verbs, its section names and its main flags, one line each. The discovery case pins the main tree and the research descriptions under their separate measured ceilings. [Ticket 0270](../tickets/0270-author-docs-and-accounting.md) records the current approved main-description cap. The [measured-caps decision](../decisions/2026-10-26-measured-entity-caps-need-no-ask.md) requires a PM ruling before a global description or full tool-list cap grows. The agent learns the whole surface from `tools/list` and asks `["help", ...]` only for detail.

`biomcp serve` speaks MCP over standard input and output from one file, `mcp.rs`, written to the specification:

- Messages are newline-delimited JSON-RPC 2.0 objects. A JSON array is refused with an Invalid Request error, because batching is not part of the pinned revisions.
- The server supports the current revision 2026-07-28 and initialization revisions 2025-11-25, 2025-06-18 and 2025-03-26. A small adapter at the protocol edge handles each era, and both feed one dispatcher. Current clients use `server/discover` and send the version in each request's metadata. Legacy clients send `initialize`. The server preserves a supported initialization revision and negotiates other initialization requests to its latest legacy revision. A request for any other revision gets the error the current specification names. A revision is added or dropped only with its cases.
- Every notification is accepted silently, including `notifications/initialized` and `notifications/cancelled`.
- `ping`, `tools/list` and `tools/call` are answered. Every other request gets a Method Not Found error.
- The main `biomcp` tool parses `args` with `grammar::parse` and runs `run`. Global flags that make no sense in a server, such as `--config`, `--record` and `--replay`, are refused.
- The main tool result carries the envelope as `structuredContent` and the same compact JSON as one text content block. `isError` is false for `complete`, `partial` and `absent`, which are answers. It is true for `usage`, `expired`, `failed` and `local`, whose envelope is the error envelope or a failed result. A malformed `tools/call`, such as a missing `args` or an unknown tool name, is a JSON-RPC Invalid Params error.
- Each `tools/call` runs on its own thread with its own `Store`. All threads share one `Fetch`. Responses go out through one locked writer.

`biomcp serve --replay DIR` runs the same server over recordings, which the runner uses. Cases cover discovery and per-request versions on 2026-07-28, `initialize` on 2025-11-25, an unsupported revision, a notification, an array, an unknown method and each `isError` value.

Rejected: an MCP library. The Rust MCP libraries pull in an async runtime, and the fixed RPC surface does not need that runtime.

### 8.8 Help and discovery

Help is JSON by default and comes from the schemas. `biomcp help` lists the entities and the verbs that apply to each. `biomcp help get gene` lists the arguments, the sections and two examples drawn from cases. `biomcp help --all` lists every command and option, and the runner's surface check reads it.

### 8.9 The HTTP mode

`biomcp serve --http ADDR` serves MCP's streamable HTTP transport at `/mcp` and a health check at `/health`, bound to 127.0.0.1 by default, as `decisions/2026-10-24-1-0-serves-mcp-over-http.md` says. Hosted clients and reverse proxies use it, as they did in 0.9. A small blocking HTTP server fits the thread model. Its handlers call the same `grammar::parse` and `run` as stdio. A non-loopback bind needs an explicit host allowlist. The wiki comes later at `/` on the same server.

The reviewed blocking server boundary uses `httparse` for HTTP syntax and a narrow socket owner in `mcp/http.rs`. It adds no async runtime. `http/wire.rs` owns `MAX_HEAD`, `MAX_BODY` and the absolute `READ_BUDGET`; `MAX_BODY` uses the stdio frame cap. `MAX_CONNECTIONS` bounds accepted worker handles with no application queue. Each socket carries one request and closes after its response. Content-Length frames are bounded before body allocation. Chunked frames use the library’s chunk-size and trailer parsers with bounded cumulative wire bytes and decoded body bytes. Ambiguous length/transfer encoding, unsupported transfer encodings and pipelined requests are refused before admission. Ctrl-C and termination signals use `ctrlc` to notify the server-owned bounded stop channel. The accept loop stops within `ACCEPT_POLL`, drops the listener and joins admitted workers under their existing read and command deadlines. No separate background process survives. The HTTP tests exercise this boundary through real isolated sockets.

HTTP uses stateless JSON responses for every supported MCP revision and never mints sessions. Each request selects its own revision and identifiers. The 2026 request mirrors follow the [official Streamable HTTP contract](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http): protocol version and method must match the body, and named methods require a matching decoded name. Metadata mismatches return HTTP 400 and error -32020. Unsupported methods return HTTP 404. Legacy initialization remains available without affecting another request. The pinned [2025-11-25 lifecycle](https://modelcontextprotocol.io/specification/2025-11-25/basic/lifecycle#version-negotiation), [2025-06-18 lifecycle](https://modelcontextprotocol.io/specification/2025-06-18/basic/lifecycle#version-negotiation) and [2025-03-26 lifecycle](https://modelcontextprotocol.io/specification/2025-03-26/basic/lifecycle#version-negotiation) require the initialization result to preserve a supported requested revision. HTTP uses the same legacy negotiation. The [2025-11-25 header rule](https://modelcontextprotocol.io/specification/2025-11-25/basic/transports#protocol-version-header) and [2025-06-18 header rule](https://modelcontextprotocol.io/specification/2025-06-18/basic/transports#protocol-version-header) require HTTP 400 for an unsupported header and recommend 2025-03-26 when a missing header leaves the version unknown. A headerless initialization identifies its legacy revision from `params.protocolVersion`; later headerless requests use 2025-03-26 because this server retains no session state. The [2026-07-28 header rule](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http#protocol-version-header) permits this fallback for servers that support older clients. An explicitly unsupported HTTP revision returns HTTP 400 and the current specification's `UnsupportedProtocolVersionError` (-32022) with the supported revision list. Accepted notifications return an empty 202. GET and DELETE on `/mcp` return 405.

Host checks cover health, errors and MCP. Authority parsing checks bracketed IPv6, ports and DNS labels before case normalization and comparison. Browser Origin is refused, CORS is disabled and forwarding headers confer no authority. Non-loopback listeners require `--allow-host` or `http.allowed_hosts`; configuration and flags combine. `/health` returns the generated `biomcp.http-health/1` body and 200 or 503 after the owned query store opens under `HEALTH_BUDGET`. It makes no source requests and exposes no paths, SQL or settings.


### 8.10 Saving files

`article text`, `article pdf` and `article supplement` take `--save PATH`, as `decisions/2026-10-24-pdfs-are-downloaded-not-read.md` says. BioMCP never extracts text from a PDF. The binary owns every file write; the libraries return bytes and never touch the file system.

- BioMCP writes only to the path the caller names. A folder PATH gets a file named from the ID or the supplement's own name. With no `--save`, nothing is written.
- The write goes to a temporary file beside PATH and is renamed on success, so a failed download leaves nothing.
- An existing file is refused with a usage error unless `--replace` is given.
- The reply gives the absolute path, the byte count, the SHA-256, the license and the source URL.
- BioMCP keeps no download folder and no list of saved files. The cache never holds saved files. The caller owns and deletes them.
- `biomcp serve --http` refuses `--save` with a usage error that gives the source URL, because the file would land on the server. Stdio MCP runs on the caller's machine and may save.
- A PDF download is credential-free, accepts only `http` or `https` with a DNS host that resolves to public addresses, checks each of at most three redirects by the same rules, stops at 25 MB, and requires a body starting with `%PDF-`. When a configured proxy applies to the publisher host, the download is refused with reason `proxy`, because BioMCP cannot check the address the proxy reaches.

## 9. Configuration, observability and performance

### 9.1 Configuration

`config.rs` is the only code that reads a file or the environment for settings, as `decisions/2026-10-23-settings-live-in-an-xdg-config-file.md` says. `paths.rs` resolves folders: the config file is `$XDG_CONFIG_HOME/biomcp/config.json`, then `~/.config/biomcp/config.json`, and on Windows `%APPDATA%\biomcp\config.json`. The cache folder follows section 7.1.

```json
{
  "budget_ms": 20000,
  "format": "json",
  "disable_sources": ["semanticscholar"],
  "proxy": "http://proxy.example:3128",
  "ca_bundle": "/etc/company/root-certificates.pem",
  "cache": { "folder": "/home/user/.cache/biomcp", "max_mb": 1024 },
  "bundles": { "sync_budget_ms": 120000, "auto_sync_max_mb": 32 },
  "keys": { "ncbi_api_key": "...", "oncokb_api_key": "..." },
  "sources": { "pubmed": { "deadline_ms": 10000 } }
}
```

```rust
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfigFile { proxy: Option<String>, ca_bundle: Option<PathBuf>, /* source settings */ } // unknown keys refuse

pub(crate) struct Settings { pub fetch: FetchConfig, pub budget: Duration, pub format: Format, pub disabled: BTreeSet<Source>, pub explain: bool }
pub(crate) fn load(globals: &Globals) -> Result<Settings, ConfigError>;
```

- Precedence: the flag, then the config file, then the built-in default. A key comes from the config file, then from its documented environment variable, which keeps its 0.9 name, such as `NCBI_API_KEY`, `ONCOKB_TOKEN` or `S2_API_KEY`. `biomcp help key <source>` leads with `biomcp key set <source>` and names both.
- Keys are registered through the CLI, as `decisions/2026-10-24-keys-are-registered-through-the-cli.md` says. `biomcp key set <source> [--replace]` prompts once and reads the key with echo off, or reads one line from standard input when it is not a terminal. The key is never a command argument, so it stays out of shell history. An empty key is refused, and a stored key is never replaced without `--replace`. `config.rs` changes only `keys.<name>` in the active config file, keeps every other setting, writes a temporary file and renames it, and sets owner-only permissions on Unix. The file is plain JSON, because these are API keys, not secrets that need encryption.
- `biomcp key list` names each source that takes a key, its config name, its environment variable and whether the key is `stored`, from the `environment` or `none`. It never prints a value. `biomcp key remove <source>` deletes `keys.<name>` and keeps everything else.
- The MCP tool refuses `key` commands with a usage error, so an agent never handles a key.
- An unknown key, a wrong type or a bad value fails with a named error that gives the key path.
- Tests set `XDG_CONFIG_HOME` and `XDG_CACHE_HOME` to empty temporary folders per case.

Corporate routing uses the config file's `proxy` for both schemes when present. Otherwise `HTTP_PROXY` serves HTTP and `HTTPS_PROXY` serves HTTPS; no scheme falls back to the other. Only uppercase names are read. `NO_PROXY` is a comma-separated list: a bare host or IP matches exactly, `.domain` matches that domain and its subdomains, `*.domain` matches subdomains only, and `*` bypasses every proxy. Bare IPv6 addresses and IPv4 or IPv6 CIDR ranges match numeric destination addresses. Rules may specify a numeric port, including bracketed IPv6 literals. Malformed ports and credentials in bypass rules refuse as local configuration failures and name `NO_PROXY` without its value. Other unsupported entries are skipped. Ordinary commands remain silent; `health` reports bounded configuration notices with the variable, entry position and notice code. Configuration failures name the environment variable or config file field without its value. Host matching ignores ASCII case and uses label boundaries.

`ca_bundle` names a PEM file read at the settings boundary. Its certificate anchors add to platform trust for every source and each request codec. Missing, unreadable, malformed, empty or oversized bundles fail locally before provider requests. The configured path is relative to the process directory unless absolute. Client certificates remain deferred. Fetch owns immutable trust settings and separate connection pools for direct, HTTP-proxy and HTTPS-proxy routes, so trust and route changes cannot reuse a previous Fetch's connections. A target TLS failure remains `tls` after a successful CONNECT; only proxy connection or negotiation failures become `proxy`. Envelope fix steps follow `help network ca_bundle` and `help network proxy`.

### 9.2 Observability

BioMCP writes nothing to standard error during a successful command. `--explain` adds an `explain` object to the envelope:

```json
"explain": {
  "total_ms": 1840,
  "stages": [ { "stage": "resolve", "ms": 3 }, { "stage": "acquire", "ms": 1590 }, { "stage": "enrich", "ms": 200 }, { "stage": "rank", "ms": 2 }, { "stage": "store", "ms": 9 } ],
  "requests": [ { "source": "pubmed", "quota": "ncbi-eutils", "endpoint": "esearch.fcgi", "cache": "miss", "status": 200, "attempts": 1, "start_ms": 0, "ms": 412, "bytes": 98211 } ]
}
```

A trace line names the source, the quota and the endpoint path, never the full URL, a credential or the query text. `start_ms` is real or virtual time from the command's start, so quota spacing is visible in live and replay runs alike. No logging crate exists.

### 9.3 Performance budget

Speed is measured against a floor worked out from first principles: the requests an answer needs, the bytes it must read and the work the CPU must do. Each gap between the floor and the measurement has a stated cause.

| Operation | Budget |
| --- | --- |
| Process start to output for `help` or `schema` | under 10 ms |
| BioMCP's own work for any command in replay with zero recorded delay | under 30 ms at the 95th percentile |
| A gene resolution from the HGNC bundle | under 5 ms |
| A command whose replies are all fresh in the cache | under 50 ms |
| A warm page of a stored pool | under 30 ms and zero requests |
| Enriching and ranking an article pool of 2,000 items in replay with zero delay | under 150 ms |
| Peak memory for the largest pool | under 100 MB |
| A cold command | the slowest source it needs plus under 50 ms, within the budget |

Measurement:

- The runner records each case's wall time. `make perf` runs a tagged case set in replay with zero delays, compares each case with its budget and prints the slowest stages from `--explain`. It runs on transport, query and grammar changes and at each milestone's end. The landing gate does not run it, because timing on a shared machine varies.
- Replay keeps virtual quota spacing, so the virtual `total_ms` of a cold article case estimates the live cold time from recorded delays and quotas. `make perf` reports it beside the wall time.
- The nightly live run records cold timings and request counts per source.
- `biomcp health` reports each source's reachability and latency on demand.

## 10. Testing

Tests are outside-in, as `decisions/2026-10-23-outside-in-tests-and-one-small-runner.md` says.

### 10.1 The runner

`crates/biomcp/tests/cases.rs` is one integration test target with `harness = false`. It uses `libtest-mimic` to turn each YAML file under `tests/cases/` into a named test, so Cargo's filter and parallel run work per case. It runs the binary Cargo built for the test.

```yaml
feature: article-search
covers: [search article, --gene, --variant, --offset, --pool]
story: AKT1 E17K leads to the 2007 discovery paper and keeps every plain PubMed hit
harvest: paper PMID 17611497
verified: 2026-10-23 live PubMed ESearch with sort=relevance
replay: akt1-e17k-2026-10-23
steps:
  - run: [search, article, --gene, AKT1, --variant, E17K, --limit, "50"]
    max_tokens: 10000
    expect:
      /status: complete
      /result/items/0/header:
        max_tokens: 150
      /result/items/*/header/pmid:
        rank_of: { "17611497": "<= 10", "33123537": "<= 2" }
  - follow: { kind: page }
    offline: true
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
- `run` sends one CLI command. `mcp` sends the same arguments through `biomcp serve --replay` as a `tools/call`. A step with both runs each and asserts the two envelopes are equal.
- `follow` runs a next step of the previous output, chosen by `kind` and an optional index. It tests chaining.
- `walk` pages through every result by following `page` steps until none remain, and asserts over the concatenated items.
- `offline: true` runs the step with `--offline` and no recordings. A step that succeeds offline made no request.
- `exit` sets the expected exit code, 0 by default.
- `max_bytes` and `max_tokens` on a step cap the whole output. As assertions under `expect`, they cap the value at one pointer, such as one section or one header. Each slice's token budget per command and per section becomes these ceilings, as `planning/slices.md` says, so growth fails a case.
- `config` sets keys in the case's temporary config, such as a placeholder OncoKB key for a replay case. Without it, keyed sources are skipped.
- `expect` maps JSON pointers to assertions. A `*` segment maps over a list.

The assertion set stays small: equality, subset, `includes`, `includes_all_of` a floor file, count range, `rank_of` ranges, `before` between two items, `unique` and regular expression. Every output is also validated against the schema it names, without any key in the case. A failure prints the pointer, the expected value and the actual value.

Tokens are counted by one fixed rule in the runner: the compact JSON's bytes divided by four, rounded up. The rule needs no tokenizer and gives the same count on every machine. The slice's comparison with 0.9 counts 0.9's output by the same rule. `make sizes` prints each case step's bytes and tokens and each section's, for the slice's budget table.

The runner takes `--cases DIR`, so case sets that cannot be published run against a built binary from outside the repo. A later `score:` block in place of `expect:` writes metrics against a stored baseline for nightly scoring runs.

The runner's YAML reader is a test dependency. The original `serde_yaml` is archived, so the runner stage picks a maintained serde YAML crate and records why. The `jsonschema` test dependency runs with default features off, so it never fetches a remote schema.

### 10.2 Checks the runner adds

- **Surface.** One test reads `biomcp help --all` and fails when any command or option appears in no case's `covers`.
- **Schema.** Every output validates against its schema.
- **Parity.** Every step with both `run` and `mcp` compares the two envelopes.
- **Quota.** The eight-parallel-call case in section 4.5.

The source ratchet permits `dead_code` only in the private `crates/biomcp-fetch/src/grpc/proto.rs` generated-message wrapper. That file contains only the checked closure include. Its pinned regeneration check must reproduce every committed generated file. The ordinary file size limits remain in force. Product code and unused imports receive no exception.

### 10.3 Coverage

`make coverage` runs the case suite under `cargo llvm-cov`. The runner spawns the binary as a subprocess, so the target uses `cargo llvm-cov show-env` to build an instrumented binary and sets a profile file pattern per process. Every spawned binary then writes its own profile, and the cases report merges them. The counted collection also runs only the reviewed public type-contract targets explicitly named with feature ownership and any required Cargo features in `sdlc/features.md`. These tests deserialize literal caller inputs through exported types and assert published validation and wire facts without hooks. The command-only figure prints separately for historical comparison. Unlisted contracts and library unit tests run in a separate excluded profile collection and never raise the floor figure. A target needs review before its feature entry can count; the historical permitted-test inventory below grants no counting authority. Both reports use the same product-source denominator; test targets, internal test modules, generated output and build scripts are excluded.

The landing gate fails when line coverage falls more than 0.2 points below the floor in `coverage.floor`. The tolerance absorbs counting noise between runs. Only the lander raises the floor, after a landing whose coverage exceeds it. `make unreached` lists the lines no case runs, for the reviewer. Each listed line is either deleted or given a case.

### 10.4 Where unit tests are allowed

The ratchet owns the authoritative file and reason list below and checks this table against it. Every other test of reachable command behavior runs through the runner. Public library contracts remain only where the binary cannot construct the state without a test hook.

| File | Reason |
| --- | --- |
| `crates/biomcp/src/grammar/reference.rs` | Generate and compare documentation at the private parser boundary without adding a public command. |
| `crates/biomcp/src/mcp/worker_tests.rs` | Inject a panic at the private worker completion boundary because public requests cannot intentionally panic. |
| `crates/biomcp/src/config/deferred_keys_tests.rs` | Observe forbidden credential environment reads at the private settings boundary while executing real commands. |
| `crates/biomcp-types/tests/author_contracts.rs` | Exercise checked source-qualified author records and standalone native paper and work-group contracts before adapters exist. |
| `crates/biomcp-types/tests/cell_line_contracts.rs` | Exercise checked cell identities, native variation members and provenance, and standalone consumer contracts through the public library API before adapters exist. |
| `crates/biomcp-types/tests/gwas_contracts.rs` | Exercise checked GWAS identities, exact probabilities and standalone consumer contracts through the public library API before adapters exist. |
| `crates/biomcp-types/tests/phenotype_contracts.rs` | Exercise neutral phenotype records and checked standalone source and section contracts before adapters exist. |
| `crates/biomcp-types/tests/pathway_contracts.rs` | Exercise neutral pathway identities and checked standalone consumer contracts through the public library API before adapters exist. |
| `crates/biomcp-types/tests/protein_contracts.rs` | Exercise neutral protein records and checked standalone consumer contracts through the public library API before adapters exist. |
| `crates/biomcp-types/tests/study_contracts.rs` | Exercise exact study identities, neutral source metadata and bounded standalone consumer contracts through the public library API before adapters exist. |
| `crates/biomcp-types/tests/diagnostic_contracts.rs` | Exercise neutral diagnostic records, native identities and checked standalone consumer contracts through the public library API before adapters exist. |
| `crates/biomcp-types/tests/variant_values.rs` | Exercise copied common variant values and constructor boundaries through the public library API. |
| `crates/biomcp-types/tests/article_ids.rs` | Exercise checked values and schema contracts through the public library API. |
| `crates/biomcp-types/tests/disease_contracts.rs` | Exercise neutral disease records and standalone consumer contracts through the public library API before adapters exist. |
| `crates/biomcp-types/tests/pgx_contracts.rs` | Exercise checked pharmacogenomics requests and neutral native record contracts before adapters exist. |
| `crates/biomcp-types/tests/adverse_event_contracts.rs` | Exercise neutral report records and checked source-specific adverse event contracts before adapters exist. |
| `crates/biomcp-types/tests/drug_contracts.rs` | Exercise published drug identifier, neutral record and consumer schema contracts through the public library API before adapters exist. |
| `crates/biomcp-types/tests/core.rs` | Exercise checked values and schema contracts through the public library API. |
| `crates/biomcp-types/tests/gene_ids.rs` | Exercise checked values and schema contracts through the public library API. |
| `crates/biomcp-types/tests/gene_model.rs` | Exercise checked values and schema contracts through the public library API. |
| `crates/biomcp-types/tests/query.rs` | Exercise checked values and schema contracts through the public library API. |
| `crates/biomcp-types/tests/response.rs` | Exercise checked values and schema contracts through the public library API. |
| `crates/biomcp-types/tests/schemas.rs` | Exercise checked values and schema contracts through the public library API. |
| `crates/biomcp-types/tests/schemas/disease.rs` | Exercise standalone disease option metadata and synthetic envelope schema contracts before command registration. |
| `crates/biomcp-types/tests/schemas/compatibility_cases.rs` | Exercise checked values and schema contracts through the public library API. |
| `crates/biomcp-types/tests/schemas/values.rs` | Exercise checked values and schema contracts through the public library API. |
| `crates/biomcp-types/tests/value_conventions.rs` | Exercise checked values and schema contracts through the public library API. |
| `crates/biomcp-adapters/src/monarch/native_contracts.rs` | Exercise retained gene-keyed Monarch rows through the private mapper without adding a gene-keyed command. |
| `crates/biomcp-adapters/tests/alphagenome_registration.rs` | Exercise restricted source registration and required-key admission before a product section exists. |
| `crates/biomcp-adapters/tests/batch_bounds.rs` | Exercise literal indexed lookup inputs outside command validation and empty/oversized batches or chunks beyond command limits. |
| `crates/biomcp-adapters/tests/pmc_body.rs` | Reconstruct complete source rows and retain JATS reference IDs and links omitted from command JSON. |
| `crates/biomcp-adapters/tests/pmc_limits.rs` | Reject invalid table spans and duplicate table IDs before allocation with a tiny decoder boundary table. |
| `crates/biomcp-fetch/src/cache/contention.rs` | Order independent SQLite snapshots and writer admission at the private transaction boundary without product hooks. |
| `crates/biomcp-fetch/src/destination_tests.rs` | Cover public DNS address and redirect rejection tables without contacting private destinations. |
| `crates/biomcp-fetch/src/grpc/dns_tests.rs` | Prove a stalled platform lookup cannot outlive its caller budget or begin a stream, without changing the host resolver configuration. |
| `crates/biomcp-fetch/src/retry_tests.rs` | Cover Retry-After forms and backoff boundaries without one recording per table row. |
| `crates/biomcp-fetch/src/xml_tests.rs` | Cover inline XML markup and decoder limits with many tiny inputs. |
| `crates/biomcp-fetch/tests/alphagenome.rs` | Exercise the selected request encoding and complete bounded envelopes through the public protocol API. |
| `crates/biomcp-fetch/tests/alphagenome/routing.rs` | Exercise required-key admission and placeholder credential routing through a local TLS stream. |
| `crates/biomcp-fetch/tests/score_variant.rs` | Exercise the ruled scoring request and complete scorer admission through the public protocol API. |
| `crates/biomcp-fetch/tests/batch.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/bundle.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/bundle/download.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/bundle/native.rs` | Exercise complete native TSV replacement sets and decoded bounds through the public transport API. |
| `crates/biomcp-fetch/tests/bundle/rows.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/cache.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/cache/credentials.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/cache/freshness.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/cache/storage.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/grpc_persistence.rs` | Exercise checked RPC cache admission and bounded recording replay through the public transport API. |
| `crates/biomcp-fetch/tests/grpc_persistence/policy.rs` | Exercise credential containment, concurrent stream capture and atomic publication through the public transport API. |
| `crates/biomcp-fetch/tests/grpc_persistence/process.rs` | Exercise ordinary checked reply cache reuse from an isolated second process through the public transport API. |
| `crates/biomcp-fetch/tests/grpc_worker.rs` | Exercise blocking streaming RPC admission, local TLS HTTP/2, proxy routing, deadlines and cancellation through the public transport API. |
| `crates/biomcp-fetch/tests/offline.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/paths.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/post.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/quotas.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/record.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/retries.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/support/cache_process.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/tensors.rs` | Exercise bounded tensor decoding with synthetic protobuf payloads through its public API. |
| `crates/biomcp-fetch/tests/transport.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/transport/network.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/transport/credentials.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-fetch/tests/xml.rs` | Exercise transport ownership, local server behavior and cache/record failures through its public API. |
| `crates/biomcp-query/tests/article_search.rs` | Exercise concurrent pool publication/hydration, injected SQL corruption, external writers and API-only deadline boundaries. |
| `crates/biomcp-query/tests/gene_resolution.rs` | Mutate isolated cache freshness and preserve a local bundle failure under an API-only short command deadline. |
| `crates/biomcp/tests/ratchets.rs` | Enforce repository structure and prove refusal of planted violations in isolated trees. |
| `crates/biomcp/tests/ratchets/proof.rs` | Enforce repository structure and prove refusal of planted violations in isolated trees. |

Everything else goes through the runner.

## 11. Ratchets and size rules

Every ratchet starts at its 1.0 target, because the code starts empty. `make landing-gate` runs `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` with the case suite, the schema test and the ratchet test, `cargo deny check licenses advisories`, and `make coverage`.

| Rule | Target | Mechanism |
| --- | --- | --- |
| Dependency direction between crates | the table in 2.1 | Cargo manifests, and the ratchet test's allowed list per crate |
| One crate names the HTTP client | `biomcp-fetch` only | the ratchet test's allowed lists |
| SQL owners | `biomcp-fetch`, and `biomcp-query/src/store/` | allowed lists and the ratchet test's path check |
| No I/O in `biodata` and adapters | none | per-crate `clippy.toml` `disallowed_methods` and `disallowed_types` |
| No global or shared mutable state outside the transport | none | per-crate `clippy.toml` `disallowed_types`, and `unsafe_code` denied |
| Threads | `fan_out.rs`, the transport and `mcp.rs` only | `disallowed_methods` with one reasoned `expect` each |
| Environment reads | `config.rs` only | `disallowed_methods` with one reasoned `expect` |
| Lint exceptions | listed files only, each with a reason | `allow_attributes_without_reason`, and the ratchet test's list of files that may hold `#[expect(` |
| File length | 600 lines; 300 in an adapter folder | the ratchet test |
| Function length | 80 lines | `clippy::too_many_lines` at 80 |
| `dead_code` and `unused_imports` allowances | none | review and the ratchet test's `#[expect(` list |
| Panics, unwraps, indexing, prints | none in product code | workspace clippy lints |
| Unit tests in product crates | the listed files | the ratchet test counts `#[test]` |
| Outputs with a checked-in, versioned schema | every command | the schema test and the runner's validation |
| Breaking schema changes without a major bump | none | the compatibility check in the schema test |
| Query schemas outside the grammar's subset | none | the subset check in the schema test |
| Commands and options without a case | none | the runner's surface check |
| Quirks without a case | none | the ratchet test checks each named case exists |
| `#[serde(default)]` in adapter raw types | none | the ratchet test |
| Rate limits outside the quota table | none | `SourceSpec` takes only a `&'static Quota` |
| Replies cached without a passing check | none | `Leg` stores only after `Reply::check`; a recorded 200 error case per adapter |
| Format branches in interface code | one dispatch in `output.rs` | review; the view takes JSON only |
| `String` fields holding IDs, closed sets, dates or coordinates | none | review, visible in each schema |
| `serde_json::Value` in adapters | none | adapters cannot name `serde_json` |
| Test hooks in product code | none | record, replay and offline are user features |
| Feature set tested against feature set shipped | the same | the binary enables `schema`; one build profile in the gate |
| Licenses | no GPL, LGPL or AGPL | `cargo deny` |
| Outside-in coverage | never more than 0.2 points below the floor | `make coverage` with `coverage.floor` |
| Output size per command and per section | under each case's `max_bytes` and `max_tokens` ceilings | the runner |
| Keyed sources called without the user's key | none | `KeyUse::Required` checked in `Fetch::leg` and the fan-out; a replay case per keyed source with no key asserts `skipped` and no recording read |
| Limiters keyed by anything but a declared source key | none | the limiter takes only a `&'static Quota`; the ratchet test confines `SourceKey::declare` and `Quota` statics to `quotas.rs` and refuses duplicate keys |
| Documented commands that fail | none | a docs check runs each documented command over recordings |

## 12. Porting 0.9 knowledge

1.0 ports 0.9's knowledge and copies none of its code, as `decisions/2026-10-23-a-clean-repo-whose-history-replaces-biomcp.md` says.

### 12.1 The quirks list

Each slice's investigation reads the 0.9 adapters for its sources and writes a quirks table in its design file, `planning/slices/<entity>.md`:

| Source | Quirk | Evidence | 1.0 handling | Case |
| --- | --- | --- | --- | --- |
| PubMed | Without `sort=relevance`, ESearch returns newest first | 0.9 `src/sources/pubmed.rs`; live check 2026-10-23 | the `sort` parameter in `search_ids` | `article-search/akt1-e17k-floor.yaml` |

In the code, each kept quirk is one line with a `// Quirk:` comment that names its case. A quirk the slice drops gets a row that says why.

The seed list below comes from the 0.9 sources and `planning/slices/gene.md`. Each slice verifies its rows against the live source before relying on them.

| Source | Quirk |
| --- | --- |
| PubMed | Without `sort=relevance`, results come newest first. |
| PubMed | ESearch returns at most the first 10,000 records. |
| PubMed | ESearch can answer 200 with an `ERROR` field and no ID list. |
| PubMed | ESummary JSON keys records by PMID and gives the order in `result.uids`. |
| NCBI E-utilities | All endpoints share one limit: 3 requests per second, 10 with a key. Eight parallel calls without a key drew seven 429s. |
| PubTator | The rate stays at 3 per second with or without the NCBI key. |
| HGNC | Exact lookups return withdrawn entries, so the caller checks `status`. Withdrawn entries carry no successor. |
| HGNC | Aliases and previous symbols are separate lists, and only HGNC keeps them apart. |
| MyGene | `GET /query?q=HER2` ignores scopes and ranks the wrong gene first, and `q=symbol:HER2` returns nothing. Call by Entrez ID. |
| MyGene | `species=human` is required, `genomic_pos` can be a list, and replaced Entrez IDs are served without a flag. |
| MyGene | `from` plus `size` may not pass 10,000. |
| CIViC | Takes only the exact official symbol. A gene with no variants and a missing gene are different answers. |
| ClinicalTrials.gov | Pages by `pageToken`, and `countTotal=true` gives the total. |
| Europe PMC | Pages by `cursorMark` starting from `*`, with `resultType=core` for abstracts. |
| Semantic Scholar | A 429 without a key comes from a shared pool, and an `x-api-key` header lifts it. |
| openFDA | Some count queries ask for an exact retry through their error body. |

### 12.2 The rest of the port

- **Recordings.** 0.9's replay recordings convert into the transport's format by a one-time script that lives in the converting ticket and is deleted after. Only recordings a 1.0 case uses are kept.
- **Spec assertions.** Each slice reads 0.9's executable specs for its entity and turns each clinician-visible fact into a case. Wording checks are dropped.
- **Help text.** 0.9 help text moves into query type doc comments where a 1.0 option keeps it.
- **Retired spellings.** Each slice adds a row to `retired.rs` for every 0.9 command it retires, from its 0.9 accounting.
- **Skills.** The agent guide is rewritten against the 1.0 tree, and its examples come from cases.
- **Release pipeline.** The five-platform release, signing and installers carry over in the parity milestone.
- **Accounting.** Every 0.9 command, section and option for an entity ends as ported, renamed or dropped by a named ruling in its slice's tickets.

## 13. Rulings applied

The PM ruled on this design's earlier open questions on 2026-10-23. This file applies each ruling.

1. **Blocking I/O.** `ureq` on standard threads, with `Leg::batch` under the source's quota, timeouts on every phase, rustls with ring, platform trust roots and proxies only from config. `decisions/2026-10-23-one-transport.md` carries the amendment.
2. **MCP.** One hand-written file written to the specification, with the rules in section 8.7 and `serve --replay DIR` for the runner.
3. **Six crates.** The store lives in `biomcp-query` and the view in the binary. Each call opens its own `Store`, and the fan-out receives only `&Fetch`.
4. **Links as next steps.** Link commands that equal a search filter are gone. Next steps carry the resolved ID, and retired 0.9 spellings return a usage error that names the new command.
5. **Several inputs per get.** `get` takes up to twenty inputs and returns items in input order, each with its own status. Sections are a closed enum per entity, with `--section` on collision.
6. **Statuses and exit codes.** Distinct statuses and codes for absent, usage, expired, failed and local errors. Ambiguous and withdrawn inputs and zero-hit searches are answers that exit 0. A capped pool is complete with `truncated` and `total`. `main` returns `ExitCode`.
7. **Bundles.** Explicit `biomcp sync` uses a lock, a separate budget and resumable downloads, and never runs offline. Stored-bundle reads never synchronize. The older `Leg::rows` path retains bounded automatic synchronization as described in section 4.11.
8. **Coverage floor.** In the gate with a 0.2-point tolerance. Only the lander raises it. The subprocess binary is instrumented.
9. **Wiki tables.** Entity, alias and link tables are deferred to the wiki's own design, and their DDL is gone from this file.

Ian ruled on 2026-10-24, and this file applies each ruling:

10. **Keyed sources.** A source that needs the user's own license runs only with the user's key. `KeyUse::Required` makes it a transport rule, and the output reports it as skipped with a `key` step (section 4.3).
11. **Limiters by source key.** Each `SourceSpec` names a quota with a declared `SourceKey`, and nothing derives a limiter from a URL or host (section 4.5).
12. **Gene slice.** HGNC ships as two local bundles and owns identity. Ambiguous inputs return ranked candidates. Gene trials, articles and drugs become next steps. OncoKB is a keyed gene source.
13. **Token budgets.** Each slice's budget per command and per section becomes `max_bytes` and `max_tokens` ceilings in its cases (section 10.1).

## 14. Open questions

None. Section 8.7 settles the pinned MCP revisions.

EFetch provenance uses `LegReport.replies`, keyed by the canonical credential-free request identity. `Leg::xml_received` returns the selected reply metadata alongside its checked value. Capture stores Unix seconds per exchange; replay restores them, and cached replies retain their original row fetch time. Legacy recordings without this field report zero. Replay cache freshness starts at import while provenance retains the captured time.

Supplement readers use explicit JATS MIME declarations when present and supported. Missing or generic octet-stream declarations fall back to the exact filename suffix. Other explicit MIME declarations return unsupported. XLSX preflight bounds container metadata and actual expanded XML before calamine, then reads sparse cells without evaluating formulas or links. Binary recordings publish one relative sidecar per exchange. Binary parser validation occurs before cache admission.

The supplement adapter uses one local RefCell during a synchronous checked-byte callback to retain its parsed workbook for the query layer. It owns no global or shared mutable state. Its exact path and reason remain listed in the lint exception ratchet for review.
### Source terms and switches

Each implemented adapter owns its publisher terms in `SourceSpec::terms`. `biomcp-adapters::registry` references those declarations and owns the bounded health endpoints. The structural check refuses missing terms and omitted source declarations. Terms metadata was checked against publisher pages on 2026-10-24. HGNC, CIViC and ClinGen explicitly publish CC0 data. gnomAD's publisher-owned [terms source](https://github.com/broadinstitute/gnomad-browser/blob/main/browser/about/policies/terms.md) names CC0 for primary data and separate conditions for some annotations; this adapter requests constraint data and does not request those restricted annotations. [Reactome's current agreement](https://reactome.org/license) dedicates database data and derived files to CC0; its graphics and software use separate licenses. [iCite's publisher support page](https://support.icite.nih.gov/hc/en-us/articles/9066501755291-Citations-Module-Overview) explicitly identifies its citation database as public domain. The UniProt and HPA declarations link their attribution terms; HPA retains its separate third-party conditions. MyGene's terms retain upstream rights. PubMed, Europe PMC, PubTator export text and PMC preserve publisher-specific text rights under the restricted category. OncoKB requires the user's license and key. These source categories do not authorize individual uses or replace a record's license. Per-section categories remain deferred.

The configuration's `disable_sources` and CLI `--disable-sources` combine by union. Expansion uses implemented source declarations and source identities, never quota keys. The transport checks disabled before credentials, quota, replay, reply caches and bundle synchronization. Stored article pools refuse disabled source rows before paging or hydration. Source and section skips retain reason `disabled` and never become confirmed absence. MCP process policy cannot be weakened by a tool call. `Fetch::restrict_sources` creates an immutable command view that unions disabled identities while sharing connection pools, quotas, replay cursors and clocks, checked settings, caches and bundle reservations. A disabled search leg does not hide rows supplied by an enabled leg; disabled header hydration leaves those rows partial.

CLI health uses these same declarations and transport policies. Its probes bypass the checked data cache and do not synchronize bundles. `PROBE_BUDGET` caps each source leg within the command budget. A skipped source has no latency; each attempted probe reports bounded elapsed time and a typed state. Local recording or cache failures remain local errors. HTTP service health separately checks owned local storage.

Exact FDA PMA retrieval uses the shared checked admission predicate to withhold an incomplete native application collection from the reply cache. Its envelope reports partial status with the additive `collection.incomplete` problem code at `meta.results.total`; it emits no selected card or supplement page from that cut collection. Request identity checks apply before cache admission and on every cache or replay read. Regulatory text uses the existing label splitter and its escaped-byte fragment budget.

PMA regulatory pages preserve the selected submission metadata even when `ao_statement` is absent. A metadata-only item omits the optional fragment `text`; an explicitly empty source statement keeps `text: ""`. Present statement fragments and their paging retain the existing representation.

### Research search and fetch compatibility

The [official OpenAI MCP contract](https://developers.openai.com/api/docs/mcp), checked on 2026-10-25, defines read-only search and fetch inputs, declared output schemas, and matching structured JSON and JSON text. Search exposes citable headers; fetch exposes the existing Markdown view. Each result carries a nonempty canonical publisher URL and an exact get argument list as its opaque identifier. Source outcomes, terms, problems and continuations retain the CLI values in metadata. The wrappers share command execution with the main tool and add no provider or ranking logic. Search metadata contains the full CLI envelope, the omitted-row count and a pointer to the main tool for uncitable rows. Fetch retains absent or ambiguous CLI envelopes as tool errors without choosing a record. Explicit entity tails and opaque get identifiers use quoted argument words without invoking a shell.

The [Company knowledge compatibility guidance](https://developers.openai.com/plugins/build/mcp-server#company-knowledge-compatibility) scopes this interface to research and Company knowledge use. [Custom MCP plugins](https://developers.openai.com/api/docs/guides/custom-mcp-server) do not universally require these tool names. This server supplies the compatibility interface; it does not register a service or establish client eligibility.
