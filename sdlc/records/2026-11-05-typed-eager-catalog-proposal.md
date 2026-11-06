# Proposal: bound the eager typed catalog

This is a measurement and design proposal for Typed tools fit eager clients. It does not implement the grouping.

## Measured baseline

Measurement job `j31538` passed on the build machine. Its source revision was `359fbba1873ec43944c0d854c1ef154ee3f2e3f3`, which adds only the tracked measurement script to landed-main product revision `6b1618cc2ca4d8cf8f608e779f76ea4dfcadfa40`. `scripts/measure_typed_catalog.py` builds that product once and requests actual `tools/list` replies in isolated, offline configurations. It counts the compact UTF-8 serialization of the entire `result.tools` array, including names, annotations and input/output schemas. It excludes the JSON-RPC envelope and server instructions. These are byte measurements, not token measurements.

| Configuration | Tools | Definition bytes |
| --- | ---: | ---: |
| Single mode, stdio | 3 | 10,401 |
| Typed mode, stdio | 61 | 72,769 |
| Hosted typed mode, stdio | 59 | 71,450 |

The hosted measurement exercises the stdio hosted policy; it is not a measurement of an AWS deployment or a caller-specific HTTP catalog. No live sources were contacted. The older figure of about 58 tools understates this revision's catalog. Typed mode also currently omits the `biomcp` escape hatch; restoring it is part of the proposal.

The job's complete elapsed time was about 249 seconds, including queue/setup. Cargo reported 144 seconds for its debug build. The job reported about 1.58 GiB peak disk and 0.80 GiB peak memory. The host was shared and reported load 15.57. These are the cost of this bounded measurement, not a product latency benchmark. The first measurement request was cancelled while queued to correct the mode flag; it never built.

## Recommended defaults

Define `DEFAULT_TYPED_TOOL_LIMIT = 15` and `DEFAULT_TYPED_DEFINITION_BYTES = 32_768`, exposed as `mcp_typed_tool_limit` and `mcp_typed_definition_bytes` in the existing configuration owner. Define the values once and have catalog construction and its outside test read the same owner.

The [research record](2026-11-04-mcp-skills-and-design-research.md), in its code-execution discussion, cites Anthropic's tool-search guidance at about ten tools or 10,000 definition tokens. It also records the recommendation to combine endpoint wrappers and assess real tasks. The ticket cites the earlier design's roughly fifteen-tool target. Neither source establishes an optimal byte budget. Fifteen is the proposed bounded compromise, not a claim that fifteen beats ten. 32 KiB sits below a rough 40 KB / 10,000-token conversion and leaves some description headroom, but bytes per token vary by model and vocabulary; this byte cap is an engineering guardrail, not a token guarantee.

Keep ten direct command tools: search/get for gene, variant, drug, trial and article. Keep `biomcp_help`, research `search` and `fetch`, restore `biomcp`, and add `biomcp_reference` for the other commands. That yields fifteen tools.

Proposed reference call:

```json
{"name":"biomcp_reference","arguments":{"command":"get_protein","arguments":{"id":"P04637"}}}
```

The `command` enum comes from the current registry and execution policy; the nested `arguments` object goes through the same schema validation and CLI conversion as its current named typed tool. The nested object does not inline all command schemas. `biomcp_help` supplies the full contract when needed. The reference tool therefore trades an immediately visible specialized schema for a smaller eager list, while keeping typed values and the existing validator. The escape hatch retains the complete CLI surface.

The ten direct tools currently total 17,756 bytes. Keeping the existing help/research definitions and restoring the existing `biomcp` definition produces fourteen tools totaling 28,648 bytes, with array punctuation accounted for. A synthetic reference definition with the remaining 48 command names, a complete description, generic nested object and annotations measures 1,426 bytes. The projected fifteen-tool catalog is therefore 30,074 bytes: about 59% fewer definition bytes and 75% fewer tools. This is a composition estimate using measured current definitions, not an implemented grouped result. The proposed byte cap leaves 2,694 bytes for complete descriptions and small changes. The description lane reports small expected growth; the combined result must be measured again, without truncating prose or raising the default to bless an overrun.

## What “least used” can and cannot mean here

