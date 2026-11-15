# 0841 — openfda-text-filters-survive-punctuation

Status: OPEN.

Milestone: 8

## Outcome

An openFDA text filter whose value holds punctuation or a non-ASCII letter, such as an applicant named "Nestlé", returns matches or an honest empty result, never a source failure.

## Why

The review of approvals-by-applicant found that a value containing `! # $ ; ? ^ | ~`, a backtick or a non-ASCII letter gets a 400 from openFDA. The reviewer expects BioMCP to report that as a source failure and did not run it. Other openFDA text filters probably share the path.

## Proof

- Proof: first reproduce on the base commit with one value per character class against each openFDA text filter, and record which fail.
- Proof: a routine case per failing filter asserts a match or an empty result for a recorded punctuated value; it fails on the base commit.

## Starts from

- Starts from: the shared openFDA query building under `crates/biomcp-adapters/src/`, and the applicant filter landed by approvals-by-applicant.

## Keeps

- Keeps: results for plain values unchanged; no wildcard or boolean operator from user text reaches openFDA as syntax; no new source, key or dependency.

## Defers

- Defers: nothing.

## Changes

- Changes: the openFDA adapters' query building, `tests/cases/`

## Progress

- 2026-11-15: Reproduced on base d61945035 in pm job j41358 (live, with a key): each of 24 openFDA text filters got its plain value and the value with one of `! # $ ; ? ^ | ~` backtick `" \ * & + ( ) : / [ { < = % @ ' ,`, `é` or `®` appended. Every filter that reaches openFDA as a quoted phrase or prefix failed as a source failure (exit 4) on `! # $ ; ? ^ | ~`, the backtick, `é` and `®`, and kept its plain total on every other character (drug recall `--status Ongoing` and `--reason contamination` fail even plain at the default page of 10 with `bad-reply`, a separate fault this ticket does not change; at `--limit 1` they answer): Drugs@FDA name and `--applicant`; label name and `--indication`; drug recall name, `--firm`, `--status` and `--reason`; FAERS text and `--reaction`; MAUDE text, `--manufacturer`, `--product-code` and `--problem`; 510(k) `--name`, `--manufacturer` and `--product-code`; PMA `--name` and `--manufacturer`; device recall `--manufacturer` and `--product-code`. Device recall `--name`, PMA `--intended-use` and 510(k) `--classification-name` already spaced the punctuation and refuse non-ASCII letters before the request, so they did not fail. Raw openFDA answers HTTP 400 to `sponsor_name:MERCK\!*` and the other escaped forms as well, and NOT_FOUND to an empty phrase `sponsor_name:""`.
- 2026-11-15: One shared rule in `biomcp-types` (`openfda_words`, `openfda_phrase`) now builds every openFDA text value: an accented Latin letter becomes its base letter (`Nestlé` is sent as `Nestle`, the applicant prefix as `NESTLE`), the refused characters and any other non-ASCII character become spaces, spaces collapse, and quotes and backslashes stay escaped inside one quoted phrase. Text openFDA already accepts is sent byte for byte as before, so plain values keep their requests and results. An applicant left with no prefix sends `sponsor_name:""`, which openFDA answers with no matches, instead of a bare wildcard that would select every sponsor. The accent table that author name matching used moved to `biomcp_types::latin_base` so both share it. `docs/cautions.md` has a new "openFDA text filters" section.
- 2026-11-15: Recorded the routine cases live in pm job j41373 (no key, bodies unchanged; tables in `tests/replies/{drug,adverse-event,diagnostic}/openfda-punctuation/README.md`): `drug/approvals-punctuation`, `drug/label-punctuation`, `drug/recall-punctuation`, `adverse-event/faers-punctuation`, `adverse-event/maude-punctuation`, `diagnostic/fda-510k-punctuation`, `diagnostic/fda-pma-punctuation` and `diagnostic/recall-punctuation`, one step per failing filter. Each punctuated value returns the plain value's total measured on the base commit (for example `infusion pump!` 2,117,766 MAUDE reports, `Abbott|` 5,726 PMA records, `Keytruda?` 2 label sets); `--applicant Nestlé` answers empty because no Drugs@FDA sponsor starts with NESTLE, and `--applicant !` answers empty with a notice that it held no letter or digit openFDA accepts. On the base commit each step's request differs, so each case fails there.
- 2026-11-15: Cheap job j41375 regenerated schemas and docs with no change and passed every touched case (`punctuation`, `diagnostic/`, `drug/approvals`, `adverse-event/`, `author/`); its ratchet refused an inline unit test, which was dropped. Gate `make --keep-going landing-gate docs-check schema-release-check` passed in pm job j41381 at 8faff00b8.
- 2026-11-15 (review): Every filter now behaves the same. Device recall `--name`, `--intended-use` and `--classification-name` no longer refuse an accented letter: it is sent as its base letter, and only a value with no letter or digit once cleaned (such as `®`) is refused before the request; the three `*Ascii` error messages are no longer sent. Removing them from `ErrorMessage` failed `make schema-generate` in cheap job j41397 (`error: incompatible contract at /error/message`): the checked-in `error.json` lists them, and narrowing it needs an entry in `release/schema-reviews.json`, so they stay in the enum, unsent (PM decision, 2026-11-15: keep them; no narrowing entry). Those three phrases still squeeze repeated spaces, so their requests are byte-identical to before. When a value is sent as other words, its openFDA source carries a notice `--X was searched as "…"` (or that it held no letter or digit), on drug, adverse event and diagnostic replies alike; device recall `--name` keeps its own phrase notice. `docs/cautions.md` says `source_query` appears only on adverse event and diagnostic replies.
