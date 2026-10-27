# 0347 — Screen dependencies, secrets and installer downloads in the gate and the release

Status: OPEN.

Milestone: 5

## Outcome

`make audit` runs `cargo deny check licenses advisories bans sources`, and `deny.toml` denies unknown registries and unknown git sources, so every dependency comes from crates.io or a named, reviewed source. The release workflow runs the same `cargo deny` check on the candidate commit at release time, so a new advisory fails a release even when landing passed. A pinned secret scanner checks the repository history in the landing gate and every staged release artifact in the release workflow. `release/install.sh` downloads only over HTTPS: it passes `--proto '=https'` to curl and the matching option to wget, and refuses a `BIOMCP_RELEASE_BASE_URL` that is not `https://`. It keeps refusing an archive whose SHA-256 does not match.

- Why: Ian's security screening request through Jarvis (m10245, 2026-10-27). Today the gate checks licenses and advisories only at landing, nothing scans for secrets, and the installer accepts a plain-HTTP base URL.
- Starts from: `Makefile` `audit`, `deny.toml`, `.github/workflows/release.yml`, `release/install.sh` and `release/test_installer.py`.
- Rules: GitHub actions and scanner versions are pinned by commit or checksum, as release.yml pins today. PM ruling 2026-10-27 (m10471): the first history scan found only a synthetic test TLS key, replay digests and public paging cursors; they and five public Cellosaurus cross-reference accessions verified against their own public links (m10471 ask 2) are allowed by exact detector fingerprint for history and by narrow regexes for ClinicalTrials.gov pageToken values and 64-hex replay digests in fixture folders, each with a reason. No path-wide wildcard, disabled rule or history rewrite; any other finding stops the work. A secret finding fails the run and prints only the file and rule, never the matched text. No key from any machine enters a test.
- Proof: a deny fixture with a git dependency from an unknown URL fails `make audit`; a planted fake token in a temporary fixture fails the scanner; installer tests show an `http://` base URL refused and a checksum mismatch refused; the hosted release run shows the deny and scanner steps passing.
- Changes: `Makefile`, `deny.toml`, `release.yml`, `release/install.sh` and their tests.
- Defers: signed build provenance for release artifacts, which goes to a 1.0 update. This ticket joins the 0336 refresh because the release workflow and installer ship with 1.0.
