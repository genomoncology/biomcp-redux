# Server configuration

Unknown keys cause a configuration error. Command flags override their configuration keys.

## Local source exclusions

Every source and component field is enabled by default, including noncommercial and separately licensed data. Publisher terms remain authoritative. Public API access grants no additional reuse rights. Commercial users must set this configuration:

```json
{"disable_sources": ["noncommercial", "licensed"]}
```

BioMCP reads `$XDG_CONFIG_HOME/biomcp/config.json` or `~/.config/biomcp/config.json` on Unix, and `%APPDATA%/biomcp/config.json` on Windows. Use `--config PATH` for another file. Source names and the `noncommercial` and `licensed` groups also work with `--disable-sources`. Configuration and command exclusions combine. Exclusions also prevent disabled data from returning through cache, replay or stored collections.

## Cache and store folders

Publisher dataset dates are disclosed separately from retrieval and record event dates. `source_data_max_age_days` sets the age after which a supplied dataset date is flagged as old (default 365 days). A missing publisher date stays unknown; a recent retrieval never establishes a recent dataset release. The flag describes age and does not change the publisher's values or imply that a historical event is wrong. Stored continuations retain the publisher date and reassess its age using the current setting. OncoKB's release date is assessed as `YYYY-MM-DD`; its annotation keeps the original `MM/DD/YYYY` text. Open Targets publishes a year and month, shown as `YYYY-MM` with no age claim.

Use `biomcp get drug imatinib regulatory --source opentargets` for FDA application events matched by drug name. The `indications` section points to this lookup and reports the Open Targets phase facts. The two sources do not link their indications. FDA unavailability is reported when you request the regulatory section.

On Unix, BioMCP keeps its cache and stored collections in `$XDG_CACHE_HOME/biomcp`, or `~/.cache/biomcp` when `XDG_CACHE_HOME` is unset. On Windows, it uses `%LOCALAPPDATA%/biomcp`. Choose a local disk for these folders. Sharing one cache between several processes through a Docker Desktop host bind mount or a network filesystem is unsupported. BioMCP uses SQLite's write-ahead log (WAL), which needs shared memory between processes on the same host; see the [storage caution](cautions.md#cache-and-store-storage).

`reply_cache_max_bytes` caps the bytes of stored source replies, as a whole number of bytes; the default is [2 GiB](../crates/biomcp-fetch/src/cache.rs). Each reply counts its body, its URL and 64 bytes. The cap is checked each time a reply is stored: past it, expired replies are removed first, then the oldest, down to nine tenths of the cap, and the reply just stored stays. A reply larger than the whole cap is answered but not stored, and `0` stores no replies, which leaves offline mode and stale fallback nothing to serve. The cache file keeps its largest size and reuses the freed space. The 2 GiB default holds about 560 typical 3.6 MiB openFDA label pages, a tenth of a 20 GiB container disk. Stored collections from `sync` are not counted.

`pdf_memory_limit_mib` sets the Linux address-space cap for each publisher PDF reader. It accepts a whole number from 1 to 4096 MiB and defaults to 1024 MiB. BioMCP stops the reader at the call's deadline. On Unix the reader also limits its own CPU time to the remaining call budget rounded up to a second plus one second. On Linux the kernel kills the reader if the server process dies. A read stopped by a time or memory limit is reported as `refused`. On macOS the deadline and CPU-time limit apply.

Set `BIOMCP_SQLITE_DIAGNOSTICS` to an existing file to opt into bounded SQLite operation and numeric error-code diagnostics (up to 4096 bytes); it does not include queries or patient data.

### Docker

Mount a Docker named volume at the container's `XDG_CACHE_HOME` path to keep the cache and stored collections on container-native storage. The published image uses `/tmp/cache`. Use a volume writable by the image's user, UID 65532. Sharing one cache between several processes through a Docker Desktop host bind mount or a network filesystem is unsupported; use a local named volume instead.

## MCP tools

