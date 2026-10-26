# 0200 — Commit the source capture and evidence recipe as one checked script

Status: OPEN.

Milestone: 1

Depends on: none

## Outcome

Delegates stop rebuilding capture and evidence recipes by hand. One committed script runs a bounded source capture plan and its evidence lifecycle, and `make source-capture` runs it. It is the correction 0060 promised after the setup and state baseline recurred: 8 setup or state failures and 2 default-selector failures, most recently 0190 j14211 (receipt list uninitialized after the first retained response, 72.1 s) and 0061 j14243 (unsupported Python `extractall(filter=...)` before any product step, 89.3 s).

- Starts from: the last successful investigation recipes (0190 j14221 continuation, 0191, 0192 j14227, 0193 and 0194 jobs), `scripts/capture_outcome.py` for reply classification, and the installed-tar extraction recipe named in 0060.
- Keeps: the bounds every investigation already used: one attempt, no redirects or retries, a per-reply deadline and byte bound, request spacing, stop on 429 or 5xx, keys inherited by name only and never recorded, raw bodies outside Git, and archive verification before scratch cleanup.
- Changes: `scripts/source_capture.py`, `scripts/test_source_capture_plan.py`, `Makefile`, `sdlc/planning/slices.md` (one line naming the script in the investigation method).
- The script takes a JSON plan of requests and an absolute output directory that must not exist yet. It initializes all receipt state before the first request. It writes each body, headers and receipt as it arrives, so a stopped run can continue by sending only undelivered requests. It writes a manifest with SHA-256 per member, extracts and verifies archives with the installed `tar`, and refuses to start when required OS variables such as `TMPDIR` are missing. Omitted selectors come from the plan, never from an assumed default.
- Proof: offline tests against a local stub server cover a full run, a stop on 429, a continuation that sends only the remaining requests, an existing output directory refused before any request, a missing OS variable refused before any request, and manifest verification failing on one changed byte. `make source-capture` with a one-request plan against the stub passes in a job.
- Defers: product replay recording, which keeps its own retained recipe, and any generic framework or shared rule.

After landing, assignments name `make source-capture PLAN=... OUT=...` for every investigation or live capture, and 0060 counts any hand-built capture recipe failure against a target of 0.
