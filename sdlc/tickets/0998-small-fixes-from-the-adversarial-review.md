# 0998 — small-fixes-from-the-adversarial-review

Status: COMPLETE.
Landed: 3b57d344ace6057047a75209e5014e49e3b4df79

Milestone: 7

## Outcome

The small accepted findings of `sdlc/records/2026-11-14-adversarial-review.md` that were still open on main 3f14da1a6 are fixed, and each choice the code makes on purpose has its reason written beside it.

## Why

PM triage, 2026-11-16, against main 3f14da1a6:

- **A4:** `drug/response.rs:340`'s `total` says "exact source records" but counts examined candidates.
- **A6:** `trial.rs:223` overwrites `outcome.dropped` with `=`. It should combine the flags.
- **C2:** plain HTTP downloads a required PDF or document, then refuses the save (`command/mcp_policy.rs:17-28`, `save/reply.rs:73`). It should refuse before any publisher request.
- **C4:** the 429 comment at `request.rs:374` says the body is never read, but a keyed body is read.
- **D6:** generated help says "native" and "literal", but the details list says "source" and "exact" (`docs/reference/commands.md:958` vs `:1094`).
- **E3:** ClinVar gene counts have a fixed 15 s limit with no `command_budget` (`clinvar/mod.rs:40`), unlike VAERS and Monarch.
- **F3:** `winnow` in `Cargo.toml:46` is used by no crate.
- **F4:** `gene::in_region` (`query/gene.rs:152`) has no caller.
- **F7:** the openFDA terms URL is written out in four SPECs.
- **Reasons to write beside the code:**
  - A2: FDA name lookups read every page on purpose, because the ranking needs the whole set.
  - B4: the local cache upgrades its own tables in place.
  - F9: PMA and 510(k) dates stay raw text while recall dates are parsed.

## Proof

- Proof: C2 is a routine case showing plain-HTTP `article pdf` refused before any publisher request. It fails on the base commit.
- Proof: A6 is a unit test where an earlier `dropped` flag survives. It fails on the base commit.
- Proof: E3 gets a declared budget, like VAERS and Monarch, and a test reads it.
- Proof: the gate passes. Changed schema descriptions follow the 1.0 rules. The release notes get a line for anything users see (C2, D6, E3).

## Starts from

- Starts from: the review's finding text for each ID.

## Keeps

- Keeps: no behaviour change beyond C2's earlier refusal and E3's longer limit.

## Defers

- Defers: F10, the comments that cite ticket numbers. That is a separate ruling.

## Changes

- Changes: the files named above, the generated docs and schemas, the notes.

## Progress

