# 1031 — study-copy-number-counts-per-gene

Status: COMPLETE.
Landed: d0f5a66150807f86914ca1d34f57f0f9e689263d

Milestone: 8

## Outcome

`get study ID` has a section that, for given genes, counts amplifications and deep deletions from the study's discrete copy-number (GISTIC) profile.

## Why

comp-bio 8.3 (bench `personas/comp-bio/runs/2026-11-16/analysis.md`); the GISTIC profile is listed but can't be queried.

## Proof

- Proof: a replayed case for brca_tcga asserting counts for one amplified gene.

## Starts from

- Starts from: an existing adapter of the same shape, and the source's own documentation and terms.

## Keeps

- Keeps: every existing command form.

## Defers

- Defers: nothing.

## Changes

- Changes: a new adapter or section, its help, schemas, cases, recordings and a release-notes line.

## Progress

- Source check (live, 2026-11-17): cBioPortal reports the counts itself. `POST /api/molecular-profiles/{id}/discrete-copy-number-counts/fetch` ("Discrete Copy Number Alteration Counts" in https://www.cbioportal.org/api/v3/api-docs) takes gene and call pairs and returns `numberOfSamples` and `numberOfSamplesWithAlterationInGene`. For brca_tcga_gistic, ERBB2 call 2 gave 133 of 1,080 and call -2 gave 1; MYC (4609) call 2 gave 229; PTEN (5728) call -2 gave 56. When no sample has the call (SRY), cBioPortal leaves `numberOfSamplesWithAlterationInGene` out; the per-sample endpoint for SRY returned no rows the same day, and for ERBB2 returned 133 amplified and 1 deleted, matching the counts. A non-discrete or unknown profile gets 404 "Molecular profile not found". So the section uses cBioPortal's counts and calculates nothing, per `sdlc/decisions/2026-10-25-entity-boundary-rulings.md`.
- Change: `get study ID cna --gene GENE` (new section word `cna`, as the comp-bio analysis proposed). It resolves the gene through cBioPortal's `/genes`, picks the study's first `COPY_NUMBER_ALTERATION` profile with datatype `DISCRETE` (or the `--molecular-profile` given), asks for calls 2 and -2, and returns the new reply `biomcp.study-copy-number/1`: `study_id`, `molecular_profile_id`, `entrez_gene_id`, `number_of_samples`, `amplified`, `deep_deleted`, `acquired`. An omitted count is 0. A profile of the wrong kind is a usage error listing the study's discrete profiles. `all` is unchanged, so existing `all --gene` replies and replays keep their shape. Help, schemas, commands reference, `sdlc/features.md` (`study-copy-number`) and one release-notes line.
- The refusal uses the existing generic argument message: a new fixed message would widen the frozen `error` reply enum, which `schema-release-check` refuses (j42196).
- Proof: `tests/cases/study/copy-number.yaml` (ERBB2: 133 amplified, 1 deep-deleted of 1,080; missing `--gene` is a usage error), `copy-number-zero.yaml` (SRY: 0 and 0) and `copy-number-profile.yaml` (mutation profile refused, valid value `brca_tcga_gistic`), replaying cBioPortal replies recorded live with `--record` in pm job j42192.
- Jobs: j42192 (build, regenerate, record), j42196 (cheap checks: study, help and mcp cases passed; failed the file line limit on the adapter rows file and the reply enum widening, both fixed).
- j42198 failed in schema generation until the checked-in error schema was restored; j42200 on its fix passed `check-cheap`, `schema-release-check` and the study, help and mcp cases, and gave the new `study-copy-number.json` schema and formatting, now committed. j42204, after merging origin/main, regenerated the new schema to list main's new WHO eEML source.
- Gate j42205 on 11377dc9a (contains origin/main as of 2026-11-17): `make --keep-going landing-gate docs-check schema-release-check` passed; the 1.0.0 baseline accepts the widened section enum, the optional query wording and the new `study-copy-number` reply.
