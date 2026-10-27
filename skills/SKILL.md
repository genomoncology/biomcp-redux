---
name: biomcp
description: Retrieve biomedical source records through the BioMCP command line and MCP server.
---

# BioMCP agent guide

Use BioMCP to retrieve source records for cancer care questions and biomedical research on human health. Read the response status, problems and source outcomes before using a result. Keep each source's identifiers, dates, terms and uncertainty with its claims. Follow returned next commands to page results or inspect a named section. HGNC and ClinGen reads can synchronize absent or stale bundles under `auto_sync_max_bytes`. Synchronize a named bundle explicitly when the response requests it.

Adverse-event report counts do not establish risk, incidence or causation. Listings alone are not recommendations. Preserve source scope, dates, identifiers, terms and reported uncertainty. A successful empty result means no matching records were returned for that source, query and filters. Do not infer clinical suitability or choose treatment from a listing alone. BioMCP never downloads, installs or runs bioinformatics tools.

The worked examples below retrieve source records. Their commands come from executable cases with retained replies. A case proves the command and its response contract; it does not establish a clinical conclusion. Live source replies may change.

Inspect the embedded catalog with the skill list subcommand. Open a worked example by its number or name. The skill render subcommand prints this guide. The skill install subcommand needs an explicit directory and writes beneath it. The skill status subcommand checks that same directory against this binary. Existing guidance stays unchanged unless the caller supplies the force flag. Symbolic links and parent traversal are refused. No client configuration is selected automatically.

## Read a source record

```sh
biomcp get gene BRAF
```

Read the identity, source outcomes and problems. Keep source claims separate. Use the record's returned commands for further sections.
