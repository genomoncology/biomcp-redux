# ClinGen region recordings

The ClinGen region curation list, https://ftp.clinicalgenome.org/ClinGen_region_curation_list_GRCh38.tsv, was recorded whole through the transport in job j39307 on 2026-11-13: 98,881 bytes, five `#` banner lines and 513 ISCA rows, unchanged. The same recording sits in ../main/clingen, so band searches there read it, and in ../clingen/clingen, so `sync clingen` reads it. ClinGen releases curated content under CC0 1.0.

The HGNC files are copies of the reviewed slices in ../main/hgnc. They hold no 16p11.2 genes, so the band step returns regions without genes.
