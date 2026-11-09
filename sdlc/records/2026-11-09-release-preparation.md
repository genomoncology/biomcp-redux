# BioMCP 1.0 release preparation, 2026-11-09

This is preparation for ticket 0294, not approval to publish or change an account. Publication follows AWS validation of the finished 1.0 code and the source review, as `decisions/2026-11-09-finish-1-0-with-aws-first.md` records. The selected candidate and its complete merged ancestry must be qualified before the final approval request.

## Account preflight

Read-only GitHub queries on 2026-11-09 found:

The GitHub environment list, secret names, Pages settings, public main and tag absence were checked again when preparing selected core `42a79b1bbdc95108447d19369b7cd69f35f8adef`. Those observations below were unchanged. No account setting was changed, and the unverified PyPI, GHCR and MCP Registry access requirements remain unverified.

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

## Current selected candidate and history merge

The coordinator selected landed core `42a79b1bbdc95108447d19369b7cd69f35f8adef`, tree `6caf63ae9a8ccf80bd5bd440f25da2d61d6f9e9d`, after the runtime zlib and owned-cache WAL retry fixes. A fresh scratch clone of the canonical public repository still selected main `561d3fe06d9dff5485f13a82a4108371f6fdfe88`. The new unpushed merge is `8ca286662dcda20ba132cc50c0e0263e5b188b47`, with public main first and this selected core second, and exactly the selected tree. The clone is nonshallow; all 77 old tags and original public refs are preserved. The selected source's 181-row ledger still hashes to `131bb9fa20e87846127943f72dea5b5f6132e86dfdd47849ad06a3a184c4b32c`, with no classification changes.

Complete-history job j36297 passed on M5 at this exact core, exit 0, scanning 16,646 commits through `--single-worktree --all HEAD` with no `since` restriction. It reported no secret findings and the same seven informational opaque-history coverage notices retained in the raw log. The executed scanner, ledger and new-core toolchain manifest hashes matched the preflight source bindings. Python 3.13 was first on PATH, including nested commands; the host reported Python 3.13.17, Git 2.54.0 and pinned Gitleaks 8.30.1, with its pinned archive hash verified. Canonical public heads and tags remained identical before and after the scan, and all old local refs and tags stayed intact. One tracked scan ran; no queue moves or ledger edits were needed.

The new packet is retained on Yellow at `data/biomcp-redux-evidence/0294/candidate-42a79b1bbdc9-public-append-8ca28666-j36297`. Its bundle is 108,028,972 bytes, SHA-256 `f8145880fecc6e1cdd64a130be1ff1c55b970dfedb8995eb75cb60345b1633b8`. The receiver verified every file in the 13-file manifest `SHA256SUMS.json`, SHA-256 `0a088f10988ad2f50e3db1374e470f73f55af6464d046f6c8dec57ecf94d90aa`, and restored the same merge, parent order, tree and public refs. The restored repository for direct read-only review is `/tmp/biomcp-0294-final-42a79b1bb.PcV453P5/repo` on Yellow; the durable bundle remains the reconstruction source after scratch expiry.

This source selection does not establish AWS validation, the source review, publishing-account readiness or Ian's approval of a public tag. The same owning session drives core and hosting under the recorded instruction; the approval and validation requirements stay in place.

## Historical candidate and retained history merge

The coordinator selected landed main `2d2dbda17ca29202bd332152ab604408cb9b5b14`, tree `dc21594efab5d386f2afca23821bfa108c90633a`. The scratch rehearsal used actual public main `561d3fe06d9dff5485f13a82a4108371f6fdfe88` as first parent and that selected candidate as second parent. The resulting unpushed merge is `d0ca20de511d1b807eb69a7eedc9fbfb7f7da1ca`, with the selected candidate's exact tree. The clone is nonshallow, all original public refs remain unchanged, all 77 old tags remain, and `v0.9.3` still resolves to `5641de7d7b79b53872f8599dbb4dba22e611dcaf`.

The retained bundle is 107,888,176 bytes, SHA-256 `1ce4d3318a19cac295bc91d40bf936cd5c65122127ef1784c22a6dafd338575a`. The receiver verified that hash and restored the same merge, ordered parents, tree and old refs on Yellow. The historical retained packet is `data/biomcp-redux-evidence/0294/candidate-2d2dbda17ca2-public-append-d0ca20de-j36159` on Yellow. Its 13-file `SHA256SUMS.json` hashes to `089215244f48635857b56f887dc9a92e1dbf39d7a4d0f7e8569850ef5e854a65`; every receiver-side file was checked against that manifest. The earlier bundle-only checkpoint remains intact in the sibling `candidate-2d2dbda17ca2-public-append-d0ca20de-j36148` directory.

The complete-history scan passed as j36159 on M5 at the selected candidate, exit 0, in about 60 seconds. It scanned all 16,643 commits reachable through `--single-worktree --all HEAD`, without a `since` restriction, using the unchanged 181-row ledger. It reported no secret findings and seven existing informational `opaque-history-blob-decoded-coverage-unavailable` notices; the raw log preserves those coverage limits. Python 3.13 was first on PATH, including nested commands; the actual host reported Python 3.13.17, Git 2.54.0 and pinned Gitleaks 8.30.1. The pinned scanner archive's SHA-256 was verified. Source-file hashes in the executed job checkout matched the selected candidate's scanner, ledger and toolchain manifest.

The postflight confirmed that all canonical public heads and tags stayed unchanged before and after the scan, the scratch clone's old refs stayed unchanged, and all 77 old tags still resolve. The historical packet retains the raw job log and job receipt, preflight and postflight identities, public-ref snapshots, old-ref snapshots, bundle and bundle verification. No public ref or ledger was changed. Earlier queue requests j36139, j36147 and j36148 were cancelled before execution; only j36159 ran that scan, after an M5 core became available. These results qualify the earlier candidate, not the later selected core.

## Approved release order

Once AWS validation, the source review, account readiness and the final candidate's qualification are complete, give Ian the reviewable merge, tag, account changes, channel limitations and docs-site result. After his release approval, push only `v1.0.0` on the prepared merge. The ordinary tag workflow qualifies, stages, screens, signs, publishes, checks public installs and promotes latest without an approval pause. Advance public main to the merge only after that workflow passes, then land the refreshed site in the same release session. Preserve old heads, tags, releases and history; no force push.

The rehearsal intentionally skips channel authentication, uploads, directory acceptance and public installs. Those remain first exercised by the tag. A failed publication may leave some channels public; inspect the failed job before retrying because a PyPI version cannot be replaced. A failed promotion may have moved GHCR latest before GitHub latest; inspect both and retry only the failed promotion job from the same run, as the operator instructions specify.
