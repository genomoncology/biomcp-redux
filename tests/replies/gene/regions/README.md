# ClinGen region recordings

The ClinGen region curation list, https://ftp.clinicalgenome.org/ClinGen_region_curation_list_GRCh38.tsv, was recorded whole through the transport in job j39307 on 2026-11-13: 98,881 bytes, five `#` banner lines and 513 ISCA rows, unchanged. The same recording sits in ../main/clingen, so band searches there read it, and in ../clingen/clingen, so `sync clingen` reads it. ClinGen releases curated content under CC0 1.0.

The HGNC files are copies of the reviewed slices in ../main/hgnc, and the complete-set slice adds every row whose location starts with 16p11.2: 245 rows, taken unchanged from HGNC's complete set recorded through the transport in job j39834 on 2026-11-13 (45,396 rows, same 53 columns as the 2026-10-24 recording). The other rows and the exchange metadata stay those of the 2026-10-24 recording. So `search gene --band 16p11.2` returns gene rows and regions together, as the rare-disease agent saw.
