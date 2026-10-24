# 0066 — Finish installed platform and HTTP proof from landed main

Status: COMPLETE.
Landed: fe3b52274b32172579c38b3fdd72b9aa62208174

Milestone: 1

Depends on: 0058
Depends on: 0059
Depends on: 0073
Depends on: 0074

## Outcome

A build delegate completes the remaining installed-artifact matrix from landed main before milestone closure and repeats it before release. Code landing remains separate from platform availability.

- Why: 0058 delivers local producers and the gate; its ARM, macOS/Homebrew, Windows and HTTP execution remain unproved.
- Starts from: 0058's installed-artifact recipe and explicit platform limits; 0059's HTTP contract; machine roles and Gdux priority. Native Linux wheel, desktop, registry and installer passed j9758; x86 manylinux and amd64 runtime subchecks passed j9726. Do not repeat those unchanged proofs merely to recreate a report.
- Keeps: standard offline installation, installed executable selection without checkout fallback, candidate-byte provenance, isolated settings and caches, all-frame stdio and ordinary shared-cache concurrency proof. Publishing still needs Ian.
- Changes: the existing release gate only if a witnessed defect requires a repair; record job results through pm progress. Raw cited evidence follows the experiment storage rules.
- Proof: run ARM containers on a capable machine, macOS wheel and local Homebrew formula on M5, Windows on the existing Gator VM only when shared capacity allows, and installed HTTP after 0059. Preserve exact candidate identity and per-channel results. Platform preparation may start after 0058; the final candidate, installer, HTTP and interrupt runs wait for 0059, 0073 and 0074. Report unavailable capacity explicitly and keep milestone proof open. Turn defects into bug tickets without reopening landed code.
- Defers: publishing and external channel changes.

## Outcome and retained proof

The remaining installed-platform rehearsal is complete. This record reuses the executed checks below. It adds no build or source request. The final proof repair, 0101, landed with 0085 at `10d145977b0d3ae53e56dbfe22d666f07b792c1a`.

- macOS arm64: j10296 passed the wheel, desktop bundle, registry-selected wheel, supplied-archive installer and local-server download installer. Each checked candidate bytes, assets, version, all stdio frames, CLI/MCP replay, ordinary-cache concurrency and installed HTTP. That job later failed only at the formula invocation. j10390 passed the corrected local Homebrew tap against the same native candidate.
- Linux arm64: j10304 passed those five channels inside the declared manylinux floor. Its later runtime-container failure became 0080. j10395 passed the corrected runtime image, replay, ordinary-cache concurrency and all five installed HTTP cases against the retained ARM candidate.
- Windows x86_64: j10477 installed the wheel and passed assets, stdio, replay and ordinary-cache concurrency before the isolated HTTP runner failed. The 0099 runner repair passed all five installed HTTP cases in j10560. It also passed normal console shutdown with `CTRL_BREAK_EVENT` and startup without a console. j10592 passed desktop, registry and supplied-ZIP installation with candidate bytes, assets, stdio, replay, ordinary-cache concurrency and HTTP before its separate download setup failed. The 0100 repair passed the download/checksum/ZIP channel and isolation check in j10615.
- Windows key reload: j10645 actually executed and passed all three `configuration/key-reload` cases through stdio and HTTP against the retained installed executable. The 0101 repair gave the synthetic certificate authority a different subject from its leaf. Verification remained enabled. The product executable was unchanged. This closes the three TLS failures after the successful download row in j10615. j10600 selected no tests and supplies no proof.
- The earlier unchanged native Linux and x86 container checks remain the accepted starting evidence from j9758 and j9726. They were not repeated.

Candidate identities:

- macOS and ARM source: `353b8907b5256df61b06e1ada169e341f6d976f4`. macOS binary SHA-256: `d089cb427891fa134156d47897042c067a33986b6afd1ac0e9cbb9017ba00433`. ARM binary SHA-256: `83215a33d8b935b71d6ec76db47378148da94fe278ae5e46c56810f5422cc985`. The final formula and runtime recipes came from `879b2291d26cfb5b445d88d881e4572d0eefe6c6`.
- Windows source: `9bd46f799c1087e7fbc670267b617b62318a04ce`. Installed binary SHA-256: `b46fb8dd12be46380ea4e1ced75efdc877e0cc8fe31f10224868b894385baddc`. The final TLS runner SHA-256 is `d1258275b11724610426073d522b9aa34dde360ee1f0d1fec2ba9ea33404a285`. j10645 confirms the cached product executable hash before and after the run.

The evidence store under ticket 0066 holds the native, ARM and Windows artifacts, manifests and raw logs. Evidence under 0099, 0100 and 0101 holds the runner repairs and their native checks. Failed aggregate jobs above supply only the explicitly successful subchecks, not whole-job passes.

These checks qualify the named historical candidates and channel recipes. They do not claim a full platform build of the current main revision, native macOS x86_64 execution, installation inside a desktop client, or publication. The release plan still requires every promised native artifact and the installed rehearsal from the final approved release candidate before publication. Publishing still needs Ian's approval.

The Windows machine is stopped. Its ticket-owned tools and retained artifacts remain for coordinator-directed cleanup. No other team's tools or retained evidence were removed.
