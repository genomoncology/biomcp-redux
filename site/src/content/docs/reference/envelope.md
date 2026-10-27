---
title: Reply envelope
description: The values a BioMCP 1.0.0 reply uses for status, source state,
  failure reason, use group, next-command kind and error code.
goal: Look up the exact values of the fields every BioMCP reply carries.
reader: A developer or agent parsing BioMCP JSON
kind: reference
verified: 2026-10-27
---

Every BioMCP 1.0.0 reply is one JSON object. Data replies carry `schema`, `status`, `query`, `result`, `sources` and `next`, and pages add `page`. Errors carry `schema` `biomcp.error/1`, `status` and `error`. The values below come from the reply schemas; `biomcp schema ID` prints a full schema. [Results](/results/) explains how to read them.

## status

Overall command outcome. Values: `complete`, `partial`, `absent`, `failed`. Error replies use `usage`, `local`, `expired`.

| Status | Meaning | Exit code |
| --- | --- | --- |
| `complete` | Every source the command needed answered. | 0 |
| `partial` | Some sources answered and others failed, or a value a source sent failed its check (`problems`). Read `sources`. | 0 |
| `absent` | The sources answered and found no record for the input. | 1 |
| `failed` | No source the command needed answered. Each one failed or was skipped. | 4 |

## sources[].state

State of a source actually asked by the command. Values: `ok`, `stale`, `syncing`, `failed`, `skipped`.

| State | Meaning |
| --- | --- |
| `ok` | The source answered. |
| `stale` | The source answered from an older local copy. |
| `syncing` | The source reads a local bundle that is not ready. The reply's `next` list names the `sync` command. |
| `failed` | BioMCP asked the source and it did not answer. The `reason` says why. |
| `skipped` | BioMCP did not ask the source, because of your settings or a missing key. The `reason` says why. |

## sources[].reason

Why a source could not supply data.

| Reason | Meaning |
| --- | --- |
| `not_found` | The requested record was not found. |
| `not_open` | The public bucket does not hold an open text record. |
| `not_pdf` | Publisher response is not a PDF. |
| `unsupported` | The local reader does not support the file or container format. |
| `too_large` | The response exceeds an explicit reader resource bound. |
| `rate_limited` | The source limited the request rate. |
| `unavailable` | The source was unavailable. |
| `tls` | Platform or configured certificate verification failed. |
| `proxy` | Connecting to or negotiating the configured proxy failed. |
| `rejected` | The source rejected the request. |
| `bad_reply` | The reply could not be used. |
| `deadline_exceeded` | The request exhausted its deadline. |
| `offline` | Network access was disabled. |
| `no_key` | A required credential was absent. |
| `disabled` | The source was disabled. |
| `syncing` | The source data was being synchronized. |
| `no_join_id` | Identity lacks the identifier required to join this source. |

## sources[].use

Reviewed source-use category, without inferring rights in individual records. Values: `open`, `attribution`, `noncommercial`, `restricted`. [Data sources](/sources/) explains each group.

## problems[].code

A stable code for a rejected field value. Values: `value.unknown`, `collection.incomplete`, `identity.conflict`. Each problem also names its `source`, `record` and `field`.

## next[].kind

Purpose of an available next command. Values: `restart`, `section`, `sort`, `page`, `fresh`, `disambiguate`, `pivot`, `sync`, `key`, `configure`. Each entry's `args` is a complete command without the leading `biomcp`.

| Kind | Meaning |
| --- | --- |
| `restart` | Runs the query again to make a new pool or snapshot, for example after one expired. |
| `section` | Fetches sections this card left out. |
| `sort` | Reads the same search in another order the source supports. |
| `page` | Reads another page. |
| `fresh` | Runs the command again without the cached reply. |
| `disambiguate` | Picks one record when a name matched several. |
| `pivot` | Follows an exact linked record in another entity. |
| `sync` | Downloads a missing or stale local bundle. |
| `key` | Explains how to set a source key. |
| `configure` | Explains a network setting that needs attention, such as the proxy. |

## error.code

The stable error code for an invalid schema request. Values: `usage.schema`, `usage.arguments`, `usage.retired`, `local.failure`, `pool.expired`, `usage.pool_mismatch`, `usage.too_many_pmids`, `usage.unsupported_filter`. [Exit codes](/reference/error-codes/) maps each status to a process exit code.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
