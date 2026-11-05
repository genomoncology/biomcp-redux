# Run release sections before the cut

After the required product changes have landed, select one immutable main commit and run the sections below through `pm job` on their actual hosts. Keep each section's command, host, source commit, inputs, exit and output location in its job record. Fix a failed section and rerun that section. A product change selects a new main commit; a tooling change names its actual revision and repeats only the affected section. The candidate cut cites the completed main preflight.

This document prepares the commands. It does not establish a passing preflight. Do not start product runs while the required product work is still landing.

## Select sources and inputs

`MAIN_REVISION` is the full main commit selected after the required changes land. It is the product source, runner source and fixture source for this preflight. `DRIVER_REVISION` is the actual commit of an outer workflow or wrapper if it differs. Record both. `BIOMCP_BUILD_REVISION`, when needed by a Git archive, must identify the source actually being compiled.

Run commands from a clean checkout of selected main using its `release/toolchain.json` pins. The artifact's `release.json`, inspected wheel and `version --verbose` must agree on the product commit and binary hash. A prepared runner's `bootstrap.json` names the source actually used to compile its case and tool runners.

The installed runner expects its own compiled commit in the product's verbose version (`crates/biomcp/tests/runner/housekeeping.rs`) and uses its linked schema types. The standalone installed-smoke command also requires product and runner source to equal the selected candidate. Therefore use product, runners and fixtures from the same `MAIN_REVISION`. A newer runner checking an older executable is not proof of new main. Do not spoof stamps, skip housekeeping or weaken schema checks to make that pairing pass.

At preparation time, the newest retained ordinary hosted artifacts came from superseded commit `c9bd901d50a506e53d168b189315d34a2c940cef`: product and runners both name that commit. Earlier `1bb97e64f41634b19efc42b937b195e55a20ece4` product evidence and its separate recovery tooling remain historical. Neither set qualifies main after new product changes. Keep their failures and successful sections under their original identities.

| Input | Contents |
| --- | --- |
| `NATIVE` | One platform's complete producer output: wheel, `release.json`, archive and checksum, registry record, licenses and desktop bundle where produced |
| `PROOF` | Matching case runner, tool runner, copied fixtures, copied release helpers and `bootstrap.json`, produced by `release/gate.py --prepare-only` |
| `ALL_NATIVE` | One subdirectory per native platform, each containing producer output from the same selected main commit |
| `STAGED` | A new output directory for the complete publication payload; absent before staging |
| `SCRATCH` | A fresh installation directory owned by one gate invocation |
| `EVIDENCE` | Retained outputs, reports and failed synthetic fixtures; never a home directory or the whole operating-system temporary directory |
| `VERSION` | The selected product's Cargo version, also present in its producer records |
| `BINARY` | An installed or unpacked executable from `NATIVE`, whose hash the channel helper checks against the wheel |
| `PLUGIN_SOURCE` | The unpacked `biomcp-plugin.zip` from `STAGED` |

Copy outputs needed by another job into the owning team's retained evidence store before producer cleanup. Record their product revision, runner revision, hashes and producer job. Do not retain credentials, Git authentication files or unrelated runner temporary files.

## Order and commands

Independent rows in one phase may run in parallel when outputs do not collide. Run them through tracked pm jobs on the named host, for example:

```sh
TMUX_PANE=%247 pm job run --machine yellow 0640 "$MAIN_REVISION" -- make secrets-full
```

Paths and environment inputs must exist on that job's host. A missing artifact, executable, host or tool is a not-run or failed section, never a pass. These are existing commands; the local sections require no new launcher or framework.

### 1. Source checks and mutations

| Section | Command | Host and inputs |
| --- | --- | --- |
| Dependency policy | `make audit` | M5 or Yellow; selected main and manifest-pinned `cargo-deny` installed first |
| Complete history secret scan | `make secrets-full` | M5 or Yellow; complete selected-main history, not a shallow archive or incremental scan |
| Release schema comparison | `make schema-release-check` | M5 or Yellow; selected main and its committed schema baseline/reviews; does not update the baseline |
| Documentation and metadata | `make docs-check` | M5 or Yellow; selected main and its normal build inputs |
| Mutation tool, if the pin is absent | `make release-mutations MUTATION_REPORT="$EVIDENCE/mutation-install.json" MUTATION_ARGS="install --started $STARTED --output $MUTATION_TOOL"` | M5 or Yellow; a new owned tool directory and the existing shared 90-minute budget |
| Mutation plan | `make release-mutations MUTATION_REPORT="$EVIDENCE/mutation-plan.json" MUTATION_ARGS="plan --previous $PREVIOUS --started $STARTED --output $MUTATIONS --measure --shards 1"` | Clean selected main and previous compared commit; initial calibration selects the existing 20-mutant sample |
| Mutation shard | `make release-mutations MUTATION_REPORT="$EVIDENCE/mutation-shard.json" MUTATION_ARGS="shard --plan $MUTATIONS/plan.json --index 0"` | The plan's native host, selected main and retained plan/inventory |
| Mutation collection | `make release-mutations MUTATION_REPORT="$EVIDENCE/mutation-collect.json" MUTATION_ARGS="collect --plan $MUTATIONS/plan.json"` | The same plan and actual shard outputs; retain survivors, timeouts and unrun work alongside the exit |

