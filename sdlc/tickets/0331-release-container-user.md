# 0331 — Run Linux release containers as the calling user

Status: OPEN.

Milestone: 5

## Outcome

Every container the Linux release scripts start that writes into the checkout or a bind mount runs as the calling user's uid and gid, so `dist`, `target` and every other host-visible output stays writable by the host steps that follow.

- Why: hosted run 40522644935 passed build, install, runtime and native Intel macOS, then failed the Linux x86 schema step because cargo could not create `target/debug` (Permission denied) in a root-owned target. This is the third root-ownership failure after `dist` in 0316 and the report in 0315, so the class is fixed at its cause instead of path by path. PM ruling 2026-10-27 (m8126 ask 70).
- Starts from: `release/linux.sh`, `release/matrix.sh`, `release.yml`, the 0313 container argument setting and the 0316 fixture.
- Rules: containers run with the caller's uid and gid and a writable home such as `HOME=/tmp`; a step that truly needs root inside the image, such as package installation into the image, writes nothing to bind mounts. The 0316 directory precreation stays.
- Proof: an outside-in fixture runs each container step, then host steps create files under `dist` and `target` as the caller; a check lists no root-owned file in the checkout after the run.
- Changes: release scripts, the workflow and the fixture.
- Defers: nothing.
