# 0294 — Append this repository's history to the public BioMCP repository

Status: OPEN.

Milestone: 5

Depends on: 0293, 0633, 0638

## Outcome

After Ian approves, the public BioMCP main gains one merge commit whose parents are its previous main and the approved 1.0 candidate and whose files equal that candidate, so the old history, tags, stars, issues and releases stay and this repository's history follows them.

- Why: Ian ruled on 2026-10-27 to append, not replace (`2026-10-27-append-history-to-biomcp.md`), because the public repository has earlier releases.
- Starts from: that decision, the selected ending core in 0293, and the current public repository's settings.
- Rules: nothing is pushed to the public repository before Ian's explicit approval of this step. A rehearsal on a scratch clone of the current public repository shows the ordered parents, that old heads and tags still resolve, that the merge tree equals the candidate, and that the complete merged history passes the existing secret scanner before the approval request. The release operator performs the push without force.
- Release order (W31, `records/2026-11-06-pre-release-review.md`): one Ian approval covers the append, the tag and publication. Build the merge locally with current public main first and the approved candidate second. Push only `v1.0.0` on that merge commit; its workflow qualifies and publishes without a manual pause. Fast-forward public main to that merge only after publication, public-channel installs and promotion pass. A failed tag run leaves public main unchanged. This ticket owns the tag step.
- Release session: the tag, publication and main update run in one operator session. The public files `CITATION.cff`, `.zenodo.json`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md` and a 1.0 `CHANGELOG.md` are in the candidate through 0534. The refreshed docs site (0401) lands in that session after publication; its refresh starts before the cut.
- Old history findings (m20294, 2026-11-02): the history scan of public rehearsal run 41441571941 found 44 findings in 18 paths reachable only from the old 0.9 `main` and `v0.9.2` (Yellow `biomcp-redux-evidence/0294/public-rehearsal-41441571941/scope-diagnosis.json`). That history is already public. Before the append, review each finding without printing values, bring any finding that is a live credential to the PM for Ian without changing it, and record the outcome; the merged history then scans clean or with reviewed ledger rows. Reviewed 2026-11-02 (m20351, PM accepted): all 44 are noncredentials (37 public identifiers, 4 controlled test inputs, 2 artifact checksums, 1 documented example), classification on Yellow `biomcp-redux-evidence/0294/public-rehearsal-41441571941/legacy-finding-classification-20261102.json` (SHA256 8d24ebd6…). That snapshot need not equal the canonical public BioMCP main, so the append still scans its real merged ancestry.
- Proof: after the push, the public repository shows both histories, v0.9.3 and older tags resolve, stars and releases are intact, and the merge tree equals the approved candidate.
- Account preflight (W8): this ticket owns the read-only check of the `release-publication` environment, PyPI Trusted Publisher, channel permissions and secret names. Prepare the exact account changes for Ian's approval before the tag. `docs/development.md` names the required settings. Never read or print secret values.
- Publication prerequisites: AWS validates the finished 1.0 code and Ian and the PM complete the source review before publication, as `decisions/2026-11-09-finish-1-0-with-aws-first.md` records. The blind comparison was moved to evals and dropped as a release dependency (m23337, m23350).
- Changes: this ticket and the release-preparation record; after approval, the public history append and tag.
- Defers: nothing.

## Evidence

- [Release preparation, 2026-11-09](../records/2026-11-09-release-preparation.md) records the read-only account findings, reviewable changes and final-candidate scratch rehearsal commands. It records no publication approval or AWS validation result.
- Current ending-core ancestry: merge `8d03d93e7d73408a35ba9e63e2ef4e19d544c330`, ordered parents public main `561d3fe06d9dff5485f13a82a4108371f6fdfe88` and ending core `b3809eb1baa6d1e470efaf6499d8be0ba600f511`, with ending tree `4baf7bddd9487d5adba82589c45351bef36fa2b2`. Scanner job j36352 passed the complete 16,647-commit history with the unchanged 181-row ledger, no secret findings and seven existing informational coverage notices. The linked record names the complete receiver-verified ending packet and manifest, including unchanged canonical public refs and all 77 old tags. This evidence establishes the scratch append and history screening, not AWS validation, account readiness or publication approval.
- Historical selected-core append rehearsal: j36297 passed against merge `8ca286662dcda20ba132cc50c0e0263e5b188b47`, ordered parents then-public main `561d3fe06d9dff5485f13a82a4108371f6fdfe88` and core `42a79b1bbdc95108447d19369b7cd69f35f8adef`, with tree `6caf63ae9a8ccf80bd5bd440f25da2d61d6f9e9d`. That retained packet does not qualify the later ending ancestry or establish AWS validation, account readiness or publication approval.
- Earlier-candidate append rehearsal: j36159 passed against merge `d0ca20de511d1b807eb69a7eedc9fbfb7f7da1ca`, ordered parents then-public main `561d3fe06d9dff5485f13a82a4108371f6fdfe88` and candidate `2d2dbda17ca29202bd332152ab604408cb9b5b14`, with candidate tree `dc21594efab5d386f2afca23821bfa108c90633a`. The linked record preserves that historical packet; it does not qualify the later selected core `42a79b1bbdc95108447d19369b7cd69f35f8adef`.
- The earlier accepted append rehearsal is j30538 at superseded candidate `c9bd901d50a506e53d168b189315d34a2c940cef`, against then-public main `57834215deb17d7772694b8bba5ad7b7093dd48e`. Its retained evidence remains intact; it does not qualify the new candidate or current public main.
