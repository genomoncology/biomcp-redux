# 0163 — Capture the pharmacogenomics policy text, order, drug filter and single annotation witnesses

Status: OPEN.

Milestone: 4

Depends on: 0153

## Outcome

One bounded keyless pm job captures the four witnesses that `planning/slices/pharmacogenomics.md` and the 0153 amendments require before the pharmacogenomics types are accepted, and the design records each one as proven, empty or failed. The witnesses are: the current ClinPGx data use policy text as text, not a JavaScript shell; a CPIC `order=` witness on a native unique column for two consecutive pages of `pair_view` for DPYD and of `recommendation_view` for DPYD, proving a stable page walk; a native `drugid` filter on CPIC recommendation records for CYP2D6 with tamoxifen `RxNorm:10324`, on whichever of `recommendation` or `recommendation_view` carries that column; and one ClinPGx single-annotation base get for PA166134421, with the returned accession compared to the requested one. Raw replies, headers, the ledger and the recipe go to the evidence store, and no product code changes.

- Why: the slice method requires each kept source's exact-request captures before its shared types are accepted, and the 0153 review left these four as proof obligations: policy text for the `noncommercial` label, deterministic order for paging, the drug filter for tamoxifen questions Q18 to Q23 over 1,316 CYP2D6 rows, and the single get behind the disambiguate step.
- Starts from: pharmacogenomics.md's bounded capture plan, source findings and deferred selectors, with the 0153 PM review rulings and recheck amendments, which win over the design text; the j12644 capture and its M01 to M06 metadata rows; the CPIC OpenAPI document in M06 for the `drugid` and `order` columns.
- Rules: the recipe names each exact URL before the job runs, sends each row once in order with one second between admissions, a 20-second deadline and a 4 MiB body bound, and makes at most eight physical requests. No retry, redirect, altered query, fallback host or extra page. A 401, 403 or 429 stops that source. No credential, cookie, script crawl or whole-database download. A failed or empty witness keeps its capability deferred: without order, paging stays unproven for a stable walk; without the drug filter, the recommendations section ships gene-keyed with no drug selector and tamoxifen questions need paging; without the base get, the single annotation get and its disambiguate step stay deferred; without policy text, the `noncommercial` label stands on the PM ruling with that reason stated. No clinical answer is calculated from a capture.
- Proof: the job ledger records every attempt with its exact URL, status, bytes, SHA-256 and timing, and the archive path and hash appear in pharmacogenomics.md beside each witness's result, and through pm progress. A reviewer checks that each witness's result names the capability it unlocks or defers. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
