# 0989 — release-notes-catch-up

Status: OPEN.

Milestone: 7

## Outcome

`release/notes.md` describes what a user of the 1.0 build gets today. It names only credentials the code reads, and it covers every user-visible change landed since the notes were last brought up to date, saved work included.

## Why

Findings D1 and D2 of `sdlc/records/2026-11-14-adversarial-review.md`, which the review calls blockers of an honest release.

- **D1:** `release/notes.md:63` names `UMLS_API_KEY`, but the code reads seven credential names and none of them is UMLS (`crates/biomcp-fetch/src/secret.rs` `environment()`).
- **D2:** since then about 120 tickets have landed (from 2026-11-13). Their user-visible changes are missing from the notes. They include:
  - GTR specimens and lab contacts, and ClinVar gene counts;
  - the FDA search, recall, PMA and companion-diagnostic changes;
  - HPO ancestors and the CPIC phenotype filter;
  - Europe PMC and bioRxiv PDFs, and `sync_timeout_seconds`;
  - the leaner reply envelope (terms moved to `help sources`, `sdlc/decisions/2026-11-15-source-terms-live-in-help-sources.md`);
  - related queries;
  - saved work: lists, searches, notes, `changed`, the typed `saved` tool and the hosted `--saved-folder`.

## Proof

- Proof: every credential name in the notes appears in `secret.rs` `environment()`, and every one there appears in the notes.
- Proof: one pass over `git log` from the commit that last updated the notes' feature list to main. Each landed ticket with a user-visible change (a command, option, filter, source, config key, credential name, output field or tool) has a line in the notes or a reason in Progress for leaving it out.
- Proof: `make docs-check` passes.

## Starts from

- Starts from: the review's D1 and D2 text, `release/notes.md`, `CHANGELOG.md`, and `sdlc/decisions/2026-11-01-no-backwards-compatibility.md` (the notes list every removed or changed command, option, config key and output field).

## Keeps

- Keeps: the notes' existing structure and plain wording. No code change. Notes and public docs name none of our private projects.

## Defers

- Defers: whether every later ticket updates the notes as it lands. That is a rule for Ian to make.

## Changes

- Changes: `release/notes.md`, and `CHANGELOG.md` if it repeats the notes.

## Progress

Writing pass, 2026-11-16. Only `release/notes.md` changed.

- Credentials (D1): `UMLS_API_KEY` is gone. The notes' list of recognized source credentials is now exactly the seven names `crates/biomcp-fetch/src/secret.rs` `environment()` returns (lines 174-184): `OPENFDA_API_KEY`, `NCBI_API_KEY`, `S2_API_KEY`, `ORCID_ACCESS_TOKEN`, `ONCOKB_TOKEN`, `ALPHAGENOME_API_KEY` and `NCI_API_KEY`, and each of those is in the notes. Two other environment names appear in the notes, and neither is a source credential: `DISGENET_API_KEY`, listed among 0.9 variables that 1.0 no longer reads, as the no-backwards-compatibility decision requires; and `BIOMCP_TRIAL_CONTINUATION_KEY`, the hosted continuation signing key that 0821 added and `crates/biomcp/src/config.rs:20` reads.
- Where the walk starts (D2): the last landing that added to the notes' feature list is 289235664 (0763, 2026-11-07). After it, 0904 (0ecf44652) and 0849 (fb7cbeed8) each changed one sentence for their own change. The walk covers `git log --first-parent 289235664..5f96d6803`: 211 landing commits, 208 distinct tickets (0766 and 0767 landed more than once), plus five idea-filing commits. That is wider than "from 2026-11-13", because tickets that landed from 2026-11-07 to 2026-11-12 (0823, 0825, 0838, 0846, 0856, 0859 and others) were missing too.
- Method: I read every ticket's Outcome, and the Progress of the 158 whose Outcome named something a user could see. I checked names against a diff of `docs/reference/commands.md` between 289235664 and main (options, section words, sources, sync names) and of `ConfigFile` in `crates/biomcp/src/config/file.rs`. The only new source is `orphadata`. The new options, sections and config keys in that diff all have a line in the notes, except `--name-snapshot`, a continuation token that replies supply.
- Added to the notes: sections "Saved work", "Drug names and variant spellings widen by default" and "Added commands, filters and sections" (drugs and regulators, devices and diagnostics, adverse events, trials, genes and related entities, articles, other commands); hosted settings and the `orphadata` label under "Hosted use and source terms"; new config keys and pacing defaults under "Configuration migration"; a list of reply changes under "Output migration". Typed mode now reads 18 tools with `saved`, and both typed examples use `id`, as the schemas name it. The 0.9 bullet that called trial `--variant` and `--criteria` literal now says they widen to other spellings, after 0856.
- Left out, because no command, option, filter, source, config key, credential name, output field or tool changed:
  - Tests, gates and release tooling: 0767, 0769, 0770, 0771, 0772, 0776, 0778, 0787, 0789, 0791, 0802, 0803, 0804, 0822, 0842, 0848, 0852, 0865, 0914, 0921, 0927, 0944, 0952, 0960, 0963 (proves `--record` folders still refuse symlinks; the review's D2 listed it), 0971, 0985 (saved-schema release check; its reply fixes refine saved commands that are new in 1.0 and described as they stand) and 0986.
  - Release qualification, publishing, reviews and diagnostics: 0293, 0640, 0788, 0792, 0798, 0806, 0905.
  - Packaging and dependency updates with no behaviour change: 0773 (bundled SQLite), 0775 (zlib in Linux containers), 0981 (BioData pin; no reply change measured).
  - Docs and skill wording only: 0764, 0783, 0797, 0922, 0937 (its field-order part is in the totals line), 0948 (no reply change), 0950 (field descriptions only), 0958.
  - Test recordings and test data: 0906, 0910, 0920, 0954 (hidden recording options only; `sync wikipathways` was not broken), 0973.
  - Internal reliability: 0777, 0781, 0785.
  - Fixes that only stop a wrong result or a failure: 0774 (advertised cell-line output schema; replies unchanged), 0780 (MyChem keeps a failed or absent status), 0794 (searches query again instead of reusing an incomplete stored answer), 0796 (trial continuation offsets), 0816 (FAERS widening without MedDRA), 0864 (device recall paging at small offsets), 0913 (alias notices print plain names), 0918 (no false EMA opinion-status problem), 0956 (clearer refusal for `--gene` with `--gene-id`).
  - Did not land as titled: 0879 (EMA label text by section; EMA publishes no usable sectioned text, so only a field description changed).
- Every other ticket in the walk has a line in the notes. Where Progress narrowed an Outcome, the notes follow Progress: 0827 carries `open_access` but no licence, 0847 adds no PMA row fields, 0887 and 0900 keep 50-row pages, 0957 refuses a drug name rather than resolving it, and 0965 explains Europe PMC's refusal rather than fetching the PDF.
- `CHANGELOG.md` is one paragraph that points at the notes and repeats none of them, so it is unchanged.
- Outside this ticket's Changes, and left as they are: `README.md` and `docs/configuration.md:31` still say typed mode offers 17 tools (configuration adds `saved` later on); `sdlc/planning/upgrade.md` still lists 0.9's `search gene --region` as dropped, while 0888 reused the name for ClinGen regions. The notes say how the new option differs.
