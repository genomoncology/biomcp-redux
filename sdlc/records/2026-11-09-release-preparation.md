# BioMCP 1.0 release preparation, 2026-11-09

This is preparation for ticket 0294, not approval to publish or change an account. Publication follows AWS validation of the finished 1.0 code and the source review, as `decisions/2026-11-09-finish-1-0-with-aws-first.md` records. The selected candidate and its complete merged ancestry must be qualified before the final approval request.

## Source and evaluation assessment before AWS validation

A read-only review of the existing evaluation reports did not establish a still-unfixed source-related product defect on core `42a79b1bb`. That review addressed source coverage, not proof of later runtime behavior. The frozen final source `0adbae81319cbfd89066c880ced85391f7a4775a` includes the later correctness fixes; its actual qualification belongs to 0788, and final AWS validation still has to confirm the relevant queries. The coverage recommendation remains preliminary and authorizes no additional source work. It does not substitute for final deployed-service validation.

The November 4 recommendations and gap recheck describe older binaries. Their accepted drug-name, coordinate, approval-ordering, filter, posted-results, label, population and device findings map to completed fixes in the current plan. The [November 6 life-sciences report](2026-11-06-life-sciences-test.md) used deployed core `1bb97e64`; its mapped repairs still need confirmation on the final hosted image. DailyMed-only labels were deliberately withdrawn in [0612](../tickets/0612-dailymed-labels.md), rather than left as an unfinished source defect.

The [additional-source list](../planning/sources-to-add.md) remains a candidate list. Posted trial results, drug-name expansion and open-source author search already address parts of the earlier motivation for AACT, name services and OpenAlex. Regulatory-event coverage, patents/exclusivity, deeper cell-line data and broader author/institution coverage are possible additional capabilities; an absent capability alone does not demonstrate that an existing command is wrong.

After AWS validation, the PM recommendation is to release with current coverage and order additional sources as subsequent work, unless the final hosted results demonstrate a specific missing capability that Ian chooses to hold 1.0 for. That decision must use the actual remaining question, rights and cost. The earlier ranking of regulatory-event coverage and drug-name normalization is dated evidence, not a new must-ship requirement. The old benchmark runs also do not establish token savings for the final 17-tool interface.

Evidence: [evaluation recommendations](../planning/eval-recommendations-2026-11-04.md), [gap recheck](../planning/eval-gap-recheck-2026-11-04.md), the [life-sciences report](2026-11-06-life-sciences-test.md) and the [candidate source list](../planning/sources-to-add.md). No new evaluations, provider calls or source fetches were run for this assessment, and no additional source work is authorized by it.

## Account preflight

Account checks and approved setup on 2026-11-09:

After Ian [approved the prepared environment](../decisions/2026-11-09-approve-publication-environment.md), the operator created `release-publication` and its sole `v*` tag policy. Independent GET requests verified custom policies enabled, protected-branch-only mode disabled, one tag rule, no required reviewers and no wait timer. No secret, deployment or release action was part of this setup. Other publisher access remains unverified.

Read-only environment and deployment-policy GETs were repeated while preparing final source `0adbae81319cbfd89066c880ced85391f7a4775a`; that approved policy is unchanged. PyPI Trusted Publisher confirmation remains pending. This history refresh changes no account and grants no further approval.

The GitHub environment list, secret names, Pages settings, public main and tag absence were unchanged during the earlier `42a79b1bb` preparation. The table includes the later approved environment setup. A subsequent read-only check confirmed public `v1.0.0` still absent and PyPI latest `0.9.3`, with no `1.0.0` release. These version checks do not verify publishing permissions; PyPI, GHCR and MCP Registry access requirements remain unverified.

| Setting | Observed | Reviewable action before the tag |
| --- | --- | --- |
| Public repository | `genomoncology/biomcp`, main `561d3fe06d9dff5485f13a82a4108371f6fdfe88`; no `v1.0.0` ref | Recheck main and release-tag absence immediately before preparing the final merge and pushing the tag. |
| `release-publication` | Created after Ian's November 9 approval; readback verified only `v*` tags, no reviewers and no wait timer. | Setup complete. Preserve this policy; release publication still follows its separate approval. |
| PyPI Trusted Publisher | Not readable through the GitHub API; not verified. | Administrator verifies or adds project `biomcp-cli`, GitHub owner `genomoncology`, repository `biomcp`, workflow `release.yml`, environment `release-publication`. No PyPI token is needed. |
| GHCR package | Anonymous registry metadata reads succeeded; the latest image's source label matches `genomoncology/biomcp`, and neither `1.0.0` nor `v1.0.0` exists. Personal GitHub API access lacks `read:packages`; Actions write access remains unverified. | Administrator verifies the package's repository association and that this repository's workflow token may write the package and attestations. Grant Actions access if missing. |
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

