# 0849 — lean-reply-envelope-by-default

Status: COMPLETE.
Landed: 40b2fcefb4db58b097df88dba6d7e555d49ba5ad

Milestone: 7

## Outcome

Every reply's per-source outcome drops `terms` and `terms_url`, keeping `source`, `use`, `state`, `count` and `notices`; the licence text and links stay in `help sources`. Every reply schema takes one coordinated new major for it. `query` stays.

## Why

`sdlc/records/2026-11-11-token-gap-root-cause.md`: about 16% of reply bytes are envelope, and every reply lands in the conversation that is re-read each turn. 0838's design review measured a live 3-row trial search: dropping the `query` echo, the per-source licence fields and the server config path saved 15%; with the config path gone in 0837 and `query` kept, this is about 10–12% on short pages. A default applies to every turn, where the opt-in `fields` option helps only the turns that use it.

## Proof

- Proof: routine cases show no `terms` or `terms_url` in replies and the licence text in `help sources`; a size table over ten recorded replies before and after; the release check passes with the new majors.

## Starts from

- Starts from: source outcome building in `crates/biomcp-types/src/response/`, the reply schemas, `help sources`, `sdlc/decisions/2026-11-01-no-backwards-compatibility.md`.
- PM decision: 2026-11-11. Build after 0837 and 0838. Ian is told before it lands, because it changes where licence text appears; attribution by source name stays on every reply.

## Keeps

- Keeps: source name and use on every reply; default rows unchanged (0847 adds fields to rows on purpose).

## Defers

- Defers: other envelope trims until measured.

## Changes

- Changes: `crates/biomcp-types/src/response/`, `schemas/`, `crates/biomcp/src/help/`, `tests/cases/`

## Progress

- 2026-11-15: Built on ec489dc1d. `SourceOutcome` keeps `source`, `use`, `state`, `reason`, `term`, `count`, `retrieved_count` and `notices` and drops `terms` and `terms_url`. `help sources` now ends with "Publisher terms by source", one `source: terms <url> (use)` entry per registered source. `health` keeps its own terms per source. Stored outcomes are still relabelled with the current `use`; old stored `terms` fields are ignored on read.
- Major: the 137 reply schemas that carry a source outcome each take one new major (`const SCHEMA` in `crates/biomcp-types/src/`, plus `$id` and every test, case and script naming the old major), the same mechanism 0838 used for `trial-page/17`. `compatibility::check_reviewed` accepts any change when the new major is higher, so `schema-release-check` passes against the 1.0.0 baseline with no review entries. Schemas new since the baseline bump too, except `saved/1`: saved-work replies carry no source outcomes, 0982 is changing the saved schemas now, and its checked-in export fixture is bound to a secret-scan exception by commit, so saved takes its next major with 0982.
- Proof: `tests/cases/help/source-terms.yaml`; `gene/braf`, `gene/sections/oncokb-no-key`, `readme/gene`, `disease/orphadata`, `diagnostic/510k-failures`, `mcp/fetch-keeps-fence-backslash` now check that replies omit `terms` and `terms_url`. Budget accounting in `source_terms.rs` now deducts only `use` and rejects a repeated `terms`.
- Size, ten recorded CLI JSON replies (job j41518: before at ec489dc1d, after on this branch before its history was squashed; the saved revert does not touch these replies):

| Case | Command | Before bytes | After bytes | Saved |
| --- | --- | --- | --- | --- |
| gene/braf | get gene BRAF | 2,373 | 2,112 | 261 (11.0%) |
| disease/orphadata | get disease MONDO:0016512 prevalence --source orphadata | 2,246 | 2,058 | 188 (8.4%) |
| drug/ema | search drug Tibsovo --source ema --limit 1 | 1,259 | 1,107 | 152 (12.1%) |
| variant/search-L858R | search variant --gene EGFR --protein L858R --build GRCh37 | 1,226 | 1,124 | 102 (8.3%) |
| gwas/studies | search gwas --pubmed 17554300 --studies --limit 50 | 2,429 | 2,325 | 104 (4.3%) |
| trial/country | get trial NCT03734029 locations --limit 1 | 2,914 | 2,760 | 154 (5.3%) |
| protein/cards | get protein P00533 Q9UIF7 P38398-2 | 2,027 | 1,956 | 71 (3.5%) |
| article/get/card | get article 33123537 | 2,376 | 2,258 | 118 (5.0%) |
| pathway/examples | search pathway Signaling by EGFR | 1,445 | 1,370 | 75 (5.2%) |
| pgx/examples | search pgx --gene DPYD --source cpic | 979 | 894 | 85 (8.7%) |
| Total | | 19,274 | 17,964 | 1,310 (6.8%) |
- 2026-11-15, review fixes (decision `2026-11-15-source-terms-live-in-help-sources.md`): each Orphadata record (`OrphanetEpidemiology`) now carries a required `notice` with the statement Orphanet's legal notice asks results to carry, taken from the same text as its terms in `crates/biomcp-adapters/src/orphadata/bundle.rs`; `disease/orphadata` asserts it. The `use` description points to `help sources`; `skills/SKILL.md` drops "terms" from its preserve line (4,085 bytes). Merged main with 0982; its saved schemas carry no source outcome and stay at `saved/1`. Orphadata row re-measured in j41539 (2,058 bytes after; the table above uses it).
