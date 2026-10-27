---
title: biomcp article recommendations
description: "Reference for the BioMCP 1.0.0 command biomcp article
  recommendations: Recommendations in the provider's recent-paper pool order."
goal: Look up the arguments, sections and examples of biomcp article recommendations.
reader: A BioMCP user or agent checking one command
sidebar:
  label: article recommendations
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 command `biomcp article recommendations`.

Recommendations in the provider's recent-paper pool order.

## Usage

```text
biomcp article recommendations [OPTIONS] <seeds...>
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--limit` | integer | no | min 1, max 100 |  | Maximum recommendations requested, in native order. |
| `--negative` | array | no | max 10 |  | Up to ten negative paper identities. |
| `--offset` | integer | no | min 0 |  | Number of stored recommendations to skip. |
| `--pool` | string | no |  |  | Fixed recommendation pool. |
| `seeds` (position 1) | array | yes | min 1, max 10 |  | One to ten positive paper identities. |

## Examples

```sh
biomcp article recommendations 34096690 --limit 5
biomcp article recommendations 34096690 32955176 --negative 33433946 --limit 5
```

## Reply schema

The help examples of this command return [`biomcp.article_recommendations/3`](/reference/schemas/biomcp-article-recommendations-3/). Print the full JSON schema with `biomcp schema biomcp.article_recommendations/3`.

## Exit codes

See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).

## Next

The [article entity](/user-guide/article/) page lists every article command and links to a worked guide.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
