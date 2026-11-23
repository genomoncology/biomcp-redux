# 1165 — gtr-first-read-within-size-limit

Status: COMPLETE.
Landed: 0508d7fcb4a79dc8217e65209cd36902d56598d7

Milestone: 7

## Outcome

`get diagnostic` and `search diagnostic` with `--source gtr` work on a cold cache: the first read of NCBI's GTR export fits BioMCP's limits, or the reply says why it cannot and what works.

## Why

Found 2026-11-23 while verifying the docs workflows "Registered genetic tests" and "Molecular test menus" (hosting ticket 0116). The GTR requests returned `syncing` because the first-read fetch of NCBI's export (https://ftp.ncbi.nlm.nih.gov/pub/GTR/data/test_version.gz) exceeded BioMCP's size limit. The export holds the requested Guardant360 accession. An agent sees `syncing` and waits for a sync that never finishes. A pm idea already records the same class of fault for gnomAD v4.1.

## Proof

- Proof: on an empty cache, a GTR get for the Guardant360 accession either returns the record or returns a refusal naming the size limit and a working next step (for example `biomcp sync gtr` on the CLI), never `syncing` with nothing in progress.
- No size ceiling is raised to fit; stream or index the export instead, or refuse honestly.

## Changes

- Changes: The GTR adapter's first-read path and its replay cases.
