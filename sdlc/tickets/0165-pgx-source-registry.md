# 0165 — Register CPIC and ClinPGx as noncommercial and retire the pharmgkb spelling

Status: OPEN.

Milestone: 4

Depends on: 0164

## Outcome

The one source registry declares `cpic` and `clinpgx` once each, labeled `noncommercial` in the `noncommercial` group, with the ClinPGx data use policy as their terms name and URL. Both start enabled. `--disable-sources`, the configured disable list and the `noncommercial` group skip either source before any request, cache read or replay, and source outcomes and health carry the use label and terms on empty, skipped and failed outcomes too. The old `pharmgkb` spelling in `--source` or `--disable-sources` returns a usage error that names `clinpgx` and makes zero provider requests.

- Why: the 0153 PM ruling labels both sources `noncommercial` under the shared ClinPGx data use policy, names the source `clinpgx` and keeps CPIC as its own source.
- Starts from: `crates/biomcp-adapters/src/registry.rs`; decision `2026-10-24-sources-are-open-by-default.md`; pharmacogenomics.md's use and terms section; 0163's policy text witness; 0125 and 0139 as the pattern; 0164's types.
- Rules: the terms record quotes the policy text 0163 captured, or states that the label rests on the PM ruling when 0163 found no text. No second name registers the same API. PharmCAT, PharmVar and DPWG get no runtime declaration. No second terms table.
- Proof: cases first. A case per declaration asserts its label, group and terms. Cases for `--disable-sources cpic` and `clinpgx`, the `noncommercial` group and the configured list assert the skip with zero requests and no cache or replay read, and that the other source stays available. Cases for `--source pharmgkb` and `--disable-sources pharmgkb` assert the replacement message with zero requests. Stub-backed, with no provider call. This ticket adds no filter or exact-ID get; each filter and exact-ID get that later pharmacogenomics tickets add has a known-positive capture through one bounded keyless pm job. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
