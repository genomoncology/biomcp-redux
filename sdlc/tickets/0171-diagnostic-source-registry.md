# 0171 — Register the diagnostic sources with use labels

Status: COMPLETE.
Landed: 5cc3e2529d52cc2f995cf1d79e347a2f2199efcc

Milestone: 4

Depends on: 0170

## Outcome

The canonical `Source` enum and the one source registry declare `gtr`, `who-ivd`, `fda-cdx` and `fda-pma` once each, using the same atom in CLI selection, query echo, response, health, replay and cache keys and disable settings. `gtr` is labeled `open` under the NCBI usage policies, `fda-cdx` `open` under the FDA website policies, and `fda-pma` `open` under the openFDA terms. `who-ivd` is labeled `noncommercial` in the `noncommercial` group under the WHO terms of use page, with the reason stated: the export carries no narrower license and the terms reserve commercial and substantial reproduction. Every source starts enabled. `--disable-sources`, the configured disable list and the `noncommercial` group skip a source before any request, cache read, bundle read or replay, and source outcomes and health carry the use label and terms on empty, skipped and failed outcomes too.

- Why: the open-by-default decision gives every source a use label and group in one registry, and the 0154 review aligned the source names and approved `fda-cdx` and `fda-pma`.
- Starts from: `crates/biomcp-adapters/src/registry.rs`; decision `2026-10-24-sources-are-open-by-default.md`; diagnostic.md's use labels table and source registry paragraph; 0125 and 0139 as the pattern; 0170's types.
- Rules: no per-command alias such as `GTR` or `openfda-device`. `fda-510k` gets no declaration. The PMA projection carries no openFDA harmonized field, so no GMDN content is exposed. Existing drug pivots use the registered `drugsfda`. No second terms table.
- Proof: cases first. A case per declaration asserts its label, group and terms. Cases for `--disable-sources gtr`, `who-ivd`, `fda-cdx` and `fda-pma`, the `noncommercial` group and the configured list assert the skip with zero requests and no cache, bundle or replay read, and that the other sources stay available. Stub-backed, with no provider call. This ticket adds no filter or exact-ID get; each filter and exact-ID get that later diagnostic tickets add has a known-positive capture through one bounded keyless pm job. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
