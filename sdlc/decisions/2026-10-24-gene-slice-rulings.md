# Gene slice rulings

Date: 2026-10-24

Decided by Ian, approving the recommendations in `planning/slices/gene.md`.

- HGNC ships as a local data bundle and owns gene identity, names, previous symbols and withdrawn status.
- An alias that matches several genes returns ranked candidates with how each matched. BioMCP never picks one.
- Gene trials, articles and drugs move to their own entities' searches. The gene card points to them through next steps that carry the resolved gene. Retired 0.9 spellings return a usage error naming the new command.
- The 0.9 gene features the slice drops stay dropped: Enrichr, GO, STRING, funding, DisGeNET and KEGG.
- The default card carries the full RefSeq summary with its date.
- Positions are shown on both genome builds.
- Genes are human only.
- OncoKB is a source when the user supplies an OncoKB API key, under `2026-10-24-keyed-sources-run-only-with-the-user-s-key.md`.
