# Variant search fixtures

The request metadata and filenames were emitted by the actual CLI with `--record`, in a new empty directory for every invocation. Their method, URL, source and identity remain unchanged.

Response bodies are synthetic compact projections using public source identity facts. They keep only genomic record keys, rsIDs, gene symbols and numeric ClinVar IDs. They contain no source annotation assertions or licensed text. They preserve the recorded source total and native row order. The complete source bodies stay outside Git because component redistribution rights are not established.

The nine protein cases and the raw rsID case reuse authoritative responses captured on 2026-10-24 during the variant investigation. A local TLS server delivered those preserved responses to the CLI recorder. This proves the command and request mapping against the preserved source answers; it is not a second public capture. The region case uses one actual public CLI capture from the same day. It preserves the first two candidates of a four-hit source page without inventing the unseen page. Synthetic local wire cases separately prove a complete two-page walk and warm cache reuse.

The complete recordings, original request metadata, capture script, certificate setup failure, corrected CA/leaf recipe, output sizes and checksums are retained on the external evidence store at `data/biomcp-redux/0088-source-captures/`. Output ceilings measure each complete captured envelope and allow fifteen percent growth. The region and raw forms have no comparable historical spelling; the protein ceilings remain below their historical whole-card outputs.
