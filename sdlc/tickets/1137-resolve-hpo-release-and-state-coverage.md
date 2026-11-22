# 1137 — resolve-hpo-release-and-state-coverage

Status: COMPLETE.
Landed: 1f451de792de5dd183ab4efec9e7da7ddf237bb8

Milestone: 7

## Outcome

A phenotype search for the current HPO release resolves one real publisher tag, uses it for every page, names it in the reply, and says which annotations were left out, so a caller never has to guess a tag.

## Why

In a six-way comparison of how agents reach BioMCP (2026-11-22, round 1 task rd-3), every connector-only agent found `disease search --source hpo --phenotype …`, but `--hpo-release` must be an exact `vYYYY-MM-DD` tag and nothing says which is current. Agents guessed: `latest`, the help example `v2099-01-01`, and a dozen dates, spending 6 to 16 searches each. Five of six answered from an older release than the current one, and the task's wrong facts came from that (an older genes-to-disease file links a gene the current one does not). BioMCP read the named release faithfully; the fault is discovery. A failed tag also replies "This product uses Human Phenotype Ontology release v2099-01-01", naming a release that was never fetched. The reply says annotations "requiring separate OMIM permission are excluded", but the code drops records whose evidence is `IEA` (`crates/biomcp-adapters/src/hpo/release.rs:108-126`), so the notice does not say what was actually left out. The same task's answer key has nine entries where BioMCP's filtered view has six. A compact-catalog chatbot run of task cb-3 showed the same pattern for gnomAD constraint: it could not load the requested `v4.1` and answered from another release.

## Changes

- `crates/biomcp-query/src/disease/hpo.rs:19-48`, `crates/biomcp-adapters/src/hpo/release.rs`, the disease search argument declaration (`crates/biomcp-types/src/query/disease/search.rs:46-104`), `docs/reference/commands.md` near line 1414.
- An omitted tag, or `latest`, resolves once to the newest published HPO release through the publisher's release metadata; the reply's `release` and every continuation use that tag.
- An unknown tag fails as release-not-found, with no "uses release" notice.
- The coverage notice names the filter actually applied (evidence code) in plain words.
- Help and the typed tool description show a working current-release call, not a fictional date.
- Keeps exact-tag reads, source policy and the existing filter.

## Proof

- Replay against two fixture releases whose genes-to-disease rows differ: omitted tag, `latest`, an exact tag, and a missing tag. Assert the resolved tag, stable continuation when the fixture's latest changes between pages, release-not-found with no "uses" notice, and the coverage notice text.
- The rd-3 call `disease search --source hpo --phenotype HP:0001083 --phenotype HP:0000098 --phenotype HP:0001634` with no tag succeeds on the first call.

## Defers

- Whether to admit the annotations the filter drops is Ian's decision (data rights); this ticket only says what is left out.
- gnomAD constraint release fallback: checked by this ticket's developer, filed separately if the cb-3 pattern holds.
