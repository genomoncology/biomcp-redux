# Pre-release whole-project review

Date: 2026-10-28

Snapshot: b5aeaced17445e7fa040e9592324d4b55ff3213a, main a1b2ab723 plus 0371 and 0392, clean detached tree. Three fresh read-only reviewers covered product code, release and packaging, and docs and tickets. They ran no builds, tests, workflows or provider calls. The PM confirmed release findings 1, 2 and 5 directly: the release workflow calls `python3` on Windows before setup-python, the tree has no LICENSE although public 0.9.3 has one, and the 0.9.3 wheel carries the MCP registry `mcp-name` marker that 1.0's wheel lacks.

## Verdict

No secret leak, trust-boundary break or data-loss defect was found. The release is not ready to tag: the hosted workflow at this head has never run, and three packaging gaps would fail a channel or drop the license. Each finding below is ticketed for 1.0.

## Findings

| Area | Label, severity | Finding | Ticket |
|---|---|---|---|
| Release | Likely defect, high | Windows qualify leg calls `python3` before setup-python | 0393 |
| Release | Confirmed defect, high | Wheel lacks the registry `mcp-name` marker, License and Project-URL | 0395 |
| Release | Confirmed defect, medium | No LICENSE or third-party notices in the tree or any artifact; the public merge would drop LICENSE | 0395 |
| Release | Likely defect, medium | Windows binary links the Visual C++ runtime dynamically | 0396 |
| Release | Design gap, medium | Protection check ignores admin bypass; verify instructions pin no tag ref; summary claims an approval the job cannot know | 0393 |
| Release | Risk, medium | manylinux and runtime images and apt packages unpinned; rustup checksum fetched from the same server | 0394 |
| Release | Inconsistency, low | Release workflow runs no fmt, clippy or test suite | 0393 |
| Release | Inconsistency, low | Gate never checks `version --verbose` commit; local recipes label a dirty tree with HEAD | 0393 |
| Release | Inconsistency, low | Two-architecture Homebrew formula only syntax-checked; plugin manifest has no version | 0393 |
| Release | Rough edge, low | Installer aborts with HOME unset, targets `/.local/bin` with HOME empty | 0393 |
| Release | Risk, low | Tag builds restore caches; no actionlint | 0394 |
| Release | Inconsistency, low | Schema baseline names a source 69 commits old | 0336 |
| Code | Confirmed defect, medium | Interrupted bundle download orphans its part file; nothing deletes it | 0398 |
| Code | Likely defect, medium | A missing released part file blocks that bundle's sync permanently | 0398 |
| Code | Risk, low | Cache folder and database use default permissions | 0399 |
| Code | Risk, low | `key set` replaces a symlinked config with a regular file | 0399 |
| Code | Design gap, low | Lower-case proxy variables ignored | 0400 |
| Code | Design gap, low | Help does not say publisher PDFs are refused through a proxy | 0400 |
| Code | Likely defect, low | Markdown output passes control characters to the terminal | 0400 |
| Code | Inconsistency, low | `--format` parsed twice; a dispatch error prints "output failed" | 0400 |
| Code | Risk, low | One oversized stdio frame ends the MCP session | 0400 |
| Docs | Confirmed defect, medium | Upgrade guide, roadmap and slice tables say GenCC is deferred; it ships | 0397 |
| Docs | Inconsistency, low | Upgrade guide, README Develop section and cautions keep pre-release hedges | 0397 |
| Docs | Inconsistency, low | Roadmap boxes and history wording drift from records | 0397 |
| Plan | Open question, medium | Plan put publication before the public append and tag | plan, 0294 |
| Plan | Design gap, medium | 0336 baseline re-export conflicts with tree equality | 0336 |
| Plan | Needs correction | 0293 rules still cite cherry-picks and evidence carry-forward | 0293 |
| Decisions | Inconsistency | Parity decision overstates 0098 coverage; transport decision says proxy comes only from config | decisions amended |

## Strengths

Secrets are hidden in debug output, loaded lazily and checked against reflection in replies. Each source reaches only its declared origins, and the PDF downloader checks resolved addresses on every redirect. The HTTP server is bounded and refuses cross-origin requests. MCP policy applies to the parsed command. Bundle sync uses leases and single-transaction publication. Every workflow action is SHA-pinned with least privilege, every channel is checked against one binary hash, and the installer forces HTTPS and checksums. Docs examples are executable and the command reference and upgrade table are generated.

## Skipped

Builds, tests, workflow and container runs, artifact inspection, macOS signing, line-by-line adapter and gRPC transport reads, load and concurrency tests, and Windows-specific paths.
