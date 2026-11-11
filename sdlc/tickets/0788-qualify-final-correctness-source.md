# 0788 — Qualify the final correctness source

Status: OPEN.

Milestone: 5

Depends on: 0794
Depends on: 0795
Depends on: 0796
Depends on: 0785
Depends on: 0786

## Outcome

One immutable landed source containing the final correctness fixes qualifies for 1.0 through the existing release sections and one ordinary no-publication rehearsal. Every artifact, installed-channel result and signed subject is bound to its actual source and bytes. Earlier b380 evidence retains its original identity.

## Why

The November 10 review adds accepted runtime fixes after the earlier qualified sources. Select the ending source after those fixes settle; earlier artifacts retain their identities and cannot qualify changed runtime inputs.

## Proof

- Run the existing full suite/schema check, cold Linux matrix and one bounded mutation sample with fresh baseline/cost evidence for the widened three-package/eight-target matrix. Retain the complete j36572 result and its failed final assertion: 20 completed, 11 caught, 2 unviable, 7 survivors, no timeouts/errors and 1,251 unrun. Independent review requires 0789 to catch three worker-ownership/single-flight mutations. The two freshness recheck variants and two output-only variants are explicitly classified nonblocking; none is called equivalent. Preserve those findings rather than replacing the failed report or adding exclusions merely for a zero score.
- Preserve the completed historical three-survivor follow-up under its actual test-driver revision and demonstrated production equivalence to source 0ad. That equivalence covers those historical test-only edits, not the new runtime fixes. For the newly settled source, select mutation coverage from its changed behavior using the existing bounded mechanism; repeat only proof whose inputs or claim changed. Existing artifacts and failures keep their genuine identities.
- Run one ordinary rehearsal on an approved non-release ref. Retain actual native/installed/staged results and platform counts, complete artifact ZIPs/API digests, subject inventory and independently verified provenance. Authentication, publication and public installs skipped in rehearsal remain explicitly unproved.
- Use genuine ending Mac artifacts and matching runners for the existing installed cleanup, pinned 0.9 installer and real Claude plugin helpers. Prepare missing runners once; private homes, no model calls.
- Retain commands, hosts, producer/tooling revisions, exits and raw receipts on Yellow with receiver hash verification. Label emulated AMD64 cold execution separately from native workflow proof.

## Starts from

- `release/qualification.md` and existing release helpers after the accepted remaining-issue fixes. Select the landed source in pm progress before product runs. Inventory changed production, packaging, schema and test inputs before deciding what to repeat; do not assume earlier artifact equivalence survives runtime edits.

## Keeps

- M5/Yellow local builds through pm jobs, pinned tools, the existing mutation cap and truthful incomplete results. Reuse causally unchanged rights/WAL/benchmark/stress receipts with their original identities instead of repeating the full stress collection. No source edits, new runner or gate framework.

## Defers

- Public tag/release/site publication, hosting changes and signed-in operational checks keep their separate approvals and owning tickets. This rehearsal grants none.

## Changes

- Qualification evidence and this ticket's outcome record; no product changes.

## Qualification outcome

