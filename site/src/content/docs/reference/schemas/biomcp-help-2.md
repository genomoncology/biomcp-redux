---
title: Reply of biomcp help (biomcp.help/2)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp help: a real example
  and the fields of schema biomcp.help/2."
goal: "Parse the biomcp.help/2 reply: see a real example and what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.help/2
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp help`](/reference/commands/help/), [`biomcp help network proxy`](/reference/commands/help-network-proxy/) and [`biomcp help network ca_bundle`](/reference/commands/help-network-ca_bundle/) reply with `schema` set to `biomcp.help/2`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp help get gene
```

`biomcp schema biomcp.help/2` prints the full JSON Schema.

## Example reply

`biomcp help get gene` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

```json
{
  "schema": "biomcp.help/2",
  "status": "complete",
  "query": ["get", "gene"],
  "result": {
    "commands": [
      {
        "command": ["get", "gene"],
        "description": "Get one to twenty human gene inputs in their original…",
        "arguments": [
          {
            "name": "input",
            "description": "Gene symbols or checked identifiers, in input ord…",
            "value_type": "array",
            "required": true,
            "position": 0,
            "minimum": 1,
            "maximum": 20,
            "choices": []
          },
          {
            "name": "--section",
            "description": "Requested section words, preserving caller order…",
            "value_type": "array",
            "required": false,
            "choices": [
              "protein", "civic", "clingen", "constraint", "expression",
              "pathways", "oncokb", "all"
            ]
          }
        ],
        "sections": [
          {
            "word": "protein",
            "description": "Reviewed human proteins."
          },
          {
            "word": "civic",
            "description": "CIViC gene curation."
          },
          … 6 more
        ],
        "section_rule": "Requested section words, preserving caller order and…",
        "examples": [["get", "gene", "KRAS"], ["get", "gene", "BRAF"]]
      }
    ]
  }
}
```

## Fields

`result` holds [Help](#help). `query` repeats the request as BioMCP read it.

### Help

Generated command reference from the shipped query schemas and cases.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `commands` | list of [HelpCommand](#helpcommand) | yes | Commands available at this help path. |

### HelpArgument

One schema argument, retaining its bounds and position.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `choices` | list of string | yes | Allowed scalar values. |
| `description` | string | yes | Field documentation. |
| `maximum` | integer or null | no | Upper numeric or array length bound. |
| `minimum` | integer or null | no | Lower numeric or array length bound. |
| `name` | string | yes | Positional name or long flag. |
| `position` | integer or null | no | Zero based positional index. |
| `required` | boolean | yes | Whether the query requires this argument. |
| `value_type` | string | yes | Schema scalar or array type. |

### HelpCommand

One command and its generated arguments.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `arguments` | list of [HelpArgument](#helpargument) | yes | Positional arguments and flags. |
| `command` | list of string | yes | Command words after the executable name. |
| `description` | string | yes | Query type documentation. |
| `examples` | list of list of string | yes | Verbatim case run steps, without replay configuration. |
| `section_rule` | string or null | no | How to disambiguate [section words](/concepts/progressive-disclosure/) from inputs. |
| `sections` | list of [HelpWord](#helpword) | yes | Accepted section words and their documentation. |

### HelpWord

A documented closed vocabulary word.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `description` | string | yes | Type variant documentation. |
| `word` | string | yes | Accepted spelling. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp help`](/reference/commands/help/) gives the command's arguments.
