# 0313 — Let the caller supply container labels to the Linux release build

Status: COMPLETE.
Landed: 718a4d70aea2fa5b27659495b121afd53123a0c0

Milestone: 5

## Outcome

`release/linux.sh` takes the container labels from its caller and passes them to every container it starts, for build, install and the runtime `container_check.py` run, and it hardcodes no labels.

- Why: preparing 0293 found that `release/linux.sh` lines 7, 25, 52 and 53 hardcode `--label pm.repo=biomcp --label pm.ticket=0289` and ignore every caller setting, so a local rehearsal cannot label its containers with its own ticket. The literals also name internal tooling in a public script. PM ruling 2026-10-27 (m8126 ask 54).
- Starts from: `release/linux.sh`, `release/container_check.py`, the Makefile target that forwards `CONTAINER_ARGS` to `matrix.sh`, and the 0289 workflow.
- Rules: one generic setting, such as `CONTAINER_ARGS` as `matrix.sh` already uses, carries the extra container arguments; with none supplied the script adds no labels and the release workflow behaves as before. No internal tool name stays in the script.
- Proof: a fixture run with supplied labels shows every container invocation receives them, and a run with none shows the existing release commands unchanged.
- Changes: `release/linux.sh`, its callers and a fixture.
- Defers: nothing.
