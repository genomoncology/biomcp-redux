# 0212 — Register WikiPathways as an open bundle source beside Reactome

Status: COMPLETE.
Landed: 8c27b579d145a8921d2aa6b0fb6b34c557ecf19b

Milestone: 4

Depends on: none

## Outcome

The one source registry declares `wikipathways` once, labeled `open` in the open group, keyless, with the CC0 waiver from the captured WikiPathways help page at `https://www.wikipathways.org/help.html` as its terms name and URL. The existing `reactome` declaration stays the one Reactome declaration, with its CC0 database data terms. Both start enabled. `--disable-sources`, the configured disable list and the open group skip either source before any request, bundle read, cache read or replay, and source outcomes and health carry the use label and terms on empty, skipped and failed outcomes too. KEGG and g:Profiler have no declaration.

- Why: pathway.md keeps Reactome and WikiPathways as distinct open sources that require no key and obey source disabling before network or bundle access, and the entity boundary ruling and the gene ruling keep g:Profiler and KEGG dropped.
- Starts from: `crates/biomcp-adapters/src/registry.rs` and `crates/biomcp-adapters/src/reactome/mod.rs`; decision `2026-10-24-sources-are-open-by-default.md`; pathway.md's source boundaries; 0165 and 0171 as the pattern.
- Rules: the terms record names the help page, because the attempted `/license.html` returned a genuine 404 and is not the terms URL. No second name registers the same source. No runtime declaration for KEGG or g:Profiler. No second terms table. The WikiPathways quota and deadline follow the bundle download contract, since reads make no request.
- Proof: cases first. A case per declaration asserts its label, group and terms. Cases for `--disable-sources wikipathways` and `reactome`, the open group and the configured list assert the skip with zero requests and no bundle, cache or replay read, and that the other source stays available. The source name list in the schema output moves to its measured size plus 15 percent under the standing rule. Stub-backed, with no provider call. This ticket adds no filter or exact-ID get; each filter and exact-ID get that later pathway tickets add has a known-positive capture through one bounded keyless pm job. Then make landing-gate runs through pm job.
