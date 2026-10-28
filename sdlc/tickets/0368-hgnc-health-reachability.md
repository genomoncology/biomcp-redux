# 0368 — Probe HGNC reachability in health without a download

Status: COMPLETE.
Landed: b2c76b843df962c3174b57ea4cf70adf6571ea74

Milestone: 5

Depends on: 0342

## Outcome

`biomcp health` reports HGNC publisher reachability through one small routed request, such as a bounded HEAD request on the bundle object, alongside the local bundle state that 0342 reports.

- Why: 0342 reports HGNC reachability as unavailable because no small routed probe exists (PM ruling m10259). Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: 0342's HGNC health entry, Fetch request methods and the replay format.
- Rules: no bundle download, no proxy bypass. A new request method is recorded and replayed like the others.
- Proof: replay cases for reachable and unreachable HGNC; the probe moves no bundle bytes.
- Changes: Fetch request method support, the HGNC health probe and cases.
- Defers: nothing.