The frozen final source is `0adbae81319cbfd89066c880ced85391f7a4775a`, tree `d4e8b3dd8f2a7fbe802f65ccc5496e98096b37ec`. The existing nonshallow clone was reused after refreshing canonical public heads and tags. Public main remains `561d3fe06d9dff5485f13a82a4108371f6fdfe88`; the new unpublished merge is `f5102bc0b944870d74b7f3635cbc47e76c82606a`, with public main first and the frozen final source second, and exactly the final source tree. All original public refs and 77 old tags remain intact. The 181-row ledger is unchanged, SHA-256 `131bb9fa20e87846127943f72dea5b5f6132e86dfdd47849ad06a3a184c4b32c`.

The ancestry-dependent full-history scan is job j36578 at that exact source, using its pinned scanner/toolchain, Python 3.13 first on PATH and the root session's notice route. The actual terminal result and canonical-ref postflight must be collected before this packet is called passing. This preparation does not repeat source-code, packaging, native-build or other qualification checks, and it does not claim that 0788 has passed.

The new bundle is 107,905,426 bytes, SHA-256 `72418a2e30d403d1c231bd733c256b76266d594289da35988b3bcafeb056c57e`. Earlier b380, 42a and 2d2 packets retain their own source identities and cannot supply the final ancestry result. Publication still follows final qualification, AWS validation, the source decision, publisher access confirmation and Ian's release approval. The approved order remains a tag on the unpublished merge first, followed by public main only after publication and public-install success.

## Historical b380 candidate and history merge (2026-11-09)

The ending core is `b3809eb1baa6d1e470efaf6499d8be0ba600f511`, tree `4baf7bddd9487d5adba82589c45351bef36fa2b2`. The fresh canonical public clone selected main `561d3fe06d9dff5485f13a82a4108371f6fdfe88`; the new unpushed two-parent merge is `8d03d93e7d73408a35ba9e63e2ef4e19d544c330`, with public main first and the ending core second, and exactly the ending tree. The clone is nonshallow, all original public refs and 77 old tags remain intact, and the unchanged 181-row ledger still hashes to `131bb9fa20e87846127943f72dea5b5f6132e86dfdd47849ad06a3a184c4b32c`.

Ancestry-dependent full-history job j36352 passed against that exact ending core, exit 0, scanning all 16,647 commits reachable through `--single-worktree --all HEAD`, with no `since` restriction and the unchanged 181-row ledger. The raw terminal log reports no secret findings and the same seven informational opaque-history coverage notices. The executed scanner, ledger and toolchain manifest hashes match the preflight bindings; Python 3.13 was first on PATH, including nested commands. The host reported Python 3.13.17, Git 2.54.0 and pinned Gitleaks 8.30.1, and the actual scanner archive matched its pinned digest. This result comes from the ending-core job, not an earlier packet; no rescan or source-code, native-build or full-gate check was run for this collection.

The ending bundle is 108,046,108 bytes, SHA-256 `a6bf69f317eaa8ad988ce6ebec84ec658e80e03c936041ffa30f43a497fd47fa`. The complete ending packet is retained on Yellow at `data/biomcp-redux-evidence/0294/candidate-b3809eb1baa6-public-append-8d03d93e-j36352`, including the actual raw log and terminal job receipt, preflight/postflight, ref snapshots and bundle. Every file in its 13-file `SHA256SUMS.json` was verified on the receiver; the manifest hashes to `8a022d085c4111cc57f974077ea6359ee3444df371aea44e4146094f648e5aea`. The receiver restored the same merge/tree/ordered parents and compared all original refs. Its read-only reviewer repository is `/tmp/biomcp-0294-ending-b3809eb1.ugWdTg6l/repo` on Yellow. The earlier 42a and 2d2 packets below remain untouched and historical; their passing scans are not substituted for this ending result.

The canonical public heads and tags are identical in the ending packet's before/after snapshots. All old local refs and 77 tags remain unchanged; `v0.9.3` still resolves to `5641de7d7b79b53872f8599dbb4dba22e611dcaf`, and the postflight records both preserved public-history roots. No public ref, account or ledger was changed.

The source assessment supports current coverage subject to final AWS validation and Ian's source decision. This packet introduces no additional source work and grants no public-tag, account, deployment or publication approval. The approved tag-first order remains: publish and verify from the approved merge tag, then fast-forward public main after success.

## Historical 42a candidate and history merge (2026-11-09)

The coordinator selected landed core `42a79b1bbdc95108447d19369b7cd69f35f8adef`, tree `6caf63ae9a8ccf80bd5bd440f25da2d61d6f9e9d`, after the runtime zlib and owned-cache WAL retry fixes. A fresh scratch clone of the canonical public repository still selected main `561d3fe06d9dff5485f13a82a4108371f6fdfe88`. The new unpushed merge is `8ca286662dcda20ba132cc50c0e0263e5b188b47`, with public main first and this selected core second, and exactly the selected tree. The clone is nonshallow; all 77 old tags and original public refs are preserved. The selected source's 181-row ledger still hashes to `131bb9fa20e87846127943f72dea5b5f6132e86dfdd47849ad06a3a184c4b32c`, with no classification changes.