There is no production usage histogram or representative frequency ranking in the supplied evidence. The retest explicitly suggested grouping protein and pathway subtools. The first evaluation says it did not test protein, gwas, cell-line, biomarker, organization or author, and it did exercise disease, diagnostic, pgx, pathway and study alongside the five proposed primary entities. Untested does not mean unused.

Selecting those five primary entities is therefore a clinical-workflow policy assumption. Grouping protein and pathway alone cannot reach fifteen tools. The proposal groups disease, the specialist entities and additional verbs of primary entities too. This wider grouping is the design expansion that needs coordinator review before implementation. A seventeen-tool alternative would retain direct disease search/get, at a projected 32,738 definition bytes after removing their two names from the reference enum. That leaves only 30 bytes under the proposed 32 KiB cap, so the expected description improvements would probably require regrouping a command. This alternative still does not establish a measured popularity ranking.

## Shared selection and configuration

Use one deterministic selection owner in a new `typed/grouping.rs`. It receives the already rendered eligible command definitions, starts with the primary get/search priority, and moves the lowest-priority direct commands behind the reference wrapper until both configured ceilings hold. Regenerate the reference enum from the same admitted set. This makes a lower cap meaningful and prevents source exclusions or larger descriptions from silently exceeding it. Larger caps need not automatically restore all specialist wrappers in this first implementation.

The fixed help/research/escape/reference surface needs at least five slots and its own minimum bytes. Reject a configuration that cannot hold that surface with a clear message naming the requested and required budgets; do not silently drop capabilities or return the current unrelated “Output schema unavailable” message. Count all definition fields, including optional output schemas.

Catalog listing and follow-up translation must consult the same plan. A grouped follow-up must name `biomcp_reference`, not an unadvertised historical name. The follow-up lane currently translates using only the CLI root; configurable byte-sensitive membership also needs effective settings and source-policy context. Pass that context through the existing response boundary once. Avoid regenerating the full catalog separately for every suggested step. Keep existing command conversion and source-policy owners intact. Existing unadvertised named calls may remain accepted as compatibility aliases, but new follow-ups use advertised tools.

This is larger than filtering a few names: it introduces one small catalog policy shared with follow-ups. A smaller alternative is a fixed fifteen-tool surface with configurable ceilings that reject undersized settings instead of changing membership. That avoids caller-aware selection plumbing, but makes lowering a budget a configuration error rather than a compact catalog. The recommendation is the shared selection owner, subject to coordinator approval of the scope.

## Proposed changes and proof

Expected product files:

- `crates/biomcp/src/mcp/typed/grouping.rs`: new selection, reference definition and wrapper dispatch owner, roughly 150–220 lines.
- `crates/biomcp/src/mcp/typed.rs`: list/parse hooks and response-context plumbing, roughly 25–45 changed lines; coordinate after the follow-up lane.
- `crates/biomcp/src/mcp/typed/summary.rs` and its follow-up module: one advertised-call adaptation hook, roughly 15–30 changed lines; no competing translator.
- `crates/biomcp/src/config.rs` and `config/file.rs`: defaults, configuration fields and validation, roughly 20–40 changed lines.
- `crates/biomcp/src/mcp/rpc.rs`: preserve the actionable invalid-budget error, roughly 5–15 changed lines if needed.

Expected proof files: `typed_tools.rs`, `typed_schemas.rs` and `typed_help.rs` replace their assumption that every command must have a separate tool; `typed_hosted.rs`, `typed_policy.rs` and the parity case helper exercise grouped calls through existing recordings; a small `typed_budget.rs` case checks actual serialized bytes, count, deterministic membership, a smaller valid configuration, invalid undersized configuration, output-schema settings and complete reachability. Reuse current parity cases for grouped entities rather than creating a second fixture collection. Expected test edits are roughly 150–250 lines, subject to the existing helper boundaries.

Run the focused installed typed cases once after the integration, followed by the routine gate on the combined branch. Verify single mode's definitions and CLI records remain equal, hosted refusals remain intact, and typed mode remains opt-in. No hosted deployment, release candidate, publication or new data source is part of this proposal. No product code has changed at this checkpoint.
