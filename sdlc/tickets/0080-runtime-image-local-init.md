# 0080 — Make the runtime image initialize before replay

Status: OPEN.

Milestone: 1

## Outcome

The produced native ARM runtime image initializes its owned local state and executes recorded commands, concurrent processes and HTTP checks using the selected candidate and read-only recordings.

- Why: 0066 job j10304 passed installed ARM channel proofs but the runtime image's first KRAS replay exited 5. Bounded runtime-only job j10320 reproduced a `biomcp.error/1` local failure before replay with empty stderr. The same candidate passed outside that image.
- Starts from: exact candidate head 353b890 and its retained ARM artifacts, provenance and runtime witness on Yellow `data/biomcp-redux/0066-arm-final/`, including `runtime-witness.json`. The witness mounted readable recordings under `/proof/fixtures/replies/gene/main` and supplied isolated home/config/cache and writable `/tmp/diag/case-cache`.
- Changes: diagnose the concrete local-initialization failure and fix its owning runtime image or installed-check setup. Preserve the public bounded error response; temporary diagnosis must not become a logging framework or expose secrets and host paths. Change product initialization only if evidence proves its defect rather than a missing image requirement.
- Keeps: byte-identical candidate selection, read-only fixture confinement, isolated writable local state, source policy and TLS checks. Do not relax confinement or source verification to make the proof pass. The five successful ARM channel paths stay proved.
- Proof: reproduce the exact failing recorded command inside the produced image without source traffic, then prove it succeeds and continue the existing process and HTTP checks. Reuse retained candidate bytes during diagnosis and run only runtime-image checks; rebuild product only when its implementation changes. Run routine checks through pm jobs. Clean owned images and scratch after evidence retention.
- Defers: Windows capability, the Homebrew tap fix and final aggregate platform completion, which remain with their owners. No public source refresh or package publication.
