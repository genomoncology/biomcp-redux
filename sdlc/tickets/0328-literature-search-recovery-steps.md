# 0328 — Show when a literature search lost its variant words, and offer recovery steps

Status: OPEN.

Milestone: 5

## Outcome

When PubMed drops supplied words from a search, the reply puts that notice first, before the article list, and offers a quoted search as a next step. A LitVar protein-name lookup that finds nothing offers rsID and alternate-name next steps. An article whose body cannot be searched offers a next step that reads its abstract.

- Why: experiment 497 (ticket 0325, evidence on the evidence store under 0325, PM evaluation 2026-10-27 for Jarvis m9494) found PubMed searches that silently widened to the whole gene (848 to 10,377 hits) while the reply still read as complete, a CDH1 protein-name lookup that returned absent although the variant's rsID found four papers, and body search replies of `not_applicable` that a careless agent could read as no evidence while the abstract held the variant.
- Starts from: the PubMed adapter's `not_found_words` notice, the article search Markdown and JSON order, the LitVar adapter's absent outcome, the article text `not_applicable` outcome, and the next step builders.
- Rules: BioMCP keeps searching exactly what was asked and never broadens or rewrites a query itself. Next steps come only from identifiers already in the reply or the request, such as an rsID the variant record already gives, and never from invented equivalences or historical names. No clinical judgment.
- Proof: replay cases for a PubMed reply with dropped words, a LitVar absent protein lookup with a known rsID and an article with no searchable body show the notice first and the right next steps; help and size caps pass.
- Changes: article search output order, next steps and cases.
- Defers: agent guide advice to union sources and try verified historical names, which waits for the docs branch the docs lead is writing.