BioMCP rejects JSON-RPC batches with `{"jsonrpc":"2.0","id":null,"error":{"code":-32600,"message":"Invalid Request"}}`, including batches sent under [MCP 2025-03-26, which allows clients to send batches](https://modelcontextprotocol.io/specification/2025-03-26/basic#batching).

`biomcp serve --tool-mode typed` offers 18 tools: 13 entity tools, `biomcp`, `help`, `search`, `fetch` and `saved`. A hosted server offers `saved` only when started with `--saved-folder`. An entity tool takes `action` and one object named after the action, such as `gene` with `{"action":"get","get":{"id":"BRAF"}}`. That object is the command's own schema, so every filter, section and paging option is named and typed. Every search takes its text as `query`, every get takes its record ID as `id`, and sections go in `section`; an ID, a section or any list argument may be one string or a list. The schemas name their arguments without forbidding others, so a client that checks arguments still sends a guessed name: the server reads the common ones (`input`, `text`, `name`, `keyword`, `keywords`, `sections`) as the argument they mean and refuses any other, naming the valid ones. Section words are plain text listed in each action's description rather than a fixed list in its schema, so a wrong one reaches the server, which answers with the call that works: article `text` names `fulltext`. A section sent as an action, such as `approvals` on `drug`, is answered with the `get` call that reads it. The same action name means the same command on every tool that carries it. `get` returns the first page of a record, and a facet action such as `features` or `text` returns all of it. Links between things are views on the tools for the things they link: `gwas` on gene, variant, disease and study, and `pgx` on gene and drug. Helpers sit on the tool whose filters use them: authors on `article`, organizations and biomarkers on `trial`.

| Tool | Reads | Actions | Example |
| --- | --- | --- | --- |
| `gene` | Genes, with pharmacogenomics and GWAS views | `search`, `get`, `pgx`, `pgx_get`, `gwas` | `{"action":"get","get":{"id":"BRAF"}}` |
| `variant` | Variants, with normalization, source assertions, population frequency, recurrence and GWAS views | `search`, `get`, `normalize`, `assertions`, `population`, `recurrence`, `gwas`, `gwas_get` | `{"action":"get","get":{"id":"rs113488022"}}` |
| `disease` | Diseases such as NCIT:C3224 (melanoma), with a GWAS view | `search`, `get`, `gwas` | `{"action":"get","get":{"id":"NCIT:C3224","source":"ols"}}` |
| `drug` | Drugs, with a pharmacogenomics view | `search`, `get`, `pgx_get` | `{"action":"get","get":{"id":"NDA021588","section":"approvals"}}` (Gleevec, imatinib) |
| `trial` | ClinicalTrials.gov trials, with organization and biomarker helpers | `search`, `get`, `document`, `organization_search`, `biomarker_search`, `biomarker_get` | `{"action":"search","search":{"condition":["melanoma"]}}` |
| `article` | Articles, with full text, PDFs, supplements, citation walks and author helpers | `search`, `get`, `text`, `pdf`, `supplement`, `citations`, `references`, `citation_evidence`, `recommendations`, `author_search`, `author_get`, `author_papers`, `grant_search` | `{"action":"search","search":{"query":"AKT1 E17K"}}` |
| `protein` | Proteins, with features, isoforms, domains, structures, sequence and complexes | `search`, `get`, `features`, `isoforms`, `domains`, `complexes`, `complex_members`, `structures`, `sequence` | `{"action":"get","get":{"id":"P04637"}}` |
| `pathway` | Pathways, with their genes and events | `search`, `get`, `genes`, `events` | `{"action":"search","search":{"query":"MAPK"}}` |
| `phenotype` | Phenotype terms | `search`, `get` | `{"action":"search","search":{"query":"seizure"}}` |
| `adverse_event` | Adverse event reports | `search`, `get` | `{"action":"search","search":{"drug":["imatinib"]}}` |
| `diagnostic` | Diagnostic tests and devices | `search`, `get` | `{"action":"search","search":{"gene":"BRCA1","source":"gtr"}}` |
| `cell_line` | Cell lines, with an assay count | `search`, `get`, `assays` | `{"action":"search","search":{"query":"A375"}}` |
| `study` | cBioPortal studies, and GWAS Catalog studies through the GWAS views | `search`, `get`, `gwas`, `gwas_get` | `{"action":"search","search":{"query":"melanoma"}}` |

`help` supplies command contracts and examples, and the `biomcp` tool retains the complete CLI grammar. Research `search` and `fetch` remain available. Typed calls return the JSON reply alone, with no summary line. Set `"mcp_tool_mode": "typed"` in configuration for the same behavior. The flag overrides configuration. `single` remains the default and keeps the CLI-argument tool and research compatibility tools. Hosted restrictions apply in either mode, and hidden sources disappear from each action's schema. Typed mode follows `mcp_reply_form`. The result cap counts the JSON value once in either tool mode. Typed `next` entries carry `kind`, `name` and `arguments`, where `name` is an entity tool and `arguments` holds its `action` and action object. Sections use the command's `section` field inside the action object. A section step, a section page's own call, and a refusal's corrected call may also carry `options`: the other arguments that call takes, under the action's own names, each with a placeholder such as `<YYYY-MM-DD>` to fill in. The CLI step carries the same `options` keyed by flag, such as `--decision-from`. Replies omit inherited CLI globals and server paths from these calls, and omit suggestions that the effective catalog cannot execute. Earlier typed tool names are retired; the [release notes](../release/notes.md#typed-tool-names-for-0x-and-hosted-users) map each to its new tool and action.

Start `biomcp serve --tool-mode typed --catalog compact` to list just `search_tools` and `call_tool` for any MCP client.
Set `"mcp_catalog": "compact"` in configuration for the same behavior. The flag overrides the setting.
The catalog defaults to `full`, and it applies only to typed mode.

1. Call `search_tools` with `{"query":"trials near a city"}` for up to five names and short descriptions.
2. Call `search_tools` with `{"name":"trial"}` for its full definition. Both discovery replies use a `tools` array, empty when nothing matches.
3. Call `call_tool` with `{"name":"trial","arguments":{"action":"search","search":{"condition":"melanoma"}}}`. It returns the same reply and errors as a direct `trial` call. The definition describes its output. Use the same wrapper for suggested next calls.

A client can also request the compact list from a full typed server by including `"biomcp/compactCatalog": true` in `tools/list` request metadata.

Typed mode includes compact output schemas for entity tools, `help` and `saved` by default. These schemas describe the reply envelope, the names and JSON types of the result's top-level fields, and one row's fields when applicable. Use the advertised schema ID with `biomcp schema <id>` for the full contract. Command help also lists reply schema IDs. Set `"mcp_typed_output_schema": false` to omit the compact schemas. The shared `biomcp`, research `search` and `fetch` tools advertise their output schemas in either setting.

`mcp_skills` defaults to `true`. BioMCP serves the `biomcp-research` skill for multi-step evidence questions over stdio and HTTP. Clients can discover it through Skills over MCP or list and read its Markdown resources starting at `skill://biomcp-research/SKILL.md`. Hosted manifests omit worked examples that require excluded sources. Set `"mcp_skills": false` to disable skill discovery and resource reads.

`mcp_typed_enum_limit` sets the largest input choice list advertised inline (default 30). Larger lists use strings, with accepted values checked by the CLI parser. Each action's description names its section choices. Both settings apply only to typed discovery, and each tool's `action` list always names every action. This configuration selects typed mode with the default schemas and choice limit.

```json
{
  "mcp_tool_mode": "typed",
  "mcp_typed_enum_limit": 30
}
```

`mcp_typed_tool_limit` and `mcp_typed_definition_bytes` set positive ceilings on the typed catalog. The defaults are 20 tools and 192 KiB of compact UTF-8 definitions. Setting `"mcp_typed_output_schema": false` uses a 128 KiB byte ceiling. An explicit `mcp_typed_definition_bytes` value overrides either default. These ceilings apply to the full typed list. [Typed budget configuration](../crates/biomcp/src/config/mcp.rs)

The 18-tool default typed `tools/list` catalog measured **184,967 bytes** over stdio. This measures the `tools` array, including titles, descriptions, annotations and output schemas. The JSON-RPC envelope and server instructions are outside that count. A catalog exceeding either ceiling returns an error naming the required size and configured limits.

## Compact gene constraint batches

For a compact gene constraint batch, request the constraint section with the fields you need.

```sh
biomcp get gene ATR RAD17 KAT5 RAD51 CHEK1 constraint --section-only --fields=loeuf,transcript
```

In typed MCP, use the same section and fields in a call to `gene`.

```json
{"action":"get","get":{"id":["ATR","RAD17","KAT5","RAD51","CHEK1"],"section":"constraint","section_only":true},"fields":["loeuf","transcript"]}
```

Each resolved row includes its requested input, HGNC ID and symbol, and the constraint section with its release, transcript and source. Omit `--section-only` or `section_only` to read the full cards.

## HTTP capacity

MCP revision `2024-11-05` is available over stdio only. Its HTTP transport requires a separate SSE endpoint that announces the POST endpoint. BioMCP's `/mcp` endpoint uses Streamable HTTP and offers `2025-11-25` when an HTTP client requests `2024-11-05` in `initialize`. Subsequent HTTP requests with an unsupported `MCP-Protocol-Version` header receive HTTP 400.

`http_max_calls` limits tool calls in flight. `http_max_connections` limits open non-health HTTP connections. Each key accepts a positive integer. The server refuses `http_max_connections` when it is below `http_max_calls`. Pending framing readers and active requests share the connection cap. Framing readers share the existing worker bound of twice the connection setting plus one. `/health` uses neither the connection cap nor the tool call cap, but idle or incomplete sockets can exhaust framing workers and make it return 503. A front end that forwards complete requests avoids this raw-socket limit.

Use the same names as flags on `serve --http`: `--http_max_calls` and `--http_max_connections`. The call default lives in [`DEFAULT_HTTP_CAPACITY`](../crates/biomcp/src/config.rs). With no connection setting, the server allows twice its call limit, so one call can wait behind each running call. Raise connections above calls to let discovery requests answer while tool calls occupy their slots.

`mcp_max_calls` limits tool calls in flight on a stdio `serve`, which serves one agent. It accepts a positive integer, and the default lives in [`DEFAULT_MCP_CAPACITY`](../crates/biomcp/src/config.rs): agents send many calls at once, and source pacing already bounds upstream load, so this ceiling only bounds local worker threads.

A call over the stdio limit, or over the HTTP limit with or without `--hosted`, waits its turn in arrival order and then runs. `mcp_queue_wait_seconds` sets how long it may wait; it accepts a whole number of seconds, `0` turns waiting off, and the default lives in [`DEFAULT_QUEUE_WAIT`](../crates/biomcp/src/config.rs). A call never waits past its own deadline: on hosted HTTP the wait counts against the whole-call deadline, and on stdio and plain HTTP it is capped at `command_timeout_seconds`, with the command's own budget starting when the call runs. A waiting HTTP call keeps its connection, so the queue holds at most `http_max_connections` minus `http_max_calls` calls, and a connection past that limit gets HTTP 503. A waiting HTTP call whose client closes its connection leaves the queue. On stdio, `mcp_queue_max_calls` sets how many calls may wait; it accepts a whole number, `0` turns waiting off, and the default lives in [`DEFAULT_QUEUE_LENGTH`](../crates/biomcp/src/config.rs). A call that arrives when the stdio queue is full is refused at once.

`pdf_max_reads` caps simultaneous publisher PDF reads in each hosted server, including PDF downloads. It accepts a positive integer up to 65535 and defaults to one reader. When every slot is busy, a call waits in arrival order for up to `pdf_slot_wait_seconds` (30 seconds by default) or half its remaining time, whichever is shorter. Set it to zero to refuse immediately. If the wait expires, BioMCP tells the caller to retry. Budget for `pdf_max_reads` times the per-reader cap, plus up to 50 MiB per read for the parent's PDF buffers and room for other requests. The default of one keeps concurrent readers within the memory available to the typical 1 GiB hosted task based on measured reads near 170 MiB. For a strict task budget, set `pdf_memory_limit_mib` and `pdf_max_reads` together to leave room for the parent and the rest of the task. The shared cap applies only in hosted mode. Restart the server after changing it.

Drug label lookups by name use the most memory per call. `get drug metformin label` reads eight 50-label openFDA pages of about 6 to 9 MB each, 54 MB in all, and keeps only the names and indications it chooses by. Replayed over hosted HTTP with every call starting at once, a release build on an Apple silicon Mac (macOS) peaked at 153 MiB resident for one call, 398 MiB for 8 calls without caller keys, 491 MiB for 8 calls with their own keys, and 952 MiB for 32 calls; an idle server held 22 MiB. With one page near the 32 MiB page bound in every lookup, 8 calls peaked at 1.1 GiB and 32 at 2.2 GiB. Set `http_max_calls` so that peak fits within a task's memory beside anything else the task runs, and allow for Linux allocating differently. `make hosted-label-memory` repeats the measurement.

A call whose wait runs out, or that finds the queue full, gets JSON-RPC error `-31000`. Its message starts with `Tool capacity exceeded`, names the limit, says how many seconds the call waited, and says after how many seconds to retry. The same number is in `error.data.retry_after_seconds`: by then the running call nearest its deadline has finished or timed out.

For example, this configuration gives discovery requests room alongside tool calls:

```json
{
  "http_max_calls": 32,
  "http_max_connections": 48
}
```

The equivalent flags are:

```sh
biomcp serve --hosted --http --http_max_calls 32 --http_max_connections 48
```

The `--rate-share` flag scales only hosted buckets with an effective source key. Keyless hosted buckets and ordinary process buckets keep the source's full rate.

## Source pacing

`source_pacing` is one map keyed by provider bucket name. The [generated source reference](reference/source-pacing.md) lists every name, default, ceiling and policy citation. Shared sources such as the BioThings APIs use one bucket.

Each map entry can set `rate`, `keyed_rate` and `in_flight`. `rate` means requests without credentials. Sources without a separate credential policy apply `rate` to both modes. `keyed_rate` changes a separately declared credential rate. Each rate is an object with positive whole-number `requests` and `seconds`, each within the unsigned 16-bit range. Requests are evenly spaced across that interval. `in_flight` is a positive whole number within the source's concurrency ceiling. Omitted fields retain their defaults.

```json
{
  "source_pacing": {
    "ncbi-eutils": {
      "rate": {"requests": 1, "seconds": 2},
      "keyed_rate": {"requests": 5, "seconds": 1},
      "in_flight": 1
    }
  }
}
```

The matching global flag accepts the same JSON map:

```sh
biomcp serve --hosted --http --source_pacing '{"ncbi-eutils":{"rate":{"requests":1,"seconds":1}}}'
```

Flag fields override file fields for that source. Other fields and sources remain as configured. Changing `rate` leaves a separate `keyed_rate` unchanged. Unknown sources, zero components and values above a source ceiling cause a named configuration error before source requests. Credential rates remain subject to `--rate-share` in hosted mode. Provider daily quotas, IP-wide limits and shared upstream pools still apply; this setting grants no extra provider capacity.

## Command deadlines and drug approvals

`command_timeout_seconds` sets a positive whole-second command budget, using the [shared command default](../crates/biomcp/src/config.rs) when omitted. A first bundle reader waits for an active synchronization within that budget and the source's deadline. A live first read fetches a missing bundle under `auto_sync_max_bytes`, and a bundle over that limit offers an explicit sync command; reads never refresh a stored copy, offline and replay never fetch, and existing snapshots remain readable during replacement. A source that normally answers slower than the command budget declares its own budget in its `SourceSpec`, and a command that calls it gets the larger of the two: 60 seconds for VAERS tables from [CDC WONDER](../crates/biomcp-adapters/src/vaers/mod.rs), 30 seconds for [Monarch phenotype similarity](../crates/biomcp-adapters/src/monarch/phenotype/matches.rs) and 30 seconds for [ClinVar gene counts](../crates/biomcp-adapters/src/clinvar/mod.rs). A batched `get trial` reads each trial as its own ClinicalTrials.gov request, so it adds one ClinicalTrials.gov pacing interval per trial after the first to its budget and to that source's deadline: 19 seconds for 20 trials at the default pacing, and never more than [60 seconds](../crates/biomcp-fetch/src/quota/admit.rs). A hosted HTTP call's whole-call deadline starts at `command_timeout_seconds` and grows to the source's declared budget, or by a batched trial get's pacing time, counted from the call's arrival, only when the command calls that source; every other hosted call still ends at `command_timeout_seconds`. An MCP client's own time limit or cancellation still ends the call first.

`auto_sync_retry_seconds` sets the wait after an unsuccessful first-read fetch. It defaults to 60 seconds. Commands and server calls using the same store share the wait for that bundle. The source notice gives the cause, the size when known, and the seconds left before another automatic attempt. If the publisher sends a Retry-After delay, the notice names that original delay separately from BioMCP’s local wait. An explicit `biomcp sync` can run immediately. For single-file sources such as GenCC and FDA companion diagnostics, a publisher’s length header can give the size before the body arrives. For multi-file sources such as GTR and Orphadata, the total becomes available once every file’s size is known. If the fetch stops before BioMCP can read or count all the lengths, the notice says the size is unknown.

`sync_timeout_seconds` sets how long, in positive whole seconds, one `sync` command may take to download, check and store a source bundle; the default is 120. GTR and WikiPathways then read their new stored copy under a second limit of the same length, so their sync can take up to about twice the setting. Raise it on a slow connection: GTR's sync downloads about 245 MB, so below about 2.5 MB/s it can need more than 120 seconds. A sync that runs out of time fails with `deadline-exceeded`, keeps any earlier copy, and its reply names the limit and this setting.

`transcript_fit_seconds` sets how long, in positive whole seconds, `variant normalize` waits for each VariantValidator call that asks on which of the gene's transcripts a mismatched reference base fits. The check first lists the gene's transcripts, then asks about them in groups of 20, MANE Plus Clinical first and then by accession number, and stops at the first group that fits; each call has this limit. Omit it to use the [measured default](../crates/biomcp-query/src/context.rs). The command budget still applies. In hosted mode, waiting in the shared VariantValidator queue counts against the limit. When a call runs out of time or fails, the check stops, and the reply keeps the service's own mismatch and offers no fit; if groups were left, a `collection.incomplete` problem says how many transcripts went unchecked.

`drug_approvals_limit` sets the number of Drugs@FDA submissions returned by the approvals section, from 1 through 50. Omit it to use the [built-in submission page size](../crates/biomcp-types/src/drug/mod.rs). An explicit `--section-limit` overrides this setting. Continue with the returned page action when more submissions remain.

`drug_label_bytes` sets the most label section text one `--label-section` reply returns, in JSON-escaped bytes, from 6144 through 32768. Omit it to use the [built-in budget](../crates/biomcp-query/src/context.rs), which holds a typical Indications and Usage section, Keytruda's included, in one reply. Text comes in whole fragments of up to 6,144 bytes, so a reply may stop short of the budget; a larger section says how many fragments remain and offers the next page. Sections named together in one call share the budget in the order given, so a later section may return no text; it then says so in its `hint`, counts all its fragments in `remaining`, and offers a page step to read it. An explicit `--section-limit` caps the fragments a reply returns.

`drug_search_name_limit` sets the most names one trial or article search sends for a drug, as a positive whole number. Omit it to use the [built-in limit](../crates/biomcp-query/src/context.rs). A capped search keeps the typed name, the record's preferred name and the shortest other names, and its reply says how many it left out. ClinicalTrials.gov refuses a name of more than 12 words, so a trial search also leaves those out and counts them.

`article_access_lookup` set to `false` stops `search article` pages from asking NCBI which rows are in PubMed Central's open-access subset; rows then say `open_access_not_checked`. It defaults to `true`. `--no-access-lookup` skips the lookup for one search. The lookup is one NCBI request per page, sent beside the PubMed title read under NCBI's request pacing, and waits at most 3 seconds. `--license-lookup` also asks Europe PMC for each row's licence and its own open-access flag, waiting at most 10 seconds; without it, rows say `license_not_checked`. When a lookup fails or runs out of time, the page names its source with that state and offers a fresh retry.

## MCP reply form

Set `mcp_reply_form` to choose where MCP tools put the JSON reply over stdio and HTTP. Each setting uses the same layout for successes and errors.

- `once` puts JSON in `structuredContent` and leaves `content` empty.
- `both` puts JSON in `structuredContent` and in a text block in `content`.
- `structured-only` puts JSON in `structuredContent` and includes a one-line note in `content`.
- `text-only` puts JSON in a text block in `content`.

Local and hosted servers default to `both`. The text block lets older clients read the reply. Set `"mcp_reply_form": "once"` if your client reads `structuredContent` and you want a single copy. Use `text-only` only with a client that ignores the output schema, since MCP requires structured results from tools that declare one. When included, the output schema describes the JSON value in every form. Next steps in MCP replies never carry the server's `--config` path.

For stdio sessions that negotiate `2024-11-05`, BioMCP always sends JSON in a text block. Tool listings for those sessions omit annotations and output schemas.

## Hosted servers

Use `biomcp serve --hosted --http` for a shared server behind a hosting front end. Hosted mode runs only over HTTP, because every per-caller control (`x-biomcp-use`, `x-biomcp-exclude-sources`, caller keys and the account header) is an HTTP request header; `serve --hosted` without `--http` exits with a `usage.arguments` error before it serves anything. Hosted mode refuses MCP file saves, `health`, `sync`, `store clean`, `--fresh` and `key` commands. Each refusal returns `usage.mcp_unavailable` and names hosted mode. Read-only queries keep the shared request cache and query pools. Public replies stay shared across callers. Replies from no-fallback sources remain available only to callers with the same key, including recorded replies and stored continuations. Hosted mode refuses older licensed recordings and stored continuations that lack caller ownership. A local bundle past its refresh date still answers as complete in hosted mode, and the reply states the date its copy was retrieved; elsewhere that reply is partial until refreshed. The HTTP `GET /health` endpoint remains available for local server checks. Hosted HTTP calls may supply source credentials through the header names reported by `biomcp key list`. The list also reports each source's fallback tier. Caller headers override server credentials for that request only. Sources in the no-fallback tier require the caller's credential even when the server has a configured key. Other sources may use the server credential when the caller supplies none. Missing required credentials direct callers to their account settings. Tool arguments never accept keys. Ordinary HTTP mode ignores these headers.

A hosting front end can check a caller's credential with `POST /internal/key-check` on the hosted loopback listener. Send a JSON body containing only the source name, such as `{"source":"ncbi"}`, and the corresponding credential header from `biomcp key list`. The response contains the source and a status, with a reason for `rejected` or `could-not-check`. Other statuses are `working` and `no-key-required`. The route uses only the selected header for that request. Keep the route behind the hosting front end.

Multi-task hosted deployments require the same `BIOMCP_TRIAL_CONTINUATION_KEY` in each core process. Set it through private deployment configuration to the URL-safe unpadded Base64 encoding of 32 random bytes. Hosted startup refuses an invalid value. The key lets each task verify ClinicalTrials.gov continuations across independent stores and recheck the current source and credential policy. It is separate from OAuth and source credentials.

A hosted process without this setting uses task-local trial continuations, as does the ordinary CLI. Task-local continuations require their original store. After upgrading from a version that omitted prior-page source dependencies, begin again from the first page. Replacing the shared key makes its existing portable continuations unverifiable. Begin the search again if the binding is unavailable or the registry rejects the cursor. The portable binding is valid for up to one day, and the registry can reject its underlying cursor sooner.

Hosted HTTP requests accept `x-biomcp-use: restricted`, `commercial` or `academic`. Missing headers select `restricted`. Restricted use admits open and attribution sources and commercially permitted records from per-record sources. A paper whose license cannot be checked in time, or that falls past the lookup limit of 20 papers, stays in the results without body text, marked `unavailable` rather than `disabled`; that reply is partial and is not stored. Sources without explicit noncommercial terms remain enabled. Caller keys never widen restricted use. Commercial use also admits a noncommercial or licensed source only with its required caller key; optional rate-limit keys never enable blocked sources. Academic use retains every configured source. Server exclusions always apply, including for cached and stored replies. Invalid declarations are refused before source access. Ordinary HTTP and stdio ignore the header.

The JSON setting `hosted_max_use` selects the highest hosted HTTP mode (`restricted`, `commercial` or `academic`, default `academic`). Deployments serving only commercially safe sources set `"hosted_max_use": "restricted"`. Higher caller declarations are refused with an error naming the allowed modes. This setting applies only to hosted HTTP; the local CLI and stdio keep every configured source.

Hosted HTTP requests may send `x-biomcp-exclude-sources`, a comma-separated list of source names to leave out of that request. An empty value leaves nothing out. The list applies to every hosted use and can only narrow what the deployment serves: it never restores a source that the server, `hosted_max_use` or the declared use has turned off. A left-out source makes no call, is reported as disabled, and is left out of the tool list, help and follow-up steps for that request. Stored replies obey the same list, and follow-up steps do not repeat it, so the caller sends the header on each request. Keyed sources keep using the key sent with the request. `biomcp source list` prints the accepted names as JSON. Names are source names only, not terms groups such as `noncommercial`. A repeated header or an unknown name is refused with HTTP 400 before source access, and the refusal lists the accepted names. Ordinary HTTP and stdio ignore the header.

The hosting front end must strip the client-sent `x-biomcp-use` header and set it from the authenticated account attestation. It must also strip all client-sent `x-biomcp-key-` headers before injecting credentials from the authenticated caller's account settings, and strip client-sent `x-biomcp-exclude-sources` headers before setting that header from the caller's account settings. BioMCP does not authenticate these headers itself.

Typed mode adds a `saved` tool for saved searches, lists and notes, and a first search page with results offers a `save` step that runs it as written. Locally it uses `saved.sqlite` in the data folder, as the `saved` command does. A hosted server offers it only when started with `--saved-folder DIR`; each call then reads and writes `DIR/<account>/saved.sqlite`, where the account is the `x-biomcp-account` header: 1 to 128 letters, digits, `-` or `_`. No tool argument names a path. Without the header a `saved` call is refused with `usage.mcp_unavailable`, and a malformed or repeated header gets HTTP 400. The front end must strip any client-sent `x-biomcp-account` and set it from the verified user, and BioMCP knows nothing else of the user. For each `saved` call the front end holds a lock for that account until the call has answered, so one account's calls run one at a time. Before the call it fetches the account's file only if the stored copy changed, and deletes any `saved.sqlite-journal` beside it before placing a fetched file, since SQLite would otherwise replay that journal onto the new file. After the reply, if the file changed, it writes the file back only if the stored copy is still the one it fetched, and only then returns the reply; on a conflict it fetches again and replays the call, up to three times, then answers unavailable. BioMCP creates `DIR/<account>/` with mode 0700 when it is missing, so only the server's user can read it: the front end must run as the same user as BioMCP, or create `DIR/<account>/` itself before the call, or it cannot write back the first save of a new account. `DIR` must be on a case-sensitive filesystem: accounts are compared exactly, so on a case-insensitive one `Alice` and `alice` would share one folder.

When a client declares form elicitation, the `saved` tool asks for confirmation if the action is `delete` and `delete.all` is `true`. The form shows the counts of searches, lists and notes. Declining or cancelling leaves the saved file untouched. MCP 2026-07-28 supports confirmation over stdio and HTTP. Hosted workers with the same continuation key can complete each other's confirmations. The 2025-06-18 and 2025-11-25 revisions support the form over stdio. Clients without form support, legacy HTTP clients and the command line delete directly.

Hosted source pacing follows each effective key. Keyless calls share one bucket per source. Set `--rate-share` to the fraction of each keyed source rate assigned to this server, such as `--rate-share 0.25` for each of four instances. Keyless hosted buckets and ordinary process buckets keep the source's full rate. The front end chooses these shares; servers do not coordinate rates with each other. Hosted HTTP calls have a whole-call wall-clock deadline of `command_timeout_seconds`, or a slow source's declared budget for a command that calls it, plus a batched trial get's pacing time (one ClinicalTrials.gov interval per trial after the first, 19 seconds for 20 trials at the default pacing, at most 60 seconds), and return the typed `deadline-exceeded` reason when it expires. The server keeps a bounded number of active calls, including timed-out work until it stops.

A hosted HTTP call runs until its deadline, its client leaves or the server shuts down. Its deadline is `command_timeout_seconds`, 20 seconds by [default](../crates/biomcp/src/config.rs), counted from its arrival, or the longer budget a slow source declares, for a command that calls it, as [command deadlines](#command-deadlines-and-drug-approvals) lists. A batched trial get adds one ClinicalTrials.gov pacing interval per trial after the first, 19 seconds for 20 trials at the default pacing, up to 60 seconds. Concurrent hosted batches share one ClinicalTrials.gov queue, so they can still run out of time; each failed trial then says why. At most `http_max_calls` calls run at once, [8 by default](../crates/biomcp/src/config.rs); a call over the limit waits up to `mcp_queue_wait_seconds`, [10 seconds by default](../crates/biomcp/src/config.rs), and that wait counts against its deadline. A client cancels a call by closing its connection: a waiting call leaves the queue, and a running call stops at once, closing its source requests, and frees its slot when its worker stops; work that is not reading a source stops at its next cancellation check. Over HTTP, `notifications/cancelled` gets HTTP 202 and is ignored, because each call has its own connection and a request ID, chosen by each client, names no single caller. A client that sends `notifications/cancelled` but keeps its request open keeps its slot until the deadline. A front end must keep its connection to BioMCP open in both directions until the reply arrives, and should close it when its client leaves. A connection whose write side is shut down counts as a client that left. Server shutdown cancels every running call.

In hosted `restricted` mode, [cBioPortal study flags](cbioportal-study-terms.md) also withhold studies with noncommercial terms or a reviewed purpose limit when reading the existing catalog, saved study pages and recurrence observations.

`GET /health` checks that the local cache/store opens with compatible metadata within two seconds. It makes no upstream requests, does not verify every stored table or row, and does not promise that sources are available or a query will succeed. Incompatible metadata or an unreadable store returns 503. A missing or damaged payload table can still return 200; this endpoint does not repair it.

HTTP tool calls write one JSON diagnostic with the registered tool name, duration in milliseconds, reply status and failed source names. Arguments, query text and credentials are excluded.
