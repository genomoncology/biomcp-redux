Synthetic. Semantic Scholar refuses the `pdf` lookup for PMID 28944238 with HTTP 429 and a ten-minute Retry-After, an authored failure with no Semantic Scholar data, as the msl persona run of 2026-11-16 met live (`semantic-scholar failed rate-limited`). The PubMed EFetch reply was recorded live in pm job j42064. The three PMC OA bucket replies copy `../../fulltext/synthetic/pmc-oa/`, whose JATS is invented; the bucket record's `license_code` is changed from `CC BY` to `TDM` and `is_manuscript` to true, so the case shows the meaning of an author manuscript's text-mining license without committing any text under that license.

The preprint step reads PPR1295425, whose Europe PMC core record is copied unchanged from `../../preprint/live/europepmc/` (pm job j42094 recorded the same request live); its Semantic Scholar lookup by DOI gets the same authored HTTP 429.

Synthetic: semantic-scholar
