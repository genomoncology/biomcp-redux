# 0214 — Register HPO as a direct source beside its Monarch and MyDisease component

Status: COMPLETE.
Landed: 6084a1895f90527d928ccdba8743e3021532a2f6

Milestone: 4

Depends on: none

## Outcome

The one source registry declares `hpo` once as a direct, keyless source for the HPO term API host that j14227 captured, labeled `attribution`, with the existing HPO license terms name and URL. It starts enabled. `--disable-sources hpo` and the configured disable list skip the direct source and the existing `hpo` component of Monarch and MyDisease before any request, cache read or replay, and source outcomes and health carry the use label and terms on empty, skipped and failed outcomes too. `--disable-sources monarch` leaves direct `hpo` available, and `restricted` alone leaves both available. Health gains one bounded read-only exact term request for `HP:0001250`.

- Why: the design keeps direct HPO for the exact term card, its synonyms and its cross-references, and today HPO appears in the registry only as an aggregator component.
- Starts from: `crates/biomcp-adapters/src/registry.rs`, `crates/biomcp-adapters/src/disease_sources/components.rs` and its `HPO` terms constant; `Source::Hpo` in `crates/biomcp-types/src/source.rs`; decision `2026-10-24-sources-are-open-by-default.md`; phenotype.md's license paragraphs; 0165 and 0139 as the pattern.
- Rules: the direct source and the components share one HPO terms declaration, with no second terms table and no second name for the same API. No key or key name is declared. The declaration states retrieval time and never fills an ontology version from a release date. No HPO search endpoint is registered for product use. No Monarch similarity, translation or bundle endpoint is declared.
- Proof: cases first. A case asserts the declaration's label, group, terms and keyless state. Cases for `--disable-sources hpo` and the configured list assert the skip of the direct source and of Monarch's `hpo` component with zero requests and no cache or replay read. A case for `--disable-sources monarch` asserts direct `hpo` stays available, and a `restricted` case asserts both stay available. The health probe case replays its recorded exact term reply. The schema output's source name list ceiling moves to measured size plus 15 percent under the slice rule. Stub-backed, with no provider call. This ticket adds no filter or exact-ID get; each filter and exact-ID get that later phenotype tickets add has a known-positive capture through one bounded keyless pm job. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
