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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp help`](/reference/commands/help/), [`biomcp help network proxy`](/reference/commands/help-network-proxy/) and [`biomcp help network ca_bundle`](/reference/commands/help-network-ca_bundle/) reply with `schema` set to `biomcp.help/2`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp help get gene
```

`biomcp schema biomcp.help/2` prints the full JSON Schema.

## Example reply

`biomcp help get gene` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

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
          … 1 more
        ],
        "sections": [
          {
            "word": "protein",
            "description": "Reviewed human proteins."
          },
          … 7 more
        ],
        "section_rule": "Requested section words, preserving caller order and…",
        "examples": [["get", "gene", "KRAS"], ["get", "gene", "BRAF"]]
      }
    ]
  }
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Help](#help). `query` repeats the request as BioMCP read it.

### Help

Generated command reference from the shipped query schemas and cases.

| Field | Type | Meaning |
| --- | --- | --- |
| `commands` | list of [HelpCommand](#helpcommand) | Commands available at this help path. |

### HelpArgument

One schema argument, retaining its bounds and position.

| Field | Type | Meaning |
| --- | --- | --- |
| `choices` | list of string | Allowed scalar values. |
| `description` | string | Field documentation. |
| `maximum` | integer or null | Upper numeric or array length bound. Optional. |
| `minimum` | integer or null | Lower numeric or array length bound. Optional. |
| `name` | string | Positional name or long flag. |
| `position` | integer or null | Zero based positional index. Optional. |
| `required` | boolean | Whether the query requires this argument. |
| `value_type` | string | Schema scalar or array type. |

### HelpCommand

One command and its generated arguments.

| Field | Type | Meaning |
| --- | --- | --- |
| `arguments` | list of [HelpArgument](#helpargument) | Positional arguments and flags. |
| `command` | list of string | Command words after the executable name. |
| `description` | string | Query type documentation. |
| `examples` | list of list of string | Verbatim case run steps, without replay configuration. |
| `section_rule` | string or null | How to disambiguate [section words](/concepts/progressive-disclosure/) from inputs. Optional. |
| `sections` | list of [HelpWord](#helpword) | Accepted section words and their documentation. |

### HelpWord

A documented closed vocabulary word.

| Field | Type | Meaning |
| --- | --- | --- |
| `description` | string | Type variant documentation. |
| `word` | string | Accepted spelling. |

## Next

[`biomcp help`](/reference/commands/help/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
