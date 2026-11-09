---
title: biomcp serve
description: "The BioMCP 1.0.0 command biomcp serve: Runs BioMCP as an MCP
  server over standard input and output, or over HTTP."
goal: Look up the arguments, sections and examples of biomcp serve.
reader: A BioMCP user or agent checking one command
sidebar:
  label: serve
kind: reference
verified: 2026-11-09
---

`biomcp serve` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It runs BioMCP as an MCP server over standard input and output, or over HTTP. This page lists its arguments and examples.

The program's help describes it:

> Serve MCP over stdio or --http [ADDR]. Non-loopback binds require --allow-host HOST or http.allowed_hosts; neither authenticates callers. HTTP refuses sync, store clean and --fresh. Stdio saves refuse --replace. --hosted refuses file saves, health details, sync, store clean, --fresh and key commands on every transport with a usage.mcp_unavailable error. MCP revisions: 2026-07-28, 2025-11-25, 2025-06-18, 2025-03-26. --rate-share scales only keyed hosted source rates. Hosted HTTP calls have bounded work and one whole-call deadline. Send MCP-Protocol-Version on HTTP requests. Without it, initialize uses its requested legacy revision; other requests default to 2025-03-26. Unsupported headers return HTTP 400.

## Usage

```text
biomcp serve [OPTIONS]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--allow-host` | string | no |  |  | Allow an HTTP Host authority; required for non-loopback listeners. Example: --allow-host api.example.org. Value form: string. Local command option; independent of source selection. |
| `--hosted` | boolean | no |  | `true`, `false` | Refuse host file saves, health details and store or key changes over MCP. Value form: boolean. Example: --hosted. Local command option; independent of source selection. |
| `--http` | string | no |  |  | Listen on an HTTP address; omit its value for the loopback default. Value form: string. Example: --http 127.0.0.1:8080. Local command option; independent of source selection. |
| `--http_max_calls` | string | no |  |  | Limit tool calls in flight. Example: --http_max_calls 8. Value form: string. Local command option; independent of source selection. |
| `--http_max_connections` | string | no |  |  | Limit open HTTP connections; must cover http_max_calls. Example: --http_max_connections 64. Value form: string. Local command option; independent of source selection. |
| `--rate-share` | string | no |  |  | Use this share of keyed source rates in hosted mode (greater than 0, at most 1). Value form: string. Example: --rate-share 1. Local command option; independent of source selection. |
| `--tool-mode` | string | no |  | `single`, `typed` | Advertise one CLI tool or generated typed command tools; default single. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --tool-mode single. Local command option; independent of source selection. |

## Examples

The program's help gives these examples.

```sh
biomcp serve
```

```sh
biomcp help serve
```

## Next

[Run as a server](/getting-started/remote-http/) starts the HTTP server. The [commands index](/reference/commands/) lists every command.