`STARTED` is one Unix start time shared by the mutation actions, not reset for a shard retry. Where retained native calibration supplies measured cost and baseline overhead, use the existing plan's `--cost-seconds` and `--baseline-seconds` options instead of repeating calibration. Collection success does not mean every possible mutant was caught; review its actual scope and outcomes.

### 2. Native production, installed gates and cold Linux

| Platform | Real host | Existing production command |
| --- | --- | --- |
| Linux x86_64 | Native Linux x86_64, including `ubuntu-24.04`; Yellow locally | `make release-linux`, with `BUILD_IMAGE` from manifest key `manylinux-x86_64` |
| Linux arm64 | Native Linux arm64, including `ubuntu-24.04-arm`; M5 runs its Linux arm64 containers | `make release-linux`, with `BUILD_IMAGE` from manifest key `manylinux-aarch64` |
| macOS arm64 | M5 or `macos-15` | `python3 release/artifacts.py build --revision "$MAIN_REVISION" --output "$NATIVE"` |
| Intel macOS | `macos-15-intel` | The same native production command, executed on Intel macOS |
| Windows x86_64 | `windows-2022` | `python release/artifacts.py build --revision "$MAIN_REVISION" --output "$NATIVE"` |

`release/linux.sh` already builds the manylinux artifacts, prepares matching runners, runs installed forms without the checkout and checks the matching runtime container. Retain its actual install report, raw log and container receipt. Do not rebuild that platform merely to repeat those checks.

| Section | Command | Host and inputs |
| --- | --- | --- |
| Native installed forms | `python3 release/gate.py --artifacts "$NATIVE" --scratch "$SCRATCH" --python python3` | Matching native host; product and runner source both selected main; use `python` on Windows and a writable job-owned `BIOMCP_EVIDENCE_DIR` |
| Prepare retained runners without rebuilding product | `python3 release/gate.py --prepare-only --artifacts "$NATIVE" --scratch "$PROOF"` | Matching source/platform; reuse an existing prepared proof when available |
| Prepared installed gate | `python3 release/gate.py --prepared --artifacts "$NATIVE" --scratch "$SCRATCH" --report "$EVIDENCE/install-report.json" --python python3` | Matching platform; first copy only the prepared proof into fresh `SCRATCH`; use Windows Python there |
| Linux runtime container alone | `python3 release/container_check.py "$IMAGE" "$ARCH" "$CONTAINER_PROOF" "$VERSION"` | Matching Linux architecture and retained same-main image; proof directory contains binary, licenses, runners and fixtures; `ARCH` is `amd64` or `arm64` |
| Cold Linux matrix | `make release-cold COLD_REPORT="$EVIDENCE/cold-report.json"` | M5, Docker and both pinned images; empty container homes; existing command collects every failed stage across both architectures |
| Cold Linux architecture retry | `make release-linux CONTAINER_ARGS="--platform linux/$ARCH"` | Selected main with `BIOMCP_RELEASE_COLD=1`, `BIOMCP_BUILD_REVISION="$MAIN_REVISION"`, corresponding pinned `BUILD_IMAGE` and owned `BIOMCP_RELEASE_REPORT`; only that architecture's existing cold recipe |

Label M5 x86_64 container runs as emulated. They do not replace the native Linux x86_64 result. The cold matrix retains product outputs in `cold-artifacts-amd64` and `cold-artifacts-arm64`, then removes its temporary prepared proofs. Final hosted native ZIPs also do not currently retain runner executables. If a later consumer needs `PROOF`, prepare it once from the same main source with the existing command above, then retain and reuse it.

The current dedicated recovery action is fixed to an older product commit. A prerequisite must parameterize the existing `recover-section.yml` and `release/section.py` at an explicit immutable main revision before it can supply these platforms independently for new main. Product, runners and fixtures must come from that checkout; record the actual outer driver revision separately. Keep the rehearsal-only repository/ref guard, pinned tools, narrow owned uploads and publishing disabled. Until that lands, the fixed-product action cannot serve as a main preflight. This recipe has no substitute local Intel Mac host.

### 3. Installed smoke and local channels

| Section | Command | Host and inputs |
| --- | --- | --- |
| Full isolated installed cleanup smoke | `python3 scripts/installed_smoke_section.py "$NATIVE" "$PROOF" "$EVIDENCE/installed-smoke" --candidate "$MAIN_REVISION"` | Native Linux x86_64; retained matching product, runners and fixtures; new output directory; no compilation in this section |
| Pinned public 0.9 installer | `python3 release/published.py legacy-installer --version "$VERSION" --revision "$MAIN_REVISION" --artifacts "$NATIVE"` | Native Unix host matching the archive, with curl; optional `--installer-script` supplies the exact pinned cached public script |

