# 0269 — Retire old author options and prove author failure cases

Status: COMPLETE.
Landed: 64943488862777f7cc64c244f138cc81a6a87171

Milestone: 4

Depends on: 0267
Depends on: 0268

## Outcome

Every 0.9 author command and option that author.md and the author rulings retire or defer returns a fixed usage error that names the supported route or the reason, and makes zero requests. Semantic Scholar and ORCID each report a failed outcome, never an empty one, on a 429, 500, timeout, broken JSON or HTTP 200 error body, and an empty search, an absent record and a source failure are three distinct states. Disabling either source, by name or by the `noncommercial` group, skips it before credentials, cache and network.

- Why: The author rulings defer topic-wide ranking and cross-provider merging, and the slice rules require failure, identity and source-disable cases for every adapter.
- Starts from: author.md's legacy command table, technical controls and case list, with the author rulings and the new-sources decision; the existing retirement table, including the article slice's retired `article authors`; 0266, 0267 and 0268.
- Rules: `-q` and `--query` stay. `list author` names `help search author` and `help get author`. A request for topic ranking, coauthors or an ORCID name search says it is not supported and names `search author TEXT` or `search article`. No retirement rewrites the query, guesses a candidate or joins by name. Empty search text fails before I/O. No command downloads, installs or runs another tool.
- Proof: outside-in cases first. A case per retired or deferred spelling asserts its message with zero provider requests. Synthetic 429, 500, timeout, broken JSON and HTTP 200 error replies for `semantic-scholar` and `orcid`, labeled synthetic, assert a failed outcome with source and terms. A case asserts a successful empty search, an absent record and a failure as distinct. Source disable by name and by group is asserted before cache and network for each source, with cached delivery refused as public issue 290 requires. A key-missing 429 offers the `key` step. Each case has a size ceiling. This ticket adds no filter or exact-ID get and makes no provider call; the filters and exact-ID gets it relies on have known-positive captures in 0264, 0266, 0267 and 0268. No licensed content enters the repo. Then make landing-gate runs through pm job.
- Defers: nothing.
- Changes: the author query code, `crates/biomcp/src/retired.rs` and its author module, cases and author.md.
- PM ruling 2026-10-26 (m7951): `list author` retires like `list phenotype`, `list disease` and `list pathway`: a usage error before I/O with code `usage.retired` that names `help author`. The route list that `list author` shows today, including `author papers`, moves to `help author` with the caps approved in m7735 for its measured size plus 15 percent. Legacy ranking, coauthor, merge and ORCID name search spellings retire as the ticket says.
- PM ruling 2026-10-26 (m7951 ask 2): `-l` stays a working alias of `--limit` on `search author` and `author papers`, as pathway, study and GWAS keep it. It is not retired.
