# 0920 — noncommercial-recordings-checked-and-synthetic

Status: OPEN.

Milestone: 7

## Outcome

No test reply recorded from a source labelled noncommercial holds that source's real data. Each such reply is synthetic and says so in a README beside it.

## Why

0910's developer (2026-11-13) searched `tests/replies/` for recordings from noncommercial sources after replacing the WHO exports, and found:
- **Semantic Scholar** (`Use::Noncommercial`, `crates/biomcp-adapters/src/semantic_scholar/mod.rs:9-12`) has what look like real recordings:
  - `article/fulltext/live-all/semantic-scholar`, used by `article/sections/grammar.yaml`;
  - `article/semantic-scholar/live`, `article/pdf/timeout` and `article/pdf/unavailable`, which no case file names; Rust tests may use them.
- **ORCID:** `orcid/translated-title` has no case that names it, so whether it is real is unknown.
- **WHO IVD:** 0695's older synthetic replies (`diagnostic/who-synthetic`, `diagnostic/who-search-synthetic`) use the real WHO product ID 0466-046-00 (cobas 4800 HPV Test). It also appears in `who.yaml`, `who-search.yaml` and several Rust tests.

0695 set the rule ("no live fetch of noncommercial data"), and `AGENTS.md:5` says "Respect source redistribution terms".

## Proof

- Proof:
  - a check, run in the gate, that lists every recording under `tests/replies/` for each source whose terms say `Use::Noncommercial`, and fails unless its folder has a README saying the rows are synthetic;
  - the Semantic Scholar, ORCID and WHO IVD items above are synthetic, or shown to be synthetic already, with invented IDs, and the cases that use them still pass;
  - unused recordings are deleted with a note of what used them.

## Starts from

- Starts from: 0910's synthetic WHO replies and READMEs; 0695.

## Keeps

- Keeps: what each case asserts about behaviour.

## Defers

- Defers: deleting real data from git history, which is Ian's call if wanted.

## Changes

- Changes: `tests/replies/` for those sources, their cases and Rust tests, a check script wired into the landing gate.

## Progress

- Noncommercial sources, from the registry's terms: Semantic Scholar, ORCID, WHO IVD, WHO PQ, DDInter and ClinPGx, plus MyVariant's CADD, dbNSFP and SNPedia fields. `scripts/article_licenses.py` (the gate's `rights` stage) covers only article licenses, so it could not list these; a scan of every `tests/replies/**/*.json` by its `source` field found 93 recordings in 30 folders (none from ClinPGx; MyVariant's noncommercial fields only in `variant/synthetic-components`, already synthetic).
- Semantic Scholar: every body was already authored (0695) except two. `article/pdf/closed` held S2's live `CLOSED` status for PMID 17611497; it now carries an invented paper ID and is authored. `article/fulltext/live-all` holds a synthetic citation count (0695) and a null open-access reply, so it holds no S2 data. `article/recommendations/empty` and `author/empty-first-source` are live empty lists with no rows. `article/pdf/timeout` and `article/pdf/unavailable` were already synthetic and are still used by the `pdf.rs` runner test and `scripts/test_article_licenses.py`, so they stay.
- Deleted `article/semantic-scholar/live`: three S2 429 bodies for "EGFR T790M osimertinib", no rows. Its only users, `tests/cases/article/semantic-scholar/live-rate-limit.yaml` and the `scholar.rs` runner, were removed in 0456 (a3ec920ce).
- ORCID: the bodies were already authored, but used the real iD 0000-0002-4299-3018. All four replies now use the invented 0000-0000-0000-0925 (valid check digit, outside ORCID's issued blocks), renamed to their new request hashes; `orcid/visibility.yaml`, `orcid/refused-member.yaml` and `mapped_refusals/source_pages.rs` (the only user of `orcid/translated-title`) follow. The real iD stays as ORCID's health probe and in `tests/native/orcid/`, which is outside `tests/replies/`.
- WHO IVD: 0466-046-00 (a real product ID in the export 0876 recorded) is now 9466-946-00 in `who-synthetic`, `who-search-synthetic`, `who.yaml`, `who-search.yaml`, three runner tests and `biomcp-types` contract tests. The other IDs in those files were checked against that export and none is real.
- DDInter: names were synthetic, but IDs DDInter1-30 are real DDInter IDs, and DDInter22-DDInter23 Moderate matched a real interaction in DDInter's export. Every ID moved up by 9000 (DDInter9001-9030; DDInter's highest is DDInter1972) in both DDInter replays, the three DDInter cases and three runner tests.
- READMEs saying the rows are synthetic now sit beside every such folder: new for `article/edges`, `article/evidence/source`, `article/recommendations`, `article/tldr`, `author`, `orcid`, `diagnostic/who-synthetic`, `diagnostic/who-search-synthetic` and `drug/ddinter-synthetic`; extended for `article/fulltext`, `article/pdf` and `drug/source-aliases`.
- Check: `crates/biomcp/tests/ratchets/noncommercial_replies.rs`, run by the `ratchets` test in the landing gate's pre-job. It reads each source's and component's terms from the registry, finds every recording from a noncommercial source (or carrying a noncommercial component field), and fails unless the nearest `README.md` above it within `tests/replies/` says "synthetic". A unit test plants unmarked replies and shows it fails, ignores open sources, rejects a README without "synthetic", and passes once one says so.
