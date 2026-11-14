# ClinVar gene counts after an NCBI throttle reply

The ClinVar and MyGene exchanges were recorded live through the transport in j40858 on 2026-11-14 with `get gene BRCA1 clinvar --record`, with no NCBI key, while a second client on the same address sent keyless E-utilities searches. NCBI answered the likely pathogenic count search with HTTP 429 and `Retry-After: 2`; the section waited the two seconds and the same search then succeeded. ClinVar was Build261108-1750.3 (last updated 2026/11/09 21:36).

The 429 body named the recording machine's address as its `api-key`; that address is replaced with the documentation address `2001:db8::1`. Everything else is unchanged.

The HGNC exchange is the BRCA1 slice from ../clinvar/hgnc, which its README describes.
