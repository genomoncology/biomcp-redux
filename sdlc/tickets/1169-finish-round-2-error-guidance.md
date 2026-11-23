# 1169 — finish-round-2-error-guidance

Status: OPEN.

Milestone: 7

## Outcome

The three round-2 guidance items that ticket 1168 left partly done are finished: every unknown-option refusal names the rejected option in its message; a source that fails with `deadline-exceeded` or `unavailable` offers a next call on every transport, including hosted HTTP where `--fresh` is not allowed; and an Europe PMC search failure offers the same search on PubMed.

## Why

Round 2 eval (biomcp-code-mode-redux branch wip/pilot-b, runs/round-2-sol.md; batches r2-apb-{l,b,o}, per-run call logs): Europe PMC search failed all 42 times it was tried (15 s source deadline; Europe PMC took 9 to 13 s per quoted search even when idle), each with `next: []`, and agents went to Europe PMC directly about 40 times. Ticket 1168 scoped the unknown-option wording to HGVS because three tests (`usage-unknown-option`, `usage-names-the-cause`, `typed-refusals-name-the-cause`) assert the old wording "unknown option: this command has no error.option; run next, or use an option valid_values lists"; that wording is the defect, so those assertions change with it (PM ruling, 2026-11-23). Ticket 1168 offered retries only in typed stdio.

## Proof

- Proof: replays: an unknown option on three commands names that option; a Europe PMC deadline on `search article` offers the PubMed search with the same terms; a source `unavailable` over hosted HTTP offers a next call that HTTP accepts. Source deadlines and other limits stay as they are.

## Changes

- Changes: usage error wording and its three tests, failed-source next calls, and replay cases.
