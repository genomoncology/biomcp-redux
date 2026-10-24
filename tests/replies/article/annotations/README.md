# Article annotation recordings

`live` records PubMed EFetch and PubTator3 abstract export for PMID33123537, a CC BY paper (https://doi.org/10.3389/fcell.2020.573599). `retracted` records PMID35061708, a PLOS CC BY paper (https://doi.org/10.1371/journal.pone.0261341). These are actual source replies captured once through the command.

Every `synthetic-*` directory contains edited or invented records for the named command boundary. The text retained in those replies comes from the CC BY captures. Synthetic annotation identities and names carry no clinical claims.

PubTator3's checked reply has numeric `gene_id`, string `identifier` and document `id`, and independent title/abstract annotations. The local `BATCH_LIMIT` bounds each GET export, including DOI expansions. NCBI's previous export API documents GET batches up to100 (https://www.ncbi.nlm.nih.gov/CBBresearch/Lu/Demo/PubTatorCentral/api.html). The application limit is deliberately narrower; no provider maximum or numeric rate limit is inferred from that earlier endpoint.

The original PMID17611497 example is captured and tested outside Git because its abstract is not CC BY.
