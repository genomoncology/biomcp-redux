# BioMCP 1.0 release preparation, 2026-11-09

This is preparation for ticket 0294, not approval to publish or change an account. Publication follows AWS validation of the finished 1.0 code and the source review, as `decisions/2026-11-09-finish-1-0-with-aws-first.md` records. The selected candidate and its complete merged ancestry must be qualified before the final approval request.

## Account preflight

Read-only GitHub queries on 2026-11-09 found:

| Setting | Observed | Reviewable action before the tag |
| --- | --- | --- |
| Public repository | `genomoncology/biomcp`, main `561d3fe06d9dff5485f13a82a4108371f6fdfe88`; no `v1.0.0` ref | Recheck main and release-tag absence immediately before preparing the final merge and pushing the tag. |
| `release-publication` | Absent. Existing environments are `biomcp-release-signing`, `github-pages`, `production`, `pypi`, `release`. | Create `release-publication`, no reviewers or wait timer, admitting release tags `v*`. Keep publication automatic after the approved tag. |
| PyPI Trusted Publisher | Not readable through the GitHub API; not verified. | Administrator verifies or adds project `biomcp-cli`, GitHub owner `genomoncology`, repository `biomcp`, workflow `release.yml`, environment `release-publication`. No PyPI token is needed. |
| GHCR package | Metadata query refused: the current GitHub credential lacks `read:packages`; not verified. | Administrator verifies that public package `ghcr.io/genomoncology/biomcp` is linked to `genomoncology/biomcp` and its workflow token may write the package and attestations. Grant Actions access if missing. |
| Homebrew | `HOMEBREW_TAP_TOKEN` exists as a repository secret; its value and permissions were not read. | Administrator verifies Contents read/write access only to `genomoncology/homebrew-biomcp` and permission to update `main`, then supplies the environment secret required by the operator instructions. |
| Pages | Built; `gh-pages` exists, serves `/`, custom domain `biomcp.org`. Workflow default token permission is write; Actions enabled. | Preserve that branch and domain. Verify the release workflow can update only `install.sh` and request a Pages build. |
| MCP Registry | GitHub OIDC namespace access is not verified by these GitHub queries. | Administrator verifies publication for `io.github.genomoncology`; no registry token is needed. |

The exact settings come from `docs/development.md`, “Publish an approved release.” Account edits wait for Ian's approval. A secret name proves neither its scope nor that its credential remains usable.

These queries repeat the GitHub portion without printing credentials:

```sh
gh api repos/genomoncology/biomcp/environments --jq '.environments[] | {name,protection_rules,deployment_branch_policy}'
gh api repos/genomoncology/biomcp/environments/release-publication/secrets --jq '.secrets[].name'
gh api repos/genomoncology/biomcp/actions/secrets --jq '.secrets[].name'
gh api repos/genomoncology/biomcp/actions/permissions/workflow
gh api repos/genomoncology/biomcp/pages --jq '{status,cname,source,build_type}'
gh api orgs/genomoncology/packages/container/biomcp --jq '{name,visibility,repository:(.repository.full_name // null)}'
gh api repos/genomoncology/biomcp/branches/main --jq '.commit.sha'
gh api repos/genomoncology/biomcp/git/ref/tags/v1.0.0 --jq '.object'
```

## Final-candidate history rehearsal

Wait for the coordinator's selected landed main commit. The previous accepted run j30538 is reusable as the recipe, not as proof of a changed candidate. Its retained `SHA256SUMS.json` still hashes to `cb14bc50f027775396a1c81eb004dd50cd537d1c7c8730528e3a715297b1b52f`; inspection confirmed its recorded ordered parents, exact candidate tree, nonshallow scan, unchanged public refs and 77 old tags, unchanged 177-row ledger and passing scan. The current public main differs from that run's main.

Prepare on the same host as the scanner job. `CANDIDATE` is the coordinator's full selected commit; `SOURCE` is its clean local checkout. The commands below create only a scratch clone and an unpushed local merge; they never update a public ref. `git commit-tree` chooses the candidate tree directly and keeps both parents without resolving unrelated source files by hand.

