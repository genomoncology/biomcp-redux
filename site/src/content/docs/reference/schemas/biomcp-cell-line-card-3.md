---
title: Reply of biomcp get cell-line (biomcp.cell_line_card/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get cell-line: a
  real example and the fields of schema biomcp.cell_line_card/3."
goal: "Parse the biomcp.cell_line_card/3 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.cell_line_card/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get cell-line`](/reference/commands/get-cell-line/) replies with `schema` set to `biomcp.cell_line_card/3`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get cell-line CVCL_0132
```

`biomcp schema biomcp.cell_line_card/3` prints the full JSON Schema.

## Example reply

`biomcp get cell-line CVCL_0132` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.cell_line_card/3",
  "status": "complete",
  "query": {"id": "CVCL_0132", "section_offset": 0},
  "result": {
    "input": "CVCL_0132",
    "status": "complete",
    "header": {
      "accession": "CVCL_0132",
      "name": "A-375",
      "category": "Cancer cell line",
      "species": {
        "items": [
          {…}
        ],
        "total": 1
      },
      "diseases": {
        "items": [
          {…}
        ],
        "total": 1
      }
    },
    "candidates": [],
    "dates": {
      "created": "2012-04-04",
      "updated": "2026-06-25",
      "version": 51
    },
    "age": "54Y",
    "sex": "Female",
    "warnings": {"items": [], "total": 0},
    "url": "https://www.cellosaurus.org/CVCL_0132",
    "sections": {},
    "more": ["names", "variants", "xrefs", "chembl"],
    "release": {"version": "57.0", "updated": "2026-10-22"},
    "fetched": 1793140297
  },
  "sources": [
    {
      "source": "cellosaurus",
      "use": "open",
      "terms": "Cellosaurus CC BY 4.0; cite Bairoch (2018)",
      "terms_url": "https://www.cellosaurus.org/description.html",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "section",
      "args": ["get", "cell-line", "CVCL_0132", "names"]
    },
    … 3 more
  ]
}
```

## Fields

`result` holds [CellLineCard](#celllinecard). `query` holds [CellLineGet](/reference/schemas/types/#celllineget), a shared type. `query` repeats the request as BioMCP read it.

### CellLineCard

One requested cell identity with explicit ambiguity and requested section states.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `age` | [Text](/reference/schemas/types/#text) or null | no | Native age expression when supplied. |
| `candidates` | list of [CellLineHeader](/reference/schemas/types/#celllineheader) | yes | Candidate identities without first-hit selection. |
| `dates` | [CellLineEntryDates](#celllineentrydates) or null | no | Selected entry dates and version, separate from release metadata. |
| `fetched` | integer or null | no | Original retrieval seconds, distinct from entry and release dates. |
| `header` | [CellLineHeader](/reference/schemas/types/#celllineheader) or null | no | Selected verified identity only. |
| `input` | [CellLineInput](/reference/schemas/types/#celllineinput) | yes | Exact original caller identity. |
| `more` | list of [CellLineSection](/reference/schemas/types/#celllinesection) | yes | Supported disclosures not requested. |
| `release` | [CellLineRelease](/reference/schemas/types/#celllinerelease) or null | no | Independently fetched publisher release. |
| `sections` | [CellLineSections](#celllinesections) | yes | Only explicitly requested sections carry a state. |
| `sex` | [Text](/reference/schemas/types/#text) or null | no | Native sex label when supplied. |
| `status` | [Status](/reference/schemas/types/#status) | yes | Resolved, absent, ambiguous or failed input outcome. |
| `url` | [Text](/reference/schemas/types/#text) or null | no | Publisher URL for the selected identity. |
| `warnings` | [Capped](/reference/schemas/types/#capped) of [CellLineWarning](/reference/schemas/types/#celllinewarning) or null | no | Selected card warnings, up to three whole rows; not a fetch state. |

### CellLineEntryDates

Entry dates and entry version are independent of dataset release and retrieval time.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `created` | [PartialDate](/reference/schemas/types/#partialdate) or null | no | Source entry creation date. |
| `updated` | [PartialDate](/reference/schemas/types/#partialdate) or null | no | Source entry update date. |
| `version` | integer or null | no | Source entry version, never a dataset release. |

### CellLineSections

Requested sections carry explicit states; absent members were not requested.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `chembl` | [Section](/reference/schemas/types/#section) of [CellLineChembl](/reference/schemas/types/#celllinechembl) of [Capped](/reference/schemas/types/#capped) of [CellLineChemblRecord](/reference/schemas/types/#celllinechemblrecord) or null | no | Requested chembl section state. |
| `identity` | [Section](/reference/schemas/types/#section) of [CellLineIdentityPage](/reference/schemas/types/#celllineidentitypage) or null | no | Requested identity section state. |
| `names` | [Section](/reference/schemas/types/#section) of [CellLineNames](/reference/schemas/types/#celllinenames) of [Capped](/reference/schemas/types/#capped) of [CellLineName](/reference/schemas/types/#celllinename) or null | no | Requested names section state. |
| `variants` | [Section](/reference/schemas/types/#section) of [CellLineVariants](/reference/schemas/types/#celllinevariants) of [Capped](/reference/schemas/types/#capped) of [CellLineVariation](/reference/schemas/types/#celllinevariation) or null | no | Requested variants section state. |
| `xrefs` | [Section](/reference/schemas/types/#section) of [CellLineXrefs](/reference/schemas/types/#celllinexrefs) or null | no | Requested xrefs section state. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get cell-line`](/reference/commands/get-cell-line/) gives the command's arguments.
