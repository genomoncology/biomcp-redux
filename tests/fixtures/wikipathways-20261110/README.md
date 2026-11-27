# WikiPathways human gene census

The fixture covers the 1,131 human pathway files in WikiPathways release 20261110. WikiPathways data is distributed under its [CC0 waiver](https://sandbox.wikipathways.org/terms.html).

- `pathways.jsonl` contains one tuple per pathway. It keeps the GPML header fields, DataNode types and IDs, and native Xrefs from the [human GPML archive](https://data.wikipathways.org/20261110/gpml/wikipathways-20261110-gpml-Homo_sapiens.zip). It encodes the matching revision's `wp:bdbEntrezGene` cross-references from the [WikiPathways RDF archive](https://data.wikipathways.org/20261110/rdf/wikipathways-20261110-rdf-wp.zip) in the fields read from the pathway JSON export. The CLI cases separately replay the publisher's actual JSON fields.
- `gmt.tsv` keeps the pathway IDs and deduplicated NCBI Gene IDs from the [human GMT](https://data.wikipathways.org/20261110/gmt/wikipathways-20261110-gmt-Homo_sapiens.gmt), independent of the RDF cross-references.
- `hgnc.tsv` keeps the HGNC fields used to resolve and compare genes. It holds rows matching the GPML references, RDF cross-references or GMT IDs, from the [HGNC complete set](https://storage.googleapis.com/public-download-files/hgnc/tsv/tsv/hgnc_complete_set.txt) captured on 2026-11-27. [HGNC terms](https://www.genenames.org/about/terms-of-use/)

| Original download | SHA-256 |
| --- | --- |
| GPML | `400d67f51f951681ab601503380f48eabdb5582fb23a8b4e7a0695ecc7a2b925` |
| RDF | `06ab0834c2973952386f6b8c64e844760326f9bb98d2cc268ae09f210fe0f481` |
| GMT | `c41856b2ee245cfbd0b394f5f0be1db9bb6dbebabdd03eaa8de6c8bf2b6315d7` |
| HGNC | `46b83736ec351f15e22331575dce2f08416c5233765b26a08c65e6b42e56b651` |
