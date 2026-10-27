---
title: biomcp sync cbioportal-studies
description: "Reference for the BioMCP 1.0.0 command biomcp sync
  cbioportal-studies: Replace the complete validated local bundle atomically."
goal: Look up the arguments, sections and examples of biomcp sync cbioportal-studies.
reader: A BioMCP user or agent checking one command
sidebar:
  label: sync cbioportal-studies
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp sync cbioportal-studies`.

Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.

## Usage

```text
biomcp sync cbioportal-studies
```

## Arguments

The help lists no arguments for this command.

## Examples

```sh
biomcp sync cbioportal-studies
biomcp sync cbioportal-studies --fresh
```

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