The legacy helper verifies the pinned 0.9.3 installer bytes and redirects only its exact archive and checksum requests to the existing loopback server. It publishes nothing and uses no source API.

Windows release proof is the hosted native `release/gate.py` result, as [the release decision](../sdlc/decisions/2026-11-05-release-finds-no-bugs.md) records. The VM wrapper remains outside the release path after its missing-file fault. After machine diagnosis, its existing command is `sh scripts/check_windows_release.sh 0640 "$ARTIFACT_REVISION" "$NATIVE"` on Gator, under an owning pm job with the same ticket label. It must report actual artifact and tooling revisions. An old artifact can exercise that wrapper but cannot qualify new main.

### 4. Complete payload preparation and screening

| Section | Command | Host and inputs |
| --- | --- | --- |
| Publication payload | `python3 release/workflow.py stage --artifacts "$ALL_NATIVE" --plugin . --version "$VERSION" --revision "$MAIN_REVISION" --output "$STAGED"` | M5 or Yellow; all five same-main native outputs, both Linux container archives and selected-main plugin source |
| Staged artifact secret scan | `python3 release/secrets.py artifacts "$STAGED"` | Actual complete staged bytes and pinned scanner; includes archive members and compiled binaries |
| Staged Homebrew formula | `python3 release/gate.py --artifacts "$NATIVE" --scratch "$SCRATCH" --staged-formula "$STAGED/biomcp.rb"` | Run separately on both macOS architectures with their matching native output and the same universal staged formula |
| Claude plugin | `python3 release/published.py plugin --version "$VERSION" --revision "$MAIN_REVISION" --artifacts "$NATIVE" --plugin-source "$PLUGIN_SOURCE" --binary "$BINARY" --claude "$CLAUDE"` | Native Unix host with real Claude CLI; same-main staged plugin and executable; no model invocation |
| Attestation subject inventory | `python3 release/workflow.py subjects --artifacts "$STAGED" --output "$EVIDENCE/subjects.sha256"` | Complete staged tree; new output outside that tree |

Staging does not scan the payload by itself. Screen those bytes before treating stage preparation as complete. History screening does not replace artifact screening. Classify a failure and fix its owning source; do not add a broad binary or archive allowlist. Each staged Homebrew run installs the actual two-architecture formula from the checked local archives, without fetching a public release. Unpack the checked stage's plugin archive for the plugin row. Its helper installs the marketplace in an isolated home, reads the installed launch declaration and calls the selected executable's MCP `help`, without invoking a model or a source API.

### 5. Ordinary hosted rehearsal and signed artifacts

After the independent sections pass, run the existing normal release workflow once from selected main in the approved public rehearsal repository, with `rehearsal=true` under an approved non-release ref, as [the public rehearsal decision](../sdlc/decisions/2026-11-02-public-rehearsals.md) describes. Do not replace the normal workflow with a recovery workflow for this check. This document does not dispatch it or approve a release tag.

```sh
gh workflow run release.yml --repo genomoncology/biomcp-release-rehearsal --ref "$REHEARSAL_REF" -f rehearsal=true
```

Bind the actual run commit, tree and normal workflow bytes to selected main. Product and runner stamps name the source actually compiled. Retain actual job results for candidate policy/history, all five native platforms, both Linux containers, staging and artifact screening, signed subjects, both staged Homebrew architectures, and downstream rehearsal jobs. A successful older product is not evidence for new main.

For each downloaded staged subject, first compare the file with the retained subject inventory, then verify its signed provenance with a current GitHub CLI:

```sh
gh attestation verify "$ARTIFACT" --repo genomoncology/biomcp-release-rehearsal --signer-workflow genomoncology/biomcp-release-rehearsal/.github/workflows/release.yml --source-ref "$REHEARSAL_SOURCE_REF" --source-digest "$MAIN_REVISION"
```

Use the actual full source ref and require the certified run to match the retained run. Every staged regular file must appear exactly once in the subject inventory. Checksums alone do not establish signed provenance. If screening stops staging before signing, subjects and downstream jobs are not run; native successes alone do not make the rehearsal pass.

The six downstream channel jobs cover GitHub assets, GHCR, PyPI, Homebrew, the installer site and MCP Registry. Dispatch skips their authentication and publication steps. The five post-publication install jobs are also no-ops, and latest-pointer promotion stays unchanged. Record these boundaries: their summaries do not prove real public-channel installation, registry acceptance or promotion. Publication still needs Ian's approval.

## Tooling checks do not qualify product

`sh scripts/check_release_channels.sh` checks channel helpers with small test candidates. `sh scripts/check_hosted_section.sh` checks recovery tooling. `python3 scripts/installed_smoke_section.py check` checks the standalone smoke helper. Run those existing focused checks when their tooling changes; they do not replace the real-host sections above.
