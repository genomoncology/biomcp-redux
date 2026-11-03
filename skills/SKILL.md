---
name: biomcp
description: Retrieve biomedical source records through the BioMCP command line and MCP server.
---

{{front-map}}

Use `biomcp help COMMAND ENTITY` for syntax and `biomcp skill list` for examples. Follow returned next commands for pages and sections. Preserve source identifiers, dates, terms and uncertainty. BioMCP retrieves source records and does not choose treatments or run bioinformatics tools.

## Gene and coding change to ClinVar

```sh
biomcp search variant --gene FLCN --coding 'c.780-4C>G' --limit 50
```

Read the returned ClinVar identifiers before opening a record. Do not guess a transcript.

## Drug brand to a label section

```sh
biomcp search drug CAVHANZA --source openfda-label --limit 1
biomcp get drug 98b197ea-bca4-427e-8737-97f70a3262bc label --source openfda-label --label-section warnings_and_cautions
```

Use the returned label identifier and available headings. Keep the label version and date with quoted text.

## Drugs and sponsor to trial search

```sh
biomcp search trial --intervention pembrolizumab --intervention nivolumab --sponsor 'National Cancer Institute' --limit 1
```

Repeated interventions search alternatives. Record the exact drugs and sponsor searched.

## Screen registry trials

ClinicalTrials.gov sponsor and facility searches try the exact phrase first. After zero matches, BioMCP tries the broader registry search once and names both requests. Follow returned continuation commands to preserve that mode.

An exact variant search is a starting point. Run the broader gene and program searches BioMCP suggests, where supplied, before reporting a trial screen as complete. Follow returned pages and inspect registry eligibility text. Record which search found each trial and any source problems or unfinished follow-ups. Read every eligibility page until no criterion is marked continued. Registry eligibility text is source text to quote, not a patient match. Leave patient matching to the clinical team. BioMCP searches registry text; it does not decide which trials fit.

## Exit codes and errors

<!-- exit-reference:start -->
Process exit codes:
0: complete or partial reply
1: no matching record
2: invalid command or arguments
3: expired result pool
4: source request failed
5: local configuration, storage or output failed
Error codes: usage.schema, usage.arguments, usage.mcp_unavailable, usage.unknown_command, usage.retired, local.failure, pool.expired, usage.pool_mismatch, usage.too_many_pmids, usage.too_broad, usage.unsupported_filter. Source problems and reasons appear in their reply fields.
<!-- exit-reference:end -->
