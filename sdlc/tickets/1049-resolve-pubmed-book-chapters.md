# 1049 — Resolve known PubMed book chapters to their Bookshelf links

Status: OPEN.

Milestone: 7

## Outcome

`get article 21882399` resolves to the exact Kabuki Syndrome GeneReviews chapter and includes its NCBI Bookshelf link, giving callers the publisher's genetic counseling and family support resource sections through one known PMID.

## Why

The rare-disease Q9 runs need a counseling reference and a way to find other families. On November 17 the BioMCP build at 31d7ecd75 reported `absent` and PubMed `ok` for PMID 21882399. [PubMed EFetch](https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=pubmed&id=21882399&retmode=xml) returns that PMID under `PubmedBookArticle/BookDocument` with the title and `ArticleId IdType="bookaccession"` of NBK62111. The [official chapter](https://www.ncbi.nlm.nih.gov/books/NBK62111/) binds the same accession and PMID and has Genetic Counseling and Resources sections, including All Things Kabuki and the Kabuki Syndrome Foundation. [Europe PMC's indexed record](https://www.ebi.ac.uk/europepmc/webservices/rest/search?query=EXT_ID:21882399%20AND%20SRC:MED&format=json&resultType=core) independently supplies the same NCBI Bookshelf URL.

## Starts from

- Starts from: `pubmed/efetch.rs` enumerates `PubmedArticle` only. `article/get.rs` validates that the returned record has the requested PMID. `article_rights/pubmed.rs` currently recognizes only journal-article containers when it applies record text policy.
- PM decision (2026-11-17): extend the existing PubMed exact-ID path using the same response. Map citation facts from the native book record and form the documented Bookshelf link only from a validated source `bookaccession`; distinguish the book title from a journal. Keep book chapter abstracts and full text out of the new path, including retained response bodies and replay fixtures. [GeneReviews terms](https://www.ncbi.nlm.nih.gov/books/NBK1116/?report=reader) limit reproduction to specified uses. This change provides a citation and link.

## Proof

- Proof: replay the minimal EFetch response for PMID 21882399 through `get article` and assert the exact PMID, source title and NBK62111 link. A mixed book-and-journal EFetch batch preserves caller order and existing journal answers. A rights-policy control asserts chapter text is absent from output and retained responses. Missing or unvalidated accessions produce no Bookshelf URLs.

## Keeps

- Keeps: PMID identity checks, missing/failed distinctions, journal article rights and results, and PubMed request limits.

## Defers

- Defers: new Bookshelf source, chapter text or organization extraction, new identifier syntax, and changes to article search.

## Changes

- Changes: `crates/biomcp-adapters/src/pubmed/efetch.rs`, `crates/biomcp-adapters/src/article_rights/pubmed.rs`, the article card response and schema, and article get replay cases.
