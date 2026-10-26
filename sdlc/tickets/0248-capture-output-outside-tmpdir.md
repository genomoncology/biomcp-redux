# 0248 — Refuse capture output under the job's TMPDIR

Status: COMPLETE.
Landed: e7dc6ceb98e1fd052984104fbf8a243ef3189cde

Milestone: 1

Depends on: none

## Outcome

`scripts/source_capture.py` refuses an output directory that lies under the job's `TMPDIR` before it sends any request, so a verified capture can never vanish when the job's temporary folder is cleaned at exit. 0213 j14506 lost its only archive this way after one successful request.

- Changes: `scripts/source_capture.py` and `scripts/test_source_capture_plan.py`.
- Fix: resolve both paths and refuse the run with a message naming a durable location when the output directory is `TMPDIR` or under it.
- Keeps: every existing check and the continuation behavior.
- Proof: an offline test with output under a fake `TMPDIR` is refused before the stub server sees a request, and a test with output outside it still runs. Then make landing-gate runs through pm job.
