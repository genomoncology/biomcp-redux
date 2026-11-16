# 1004 — variant-oncokb-section-refusal-names-the-real-cause

Status: COMPLETE.
Landed: a585fce2e6ffb2db984d01ff3c8396a0c371c322

Milestone: 7

## Outcome

`biomcp get variant clinvar:13961 oncokb`, and the same with `--tumor-type Melanoma`, never answers with a usage error that rejects `section` while listing `oncokb` among the valid values. With no OncoKB key it answers like every other licensed section: `oncokb` skipped, reason `no-key`, `next` pointing at `help key oncokb`. If OncoKB needs something the input lacks (a build, a protein change), the error names that option, not `section`.

## Why

The tumor-board persona on 3521c503a, the hosting candidate (bench ticket 0057, job j41867, `calls.md` lines 2033-2044 of `0057-persona-tumor-board-pair/biomcp-cli`), ran `biomcp get variant clinvar:13961 oncokb --tumor-type 'Melanoma'` and then without `--tumor-type`. Both answered `usage.arguments` on `option: section`, "option is missing or invalid", with `valid_values` that include `oncokb`. Its report says: "The CLI rejected the `oncokb` section word with a generic usage error rather than a 'no-key' refusal", and "the current generic usage error hides the cause". On 2026-11-12, `get variant CA126713 oncokb --build GRCh37 --tumor-type …` reached the no-key skip, so the refusal depends on the input form.

## Proof

- Proof: routine cases for `get variant clinvar:13961 oncokb` with and without `--tumor-type`, with no OncoKB key: each answers with the `no-key` skip or with an error naming the real missing option. The cases fail on the base commit.

## Starts from

- Starts from: `get variant` section parsing and the OncoKB section's preconditions; the error that names `section`.

## Keeps

- Keeps: OncoKB's licence gate and the no-key refusal wording other licensed sections use.

## Defers

- Defers: nothing.

## Changes

- Changes: the variant get's section validation, cases and their recordings.

## Progress

- Cause: `VariantGet`'s `TryFrom` (`crates/biomcp-types/src/query/variant.rs`) refused any request naming the `oncokb` section without both `--build` and `--tumor-type`, with `QueryInvalid` arguments `["section", "build", "tumor_type"]`. The grammar's `validation::checked` names the first listed argument the schema holds, so the refusal named `section` and listed the section words, `oncokb` among them. It ran before the key check, so a keyless call never reached the `no-key` skip. CA126713 with `--build` and `--tumor-type` passed the check, which is why the 2026-11-12 call skipped.
- Decision: drop that type-level check. The OncoKB section already checks at run time, in order: disabled, no key (`no-key` skip, next `help key oncokb`), missing build or tumor type (skipped with `help get variant` as the next step, as `all` already does). So `get variant ID oncokb` now answers like `all` and every other licensed section. A chromosome input without `--build` is still refused by the existing identity rule, which names `--build`, not `section`. I kept the keyed missing-context answer as the existing skip rather than adding a new usage error, so `oncokb` and `all` agree.
- Proof: new case `tests/cases/variant/oncokb-no-key.yaml` (`get variant clinvar:13961 oncokb`, with and without `--tumor-type Melanoma`; no key, no source requests) asserts the `no-key` skip and the `help key oncokb` step; on the base commit both calls exit 2 with `usage.arguments` on `section` (bench 0057 j41867 calls.md 2033-2044). The wire test `variant-oncokb/oncokb-key-policy-wire` asserted the old exit 2 for keyless missing build and missing tumor type; it now asserts the keyless missing tumor type skips as `no-key` and the chromosome missing build refusal does not name `section`. `docs/reference/commands.md` regenerated: `--tumor-type`'s example now takes `Melanoma` from the new case. One line added to `release/notes.md`.
- Jobs: j41883 (check-cheap passed; showed the docs line; my two-filter cargo call was malformed), j41888 (variant cases pass, new case passes; wire test failed on the old exit 2), j41891 (fmt and docs clean; wire test failed on the chromosome build refusal, fixed in b6609454d).
- Gate j41892 passed on d8d747da4 (`make --keep-going landing-gate docs-check schema-release-check`; branch level with origin/main).
- Review fixes: the wire test now reruns the missing-tumor call after the key is set and asserts exit 0, `oncokb` skipped as `no-join-id` and a `configure` step; its one-request count proves nothing reached OncoKB. The new case cannot make a live request: an offline step needs the case cache that a replay step creates (a first offline step fails with local `version`, j41900), so step 1 runs under the `variant/id-forms` replay, where an unrecorded request fails, and step 2 runs `offline`. Jobs j41898 (fmt layout), j41900 (offline-first failure), j41906 passed fmt check, docs-check and the six `oncokb` trials on 37be2509c.
- Land gate j41911 failed clippy's line limit on `wire` (94/80 lines). The keyless checks and the keyed missing-tumor check moved into helpers `keyless` and `keyed_missing_context`; j41912 passed `make check-cheap` and the six `oncokb` trials on 252c025aa.
