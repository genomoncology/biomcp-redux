---
title: Reply of biomcp health (biomcp.health/6)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp health: the fields
  of schema biomcp.health/6."
goal: "Parse the biomcp.health/6 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.health/6
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp health`](/reference/commands/health/) replies with `schema` set to `biomcp.health/6`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.health/6` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_Health](#reply_health). `query` holds [Reply_EmptyQuery](/reference/schemas/types/#reply_emptyquery), a shared type. `query` repeats the request as BioMCP read it.

### Reply_Health

Source reachability and publisher terms, independent of local HTTP service health.

| Field | Type | Meaning |
| --- | --- | --- |
| `configuration` | [Reply_ConfigurationHealth](#reply_configurationhealth) or null | Bounded local settings notices, independent of source probes. Optional. |
| `disable_groups` | list of [Reply_DisableGroup](#reply_disablegroup) | Sources and carried components excluded by each supported terms group. Optional. |
| `sources` | list of [Reply_SourceHealth](#reply_sourcehealth) | Every implemented adapter, in declaration order. Optional. |
| `store` | [Reply_StoreSize](#reply_storesize) | File lengths measured locally without attributing shared pages to [bundles](/local-data/). |

### Other types in this reply

11 more types appear only in this reply: [Reply_BundleSize](#reply_bundlesize), [Reply_ComponentHealth](#reply_componenthealth), [Reply_ConfigurationHealth](#reply_configurationhealth), [Reply_ConfigurationNotice](#reply_configurationnotice), [Reply_ConfigurationNoticeCode](#reply_configurationnoticecode), [Reply_ConfigurationVariable](#reply_configurationvariable), [Reply_DisableGroup](#reply_disablegroup), [Reply_DisabledComponent](#reply_disabledcomponent), [Reply_LocalBundleState](#reply_localbundlestate), [Reply_SourceHealth](#reply_sourcehealth), [Reply_StoreSize](#reply_storesize). Open the list below to read their fields.

<details>
<summary>Fields of the 11 other types</summary>

### Reply_BundleSize

Why physical bytes cannot be attributed to one local bundle.

Values: `shared-storage`.

### Reply_ComponentHealth

Policy labels for one carried source component.

| Field | Type | Meaning |
| --- | --- | --- |
| `disabled` | boolean | Explicit source or component group policy prevents this component. |
| `name` | [Reply_Text](/reference/schemas/types/#reply_text) | Native component name. |
| `terms` | [Reply_ComponentTerms](/reference/schemas/types/#reply_componentterms) | Independently reviewed publisher terms. |

### Reply_ConfigurationHealth

Local configuration notices contain names and positions, never supplied entries.

| Field | Type | Meaning |
| --- | --- | --- |
| `notices` | list of [Reply_ConfigurationNotice](#reply_configurationnotice) | First skipped entries in their original comma-separated order. Optional. |
| `omitted_entries` | integer (uint32) | Remaining skipped entries omitted from this bounded section. |

### Reply_ConfigurationNotice

One unsupported bypass entry skipped without changing other routing rules.

| Field | Type | Meaning |
| --- | --- | --- |
| `code` | [Reply_ConfigurationNoticeCode](#reply_configurationnoticecode) | Stable notice identity. |
| `entry` | integer (uint32) | One-based position in the comma-separated environment setting. |
| `variable` | [Reply_ConfigurationVariable](#reply_configurationvariable) | Environment variable owning the entry. |

### Reply_ConfigurationNoticeCode

Nonfatal local configuration conditions.

Values: `unparseable-entry`.

### Reply_ConfigurationVariable

Settings whose ignored entries can be reported safely.

Values: `NO_PROXY`.

### Reply_DisableGroup

The consequences of one source-disable selector, regardless of current configuration.

| Field | Type | Meaning |
| --- | --- | --- |
| `components` | list of [Reply_DisabledComponent](#reply_disabledcomponent) | Carried fields stopped while their aggregators remain available. Optional. |
| `group` | [Reply_Use](/reference/schemas/types/#reply_use) | Terms category accepted by disable_sources. |
| `sources` | list of [Reply_Source](/reference/schemas/types/#reply_source) | Whole sources stopped by this group. Optional. |

### Reply_DisabledComponent

One independently labeled carried field excluded by a terms group.

| Field | Type | Meaning |
| --- | --- | --- |
| `name` | string | Registry-owned component name. |
| `source` | [Reply_Source](/reference/schemas/types/#reply_source) | Aggregator carrying the field. |

### Reply_LocalBundleState

Local bundle availability. Persisted synchronization leases do not prove process liveness.

Values: `missing`, `fresh`, `stale`.

### Reply_SourceHealth

One bounded source probe, without credentials or provider error text.

| Field | Type | Meaning |
| --- | --- | --- |
| `bundle` | [Reply_LocalBundleState](#reply_localbundlestate) or null | Last validated local bundle, independent of publisher reachability. Optional. |
| `bundle_size` | [Reply_BundleSize](#reply_bundlesize) or null | Per-bundle physical size is unavailable in shared storage. Optional. |
| `components` | list of [Reply_ComponentHealth](#reply_componenthealth) | Independently licensed carried components, without separate network probes. Optional. |
| `disabled` | boolean | Whether source policy prevents all requests and cached reads. |
| `latency_ms` | integer or null | Probe duration in milliseconds; omitted when policy skipped the probe. Optional. |
| `notices` | list of [Reply_SourceNotice](/reference/schemas/types/#reply_sourcenotice) | Local probe limits and recovery guidance. Optional. |
| `reason` | [Reply_Reason](/reference/schemas/types/#reply_reason) or null | Why the source failed, or why BioMCP answered with older stored data. Optional. |
| `source` | [Reply_Source](/reference/schemas/types/#reply_source) | Implemented source identity. |
| `state` | [Reply_SourceState](/reference/schemas/types/#reply_sourcestate) | What happened: `ok`, `stale`, `syncing`, `failed` or `skipped`. |
| `terms` | [Reply_Text](/reference/schemas/types/#reply_text) | Publisher license or terms name. |
| `terms_url` | [Reply_Text](/reference/schemas/types/#reply_text) | Publisher terms page. |
| `use` | [Reply_Use](/reference/schemas/types/#reply_use) | Reviewed source category, not blanket permission for each record. |

### Reply_StoreSize

Measured lengths of owned files, including unused database pages and present sidecars.

| Field | Type | Meaning |
| --- | --- | --- |
| `active_bytes` | integer (uint64) | Active database plus its present WAL and SHM files, counted once each. |
| `partial_download_bytes` | integer (uint64) | Incomplete download files belonging to active or retained databases. |
| `retained_bytes` | integer (uint64) | Retained database generations plus their present WAL and SHM files. |
| `total_bytes` | integer (uint64) | All measured owned files, including retained generations and partial downloads. |

</details>

## Next

[`biomcp health`](/reference/commands/health/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
