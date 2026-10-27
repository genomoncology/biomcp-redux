# 0293 — Rehearse the final release candidate on every platform

Status: COMPLETE.
Landed: 7eb644cdee6e795f5981130cb6d7fac7dbc7aa5a

Milestone: 5

Depends on: 0289, 0290, 0292, 0297, 0319, 0332, 0336

## Outcome

The final release candidate's artifacts install and run on every promised platform, repeating the 0066 installed rehearsal: wheel, installer archives, Homebrew formula, container, desktop bundle and registry entry, each running `version`, `health`, one replayed search and get, and stdio and HTTP MCP. The report names any platform that could not be exercised.

- Why: the release plan requires final-candidate installed proof before Ian approves publication.
- Starts from: 0066's rehearsal recipe, `release/gate.py` and `release/matrix.sh`, release.md and the machine registry.
- Rules: the rehearsal includes one 0289 workflow dry run, on a fork tag or workflow_dispatch with publication disabled, that builds every artifact on the five platforms and two containers; Ian approves that external run before it starts. The rehearsal publishes nothing otherwise. Machines are booked through pm job. The rehearsal verifies or refreshes the 0290 schema baseline against the selected final candidate, so the published 1.0 schema set becomes the baseline 1.0.1 is compared with; 0290's snapshot is provisional input only (PM ruling 2026-10-26, m8126 ask 31).
- Candidate rule, PM ruling 2026-10-27 (m8126 ask 56): installed-product evidence from an earlier candidate carries forward to a later one when the diff between them touches no product source, lockfile or packaging input, shown by an exact path list with both commit hashes in the report. A release tooling fix such as 0313 or 0315 does not force a product rebuild; the external dry run qualifies the exact final workflow candidate.
- Release branch, PM ruling 2026-10-27 (m8126 ask 64): the 1.0 candidate lives on a `release/1.0` branch in this repo, created from the b18 candidate. Release-only fixes such as 0324 land on main as usual and are cherry-picked onto `release/1.0`, so main keeps later work such as the AlphaGenome codec. The diff from b18 to the branch head must touch only workflow and release tooling for b18 evidence to carry forward. The external dry run and the eventual 1.0 tag use that branch head.
- Proof: one report per platform with exact commands, versions and exits, reviewed by a fresh reviewer.
- Changes: none expected beyond fixes the rehearsal finds, each its own ticket.
- Defers: publication, which needs Ian.

## Delivered evidence

The qualified release candidate is `release/1.0` at `ff2e9b64c8b6f205ecd60c3bc51874bfa7223b55`. Its product source is `e4b72e59f590f4b04189718d8ecdd633e4238307`. The refreshed baseline contains 275 schemas generated from that exact source. Local job `j17367` passed baseline equality, schema comparison, the six fixes, container checks and the routine gate.

[Hosted run 40542150253](https://github.com/genomoncology/biomcp-redux/actions/runs/40542150253) qualified that exact release candidate through `workflow_dispatch`, attempt 1. The repository was public, and every runner label was a standard free hosted runner. All five native platform jobs passed: Linux x86_64, Linux arm64, macOS Intel, macOS arm64 and Windows x86_64. Both Linux container checks passed. Each platform's retained install report records version, isolated offline health with all 56 source IDs disabled, replayed search and get, and stdio and HTTP MCP checks for the installed channels. Both macOS platforms passed actual Homebrew installation. Homebrew was deferred on Linux and Windows. The Linux x86_64 schema step and publication payload staging passed. The release operator job was skipped. Nothing was published.

The raw evidence is retained on Yellow at `/home/ian/workspace/data/biomcp-redux/0293-release-rehearsal-evidence/candidate-ff2e9b64c8b6/hosted-40542150253`. Its 132-file manifest has SHA-256 `0abc92aaebe5de95ad61504ac89256621c574688c5620f7fabf33e7d062b15b3`. The archive holds terminal API records, raw logs, exact workflow and tooling, five platform install reports, artifact download receipts, member hashes and binary identity receipts. It contains no product archives or executables.

A fresh independent reviewer accepted the hosted evidence after checking all 132 manifest entries, all 79 raw log members, all six downloaded artifact ZIPs against API sizes and digests, every artifact member, 15 native executables, two container executables, and the platform reports against the raw logs. The review confirmed offline source isolation, both Homebrew installs, schema comparison, staging and the skipped release operator. The build delegate reverified the durable archive and removed its owned temporary downloads after acceptance. GitHub artifacts remain available. Publication and the history append remain deferred for Ian's approval.
