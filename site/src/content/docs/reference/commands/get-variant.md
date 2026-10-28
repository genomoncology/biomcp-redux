---
title: biomcp get variant
description: "The BioMCP 1.0.0 command biomcp get variant: Reads a variant by
  rsID, ClinVar ID, ClinGen allele ID or HGVS, and optionally sections such as
  ClinVar or population."
goal: Look up the arguments, sections and examples of biomcp get variant.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get variant
kind: reference
verified: 2026-10-28
---

`biomcp get variant` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads a variant by rsID, ClinVar ID, ClinGen allele ID or HGVS, and optionally sections such as ClinVar or population. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Read an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome notation requires --build. Use search variant --gene BRAF --protein V600E for shorthand. Every candidate remains available; source assertions do not classify the variant for the caller.

## Usage

```text
biomcp get variant [OPTIONS] <input> [section...]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--build` | value | no |  |  | Required for chromosome notation; no assembly is guessed. |
| `--civic-cursor` | value | no |  |  | Native CIViC cursor, retained unchanged. |
| `--civic-kind` | value | no |  |  | Native CIViC claim collection within the selected profile. |
| `--civic-profile` | value | no | min 1, max 2147483647 |  | Selected CIViC molecular profile for native claim paging. |
| `--limit` | integer | no | min 1, max 50 |  | Candidate rows per page; more remain reachable through typed next commands. |
| `--offset` | integer | no | min 0 |  | Candidate rows to skip in source order. |
| `--resolver` | string | no |  | `car`, `mutalyzer`, `variantvalidator` | Resolver for reference-qualified notation; never repairs the input locally. |
| `--section-limit` | integer | no | min 1, max 50 |  | Rows per named source section page; omitted uses the native source default. |
| `--section-offset` | integer | no | min 0 |  | Source rows to skip independently of identity candidate paging. |
| `--tumor-type` | value | no |  |  | Explicit tumor context for keyed OncoKB annotation. |
| `input` (position 1) | string | yes |  |  | Published rsID, CAid, ClinVar Variation ID or reference-qualified HGVS. |
| `section` (position 2) | array | no |  | `clinvar`, `erepo`, `population`, `gwas`, `civic`, `oncokb`, `hotspots`, `predict`, `conservation`, `cosmic`, `all` | Named source sections; omitted by default. Each calls its own source. all keeps bounded pages and source skips; it never enables a missing key. |

## Sections

Named source sections; omitted by default. Each calls its own source. all keeps bounded pages and source skips; it never enables a missing key.

| Section | Description |
| --- | --- |
| `clinvar` | ClinVar native assertions. |
| `erepo` | ClinGen native assertions. |
| `population` | gnomAD population observations. |
| `gwas` | GWAS Catalog associations. |
| `civic` | CIViC molecular-profile evidence. |
| `oncokb` | Keyed OncoKB annotation with explicit tumor context and build. |
| `hotspots` | Cancer Hotspots candidate residue counts; no established exact allele join or clinical actionability. |
| `predict` | MyVariant source predictor scores. |
| `conservation` | MyVariant conservation scores. |
| `cosmic` | Historical COSMIC fragments carried by MyVariant; parent-to-fragment allele identity remains unverified. |
| `all` | Every implemented variant section. |

## Examples

The program's help gives these examples.

```sh
biomcp get variant clinvar:13961 clinvar
```

```sh
biomcp get variant CA000434 erepo
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.variant_card/5`](/reference/schemas/biomcp-variant-card-5/), whose page shows a real reply and its fields.

## Next

The [variant guide](/user-guide/variant/) shows every variant command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
