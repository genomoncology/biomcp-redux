# 1083 — label-history-help-uses-a-neutral-example

Status: COMPLETE.
Landed: eca20c9de50b29311e7705ce7934c6a841010373

Milestone: 7

## Outcome

The help, the MCP tool descriptions and the docs for `label-history` show one example, on osimertinib's application: `get drug NDA208065 label-history --source drugsfda --label-section warnings_and_cautions --find vasculitis`. No help or doc text names the five drugs or phrases the ticket 1066 tests use.

## Why

`help get drug` gives `--find Raynaud` as its example. That phrase and the five test drugs (Olumiant, Xeljanz, Rinvoq, Aimovig, Livalo) are also used to measure agents on this command, so text an agent reads must not carry them. Checked 2026-11-20 on main a8ee208b8: the osimertinib call answers "SUPPL 19, approved 2020-12-11; SUPPL 18, approved 2020-12-09, the version before it, lacks it" in 4.7 seconds and 2,623 bytes.

## Proof

- Proof: a search of help output, tool descriptions, `docs/` and `README.md` for Raynaud, Aimovig, erenumab, Olumiant, baricitinib, Xeljanz, tofacitinib, Rinvoq, upadacitinib, Livalo and pitavastatin finds nothing about label history; the cases and fixtures under `tests/` keep them.

## Starts from

- Starts from: the `--find` option's example and the "Label section history" part of docs/cautions.md.

## Keeps

- Keeps: behaviour and tests unchanged.

## Defers

- Defers: nothing.

## Changes

- Changes: help text, generated command reference and schemas, docs.
