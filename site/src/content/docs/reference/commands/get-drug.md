---
title: biomcp get drug
description: "The BioMCP 1.0.0 command biomcp get drug: Reads one drug record by
  its ID in one source, and optionally a section such as its label or
  approvals."
goal: Look up the arguments, sections and examples of biomcp get drug.
reader: A BioMCP user or agent checking one command
sidebar:
  label: get drug
kind: reference
verified: 2026-11-09
---

`biomcp get drug` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It reads one drug record by its ID in one source, and optionally a section such as its label or approvals. A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. This page lists its arguments, sections, examples and reply.

The program's help describes it:

> Get drug and medicine details. Sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq.

## Usage

```text
biomcp get drug [OPTIONS] <input> [section...]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--civic-cursor` | string | no |  |  | CIViC continuation. Source continuation cursor for that exact drug, profile and claim collection. Value form: string. Example: --civic-cursor 0:A2. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--civic-kind` | string | no |  | `evidence`, `assertions` | CIViC collection; help get drug CIViC collection. Independent source evidence or assertion collection within the selected profile. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --civic-kind evidence. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--civic-profile` | integer | no | min 1, max 2147483647 |  | Selected source CIViC molecular profile for a drug-scoped claim stream. Value form: integer. Example: --civic-profile 1. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--document-id` | string | no |  |  | FDA letter document ID. Exact source Drugs@FDA approval letter document ID. Text is bounded to 25 MiB, 100 pages and 6144 escaped bytes per fragment. No-text scans are explicit. Value form: string. Example: --document-id 64255. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--evidence-level` | array | no |  | `A`, `B`, `C`, `D`, `E` | A–E; help get drug A–E Source CIViC evidence levels; repeat to include several levels. Listed choices accept any letter case; replies use their listed spelling. Value form: repeatable values (repeat the flag). Example: --evidence-level A. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--evidence-status` | string | no |  | `accepted`, `submitted`, `rejected` | Status; help get drug Status Source CIViC evidence status; omitted preserves the source's existing default. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --evidence-status accepted. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--label-section` | string | no |  |  | Exact openFDA SPL field. Exact published openFDA SPL section field, including table fields. Omit it to list names. Value form: string. Example: --label-section warnings_and_cautions. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--product-number` | string | no |  |  | FDA product number. Explicit source application product number, only for approvals. Value form: string. Example: --product-number 001. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--section-limit` | integer | no | min 1, max 50 |  | Named section page size, refused on an identity-only card. Value form: integer. Example: --section-limit 2. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--section-offset` | integer | no | min 0 |  | Named section continuation, refused on an identity-only card. Value form: integer. Example: --section-offset 1. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `--source` | string | no |  | `who-pq`, `fda-drug-recall`, `mychem`, `chembl`, `opentargets`, `civic`, `openfda-label`, `drugsfda`, `ema`, `ddinter` | Omit to infer the source of an unambiguous section; identity or; help get drug Omit to infer the source of an unambiguous section; identity or mixed sections use MyChem. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source ema. Command sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq; source restrictions in the field description apply. |
| `input` (position 1) | string | yes |  |  | Source ID, brand or generic name. Source-source ID or complete brand or generic name; WHO references retain source spaces. Exact brands rank before generics; Drugs@FDA prefers NDA/BLA before ANDA, then earliest approval. Other sources retain search order. The chosen record reports up to five alternatives and their total. Generic names also accept one trailing listed salt or ester term: mesylate, hydrochloride, sodium, potassium, calcium, sulfate, maleate, tartrate, citrate, phosphate, acetate, besylate, fumarate, succinate, tosylate, bromide, chloride. Brand names identical to their own ingredient count as generic. USAN nomenclature: https://www.ama-assn.org/about/united-states-adopted-names-usan/united-states-adopted-names-naming-guidelines |
| `section` (position 2) | array | no |  | `xrefs`, `targets`, `indications`, `civic`, `label`, `approvals`, `regulatory`, `interactions` | Source sections; help get drug Listed choices accept any letter case; replies use their listed spelling. |

## Sections

One named disclosure per call for every source; empty means identity only.

| Section | Description |
| --- | --- |
| `xrefs` | All source identity cross-references and product links, mechanically paged. |
| `targets` | Source target and mechanism relations. |
| `indications` | Source indication wording. |
| `civic` | Source CIViC therapy evidence and profile context. |
| `label` | One exact source label heading. |
| `approvals` | Source application and product history rows. |
| `regulatory` | Source-scoped regulator events. |
| `interactions` | Source DDInter partner rows and levels; five rows per default page, with explicit section limits preserved. |

## Sources

`--source` picks the source to ask: [who-pq](/sources/who-prequalification/), [fda-drug-recall](/sources/fda-drug-recall/), [mychem](/sources/mychem/), [chembl](/sources/chembl/), [opentargets](/sources/opentargets/), [civic](/sources/civic/), [openfda-label](/sources/openfda/), [drugsfda](/sources/drugsfda/), [ema](/sources/ema/), [ddinter](/sources/ddinter/).

## Examples

The program's help gives these examples.

```sh
biomcp get drug EMEA/H/C/000406 --source ema
```

```sh
biomcp get drug EMEA/H/C/000406 regulatory --source ema
```

```sh
biomcp get drug BLA761445 approvals --source drugsfda --product-number 001
```

```sh
biomcp get drug BLA761445 approvals --source drugsfda --section-limit 2
```

```sh
biomcp get drug 5 civic --source civic --civic-kind evidence \
  --section-limit 25 --evidence-level A --evidence-status accepted
```

```sh
biomcp get drug 5 civic --source civic --civic-kind evidence --section-limit 2 \
  --evidence-level B --evidence-level A --evidence-level A \
  --evidence-status accepted --civic-cursor 0:A2
```

```sh
biomcp get drug DDInter1 interactions --source ddinter --section-limit 1 \
  --section-offset 1 --fresh
```

```sh
biomcp get drug EMEA/H/C/005248 regulatory --source ema --format md
```

```sh
biomcp get drug 98b197ea-bca4-427e-8737-97f70a3262bc label \
  --source openfda-label --label-section warnings_and_cautions
```

```sh
biomcp get drug NDA021588 approvals --source drugsfda --document-id 64255
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.drug-approvals/8`](/reference/schemas/biomcp-drug-approvals-8/), [`biomcp.drug-card/6`](/reference/schemas/biomcp-drug-card-6/), [`biomcp.drug-civic/11`](/reference/schemas/biomcp-drug-civic-11/), [`biomcp.drug-indications/8`](/reference/schemas/biomcp-drug-indications-8/), [`biomcp.drug-interactions/8`](/reference/schemas/biomcp-drug-interactions-8/), [`biomcp.drug-label/7`](/reference/schemas/biomcp-drug-label-7/), [`biomcp.drug-recall-card/3`](/reference/schemas/biomcp-drug-recall-card-3/), [`biomcp.drug-regulatory/6`](/reference/schemas/biomcp-drug-regulatory-6/), [`biomcp.drug-targets/8`](/reference/schemas/biomcp-drug-targets-8/), [`biomcp.drug-who-card/3`](/reference/schemas/biomcp-drug-who-card-3/), [`biomcp.drug-xrefs/5`](/reference/schemas/biomcp-drug-xrefs-5/), whose page lists its fields.

## Next

The [drug guide](/user-guide/drug/) shows every drug command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
