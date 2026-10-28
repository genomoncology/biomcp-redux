# 0396 — Link the Windows C runtime statically

Status: OPEN.

Milestone: 5

## Outcome

The Windows binary links the C runtime statically, so it runs on a clean Windows machine without the Visual C++ redistributable. The install gate fails when the Windows binary imports `VCRUNTIME140.dll` or another redistributable DLL.

- Why: pre-release review `sdlc/records/2026-10-28-pre-release-review.md`, release finding 3. The hosted Windows runner has the redistributable installed, so the current gate cannot catch this.
- Starts from: the Cargo configuration, `release/artifacts.py` and `release/gate.py`.
- Rules: set `+crt-static` for `x86_64-pc-windows-msvc` only.
- Proof: a gate case fails on a PE import list containing `VCRUNTIME140.dll`; the 0336 hosted Windows leg passes the check.
- Changes: the Cargo configuration, the gate check and its case.
- Defers: nothing.
