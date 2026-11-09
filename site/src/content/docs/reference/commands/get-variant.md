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
verified: 2026-11-09
---

`biomcp get variant` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads a variant by rsID, ClinVar ID, ClinGen allele ID or HGVS, and optionally sections such as ClinVar or population. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Get genetic variants. Sources: alphagenome, car, civic, clinvar, erepo, gnomad, gwas, hotspots, mutalyzer, myvariant, oncokb, variantvalidator.

## Usage

```text
biomcp get variant [OPTIONS] <input> [section...]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--build` | string | no |  | `GRCh37`, `GRCh38` | Assembly; help get variant Assembly MyVariant identity assembly defaults to GRCh38; chromosome genomic HGVS requires an explicit build. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --build GRCh37. Command sources: alphagenome, car, civic, clinvar, erepo, gnomad, gwas, hotspots, mutalyzer, myvariant, oncokb, variantvalidator; source restrictions in the field description apply. |
| `--civic-cursor` | string | no |  |  | Cursor Source CIViC cursor, retained unchanged. Value form: string. Example: --civic-cursor 0:E2. Command sources: alphagenome, car, civic, clinvar, erepo, gnomad, gwas, hotspots, mutalyzer, myvariant, oncokb, variantvalidator; source restrictions in the field description apply. |
| `--civic-kind` | string | no |  | `evidence`, `assertions` | Claims; help get variant Claims Source CIViC claim collection within the selected profile. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --civic-kind evidence. Command sources: alphagenome, car, civic, clinvar, erepo, gnomad, gwas, hotspots, mutalyzer, myvariant, oncokb, variantvalidator; source restrictions in the field description apply. |
| `--civic-profile` | integer | no | min 1, max 2147483647 |  | Selected CIViC molecular profile for source claim paging. Value form: integer. Example: --civic-profile 12. Command sources: alphagenome, car, civic, clinvar, erepo, gnomad, gwas, hotspots, mutalyzer, myvariant, oncokb, variantvalidator; source restrictions in the field description apply. |
| `--evidence-level` | array | no |  | `A`, `B`, `C`, `D`, `E` | A–E; help get variant A–E Source CIViC evidence levels; repeat to include several levels. Listed choices accept any letter case; replies use their listed spelling. Value form: repeatable values (repeat the flag). Example: --evidence-level A. Command sources: alphagenome, car, civic, clinvar, erepo, gnomad, gwas, hotspots, mutalyzer, myvariant, oncokb, variantvalidator; source restrictions in the field description apply. |
| `--evidence-status` | string | no |  | `accepted`, `submitted`, `rejected` | Status; help get variant Status Source CIViC evidence status; omitted preserves the source's existing default. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --evidence-status submitted. Command sources: alphagenome, car, civic, clinvar, erepo, gnomad, gwas, hotspots, mutalyzer, myvariant, oncokb, variantvalidator; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 50 |  | Candidate rows per page; more remain reachable through typed next commands. Value form: integer. Example: --limit 10. Command sources: alphagenome, car, civic, clinvar, erepo, gnomad, gwas, hotspots, mutalyzer, myvariant, oncokb, variantvalidator; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Candidate rows to skip in source order. Value form: integer. Example: --offset 0. Command sources: alphagenome, car, civic, clinvar, erepo, gnomad, gwas, hotspots, mutalyzer, myvariant, oncokb, variantvalidator; source restrictions in the field description apply. |
| `--resolver` | string | no |  | `car`, `mutalyzer`, `variantvalidator` | Resolver; help get variant Resolver. Resolver for reference-qualified notation; never repairs the input locally. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --resolver variantvalidator. Command sources: alphagenome, car, civic, clinvar, erepo, gnomad, gwas, hotspots, mutalyzer, myvariant, oncokb, variantvalidator; source restrictions in the field description apply. |
| `--section-limit` | integer | no | min 1, max 50 |  | Rows per named source section page; omitted uses the source default. Value form: integer. Example: --section-limit 2. Command sources: alphagenome, car, civic, clinvar, erepo, gnomad, gwas, hotspots, mutalyzer, myvariant, oncokb, variantvalidator; source restrictions in the field description apply. |
| `--section-offset` | integer | no | min 0 |  | Source rows to skip independently of identity candidate paging. Value form: integer. Example: --section-offset 0. Command sources: alphagenome, car, civic, clinvar, erepo, gnomad, gwas, hotspots, mutalyzer, myvariant, oncokb, variantvalidator; source restrictions in the field description apply. |
| `--tumor-type` | string | no |  |  | Tumor Explicit tumor context for keyed OncoKB annotation. Value form: string. Example: --tumor-type &lt;tumor-type&gt; (syntax; replace the placeholder with your retained value). Command sources: alphagenome, car, civic, clinvar, erepo, gnomad, gwas, hotspots, mutalyzer, myvariant, oncokb, variantvalidator; source restrictions in the field description apply. |
| `input` (position 1) | string | yes |  |  | rs/CA/clinvar:N/VCV/HGVS/VCF rsID, CAid, clinvar:N, VCV accession, reference-qualified HGVS or VCF-style genomic coordinates. |
| `section` (position 2) | array | no |  | `clinvar`, `erepo`, `population`, `gwas`, `civic`, `alphagenome`, `oncokb`, `hotspots`, `predict`, `conservation`, `cosmic`, `all` | Sections; help get variant Listed choices accept any letter case; replies use their listed spelling. |

## Sections

Named source sections; omitted by default. Each calls its own source. all keeps bounded pages and source skips; it never enables a missing key.

| Section | Description |
| --- | --- |
| `clinvar` | ClinVar source assertions. |
| `erepo` | ClinGen source assertions. |
| `population` | gnomAD population observations. |
| `gwas` | GWAS Catalog associations. |
| `civic` | CIViC molecular-profile evidence. |
| `alphagenome` | Keyed AlphaGenome source numeric scorer values. |
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

```sh
biomcp get variant 'chr5:g.118860951A>G' --build GRCh37
```

```sh
biomcp get variant CA123643 civic --disable-sources myvariant \
  --section-limit 2 --evidence-level A --evidence-status submitted
```

```sh
biomcp get variant CA123643 civic --disable-sources myvariant \
  --section-limit 2 --evidence-level A --evidence-status submitted \
  --civic-profile 12 --civic-kind evidence --civic-cursor 0:E2
```

```sh
biomcp get variant CA123643 --resolver variantvalidator --build GRCh38
```

## Reply

The reply is one JSON object. Its schema is [`biomcp.variant-card/15`](/reference/schemas/biomcp-variant-card-15/), whose page lists its fields.

## Next

The [variant guide](/user-guide/variant/) shows every variant command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
