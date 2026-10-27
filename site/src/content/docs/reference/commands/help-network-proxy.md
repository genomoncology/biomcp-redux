---
title: biomcp help network proxy
description: "Reference for the BioMCP 1.0.0 command biomcp help network proxy:
  Set proxy in config.json to an HTTP or HTTPS proxy URL."
goal: Look up the arguments, sections and examples of biomcp help network proxy.
reader: A BioMCP user or agent checking one command
sidebar:
  label: help network proxy
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp help network proxy`.

Set proxy in config.json to an HTTP or HTTPS proxy URL. Otherwise HTTPS_PROXY and HTTP_PROXY select routes for their schemes; NO_PROXY bypasses exact hosts/IPs, .domain, *.domain, optional ports, or *. An explicit proxy overrides proxy environment settings.

## Usage

```text
biomcp help network proxy
```

## Arguments

The help lists no arguments for this command.

## Examples

The help lists no examples for this command.

## Reply schema

This command returns [`biomcp.help/2`](/reference/schemas/biomcp-help-2/). Print the full JSON schema with `biomcp schema biomcp.help/2`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [commands index](/reference/commands/) lists every command.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
