# 0180 — Keep drift witnesses from the nightly live check

Status: OPEN.

Milestone: 3

Depends on: 0155

## Outcome

When the nightly live check reports a drifted or errored case, the evidence it copies to the evidence store holds enough to diagnose that case without another provider run: each drifted request's sanitized URL, status, the result identities, the response shape and the product output beside the recorded baseline. Cases without drift keep only today's summary fields.

- Why: the first reviewed report, j13095 from eb9bb25c, found 11 drifted cases and no diagnostic errors. It kept request hashes and statuses but lost URLs, result IDs, response shape and product output when the temporary observations were deleted, so no drift could be classed as a source change or a product bug.
- Starts from: 0155's `make nightly-live-check`, the live-check report and its temporary observations, and the j13095 report in Yellow data/biomcp-redux/0068-scheduled/20261025T092441Z-t5e82dfq (report SHA256 a472e5790aac4eb73379d3b78994d867526e43ef12084a6fc3a36ef56ab278ad).
- Rules: keep witnesses only for drifted or errored cases. Strip keys, tokens, credentials and key-bearing query parameters from URLs and headers before anything is written; never write a key value. Keep no reply body whose source terms bar storage; keep its shape and identities only. Witnesses go in the run's existing evidence folder and manifest, and M5 scratch is still deleted. Total witness size has a stated bound, and a case that exceeds it records that it was cut. No refresh of baselines, no recording writes and no whole-run retry.
- Proof: outside-in offline cases first, with a fake report and a local stand-in for the evidence store: a drifted case keeps its sanitized URL, status, identities, shape and output; a synthetic key in a URL and a header never reaches the evidence; an undrifted case keeps no witness; the size bound cuts and says so. No provider call. Then make landing-gate runs through pm job. After landing, the next scheduled report is read and its drifted cases are diagnosed from the kept witnesses, each source defect or product bug filed as its own ticket. The GTR bundle failure is already 0179 and is not filed again.
