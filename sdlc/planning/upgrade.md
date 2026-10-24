# Upgrade from BioMCP 0.9 to 1.0

This note describes the planned 1.0 migration. Install the approved release through your existing [distribution channel](release.md). Retain the `biomcp-cli` package name, `biomcp` command and stdio `serve` argument. MCP clients call the single `biomcp` tool with the CLI argument array; replace bindings to the old separate typed tools. JSON is the default CLI output. Use `--format md` for Markdown or `--format json` to override configuration.

## Commands

The generated table below lists decided leaf commands and their dropped options from the parity ledger's slice accounting. Its ruling links explain replacements. Section names, aliases and global flags lie outside that ledger; each slice's linked investigation covers them. Unclosed slices and rows marked undecided remain migration work and cannot be treated as approved removals. Regenerate this block with `python3 scripts/parity.py --upgrade` when the accounting changes.

<!-- upgrade-commands -->
| 0.9 command | Outcome | Dropped options | Migration ruling |
| --- | --- | --- | --- |
| `search article` | Ported or renamed | `--ranking-mode`, `--weight-semantic`, `--weight-lexical`, `--weight-citations`, `--weight-position`, `--max-per-source`, `--session`, `--debug-plan`, `--full` | [Section 5 maps selectors and paging to typed search, replaces keyword aliases with TEXT, and retires blends and hidden planning under the no-text-heuristics ruling.](slices/article.md) |
| `get article` | Ported or renamed | `--asset-view`, `--asset-limit`, `--asset-offset` | [Section 5 moves parts into the reader. Explicit-path text and supplement saves remain pending 0076; PDF remains pending 0052 under the download-only ruling.](slices/article.md) |
| `article authors` | Ported or renamed | — | [Section 1 maps author disclosure to the article card.](slices/article.md) |
| `article entities` | Ported or renamed | `--limit`, `--full` | [Section 5 maps normalized entities to the annotations section with one bounded shape.](slices/article.md) |
| `article batch` | Ported or renamed | — | [Section 1 maps repeated inputs to get article.](slices/article.md) |
| `article citations` | Ported or renamed | — | [Section 5 keeps the Semantic Scholar graph one source option away and adds iCite as the default.](slices/article.md) |
| `article references` | Ported or renamed | — | [Section 5 keeps contexts and intents through the explicit graph source.](slices/article.md) |
| `article citation-evidence` | Ported or renamed | — | [Section 5 preserves the pair reader and optional citing paragraphs.](slices/article.md) |
| `article supplement-search` | Ported or renamed | — | [Section 5 replaces the asset selector with the exact file positional input of article supplement.](slices/article.md) |
| `article recommendations` | Ported or renamed | — | [Section 5 preserves native positive and negative seed recommendations.](slices/article.md) |
| `variant articles` | Ported or renamed | `--input`, `--debug-plan`, `--verify-identity`, `--confirmed-only`, `--strategy` | [The no-text-heuristics ruling replaces hidden plans and identity scores with explicit variant and source choices; variant notation belongs to its slice.](slices/article.md) |
| `disease articles` | Ported or renamed | — | [Section 1 moves the capability to search article --disease.](slices/article.md) |
| `search gene` | Ported or renamed | `--query`, `--region`, `--pathway`, `--go` | [Section 1 maps type and chromosome, preserves paging and removes the query alias and unused filters.](slices/gene.md) |
| `get gene` | Ported or renamed | — | [Section 1 preserves the gene card; section values do not count as option slots.](slices/gene.md) |
| `gene definition` | Ported or renamed | — | [Section 1 replaces this leaf and its get alias with get gene.](slices/gene.md) |
| `gene trials` | Ported or renamed | — | [The accepted trial rulings move discovery to search trial --gene, preserve the limit and source choices, and rename source paging to --next-page.](slices/gene.md) |
| `gene articles` | Ported or renamed | — | [Section 1 maps this capability to search article --gene; article search preserves paging.](slices/gene.md) |
| `gene pathways` | Ported or renamed | — | [Section 1 maps the command to the pathways section and moves old paging to the pathway slice.](slices/gene.md) |
| `search trial` | Ported or renamed | `--no-alias-expand`, `--prior-therapies`, `--progression-on`, `--line-of-therapy` | [The accepted trial rulings map literal discovery, source selection, bounded sections, saves and explicit retirements.](slices/trial.md) |
| `get trial` | Ported or renamed | — | [The accepted trial rulings map literal discovery, source selection, bounded sections, saves and explicit retirements.](slices/trial.md) |
| `batch trial` | Ported or renamed | `--mode`, `--pdf` | [The accepted trial rulings map literal discovery, source selection, bounded sections, saves and explicit retirements.](slices/trial.md) |
| `variant trials` | Ported or renamed | — | [The accepted trial rulings map literal discovery, source selection, bounded sections, saves and explicit retirements.](slices/trial.md) |
| `disease trials` | Ported or renamed | — | [The accepted trial rulings map literal discovery, source selection, bounded sections, saves and explicit retirements.](slices/trial.md) |
| `drug trials` | Ported or renamed | `--no-alias-expand` | [The accepted trial rulings map literal discovery, source selection, bounded sections, saves and explicit retirements.](slices/trial.md) |
| `search variant` | Ported or renamed | — | [Use declared gene plus --protein, explicit region/build, or unchanged --query with --source myvariant. --hgvsp names --protein in its retirement message. Only the three captured ClinVar filters are offered; the other mappings remain undecided. Native candidate pages retain selectors and source order.](slices/variant.md) |
| `get variant` | Ported or renamed | — | [Use published-grammar rsID, CAid, ClinVar Variation ID or reference-qualified HGVS. --assembly names --build; chromosome notation needs it. CAR is the default HGVS resolver. Shorthand names search variant --gene BRAF --protein V600E. Every candidate stays available.](slices/variant.md) |
| `variant structure` | Dropped by ruling | — | [Deferred by ruling 6. Its retirement message names get gene <SYMBOL> protein; no structure or clinical equivalence is inferred.](slices/variant.md) |
| `variant oncokb` | Ported or renamed | — | [Use get variant <ID> oncokb with explicit --build and --tumor-type. Missing keys skip before source access; synthetic keyed cases cover annotation and credential containment. Actual licensed-tier drug-response proof remains separate and does not follow from a configured key.](slices/variant.md) |
| `variant erepo` | Ported or renamed | — | [Use variant assertions <CAid> --source erepo; the input becomes positional. --assertion <PCERid> with optional --version selects details. Limit and offset page native assertions. The old leaf names this route. No gene-wide scan replaces missing allele assertions; the legacy --gene slot remains undecided pending an explicit product ruling.](slices/variant.md) |
| `variant normalize` | Ported or renamed | — | [Use variant normalize --source car, mutalyzer, variantvalidator or all with --hgvs TEXT. The old positional service and --input name this form. VariantValidator and all require --build. Results and mappings remain independent; there is no local repair or consensus.](slices/variant.md) |
<!-- /upgrade-commands -->

