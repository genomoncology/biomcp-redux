# Expression source recordings

Job j9060 captured the HPA CLDN18, MIR21 and TP53 identities, HGNC REST CLDN18 identity, and MyGene.info CLDN18 identity once through the shared transport on 2026-10-24. The TP53 HPA reply lives in gene/all/hpa. MIR21 returned HTTP 404. Job j9057 stopped before HTTP because the exclusive recording directory's parent did not exist.

The HGNC published-file slice reuses the captured MIR21 and AKAP2 rows. Its CLDN18 row is assembled from the retained HGNC REST capture by mapping existing header names, preserving scalars and joining list values with the published-file pipe separator. This assembled published-file fixture is synthetic. MyGene MIR21 reuses the existing constraint capture. Complete source prose and HPA tissue names are retained.

The synthetic-* directories alter only the stated HPA response. They provide every retry exchange and preserve successful identity and MyGene cache entries for later offline steps. They are synthetic failures, not observed provider incidents.

HPA data is attributed to the Human Protein Atlas, https://www.proteinatlas.org/. Its current license is CC BY 4.0, https://www.proteinatlas.org/about/licence, with separate conditions for third-party data. The RNA tissue units are nTPM; the cancer units are pTPM, as described at https://www.proteinatlas.org/humanproteome/tissue/data and https://www.proteinatlas.org/humanproteome/cancer/method. This gene-level summary makes no isoform or patient-assay claim.
