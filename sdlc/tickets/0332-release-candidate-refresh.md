# 0332 — Refresh the 1.0 release candidate with the outside-test fixes

Status: COMPLETE.
Landed: d2b3ecf34d482ad3e03f6dc724bca88d98cb1f78

Milestone: 5

Depends on: 0326, 0327, 0328, 0329, 0330, 0331

## Outcome

`release/1.0` gains only the 0326 to 0331 fix commits, cherry-picked from main onto 2b5d45d7, and its schema baseline is exported from that exact release product source, so the refreshed candidate holds the outside-test fixes and none of the AlphaGenome transport work on main.

- Why: experiments 495 and 497 produced fixes that change product output (supplement `first_row`, health `disable_groups`), so the b18 candidate and its baseline are stale. Main also holds the unfinished 0320 to 0322 transport, which must stay out of 1.0. PM ruling 2026-10-27 (m8126 ask 71).
- Starts from: `release/1.0` at 2b5d45d7, the landed 0326 to 0331 commits, the 0314 snapshot recipe and the 0293 rules.
- Rules: cherry-pick only those fixes, resolving conflicts without bringing in other main changes; the manifest names the release product source commit, never its own landing commit. The diff from b18 to the new head lists every path, and no file under the AlphaGenome transport appears. Because product source changed, b18 installed evidence no longer carries forward; the approved hosted release.yml rerun on the new head is the installed proof for every hosted platform, and local supplements run only for channels the hosted run cannot cover.
- Proof: the gate passes against the refreshed baseline, baseline files equal the release head's generated schemas byte for byte, the 0326 to 0331 cases pass on the release head, and the hosted rerun on that head passes.
- Changes: the `release/1.0` branch and its baseline snapshot.
- Defers: publication and the history append, which need Ian.


## Delivered result and evidence

`origin/release/1.0` preserves candidate `ff2e9b64c8b6f205ecd60c3bc51874bfa7223b55` and its full history. The six product patches equal the accepted 0326 to 0331 patches byte for byte. The complete diff from `b18b67aeff6242088f4901c22131529f74ce7dd4` contains 73 paths and no AlphaGenome transport changes. The fetch tree and dependency inputs remain unchanged from b18.

The manifest names product source `e4b72e59f590f4b04189718d8ecdd633e4238307`. All 275 generated schemas equal the baseline byte for byte. The archive reconstructs deterministically from that source with SHA-256 `90da9a1c54c29a6c210206e3ef84f0724b1173311ba4f4dfd93733cb9f8c9c15`. Jobs j17360 and j17367 passed schema export, regeneration, compatibility, the six fixes and the routine gate. Raw recipes, logs and the complete path inventory reside on Yellow under `/home/ian/workspace/data/biomcp-redux/0332-release-refresh/`, in the full product-source and candidate SHA directories.

[Hosted release run 40542150253](https://github.com/genomoncology/biomcp-redux/actions/runs/40542150253) succeeded on that exact candidate. All five native qualifications, both container checks, candidate qualification and staging passed. Both macOS Homebrew checks and the Linux x86 schema check passed. The five installed reports agree with the raw logs. The independent review verified artifact digests, archive members and installed binary hashes. The release operator was skipped; nothing was published. Homebrew was unavailable on Windows and Linux; both macOS architectures exercised it. The separate 0323 transport measurement on native Intel Mac remains outside this release refresh.

The hosted evidence resides on Yellow at `/home/ian/workspace/data/biomcp-redux/0293-release-rehearsal-evidence/candidate-ff2e9b64c8b6/hosted-40542150253/`. Its `manifest.json` SHA-256 is `0abc92aaebe5de95ad61504ac89256621c574688c5620f7fabf33e7d062b15b3`. The archive contains raw logs, reports, API receipts and hashes, with no retained product binaries.

This main-based revision records only this ticket's delivery and proof. The qualified release history and baseline remain on `release/1.0`. Publication and the history append still need Ian's approval.
