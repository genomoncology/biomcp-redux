# 0335 — Offer recovery steps when a LitVar lookup returns nothing

Status: COMPLETE.
Landed: 78713523ad7397212ff80f60d152e4ef275a117b

Milestone: 5

## Outcome

When a LitVar lookup returns no variant and no candidates, the reply offers next steps that search Europe PMC and PubMed with the same gene and variant text, and says that an rsID, if the user has one, can be looked up in LitVar.

- Why: the experiment 497 rerun on release/1.0 at ff2e9b64 (Jarvis m9946, evidence on the evidence store, PM ruling 2026-10-27) found that `--gene CDH1 --variant D882N --source litvar` still returns absent with no next step, although the variant's rsID finds four papers and Europe PMC finds the reference papers. 0328 covered LitVar replies with candidates but not the empty case.
- Starts from: the LitVar adapter's absent outcome, the 0328 candidate steps, and the article search next step builders.
- Rules: steps use only the request's own gene and variant text; BioMCP does not guess an rsID or alias. Disabled sources are left out, as 0327 does.
- Proof: a replay case for an empty LitVar lookup shows the Europe PMC and PubMed steps and the rsID hint; a case with Europe PMC disabled leaves its step out.
- Changes: LitVar next steps and cases.
- Defers: nothing.
