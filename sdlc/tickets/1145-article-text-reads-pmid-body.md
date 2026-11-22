# 1145 — article-text-reads-pmid-body

Status: OPEN.

Milestone: 7

## Outcome

A caller who has a PubMed ID reads an open-access article's body with one call, through either `get article ID text` or `article text ID`.

## Why

Found 2026-11-22 by scouts exploring BioMCP for multi-entity questions (about 760 CLI calls on build b7def4651). All three scouts hit it. `biomcp get article 40343074 text` exits 2 (`usage.arguments`, suggests `fulltext`); `get article ID fulltext` returns section metadata and sometimes `fulltext.state=not-applicable` for an open paper; `biomcp article text 40343074` returns only the abstract card; the body needs `article text PMC12060791 --part N`. Agents spent several calls per paper finding the form.

## Changes

- `article text PMID` resolves the PMID to its open-access PMC copy and reads the body; when there is none, it says so and names the PMC ID or the reason.
- `get article ID text` either works as `article text` or its refusal carries that exact working call.
- `fulltext` replies say `not-open` versus `not-applicable` accurately.

## Proof

- Replay: a PMID with an open PMC copy returns body parts through both forms; a PMID with no open copy returns a clear `not-open` reply; the old refusal case now succeeds or returns the working call.
