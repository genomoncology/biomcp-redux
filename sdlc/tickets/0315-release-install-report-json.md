# 0315 — Keep the Linux install gate report as complete JSON

Status: OPEN.

Milestone: 5

## Outcome

`release/linux.sh` writes the install gate's whole report to `dist/install-report.json` as valid JSON holding every reported row, on a path the caller can read after the container exits.

- Why: 0293 ARM64 job j16558 passed every installed channel, but the script appends the pretty-printed report to `install-report.log` and keeps only `tail -n1`, so `dist/install-report.json` held two bytes, the closing bracket and a newline. The product is unaffected; the release report is not trustworthy until fixed. PM ruling 2026-10-27 (m8126 ask 56).
- Starts from: `release/linux.sh`, `release/gate.py`, the j16558 evidence and the 0293 rules.
- Rules: the gate writes its report straight to the output file; no parser picks lines out of a log.
- Proof: a fixture with multi-line pretty JSON produces a valid report whose rows equal the gate's rows, and the report survives the container's removal.
- Changes: release scripts and a fixture.
- Defers: nothing.
