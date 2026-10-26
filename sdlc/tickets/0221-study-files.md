# 0221 — Add the study files section with per-study license notices

Status: OPEN.

Milestone: 4

Depends on: 0210
Depends on: 0238

## Outcome

`get study brca_tcga_pan_can_atlas_2018 files` reads the exact DataHub GitHub contents directory for that study, as the j14214 capture did, and returns five native file entries per page with path, name, publisher link, Git blob SHA and the GitHub `size` labeled as repository blob size, paged with `--section-limit` and `--section-offset`. When the directory lists a LICENSE file, the files page and the card carry its publisher link as a notice. A study with no DataHub directory returns an absent files section, and the study card still answers. The card gains its `files` section step, and `all` includes the files section's first page.

- Why: the 2026-10-26 study rulings accept the `files` section and say per-study LICENSE links appear as notices, and an agent needs safe source file references instead of a whole archive download.
- Starts from: study.md's file reference row, bundles, files, rights and cadence section and measured disclosure budget, with the 2026-10-26 study rulings, which win over the design text; the j14214 DataHub contents captures and the j14223 LICENSE captures; 0207's absent-directory witness; 0238's card and section dispatch; 0210's types.
- Rules: the source-wide `restricted` use for cBioPortal stays for every study, including one whose LICENSE states ODbL. A LICENSE link is a notice only, and per-study use labels wait until a need appears. The portal software's AGPL license is never labeled a study data license. GitHub `size` is never labeled data content size, and an LFS pointer is never read as the file's content. No data type is inferred from a file name. The section never downloads an archive or a data file and never runs a tool. DataHub catalog membership differs from the live catalog, so an absent directory is an absent section, never an absent study. The DataHub request runs under the existing `cbioportal` source declaration with no new source name, and a disabled source or group skips it before cache or network.
- Proof: outside-in cases first, watched to fail. Before section code, one bounded keyless pm job captures the BRCA and MSK directories once, with zero retries, and records the replies. File cases assert BRCA's 58 entries and MSK's 18 entries across a complete page walk with no gap or duplicate; a LICENSE case asserts the BRCA notice links its native LICENSE and MSK's notice links its native LICENSE, while both keep the `restricted` use; an LFS case asserts the blob size label; the absent-directory case from 0207 asserts an absent section beside a working card. Recorded 429, 500, timeout and broken-body cases assert a failed outcome with source and terms, never an empty page. The job measures the section in CLI and MCP output, starting from the design's 1,721-byte projection for a five-entry page, and each case gets a ceiling at measured size plus 15 percent, recorded in study.md. Real replies replay from outside the repo through `--cases DIR`, and synthetic replies prove failure mechanics. No LICENSE text or other licensed content enters the repo. Then make landing-gate runs through pm job.
