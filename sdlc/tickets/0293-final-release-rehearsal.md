# 0293 — Rehearse the final release candidate on every platform

Status: OPEN. Reopened: 0336 adds 0334 and 0335 to the candidate; the final rehearsal must qualify the refreshed release head (m9956).

Milestone: 5

Depends on: 0289, 0290, 0292, 0297, 0319, 0332, 0336, 0621, 0622, 0634, 0635, 0636, 0638, 0639, 0640, 0651, 0652, 0653, 0654, 0655, 0656, 0657, 0658, 0659, 0660, 0661, 0662, 0663, 0665, 0666, 0667, 0668, 0669, 0671, 0675, 0677, 0678, 0679, 0680, 0681, 0682, 0683, 0684, 0686, 0687, 0641, 0649, 0670, 0676, 0689, 0690, 0691, 0692, 0693, 0694, 0695, 0696, 0697, 0710, 0711, 0712, 0713, 0714, 0715, 0717, 0718, 0719, 0720, 0721, 0722, 0723, 0724, 0725, 0726, 0727, 0729, 0731, 0732

## Outcome

The final release candidate's artifacts install and run on every promised platform, repeating the 0066 installed rehearsal: wheel, installer archives, Homebrew formula, container, desktop bundle and registry entry, each running `version`, `health`, one replayed search and get, and stdio and HTTP MCP. The report names any platform that could not be exercised.

- Why: the release plan requires final-candidate installed proof before Ian approves publication.
- Starts from: the coordinator's selected ending core `b3809eb1baa6d1e470efaf6499d8be0ba600f511`, tree `4baf7bddd9487d5adba82589c45351bef36fa2b2`, current `release/qualification.md`, the ordinary `release.yml`, and the machine registry. Keep execution results in pm; no historical `release/1.0` head is the default candidate.
- Rules: qualify that selected commit section by section. Reuse a passing section only when its relevant inputs still apply; after a fix, rerun the failed section. Run the ordinary workflow with `rehearsal=true` in the public `genomoncology/biomcp-release-rehearsal` repository at an exact selected-main ref, on standard free runners, under `decisions/2026-11-02-public-rehearsals.md`. Bind run commit, tree, workflow and artifact identities to selected main. The rehearsal exercises the Claude Code plugin channel and the installed AlphaGenome local TLS proof from 0392 on five native targets and both Linux containers. Use the committed schema baseline and immutable migration baseline, without refreshing either to hide a failure. Local builds run through pm job on M5 or Yellow, as `decisions/2026-11-09-finish-1-0-with-aws-first.md` records. Nothing is published.
- Proof: one report per platform with exact commands, versions and exits, plus passing complete-payload screening, signed-subject verification and both staged Homebrew architectures, reviewed by a fresh reviewer. Skipped authentication and public installs remain explicitly unproved until the tag.
- Changes: none expected beyond fixes the rehearsal finds, each its own ticket.
- Defers: publication, which needs Ian.

- Release review (2026-11-01): the rehearsal also runs the published 0.9 installer script against the 1.0 archives, and resolves or classifies the amd64 Docker containerd ENOENT fault before it passes. Load tests (0641, 0670) and follow-ups 0649 and 0676 precede the rehearsal, per `decisions/2026-11-05-release-finds-no-bugs.md`. Withdrawn wheel-deduplication ticket 0716 is not a dependency.

## Historical evidence

This evidence identifies earlier candidates and does not qualify the current selected main. The earlier qualified release candidate was `release/1.0` at `ff2e9b64c8b6f205ecd60c3bc51874bfa7223b55`. Its product source was `e4b72e59f590f4b04189718d8ecdd633e4238307`. The refreshed baseline contained 275 schemas generated from that exact source. Local job `j17367` passed baseline equality, schema comparison, the six fixes, container checks and the routine gate.

[Hosted run 40542150253](https://github.com/genomoncology/biomcp-redux/actions/runs/40542150253) qualified that exact release candidate through `workflow_dispatch`, attempt 1. The repository was public, and every runner label was a standard free hosted runner. All five native platform jobs passed: Linux x86_64, Linux arm64, macOS Intel, macOS arm64 and Windows x86_64. Both Linux container checks passed. Each platform's retained install report records version, isolated offline health with all 56 source IDs disabled, replayed search and get, and stdio and HTTP MCP checks for the installed channels. Both macOS platforms passed actual Homebrew installation. Homebrew was deferred on Linux and Windows. The Linux x86_64 schema step and publication payload staging passed. The release operator job was skipped. Nothing was published.

The raw evidence is retained on Yellow at `/home/ian/workspace/data/biomcp-redux/0293-release-rehearsal-evidence/candidate-ff2e9b64c8b6/hosted-40542150253`. Its 132-file manifest has SHA-256 `0abc92aaebe5de95ad61504ac89256621c574688c5620f7fabf33e7d062b15b3`. The archive holds terminal API records, raw logs, exact workflow and tooling, five platform install reports, artifact download receipts, member hashes and binary identity receipts. It contains no product archives or executables.

A fresh independent reviewer accepted the hosted evidence after checking all 132 manifest entries, all 79 raw log members, all six downloaded artifact ZIPs against API sizes and digests, every artifact member, 15 native executables, two container executables, and the platform reports against the raw logs. The review confirmed offline source isolation, both Homebrew installs, schema comparison, staging and the skipped release operator. The build delegate reverified the durable archive and removed its owned temporary downloads after acceptance. GitHub artifacts remain available. Publication and the history append remain deferred for Ian's approval.

Earlier scope changes on 2026-11-05 required new proof after typed mode (0636), the cache-startup `Busy` reproduction at `015aea0a` (m26035, j30327, 1 of 24 concurrent starts), and the expanded 1.0 scope that superseded `1bb97e64f`. The 12 staged-artifact findings recorded in earlier progress as unclassified were classified (m26534: one harmless compiled pairing of the public AlphaGenome address and a neighboring message, no credential) and fixed by 0665. Those historical identities remain unchanged.

## Publish commands unproved until the tag

From 0697. The rehearsal skips every step below, so each first runs at `v1.0.0`. Before the tag, the `staged-payload` job checks the staged wheels with `twine check` and compares the staged `install.sh`, and `staged-homebrew` installs the staged formula. None of these contacts a channel. No TestPyPI or scratch registry stands in.

| Job | Command that stays unproved |
| --- | --- |
| `publish-github` | `release/publication.py github`: `gh release create` |
| `publish-ghcr` | GHCR login, `release/publication.py ghcr`: `docker push` and `docker manifest`, then the container attestation |
| `publish-pypi` | `release/publication.py wheels`, then `pypa/gh-action-pypi-publish` with Trusted Publishing and attestations |
| `publish-homebrew` | The tap checkout with `HOMEBREW_TAP_TOKEN`, then `release/publication.py homebrew`: the tap commit and push |
| `publish-installer` | The `gh-pages` checkout, then `release/publication.py installer`: the commit and push, and the Pages build request |
| `publish-registry` | The cosign install, `release/toolchain.py install-mcp-publisher`, then `release/publication.py registry`: `mcp-publisher login github-oidc` and `publish` |
| `post-publish` | `release/published.py` for `pypi`, `ghcr`, `homebrew`, `installer` and `registry` against the public channels |
| `publish-latest` | GHCR login, `release/publication.py promote`: the latest image and `gh release edit --latest` |