The qualified product source is `c5cf943d8417e789415a37295736476ed9b43790`. The final ordinary rehearsal is [42925424966](https://github.com/genomoncology/biomcp-release-rehearsal/actions/runs/42925424966), attempt 1, from owned ref `rehearsal-0788-c5cf943d8417-c6f262`. Its 22 jobs completed successfully. A later evidence-record commit does not change the source recorded in those artifacts.

Independent verification completed on Yellow `data/biomcp-redux-evidence/0788/rehearsal-c5cf943d-42925424966`. All 193 files were re-read and verified against `FINAL-SHA256SUMS`, digest `08c9788a128a5cff70805c43c341a4206484028b3a212158afb1708bb3104f0b`. The separately checked ending packet contains 145 files under `ENDING-SHA256SUMS`, digest `be0eb0747e9ce21277581fab57bb5ff9aa16a3f184362b19762fba0eb0b5e30b`. All 12 artifact ZIPs match their API SHA-256 digests. All 26 exact staged subjects have cryptographically verified attestations, using gh 2.92, bound to this source, ref, `release.yml` and run invocation. The attestation bundle digest is `5f08aa324c755cf3c19c2d285722652458c9c46e93107f648ab0a8bad97fdbde`. Run terminal time was 2026-11-11 01:50:04 UTC. Source tree is `b830ec438ca4e1fd722d096b8890ab6106db1e30` and workflow SHA-256 is `eca73a57e238fab363a547f1911c83d9613e56b37a9c4b6fc5aebd7bf1993fc0`.

### Final-source evidence

- Routine landing gate `j37296` tested and published this exact commit. All 780 CLI/MCP cases and 47 structural checks passed. The retained batch packet is Yellow `data/biomcp-redux-evidence/0821-0822/landed-c5cf943d8`, manifest `98c9d040c9ce33dab7becc6eaa8af3e057568e1c1457186d10386cd6c0659847`. Supplemental label witness `j37279` is retained there under `SUPPLEMENT-SHA256SUMS`, digest `fa6cd5f5486131ad49f0e0a8de0aae3e8367d812cc62f95d9f94fe4d77942664`.
- Final preflight `j37297` passed rights, pinned dependency audit, full secret screening and workflow checks on `c5cf943d8`, with Python 3.13 and Rust 1.95.0. Its exact tool versions, commands, source tree and section receipts are receiver-verified at Yellow `0788/preflight-c5cf943d`, producer manifest `bd7e17d59b80d77d560d28e9b74ac8c96f8c1e00545297cfda22048d5b40f46c` and archive manifest `d7525be996713dcf9f52192e0ad3082116912d528bd8d7173249ebbbbaf83356`.
- The final rehearsal supplied actual native Linux AMD64/ARM64, macOS AMD64/ARM64 and Windows AMD64 outputs, installed-package checks, staging, container qualification and staged-payload checks. Unix native case counts were 780 and Windows was 774. Both macOS native/staged Homebrew checks passed. Linux/Windows Homebrew was explicitly deferred by the workflow.
- Mac follow-up `j37305` passed installed cleanup and the pinned 0.9.3 installer using the genuine Apple Silicon artifacts from rehearsal 42925424966 and matching prepared runners. The wheel, archive and MCPB contain the same executable SHA-256 `d9e5a80d8e1a1d532f91f585812401702b9dd3fe46c41724eddae558bcb77f45`. Producer manifest SHA-256 is `061f113d7af8078119262172258c83f043002f97e0e9da648c14442cfc272efe`. Receiver evidence is `0788/mac-c5cf943d/manual-42925424966`, manifest `9de4b573a8f05d02486edf6ddb0a283bc45fe23ce734734981b6f11b85f76a59`.
- Real Claude plugin check `j37306` passed using that same installed executable and the final staged `biomcp-plugin.zip`, SHA-256 `7f4bf3bfb9fcda1edf64f0014dc24aec1c41e69f595445c70ed81fb33039956c`. The helper used an isolated home and the real installed Claude CLI. Exact command, executable identities and output are receiver-retained at `0788/mac-c5cf943d/plugin-42925424966`, manifest `61ab91455a0b1f7aa8069a14ce1060f07856eab60557a1fbf6cad5a70a37a342`.

### Mutation results and their scope

The bounded mutation campaign was run on `27c43260f29a9546fc13e82b6c43c94b24e429f6` against previous source `ca945a56ac403d0c823c6b701bc813a9270da8da`. Its first attempt `j37225` and the bounded workaround `j37237` remain preserved. The workaround retained the original plan bytes, selection and deadline. A successful baseline preceded all 39 selected mutations. The final collection completed all 39 with 20 caught, 18 unviable and one survivor, zero timeouts/errors, and 1,864 inventory mutations unrun. Its campaign exit remains 1.

The sole survivor was `NameTier::exact` forced false in `openfda_label/search/expanded.rs:21`. Supplemental `j37279` at actual driver `cf40f6c0c09c9bbddf195a94767d549f2b12f20a` demonstrated the existing runtime passes and the exact observed replacement fails the strengthened exact-field assertion. Ticket 0822 landed that test at the final source. This supplemental witness does not rewrite the earlier campaign score or qualify mutations outside that sample.

The original failure, workaround source and selection, complete native results, failed collection and supplemental witness remain at Yellow `0788/j37225-mutations-27c43260`, `0788/j37237-mutations-27c43260/{early,terminal}`, `0788/j37237-first-survivor` and `0788/label-exact-tier-cf40f6c0c` under the evidence root. The terminal summary explicitly records `failed-on-one-survivor`. Its receiver manifest is `bd852febdec31d7c3ba7433cf11a7dbf165e69cdf49432b330e41c85f33e391c`; the supplemental exact-tier packet manifest is `224708eb1f0af04874efe89692be2a4a487bdf011843663119794c8993ab9466`. Earlier 34d and 0ad mutation reports and their classified survivors retain their original identities and dispositions. No exclusions or scores were changed to obtain a clean report.

### Causal reuse of prior evidence

Full suite/schema proof `j36864` and cold Linux matrix `j36888` belong to `34d6e02d1ebeac4d55e44e05099854f15e718d57`. The cold results distinguish native ARM64 from emulated AMD64. They remain historical evidence for the same compiler, package assembly, platform and container mechanisms. Final `c5cf943d8` bytes and native AMD64 execution are established by the final ordinary rehearsal and its source-bound artifacts. Historical full-suite/schema packet `0788/j36864` has manifest `1d5a8113db9766b3f3263b3503c2b058cc83f240cdbf18888d24fa67cd31b3ea`; cold packet `0788/cold-34d` has manifest `9502029059bbb0dd8b26c7cd93ced56e4b56453aa20627265fc20b0157aadee9`.

The changes since those receipts affect query correctness, pagination, alias handling, source admission and presentation. Their owning focused proofs, later routine batches and final native rehearsal exercised the affected behavior. Rights, WAL, benchmark and stress receipts are reused only for causally unchanged inputs and retain their original source, commands and host. This record does not relabel historical full-suite or cold outputs as final-source runs.

The final portable-cursor change adds direct use of pinned `ring` 0.17.14, which already existed transitively through rustls, and changes cursor/store/config behavior. Final preflight audits the dependency graph and the final rehearsal builds/tests actual final artifacts on all five platforms. The final Linux x86 workflow also passed a fresh generated-schema baseline comparison, as recorded in `metadata/final-summary.json`. The 27c mutation sample predates these cursor changes. Portable-cursor ownership, caller controls, dependency rights and cross-core continuation have their own accepted original two-core red witnesses and exact final `j37295` proof in ticket 0821. The accepted affected-proof plan calls for this targeted proof plus final native/installed rehearsal. No final portable-cursor mutation sample exists, so the 27c campaign does not establish mutation coverage of the new cursor authentication or store paths.

### Remaining release work

The archive records 26 skipped rehearsal publication, authentication, public-channel install and promotion steps. All 31 pre-existing remote refs were unchanged. The one owned rehearsal ref remains retained for separately coordinated cleanup. Real PyPI OIDC acceptance, public publication and final public-channel installs remain work for the separately approved release session. Hosting deployment and signed-in AWS operational checks retain their own evidence, inputs and approvals. The no-publication rehearsal and this outcome record perform no public release or account mutation.
