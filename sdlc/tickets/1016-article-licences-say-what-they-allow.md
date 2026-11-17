# 1016 — article-licences-say-what-they-allow

Status: OPEN.

Milestone: 7

## Outcome

Article licence codes come with a plain meaning (for example `tdm`: text and data mining, not redistribution), and a failed `pdf` section offers `--section fulltext`.

## Why

msl treated a PMC copy as shareable (bench `personas/msl/runs/2026-11-16/analysis.md` 9.1, 9.2).

## Proof

- Proof: replayed cases asserting the meanings and the next step; fail on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.

## Progress

- Cause: `get article` sections `fulltext`, `supplements` and `pdf` carried only a bare `license` code (`tdm`, `other`), and a `pdf` section that failed at Semantic Scholar (rate-limited in the msl run, calls.md L1918-1941) offered no next step, though the paper's PMC copy and its license were one `fulltext` call away.
- Change: `FullText`, `Supplements` and `PdfLink` (so also the `article pdf` save receipt) gain the optional `license_terms`, filled from `biomcp_types::license_terms` in `crates/biomcp-types/src/response/article/terms.rs`. When the `pdf` section fails for a reason other than `not-open`, `get article` adds one `get article IDS fulltext` step for the papers with a PMCID or Europe PMC ID whose `fulltext` the reply does not already hold. The `fulltext` and `pdf` section help names the terms. Release notes gain one Articles line.
- Sources of each meaning, read 2026-11-16 (no wording from memory):
  - `cc_by`, `cc_by_sa`, `cc_by_nd`, `cc_by_nc`, `cc_by_nc_sa`, `cc_by_nc_nd`: each license's own Creative Commons deed, https://creativecommons.org/licenses/{by,by-sa,by-nd,by-nc,by-nc-sa,by-nc-nd}/4.0/ ("Share — copy and redistribute the material in any medium or format for any purpose, even commercially"; "Adapt — remix, transform, and build upon the material"; "Attribution — You must give appropriate credit, provide a link to the license, and indicate if changes were made"; "NonCommercial — You may not use the material for commercial purposes"; "ShareAlike — ... you must distribute your contributions under the same license as the original"; "NoDerivatives — If you remix, transform, or build upon the material, you may not distribute the modified material").
  - `cc0`: https://creativecommons.org/publicdomain/zero/1.0/ ("You can copy, modify, distribute and perform the work, even for commercial purposes, all without asking permission").
  - `public_domain`: https://creativecommons.org/publicdomain/mark/1.0/ (same sentence) and PMC's Copyright Notice, https://pmc.ncbi.nlm.nih.gov/about/copyright/ ("open access and public domain articles may still contain photographs or illustrations copyrighted by other commercial organizations or individuals").
  - `tdm`: PMC Article Datasets, https://pmc.ncbi.nlm.nih.gov/tools/textmining/, Author Manuscript Dataset ("Default license: 'This file is available for text mining. It may also be used consistent with the principles of fair use under the copyright law.'"), and the Copyright Notice ("Transmission, reproduction, or reuse of protected material, beyond the license terms or those allowed by the fair use principles of the copyright laws, requires the written permission of the copyright owners").
  - `other`: the same Datasets page ("Other: no machine-readable Creative Commons license, no license tagged, or a custom license") and the Copyright Notice ("In the absence of a copyright statement, users should assume that standard copyright protection applies, unless the article contains an explicit statement to the contrary"; "Many articles in PMC are protected by U.S. and/or foreign copyright laws, even though PMC provides free access to it").
- Live check, job j42064 on 906e97213: `get article 28944238 fulltext supplements pdf` gave `cc_by` with its terms in all three sections; `get article 34161704 pdf fulltext` (msl's Awad paper) gave `pdf` `other` and `fulltext` `tdm`, each with its terms. Its PubMed EFetch for 28944238 is the replay's PubMed reply.
- Proof: `tests/cases/article/pdf/license-terms.yaml` on the synthetic replay `tests/replies/article/pdf/synthetic-terms/` (README there): a rate-limited `pdf` section offers `get article 28944238 fulltext`, and the `fulltext` and `supplements` sections say what `tdm` allows. The PMC bucket record is the existing synthetic one with `license_code` TDM, so no text under that license is committed. `tests/cases/article/pdf/synthetic.yaml` also asserts the `cc_by` terms on the save receipt.
- Job j42069 on 91291500d: schemas and docs regenerated (article-cards, article-get, article-pdf-result and request schemas gain only the optional field or changed descriptions), `cargo fmt` applied. The new case passes, and with base 253c3b4f7's crates it fails at step 1 (no `fulltext` step; the only `next` is a fresh `pdf` retry). Fixed after it: the replay README gained `Synthetic: semantic-scholar` for the structural ratchet, and two article replies grew past their caps by the new field, so `article/preprint/reader.yaml` (4,653 bytes measured) and `article/supplements/file-list.yaml` (3,876) take measured plus 15%: 5,351 and 4,457. `make rights` passes with 58 captures; mcp-parity and help cases pass.
- Gate j42074 on 705d3c7d8 (contains origin/main b8e43c4f1): `make --keep-going landing-gate docs-check schema-release-check` passed.
- Review fixes: the `fulltext` step named the paper by the ID the PDF lookup used, so a preprint with no PMID got `get article <DOI> fulltext`; it now names the paper by PMID, else Europe PMC ID, as `article/next.rs` does. `license_terms` is now given only when the bucket or Europe PMC code names its license exactly (or names no known open license, `other`, which grants nothing); a code that `pmc_oa::license` reads from its words alone, such as a URL containing "NC", gets no terms (`Document::license_stated`). Semantic Scholar's mapping is exact already. The case gains a preprint step, `get article PPR1295425 pdf` with an authored 429 for its DOI lookup (Europe PMC core record copied from the preprint replay; job j42094 recorded the same request live), which expects `get article PPR1295425 fulltext`. The file-list size note now states only the 1016 measurement.
- Job j42097 on dc259c61b: `check-cheap` (after its `cargo fmt` of `pmc_oa/mod.rs`, committed), `make rights` (58 captures), all 58 article cases and mcp-parity pass. With the earlier head 3f7e116eb's crates the case fails at the preprint step: the step was `get article 10.64898/2026.08.05.26359768 ...`, the DOI.
- Gate j42102 on b20a93157 (contains origin/main 94140f331): `make --keep-going landing-gate docs-check schema-release-check` passed.