Complete-history job j36297 passed on M5 at this exact core, exit 0, scanning 16,646 commits through `--single-worktree --all HEAD` with no `since` restriction. It reported no secret findings and the same seven informational opaque-history coverage notices retained in the raw log. The executed scanner, ledger and new-core toolchain manifest hashes matched the preflight source bindings. Python 3.13 was first on PATH, including nested commands; the host reported Python 3.13.17, Git 2.54.0 and pinned Gitleaks 8.30.1, with its pinned archive hash verified. Canonical public heads and tags remained identical before and after the scan, and all old local refs and tags stayed intact. One tracked scan ran; no queue moves or ledger edits were needed.

The new packet is retained on Yellow at `data/biomcp-redux-evidence/0294/candidate-42a79b1bbdc9-public-append-8ca28666-j36297`. Its bundle is 108,028,972 bytes, SHA-256 `f8145880fecc6e1cdd64a130be1ff1c55b970dfedb8995eb75cb60345b1633b8`. The receiver verified every file in the 13-file manifest `SHA256SUMS.json`, SHA-256 `0a088f10988ad2f50e3db1374e470f73f55af6464d046f6c8dec57ecf94d90aa`, and restored the same merge, parent order, tree and public refs. The restored repository for direct read-only review is `/tmp/biomcp-0294-final-42a79b1bb.PcV453P5/repo` on Yellow; the durable bundle remains the reconstruction source after scratch expiry.

This source selection does not establish AWS validation, the source review, publishing-account readiness or Ian's approval of a public tag. The same owning session drives core and hosting under the recorded instruction; the approval and validation requirements stay in place.

## Historical 2d2 candidate and retained history merge (2026-11-09)

The coordinator selected landed main `2d2dbda17ca29202bd332152ab604408cb9b5b14`, tree `dc21594efab5d386f2afca23821bfa108c90633a`. The scratch rehearsal used actual public main `561d3fe06d9dff5485f13a82a4108371f6fdfe88` as first parent and that selected candidate as second parent. The resulting unpushed merge is `d0ca20de511d1b807eb69a7eedc9fbfb7f7da1ca`, with the selected candidate's exact tree. The clone is nonshallow, all original public refs remain unchanged, all 77 old tags remain, and `v0.9.3` still resolves to `5641de7d7b79b53872f8599dbb4dba22e611dcaf`.

The retained bundle is 107,888,176 bytes, SHA-256 `1ce4d3318a19cac295bc91d40bf936cd5c65122127ef1784c22a6dafd338575a`. The receiver verified that hash and restored the same merge, ordered parents, tree and old refs on Yellow. The historical retained packet is `data/biomcp-redux-evidence/0294/candidate-2d2dbda17ca2-public-append-d0ca20de-j36159` on Yellow. Its 13-file `SHA256SUMS.json` hashes to `089215244f48635857b56f887dc9a92e1dbf39d7a4d0f7e8569850ef5e854a65`; every receiver-side file was checked against that manifest. The earlier bundle-only checkpoint remains intact in the sibling `candidate-2d2dbda17ca2-public-append-d0ca20de-j36148` directory.

The complete-history scan passed as j36159 on M5 at the selected candidate, exit 0, in about 60 seconds. It scanned all 16,643 commits reachable through `--single-worktree --all HEAD`, without a `since` restriction, using the unchanged 181-row ledger. It reported no secret findings and seven existing informational `opaque-history-blob-decoded-coverage-unavailable` notices; the raw log preserves those coverage limits. Python 3.13 was first on PATH, including nested commands; the actual host reported Python 3.13.17, Git 2.54.0 and pinned Gitleaks 8.30.1. The pinned scanner archive's SHA-256 was verified. Source-file hashes in the executed job checkout matched the selected candidate's scanner, ledger and toolchain manifest.

The postflight confirmed that all canonical public heads and tags stayed unchanged before and after the scan, the scratch clone's old refs stayed unchanged, and all 77 old tags still resolve. The historical packet retains the raw job log and job receipt, preflight and postflight identities, public-ref snapshots, old-ref snapshots, bundle and bundle verification. No public ref or ledger was changed. Earlier queue requests j36139, j36147 and j36148 were cancelled before execution; only j36159 ran that scan, after an M5 core became available. These results qualify the earlier candidate, not the later selected core.

## Approved release order

Once AWS validation, the source review, account readiness and the final candidate's qualification are complete, give Ian the reviewable merge, tag, account changes, channel limitations and docs-site result. After his release approval, push only `v1.0.0` on the prepared merge. The ordinary tag workflow qualifies, stages, screens, signs, publishes, checks public installs and promotes latest without an approval pause. Advance public main to the merge only after that workflow passes, then land the refreshed site in the same release session. Preserve old heads, tags, releases and history; no force push.

The rehearsal intentionally skips channel authentication, uploads, directory acceptance and public installs. Those remain first exercised by the tag. A failed publication may leave some channels public; inspect the failed job before retrying because a PyPI version cannot be replaced. A failed promotion may have moved GHCR latest before GitHub latest; inspect both and retry only the failed promotion job from the same run, as the operator instructions specify.
