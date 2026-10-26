# 0265 — Add the author types and generated schemas

Status: COMPLETE.
Landed: 0e68f897c9dbdb7f870cc0cc34b77953b67f3f00

Milestone: 4

Depends on: 0264

## Outcome

`biodata` holds a checked source-qualified `AuthorId` with a Semantic Scholar variant and an ORCID variant whose checksum is validated, the author header, the author card, the compact author paper header, the detailed author paper and a separately typed `OrcidWorkGroup` with ordered assertion summaries and each external identifier's type, value and relationship, all with no I/O. `biomcp-types` holds the `search author` request with a positional query or `--query`, `--source semantic-scholar`, `--limit` from 1 to 25 with a default of five, a nonnegative `--offset` and `--pool`; the `get author` request with one or more qualified IDs; and the `author papers ID` request with the `details` section, `--full` as its alias, `--limit`, `--offset` and `--pool`. The source spelling `semantic-scholar` is canonical, and `semanticscholar` parses as its alias in both `--source` and ID prefixes. The generated, checked-in schemas are `biomcp.author_page/1`, `biomcp.author_card/1`, `biomcp.author_papers/1`, `biomcp.author_paper_details/1` and `biomcp.orcid_works/1`.

- Why: The slice method lands types and schema before adapters, and the accepted design with the author rulings fixes the records, the command tree and the page shapes.
- Starts from: author.md's proposed command and disclosure shape and structural risks, with the author rulings; 0264's witnesses; `rust-standards.md`; the existing checked `Pmid`, `Pmcid`, `Doi`, `SemanticScholarId` and `Curie` types. PM ruling 2026-10-26 (m7425): no shared publication or authorship record exists, so native byline assertions live in author-owned checked records and no cross-entity abstraction is added.
- Rules: an author ID identifies one provider record and never a person, and no type links a Semantic Scholar ID to an ORCID iD. Source counts are optional and state their scope. An ORCID work group keeps every assertion summary in source order and is never counted as several publications. External identifiers keep their native type and relationship before any article pivot. A paper row keeps its native source paper ID and carries a PMID or DOI only where the source supplied one. No field promises a global publication total, current employment, contact details, biography or verified person identity. Optional absence never means not fetched. There is no coauthor aggregation, topic score, affiliation filter or merged author type.
- Proof: cases first. Type review checks each field's absence, multiplicity and collection scope against 0264's witnesses, including a byline with repeated display names, a paper with no PMID, a missing affiliation, an ORCID work group with several assertions and the public lowercase visibility value from public issue 289. Round-trip tests cover both ID forms and the `semanticscholar` alias, and refusal of a bad ORCID checksum, an unqualified ID, a zero or over-25 limit, a negative offset and each before-I/O failure. The generated schemas are checked in with snapshot and compatibility checks, and existing schemas stay unchanged. This ticket makes no provider call. No licensed content enters the repo. Then make landing-gate runs through pm job.
- Defers: executable help, Markdown parity and the shared request, step and envelope schema versions move to 0266, which registers author in the shared request.
- Changes: `crates/biodata/src/`, `crates/biomcp-types/src/`, their schemas and contract tests.
