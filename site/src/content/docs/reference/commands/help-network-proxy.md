---
title: biomcp help network proxy
description: "The BioMCP 1.0.0 command biomcp help network proxy: Shows how to
  make BioMCP reach the sources through a proxy."
goal: Look up the arguments, sections and examples of biomcp help network proxy.
reader: A BioMCP user or agent checking one command
sidebar:
  label: help network proxy
kind: reference
verified: 2026-11-09
---

`biomcp help network proxy` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It shows how to make BioMCP reach the sources through a proxy. This page lists its reply.

The program's help describes it:

> Set proxy in config.json to an HTTP or HTTPS proxy URL. Otherwise https_proxy/HTTPS_PROXY and http_proxy/HTTP_PROXY select routes for their schemes; lower case wins when both names are set. no_proxy/NO_PROXY bypasses exact hosts/IPs, CIDR ranges, .domain, *.domain, optional ports, or *. Bare IPv6 is accepted. Unsupported entries are skipped and health reports configuration notices. Malformed ports and credentials refuse with the variable name. An explicit proxy overrides proxy environment settings. Publisher PDF downloads are refused when a proxy applies.

## Usage

```text
biomcp help network proxy
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.help/6`](/reference/schemas/biomcp-help-6/), whose page lists its fields.

## Next

[Keys and configuration](/getting-started/api-keys/) sets keys, a proxy and a certificate [bundle](/local-data/). The [commands index](/reference/commands/) lists every command.
