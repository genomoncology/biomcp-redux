# 0272 — Diagnose the MCP broken-output watchdog failure

Status: OPEN.

Milestone: 1

## Outcome

A bounded diagnosis names the cause of the `mcp/broken-output-with-open-input` failure in j15216, or states that the preserved evidence cannot name it and what one added witness would. A witnessed product or test-isolation defect gets a fix in this ticket with a deterministic case that fails without it.

- Why: the case failed in j15216 and the unchanged instrumented case passed in j15223, so the stdio watchdog may have a timing defect that a passing rerun hides.
- Starts from: the case in `crates/biomcp/tests/runner/protocol.rs`, the stdio watchdog in the MCP server, and the Yellow asset `0259-cell-line-diagnostics/mcp-broken-output-j15216.tar.gz` (SHA-256 `fe0d22e819126907713400ffcb4ab0b9aa9ad90008c65aad87f012f686b57dad`) with the original failure, inputs, source hashes, the exact pass and stdout, stderr and timing.
- Keeps: the watchdog's exit on broken output, the case's assertions and the accepted cell line work. No blind retry loop, timeout inflation or new framework.
- Changes: the watchdog code only if a product defect is witnessed, and the case for added witnesses or a deterministic reproduction.
- Proof: cite j15216 and j15223. If a cause is found, a deterministic case reproduces it before the fix and passes after it. If not, the case keeps added status and timing retention so the next failure carries its witness, and the ticket records that limit.
- Defers: nothing.
