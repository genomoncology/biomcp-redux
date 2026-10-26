# 0235 — Register InterPro and Complex Portal as open sources

Status: COMPLETE.
Landed: ccf7f16eb85c5c10ca9ca93d0948f9dbc0c93951

Milestone: 4

Depends on: none

## Outcome

The one source registry declares `interpro` and `complex-portal` once each, keyless, labeled `open` with CC0 as their terms name, the InterPro license page as InterPro's terms URL and the Complex Portal resource paper that states its CC0 terms as Complex Portal's terms URL. Both start enabled. UniProt keeps its existing declaration, labeled `attribution` under CC BY 4.0. `--disable-sources` and the configured disable list skip either new source before any request, cache read or replay, and source outcomes and health carry the use label and terms on empty, skipped and failed outcomes too. STRING gets no runtime declaration.

- Why: the 2026-10-26 protein rulings keep InterPro location matches and Complex Portal exact-member access and defer STRING, and the design finds both kept sources open with no restricted-use group.
- Starts from: `crates/biomcp-adapters/src/registry.rs`; the existing `uniprot` declaration and quota; decision `2026-10-24-sources-are-open-by-default.md`; protein.md's source findings and terms table; 0165 and 0171 as the pattern.
- Rules: no second name registers the same API, and no new source group is added. Each declaration's deadline and quota follow the existing keyless sources, and the quota comment says it is local policy, not a published provider limit. Member-database material inside InterPro keeps its upstream rights, so the adapters request integrated InterPro entries only. No second terms table. No STRING, InterProScan or bundle declaration.
- Proof: cases first. A case per declaration asserts its label, terms name and terms URL, and a case asserts UniProt's declaration is unchanged. Cases for `--disable-sources interpro` and `complex-portal` and the configured list assert the skip with zero requests and no cache or replay read, and that the other sources stay available. A case asserts the source list holds no STRING declaration. Stub-backed, with no provider call. This ticket adds no filter or exact-ID get; each filter and exact-ID get that later protein tickets add has a known-positive capture through one bounded keyless pm job. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
