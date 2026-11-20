# 1085 — refusals-agents-hit-most-say-what-to-change

Status: OPEN.

Milestone: 7

## Outcome

The four refusals agents met most often either go away or say exactly what to change.

## Why

Twelve traced agent runs on 2026-11-20, on a Linux build of main 5db190f8a, made 338 runs of `biomcp`; 50 were refused or failed. Reproduced by hand on the same build:

- `biomcp get drug mobocertinib markets --source drugsfda` (11 refusals). The message is clear, and agents still add `--source` because every other drug section takes it.
- `biomcp get drug NDA022363 label-history --source drugsfda --label-section warnings_and_cautions --find myasthenia --offset 10` exits 2 with `option: section`, `valid_values: ["section","--find","--label-section"]` and "error.option is not accepted with the other options as given". The cause is that `--offset` needs `--every-version`; the message names neither.
- `biomcp get drug NDA020702 label-history --source drugsfda --label-section clinical_pharmacology --find fold` gives the same message. The cause is a section `label-history` does not read; the message does not say so or list the sections it reads.
- `biomcp get drug NDA203214 approvals --source drugsfda --fields=submission_number,submission_type,submission_class,action_date,doc_type` (16 refusals). The agent used openFDA's own field names. The refusal lists the valid names, and its `next` keeps only `submission_number`.

## Proof

- Proof: `markets --source drugsfda` returns the United States market's rows, and `--source ema` the European Union's, each the same as that market's part of the plain call; a source no market reads is refused with the list. It fails on the base commit.
- Proof: the two `label-history` calls above are refused with a message naming the real cause: "--offset needs --every-version", and the list of sections `label-history` reads. Each fails on the base commit.
- Proof: the `--fields` refusal's `next` maps each openFDA Drugs@FDA field name that has a clear counterpart to BioMCP's name (for example `submission_class_code_description`), so the suggested call keeps the columns the caller meant; names with no counterpart are listed as unplaced.

## Starts from

- Starts from: the commands above; tickets 1071 and 1073 for how refusals build `next`; `biomcp help get drug`, which says markets "takes a name and no source".
- PM decision, 2026-11-20: `--fields` still refuses an unknown name and never drops one silently.

## Keeps

- Keeps: a plain `get drug NAME markets` reads every market as now.

## Defers

- Defers: nothing.

## Changes

- Changes: the drug markets and label-history commands, the `--fields` refusal, help text and cases.