```sh
S=$(mktemp -d -t biomcp-0294-append.XXXXXXXX)
git ls-remote --heads --tags https://github.com/genomoncology/biomcp.git > "$S/public-before.txt"
git clone --no-checkout https://github.com/genomoncology/biomcp.git "$S/repo"
PUBLIC_MAIN=$(git -C "$S/repo" rev-parse refs/remotes/origin/main)
git -C "$S/repo" for-each-ref --format='%(refname) %(objectname)' refs/remotes/origin refs/tags > "$S/old-refs.txt"
git -C "$S/repo" fetch --no-tags "$SOURCE" "$CANDIDATE"
CANDIDATE_TREE=$(git -C "$S/repo" rev-parse "$CANDIDATE^{tree}")
MERGE=$(git -C "$S/repo" commit-tree "$CANDIDATE_TREE" -p "$PUBLIC_MAIN" -p "$CANDIDATE" -m 'Append BioMCP 1.0 history')
git -C "$S/repo" checkout --detach "$MERGE"
test "$(git -C "$S/repo" show -s --format='%P' HEAD)" = "$PUBLIC_MAIN $CANDIDATE"
test "$(git -C "$S/repo" rev-parse 'HEAD^{tree}')" = "$CANDIDATE_TREE"
test "$(git -C "$S/repo" rev-parse --is-shallow-repository)" = false
git -C "$S/repo" cat-file -e 'refs/tags/v0.9.3^{commit}'
git -C "$S/repo" for-each-ref --format='%(refname) %(objectname)' refs/remotes/origin refs/tags > "$S/old-refs-after.txt"
cmp -s "$S/old-refs.txt" "$S/old-refs-after.txt"
```

The scanner job runs existing tracked code, against that scratch clone, using the selected candidate's unchanged ledger and pins. From the selected source checkout on that host:

```sh
pm job run 0294 "$CANDIDATE" -- python3 release/secrets.py history "$S/repo" --cache "$S/security-tools"
```

Use the returned job ID for its completion result. Do not shorten history scope, add a new ledger entry without classification, or print a matched value. After the passing scan, compare current public refs with the snapshot and retain the bundle and job output in the existing evidence store:

```sh
git ls-remote --heads --tags https://github.com/genomoncology/biomcp.git > "$S/public-after.txt"
cmp -s "$S/public-before.txt" "$S/public-after.txt"
git -C "$S/repo" bundle create "$S/append-history.bundle" --all HEAD
git -C "$S/repo" bundle verify "$S/append-history.bundle"
```

If public refs move, refresh the scratch clone and repeat only the ancestry-dependent checks against the new public main. Retain selected commit, public main, merge, ordered parents, tree, old refs, scanner job result and bundle with the ticket. A fresh reviewer accepts the final packet before approval is requested.

## Selected candidate and retained history merge

The coordinator selected landed main `2d2dbda17ca29202bd332152ab604408cb9b5b14`, tree `dc21594efab5d386f2afca23821bfa108c90633a`. The scratch rehearsal used actual public main `561d3fe06d9dff5485f13a82a4108371f6fdfe88` as first parent and that selected candidate as second parent. The resulting unpushed merge is `d0ca20de511d1b807eb69a7eedc9fbfb7f7da1ca`, with the selected candidate's exact tree. The clone is nonshallow, all original public refs remain unchanged, all 77 old tags remain, and `v0.9.3` still resolves to `5641de7d7b79b53872f8599dbb4dba22e611dcaf`.

The retained bundle is 107,888,176 bytes, SHA-256 `1ce4d3318a19cac295bc91d40bf936cd5c65122127ef1784c22a6dafd338575a`. The receiver verified that hash and restored the same merge, ordered parents, tree and old refs on Yellow. Its retained packet is `data/biomcp-redux-evidence/0294/candidate-2d2dbda17ca2-public-append-d0ca20de-j36148` on Yellow. The preflight binds the selected scanner, toolchain manifest and unchanged 181-row ledger by hash.

The complete-history scan uses job j36148 at the selected candidate, with Python 3.13 first on PATH, including nested commands. Yellow reports Python 3.13.17 and Git 2.51.0. The job scans `--single-worktree --all HEAD`; it does not scan only recent commits or change the ledger. Queued M5 job j36139 and initial Yellow launcher j36147 were cancelled before execution when moving the scan to an available-host queue and choosing Bash for Linux's pipefail support. The final packet must include j36148's actual result and a postflight comparison of canonical public refs before a reviewer accepts it.

## Approved release order

Once AWS validation, the source review, account readiness and the final candidate's qualification are complete, give Ian the reviewable merge, tag, account changes, channel limitations and docs-site result. After his release approval, push only `v1.0.0` on the prepared merge. The ordinary tag workflow qualifies, stages, screens, signs, publishes, checks public installs and promotes latest without an approval pause. Advance public main to the merge only after that workflow passes, then land the refreshed site in the same release session. Preserve old heads, tags, releases and history; no force push.

The rehearsal intentionally skips channel authentication, uploads, directory acceptance and public installs. Those remain first exercised by the tag. A failed publication may leave some channels public; inspect the failed job before retrying because a PyPI version cannot be replaced. A failed promotion may have moved GHCR latest before GitHub latest; inspect both and retry only the failed promotion job from the same run, as the operator instructions specify.
