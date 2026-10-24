# Protein case recordings

Job j9066 captured the UniProt accession requests for CDKN2A and TP53 and the missing MyGene CDKN2A identity through shared transport on 2026-10-24. Complete function paragraphs are preserved. HGNC and MIR21 MyGene replies reuse gene/constraint captures. The HGNC published-file fixture is trimmed to CDKN2A and MIR21. The all case reuses the TP53 UniProt capture.

The first capture attempt failed before saving a reply because UniProt served two gzip layers with one Content-Encoding header. A bounded wire probe on the build machine confirmed that one decompression left gzip magic and a second exposed JSON. An identity-encoding request returned plain JSON. The endpoint now declares identity encoding; other endpoints retain their existing behavior. No repeated decompression is implemented.

Every synthetic-* directory is an edited fixture, not an observed provider failure. Failure fixtures mutate only UniProt status, delay or body and provide every retry exchange. Mapping fixtures reorder results, duplicate or omit an accession, change taxon or review status, or insert an invalid optional function text. The owning cases retain valid sibling proteins and complete valid prose. Failure/offline pairs preserve successful identity and MyGene cache entries while proving rejected UniProt replies are not cached.
