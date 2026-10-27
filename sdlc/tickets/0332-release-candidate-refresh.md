# 0332 — Refresh the 1.0 release candidate with the outside-test fixes

Status: OPEN.

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