- Base proofs on 82360be61 plus the new tests only (branch `proof/0998-base`, 8900f6556): the A6 unit test fails, `assertion failed: outcome.dropped` (j41710). The C2 case fails: plain-HTTP `article pdf ... --save` asks Semantic Scholar first and, offline, ends `local.failure` instead of a refusal (j41716). The E3 case also fails there: with `command_timeout_seconds` 1, `get gene BRCA1 clinvar` ends deadline-exceeded (j41716).
- A4: `DrugNameResolution.total` now says "Distinct exact-name source records found among the rows read, including the selected record; MyChem reads at most 250 rows." A description change only; drug schemas regenerated.
- A6: `trial.rs` keeps an earlier `dropped` flag (`|=`); `trial/page_tests.rs` proves it, listed in the inline-test ratchet.
- C2: on plain HTTP, `mcp_policy::refusal` refuses a command whose save is required (`article pdf`, `trial document`) with `usage.arguments` naming `--save`, before any source request. An optional `--save` on `article text` or `article supplement` keeps its old reply, which reads the source and refuses with the source link, because `article_save.rs` promises that link (gate j41722 caught a first version that refused it too). The review's "downloads the whole document" no longer held on 82360be61: the PDF bytes were already not fetched, but the Semantic Scholar or ClinicalTrials.gov lookup still ran. Case `tests/cases/mcp/http-save-refused.yaml` runs `article pdf` and `trial document` offline on a plain HTTP server; the case runner gains an `http` step key for that. `mcp_hosted.rs` now expects the refusal for plain-HTTP `article pdf` and `trial document`.
- C4: the 429 comment says a keyed request reads the throttle body only to scan it for its key.
- D6: clap help gets the same `plain()` wording as the details list ("source", "exact"); `docs/reference/commands.md` regenerated (j41711). The `; help` pointer difference is deliberate (`drop_pointer`) and stays.
- E3: `clinvar::GENE_COUNTS` is the gene-count leg's own spec, 30 s leg and 30 s `command_budget`, the Monarch pattern; `get gene ... clinvar` (or `all`) starts its deadline from `Fetch::source_budget`, which also lengthens a hosted call. Why 30 s: 0946 measured keyless BRCA1 reads at 7.3-7.6 s once paced (j40864), and reads that met NCBI throttle replies (2 s each) ran 14-17 s and failed the 15 s leg (j40834); the recorded keyed read spans about 2 s over 17 requests and the throttled one about 7 s (tests/replies/gene/clinvar and clinvar-throttled). 30 s is about four times a paced keyless read and twice the limit that failed, matching Monarch's 30 s; VAERS's 60 s is for a single 37 s reply, which ClinVar does not have. Case `tests/cases/gene/sections/clinvar-budget.yaml` reads it: the BRCA1 recording finishes with a 1-second command budget. `docs/configuration.md` lists the budget.
- F3: `winnow` removed from the workspace dependencies (still in Cargo.lock as a transitive dependency of other crates).
- F4: `GeneSearch::in_region` removed. `UsageError::argument_values`, the review's other F4 constructor, now has callers, so it stays.
- F7: openFDA's device, recall and report terms are one `openfda::TERMS`; the four device and recall SPECs and the adverse-event `FDA_TERMS` use it.
- Reasons written beside the code: A2 in `drug/resolve.rs` (FDA name lookups read every page because the ranking needs the whole set); B4 in `cache/schema.rs` and `store/mod.rs` (the local cache upgrades its own tables in place; it is local state, not a CLI or MCP format); F9 in `response/diagnostic.rs` as `//` comments, so no schema text changed.
- Release notes: lines for C2 (plain HTTP refuses saves before any source), E3 (ClinVar gene counts get 30 s, also in the hosted slow-source list) and D6 (generated help uses "source" and "exact").
- Cheap job j41711 (d3c38fd6a): fmt, schema and docs regeneration committed; it caught the inline-test placement, `biomcp-fetch/src/lib.rs` over 600 lines and the old plain-HTTP expectation in `mcp_hosted.rs`, all fixed in ac9f78700. The new C2 and E3 cases, the help, article-pdf and diagnostic cases and the command-budget tests passed there.
- Gate `make --keep-going landing-gate docs-check schema-release-check` passed in job j41732 at ea2c03386 (branch merged with origin/main ce011383d). Earlier gates j41718 (fmt and the architecture test list) and j41722 (the optional-save source link, see C2) failed and were fixed.
- Review fix, D6 doubled words: applying `plain()` to clap help copied four doubled words into terminal help ("Source source row offset", "Source source offset", "the source's source filter", "Source-source ID"); the details list already had the first three. Reworded at the root: phenotype and PGx offsets say "Source row offset" and "Source offset", `--no-preprints` says "the source's own filter", and `get drug` says "Source ID" (and "by source ID" in its description). `plain()` now folds "source source" and "source-source" in any letter case, and `reference_matches_grammar`, which docs-check runs, fails on "source source", "source-source" or "source's source" anywhere in the generated reference. Schemas and docs regenerated in cheap job j41755, which passed check-cheap and the reference, MCP, help and drug cases.
- Gate `make --keep-going landing-gate docs-check schema-release-check` passed again after the review fix in job j41762 at bae4dc24d (branch merged with origin/main ce011383d).
