# 0334 — Keep PubMed's phrase warnings and suggest a retry that keeps the variant

Status: COMPLETE.
Landed: 0a4dd8835ad5269c28447c9e8f3d1028440c2f3a

Milestone: 5

## Outcome

Every warning in PubMed's ESearch `warninglist`, including `quotedphrasesnotfound` and `phrasesignored`, reaches the reply as a source notice and appears first in Markdown. When PubMed drops variant words, the suggested retry searches the gene and the variant as separate title and abstract terms, such as `ERCC2[Title/Abstract] AND Ala717Gly[Title/Abstract]`, never the quoted whole phrase.

- Why: the experiment 497 rerun on release/1.0 at ff2e9b64 (Jarvis m9946, evidence on the evidence store, PM ruling 2026-10-27) found that all five suggested quoted retries became gene-only searches (848 to 10,382 hits) while BioMCP dropped PubMed's `quotedphrasesnotfound` warning and reported `complete`, so an agent could not tell, and the five retries returned 29,129 tokens of gene-only results. A field-tagged control kept the variant and honestly returned zero.
- Starts from: the PubMed adapter's ESearch reply parsing and notices, the 0328 recovery next step, and the five public sweep inputs in the rerun evidence.
- Rules: notices carry PubMed's own warning names and phrases unchanged. The retry uses only words from the request and runs on PubMed explicitly. A zero result from the strict retry is reported as zero for that query, not as absence of evidence. No automatic retry and no query rewriting by BioMCP.
- Proof: replay cases from the five inputs show the warning notices first and a strict retry step; a case for the strict retry keeps the variant term and reports zero; caps pass.
- Changes: the PubMed adapter notices, the recovery step and cases.
- Defers: nothing.
