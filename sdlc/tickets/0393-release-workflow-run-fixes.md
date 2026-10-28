# 0393 — Make the release workflow run and prove what it claims

Status: OPEN.

Milestone: 5

## Outcome

The release workflow runs on all five hosted legs and qualifies its own candidate. Every leg calls the Python that exists on its runner, so the Windows leg starts. The x86_64 Linux leg runs fmt, clippy and the test suite, like the landing gate. The installed gate checks that `version --verbose` reports the candidate commit, and local recipes refuse a dirty tree unless a revision is given. The protection check also rejects `can_admins_bypass: true`. The verify instructions pin `--source-ref refs/tags/vX.Y.Z`, and the operator summary states what the job checked, not an approval it cannot see. The rehearsal summary names the protection and operator steps it skipped. The staged two-architecture Homebrew formula is installed in the macOS leg, and the plugin manifest carries the version. The installer refuses an unset or empty HOME with a clear message when no install directory is given.

- Why: pre-release review `sdlc/records/2026-10-28-pre-release-review.md`, release findings 1, 4, 7, 9, 10, 11 and 13. The workflow at this head has never run on hosted runners.
- Starts from: `.github/workflows/release.yml`, `release/workflow.py`, `release/gate.py`, `release/install.sh`, `release/matrix.sh`, `release/linux.sh` and the staging code.
- Rules: no publication; no new framework; native installed proof stays separate from Linux containers.
- Proof: workflow checks show each changed step fails on a wrong build: a wrong commit, a dirty tree, an admin-bypass protection reply, a missing plugin version and an unset HOME each fail. The 0336 hosted run passes all five legs.
- Changes: the files above and their tests.
- Defers: moving PyPI and GHCR publishing into the protected environment job, which needs Ian.
