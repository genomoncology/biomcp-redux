# 0969 — variant-hgvs-refusal-suggests-gene-coding

Status: COMPLETE.
Landed: 4708db5999a911930799d2686bb76d82d0f88ea9

Milestone: 7

## Outcome

When `search variant --variant` gets a full HGVS name it does not take, the refusal shows the runnable `--gene ... --coding ...` form built from that name, not only a pointer to help.

## Why

dx-lab-director round 4 (2026-11-15): the refusal sent the agent to help and cost 2 extra calls (calls 73-75). Evidence: bench `personas/dx-lab-director/runs/2026-11-15-r4/` (job j41109, BioMCP 95a74e246, bench 0051).

## Proof

- Proof: a case: `search variant --variant` with the persona's HGVS name refuses with the `--gene`/`--coding` command an agent can run as is.

## Starts from

- Starts from: the variant search argument checks and refusal wording; 0956 and 0957 for the refusal-shows-the-form pattern.

## Keeps

- Keeps: what `--variant` accepts; the refusal schema.

## Defers

- Defers: nothing.

## Changes

- Changes: variant search refusal, its case.

## Progress

- 2026-11-15: the refusal for an unknown `--variant` on `search variant` now puts the runnable search first in `next`, ahead of the help step: the name's gene goes to `--gene` and its change to `--coding` (`c.`) or `--protein` (`p.`). It takes `TRANSCRIPT(GENE):change` and `GENE:change`, spaced or inline (`--variant=`). A name without a gene, such as `NM_007294.4:c.68_69del`, keeps only the help step. The suggested call goes through the ordinary parser, so a change the search would refuse is never suggested. The message, option list and refusal schema are unchanged (`crates/biomcp/src/grammar/positional.rs` `hgvs_flag`, called from `flag_fix` in `errors.rs`).
- Measured in job j41186 (debug build of 10984b552): `search variant --variant "NM_007294.4(BRCA1):c.68_69del"` exits 2 with `next[0]` = `search variant --coding c.68_69del --gene BRCA1`; that command, run live, returns 1 hit, `clinvar:17662`, as call 75 did. `--gene BRAF --protein p.Val600Glu` returns rs113488022 / clinvar:13961 live. Schema and docs regeneration changed nothing; `check-cheap` passed.
- Case: `tests/cases/variant/search-hgvs-name-refusal.yaml` (local-only; no source requests).
- 2026-11-15, review fix: a bare prefix that is a sequence accession is no longer taken as the gene. `ENST00000357654:c.68_69del`, `ENSP00000350283:p.Val600Glu` and `U14680:c.68_69del` now keep only the help step (an Ensembl ID, or a GenBank accession of one letter and five digits or two letters and six or eight digits; gene symbols such as LINC00473 still pass). `--gene=` given inline also suppresses the suggestion, as `--gene` did. The case gained steps for `BRCA1:c.`, `ENST…:c.`, `ENSP…:p.` and `U14680:c.`.
- 2026-11-15, second review fix: GenBank protein accessions (three letters and five or seven digits, such as `AAA03565:p.Val600Glu`) and an accession in the brackets (`NC_000017.11(ENST00000357654):c.68_69del`) also keep only the help step; the accession check now runs on the gene place whether or not it is bracketed. The case gained a step for each.
