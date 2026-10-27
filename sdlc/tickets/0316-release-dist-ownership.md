# 0316 — Create the Linux release output directory before the container mounts it

Status: OPEN.

Milestone: 5

## Outcome

`release/linux.sh` creates `dist` as the caller before the first container bind mount, so later host writes such as `docker save -o dist/biomcp-container-$arch.tar` succeed.

- Why: 0293 x86 job j16559 found `dist` created by Docker as root with mode 0755, and the host could not write into it. The 0315 reviewer showed that the later host archive write at `release/linux.sh` line 51 would hit the same wall. PM ruling 2026-10-27 (m8126 ask 57).
- Starts from: `release/linux.sh`, the host cache directory it already creates, and the 0315 fixture.
- Rules: the caller creates `dist` beside the existing host cache directory; no permission change inside the container.
- Proof: the fixture runs a container step with default Docker ownership, then a host archive write into `dist` succeeds.
- Changes: `release/linux.sh` and the fixture.
- Defers: nothing.
