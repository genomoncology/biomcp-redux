# Source transcript replay

`myvariant-hsd17b4.json` contains the exact response body from one public MyVariant annotation GET on 2026-10-27. Its SHA-256 is `e881312674468ed0d961fad7c9e12131287d2912c1144b2fee11fa5ea9250138`.

The request was `https://myvariant.info/v1/variant/chr5%3Ag.118860951A%3EG?fields=_id%2Cdbsnp.rsid%2Cdbsnp.gene.symbol%2Cclinvar.variant_id%2Csnpeff.ann&assembly=hg19`. It returned HTTP 200. The response retains the publisher's component license links.

The replay keeps each source transcript identifier, coding notation and protein notation together in source order. The search replies wrap that captured record in synthetic one-hit pages. They prove product mapping and matching through the CLI and MCP. They do not claim a current live search result or supply absent source assembly metadata. The coding-match TLS test uses a synthetic resolver identity and source transcript rows from this record.
