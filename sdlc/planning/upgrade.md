# Upgrade from BioMCP 0.9 to 1.0

This note describes the planned 1.0 migration. Install the approved release through your existing [distribution channel](release.md). Retain the `biomcp-cli` package name, `biomcp` command and stdio `serve` argument. MCP clients call the single `biomcp` tool with the CLI argument array; replace bindings to the old separate typed tools. JSON is the default CLI output. Use `--format md` for Markdown or `--format json` to override configuration.

## Commands

The generated table below lists decided leaf commands and their dropped options from the parity ledger's slice accounting. Its ruling links explain replacements. Section names, aliases and global flags lie outside that ledger; each slice's linked investigation covers them. Unclosed slices and rows marked undecided remain migration work and cannot be treated as approved removals. Regenerate this block with `python3 scripts/parity.py --upgrade` when the accounting changes.

<!-- upgrade-commands -->
| 0.9 command | Outcome | Dropped options | Migration ruling |
| --- | --- | --- | --- |
| `search gene` | Ported or renamed | `--query`, `--region`, `--pathway`, `--go` | [Section 1 maps type and chromosome, preserves paging and removes the query alias and unused filters.](slices/gene.md) |
| `get gene` | Ported or renamed | — | [Section 1 preserves the gene card; section values do not count as option slots.](slices/gene.md) |
| `gene definition` | Ported or renamed | — | [Section 1 replaces this leaf and its get alias with get gene.](slices/gene.md) |
| `gene articles` | Ported or renamed | — | [Section 1 maps this capability to search article --gene; article search preserves paging.](slices/gene.md) |
| `gene pathways` | Ported or renamed | — | [Section 1 maps the command to the pathways section and moves old paging to the pathway slice.](slices/gene.md) |
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

Omit unused keys. The corresponding environment variables are `ONCOKB_API_KEY`, `NCBI_API_KEY` and `SEMANTIC_SCHOLAR_API_KEY`; nonempty environment values override file values. Replace the old desktop `ONCOKB_TOKEN` and `S2_API_KEY` settings with those names. The 0.9 DisGeNET credential has no 1.0 gene section; the gene slice ruling drops that section. Keep credentials local and outside version control.

Optional `ncbi_email` identifies NCBI requests. Optional `ca_bundle` names a PEM file that adds company certificates to platform trust. Optional `proxy` names an HTTP or HTTPS proxy and overrides `HTTPS_PROXY`, `HTTP_PROXY` and `NO_PROXY`. Without it, routing uses those environment variables. `disable_sources` accepts individual sources and the `restricted` and `noncommercial` groups; CLI exclusions add to file exclusions. Run `biomcp health` to inspect enabled sources and reachability.

## Hosted clients

The approved HTTP interface is `biomcp serve --http [ADDR]`, with MCP at `/mcp` and store health at `/health`. The default bind is `127.0.0.1:8080`. A non-loopback listener requires repeatable `--allow-host HOST` arguments or `http.allowed_hosts` in configuration. Foreign hosts and browser origins are refused. Keep public TLS and authentication at your reverse proxy. Source reachability uses `biomcp health`; the HTTP health endpoint makes no source request. The [HTTP decision](../decisions/2026-10-24-1-0-serves-mcp-over-http.md) owns this contract. Complete installed HTTP proof before publishing 1.0.