## Configuration and keys

Create `biomcp/config.json` under `$XDG_CONFIG_HOME`, or under `$HOME/.config` when that variable is absent. On Windows use `%APPDATA%/biomcp/config.json`. The configuration is JSON and rejects unknown fields. Move settings deliberately; do not copy a 0.9 configuration wholesale.

```json
{
  "format": "json",
  "disable_sources": [],
  "keys": {
    "oncokb_api_key": "YOUR_ONCOKB_KEY",
    "ncbi_api_key": "YOUR_NCBI_KEY",
    "semantic_scholar_api_key": "YOUR_SEMANTIC_SCHOLAR_KEY"
  }
}
```

Register each key with `biomcp key set <source>`, which prompts for the key, keeps it out of shell history and writes it to this file. Omit unused keys. The corresponding environment variables are `ONCOKB_API_KEY`, `NCBI_API_KEY` and `SEMANTIC_SCHOLAR_API_KEY`; stored file values override environment values. Replace the old desktop `ONCOKB_TOKEN` and `S2_API_KEY` settings with those names. The 0.9 DisGeNET credential has no 1.0 gene section; the gene slice ruling drops that section. Keep credentials local and outside version control.

Optional `ncbi_email` identifies NCBI requests. Optional `ca_bundle` names a PEM file that adds company certificates to platform trust. Optional `proxy` names an HTTP or HTTPS proxy and overrides `HTTPS_PROXY`, `HTTP_PROXY` and `NO_PROXY`. Without it, routing uses those environment variables. `disable_sources` accepts individual sources and the `restricted` and `noncommercial` groups; CLI exclusions add to file exclusions. Run `biomcp health` to inspect enabled sources and reachability.

## Hosted clients

The approved HTTP interface is `biomcp serve --http [ADDR]`, with MCP at `/mcp` and store health at `/health`. The default bind is `127.0.0.1:8080`. A non-loopback listener requires repeatable `--allow-host HOST` arguments or `http.allowed_hosts` in configuration. Foreign hosts and browser origins are refused. Keep public TLS and authentication at your reverse proxy. Source reachability uses `biomcp health`; the HTTP health endpoint makes no source request. The [HTTP decision](../decisions/2026-10-24-1-0-serves-mcp-over-http.md) owns this contract. Complete installed HTTP proof before publishing 1.0.

## Trial discovery and documents

`search trial TEXT` sends general registry text; `--condition TEXT` searches conditions only. `--gene`, `--variant` and `--criteria` support discovery and never determine clinical eligibility. `--mutation` and `--biomarker` retire in favor of literal `--variant` text. The semantic therapy-line filters retire with `--criteria` guidance. Registry update dates use `--updated-since` and `--updated-until`. Search offset paging becomes the opaque `--next-page` action; keep the same filters.

ClinicalTrials.gov is the default. Explicit `--source nci` requires the user's NCI key and preserves NCI's own counts and statuses without fallback. NCI condition filters require an NCIt identifier; use `--keyword` for explicit free text. Unsupported registry-only filters return guidance before any request. Automatic disease resolution is not shipped.

Use multiple NCT inputs on `get trial` instead of `batch trial`, and trailing section words instead of batch `--sections`. Cards precede requested sections. Every truncated section has its own continuation. A registry reference's type and position do not identify a primary publication. List `get trial NCT documents`, then save one exact manifest filename with `trial document NCT FILE --save PATH`. Documents are downloaded without text extraction. CLI and stdio MCP support saves; HTTP refuses and returns the source URL.
