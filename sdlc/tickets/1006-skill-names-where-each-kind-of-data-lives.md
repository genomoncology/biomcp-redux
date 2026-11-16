# 1006 — skill-names-where-each-kind-of-data-lives

Status: OPEN.

Milestone: 7

## Outcome

`skills/SKILL.md` and its use cases tell an agent, in a line each, where data it is likely to assume missing lives: FDA 510(k), PMA and device recalls under `search diagnostic --source fda-510k|fda-pma|fda-device-recall`; SEER incidence and survival under `search disease --source seer` then `get disease SITE survival|incidence --source seer`; an author's papers with `search article --author "Surname AB"` and `--sort date|citations`; a citing paper's own paragraph with `article citation-evidence … --fulltext`; CPIC gene-drug pairs with levels and PMIDs with `search pgx --gene G --source cpic`; a label's other sections with `--label-section`. The skill stays within its size ratchet.

## Why

Bench analyses of six persona runs on 2026-11-16 found 64 of 99 missed or wrong facts were reachable in BioMCP. life-sciences-ci's report said BioMCP has no device or survival data, and fetched both by curl (bench `personas/life-sciences-ci/runs/2026-11-16/analysis.md`, calls.md 174, 375, 595, 639). msl missed an author's trail and citing paragraphs (bench `personas/msl/runs/2026-11-16/analysis.md` 2.3, 3.2); pt-committee missed CPIC pair levels (bench `personas/pt-committee/runs/2026-11-16/analysis.md` 2.1).

## Proof

- Proof: a skill check (the repo's skill tests) that names each route above; `make docs-check` passes.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.
