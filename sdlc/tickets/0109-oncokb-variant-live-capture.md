# 0109 — Capture a real keyed OncoKB variant annotation

Status: OPEN.

Milestone: 2

Depends on: 0095

## Outcome

The approved OncoKB demo capture verifies the BRAF V600E reply shape and satisfies the variant source roadmap box. The product requires the caller's configured key and has no demo fallback.

- Proof: j11262 ran from landed main `3d8901f66e57a1aa7c86da75bfb4e206c5b91f06`. One keyless demo GET returned HTTP 200 and 23,634 bytes. Its native query states `7:g.140453136A>T`, `GRCh37`, `Melanoma`, `BRAF` and `V600E`.
- External case: the actual CLI recorder and offline replay both returned the OncoKB section as data using the unchanged demo body delivered through local TLS with a synthetic local credential. Missing-key replay returned `skipped/no_key` with no additional HTTP request. The recorded production-host identity describes local delivery, not a production capture; separate provenance retains the original demo URL.
- Evidence: Yellow `data/biomcp-redux/0109-oncokb-live/biomcp-redux-0109-DEMO-pcpcmsi6`, with all 13 manifest files verified. Archive SHA256 `031e83768d94081c24dc705f0a6ef68ff1c5c033a4d820741cee5bb33afba1ec`; body SHA256 `5c9db76d0d9c2421f8680c2b62d1d3578d93d27fe4f04b4cab73f39d3631e6ab`. Raw replies and the replay case remain outside Git.
- Job limit: j11262 exited 1 in a receipt-only SSH quoting command after capture, replay and transfer succeeded. Independent transfer verification succeeded. No provider retry or credential inspection occurred. This establishes the approved demo proof and makes no keyed production claim.
