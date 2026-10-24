# CIViC recordings

CIViC VSIR, ERBB2 and TP53 replies were captured through the shared transport on 2026-10-24 in job j9058. The same job captured the missing VSIR HGNC REST and MyGene identities. The HGNC published-file fixture reuses existing ERBB2, TP53 and AAVS1 rows and adds the VSIR REST fields verbatim, joining lists with pipes. This trimmed bundle is synthetic; source spelling and complete descriptions are retained. Other MyGene identities reuse existing captures.

Synthetic siblings mutate only the named CIViC envelope or identity. HTTP failures supply each retry exchange and offline follow-ups prove failed raw replies never enter the cache. Null and mismatched Entrez replies are synthetic. A no-join case uses the existing AAVS1 HGNC row without any CIViC recording.
