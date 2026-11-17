# 1036 — protein-all-replay-records-every-section

Status: COMPLETE.
Landed: a56ddaa679f56bbfbdfb74c0685ab04a8c488061

Milestone: 7

## Outcome

The replayed `get protein P00533 all` step in `tests/cases/protein/native-sections.yaml` has a recorded reply for every source the card reads, and it asserts each section's state, so a section silently coming back "unavailable" fails the case. Any other replayed `all` case with the same gap is found and fixed.

## Why

1029 (2026-11-17) found that step's STRING section was unrecorded, and fixed it; its developer then noticed the domains section is still "unavailable" because no InterPro reply was ever recorded. The step never checked overall status, so the gap passed unnoticed.

## Proof

- Proof: the step asserts every section's state is data (or a stated, source-given empty), and removing any one recording makes it fail.

## Starts from

- Starts from: `tests/cases/protein/native-sections.yaml:65-80` and the replay recordings.

## Keeps

- Keeps: the card's sections and reply shape.

## Defers

- Defers: nothing.

## Changes

- Changes: recordings, the case, and any similar `all` cases found.

## Progress

- j42248 (d42922dfe): recorded `get protein P00533 domains --section-limit 2 --fresh` live. InterPro's integrated-entry reply for P00533 (exact request `page_size=25`, body SHA256 f07c5238…525a46) lists 15 entries led by IPR000494 (Receptor L-domain); its UniProt request was byte-for-byte the one already recorded, so only the InterPro file was added. The job's survey of other replayed `all` steps passed `--case-cache` without `--offline` and every command stopped locally, so that survey was rerun in the next job.
- The `get protein P00533 all` step now asserts `/status` complete and the state of all eight sections: function, sequence, isoforms, structures, features, domains and interactions are data; complexes is Complex Portal's stated empty collection. It also asserts the domains source, total (15) and first entry.
- j42250 (67d2881e8): replayed `all` now reads `complete` with every section data, except complexes, which is Complex Portal's stated empty collection. With one recording removed at a time, the bare command turns `partial` (InterPro, STRING or Complex Portal missing: that section becomes `unavailable`, `replay-miss`) or `failed` (UniProt missing), and the case fails at the `all` step each time (domains id, interactions first partner, complexes state "empty" vs "unavailable"). `make check-cheap` and the 30 protein cases passed.
- Survey of every other replayed `all` step (same job; the CLI outside the runner's clean environment saw an AlphaGenome key, so its AlphaGenome reply-miss is the survey's, not the case's): cell-line `native-all`, `pathway-sections/all`, `trial/nci-get` and `study/card-section-totals` already assert either `complete` status or every section; the six `variant/id-forms` steps assert all eleven sections. `article/preprint/all` asserts `partial` status, which cannot tell a new unavailable section from the disabled Semantic Scholar one, and left fulltext, indexing and pdf unasserted; it now asserts every section's state (fulltext data; indexing, annotations, citations, supplements not-applicable; pdf and tldr skipped as disabled). `gene/retired` is a retired route with no sections.
- Gate j42254 on a2745ef88 (`make --keep-going landing-gate docs-check schema-release-check`): passed, including all 1,031 cases.
- Merged origin/main cd60e1927 (1030 adds the `chembl` protein section and its P00533 recordings). The only conflict was this case: kept both sets of checks, so the `all` step now asserts nine section states (chembl data, CHEMBL203 with 5,966 assays and 58,847 activities, from 1030), and joined both provenance notes.
- Gate j42261 on 3edd4dc6f (`make --keep-going landing-gate docs-check schema-release-check`): passed, including all 1,032 cases.
