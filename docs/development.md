# Development

Run `make schema-generate` after changing schema types. The command uses the schema contract test to check compatibility before writing the checked-in schemas. It fails unless the named generation test runs and passes.

Run `make schema-release-check` to compare the generated schemas with the recorded release baseline and check for drift. This command leaves the checked-in schemas and release baseline unchanged.

## Compare replies while restructuring

Set `BIOMCP_TRANSCRIPT_DIR` to an empty output directory when running the case runner. It records exact stdout bytes, exit codes, surfaces and arguments before assertions or JSON parsing. Without the variable, tests run normally. Keep transcript directories outside Git.

Run the same selected cases and trials at the base and head commits. Then run `python3 scripts/reply_diff.py BASE HEAD` through `pm job`. A difference exits nonzero and names the step and JSON pointer, or a character offset for text and formatting differences. The default report stops after twenty differences. This comparison is optional and does not run in the routine gate.

Case keys contain the relative case path and zero-based step index. Each case transcript holds its CLI and MCP executions in order. Walk pages append to that same step. MCP records the raw server stdout before parsing. Ordinary trial keys contain the trial name, an argument hash and the number of earlier identical argument lists. The launcher allocates those keys before execution. Isolated environment and checkout paths become placeholders for hashing and comparison.

The comparison masks the version command’s package version line and verbose commit, config and cache lines. It masks the executable command field and launch line from `mcp-config`. It replaces isolated environment and checkout prefixes only in argument path tokens. Source data and other stdout bytes and exit codes remain significant. Calibration uses two commits with unchanged source in separate checkout paths. Every additional mask requires a calibration finding and a comment in the script.

Capture covers YAML steps and trial commands whose existing consumer obtains complete stdout and status. It leaves process transport and lifecycle checks unchanged. The script lists every trial with uncaptured commands or no captured command. A structure change touching these areas must use those trials' own assertions. The architecture work decides whether to extend this coverage.

## Exercise installed artifacts

The release rehearsal copies all of `tests/` outside the checkout and runs the complete `cases` target against each installed executable. `BIOMCP_TEST_EXECUTABLE` selects the installed executable and `BIOMCP_TEST_FIXTURES` selects the copied tree. Every case directory registers the same Rust trials. Use ordinary feature names to select the cache publication, HTTP and AlphaGenome trials.

Reports include the YAML case count, total trial count and elapsed time. The first complete installed run measures the suite timeout; later installation channels use that measurement plus 15 percent. The platform job bounds the first measurement. Fixture path checks reject direct manifest-directory reads outside the launcher.

The cases target still contains in-process schema registry and assertion boundary tables, parity validation, synthetic adapter projection and budget calculations. These execute during both routine and installed runs, but their library calls do not test the installed executable. Their removal or replacement belongs to the in-process trial cleanup. Repository ratchets remain a separate target.

## Measure command coverage

`make coverage` runs `cases` against an instrumented product binary. The launcher gives every child a `bin-%m-%p.profraw` profile name. The script merges only those profiles and reports only the actual binary object through LLVM tools. The denominator contains linked product source lines and excludes generated protobuf code. Harness execution and in-process library contracts never count. `make library-tests` validates those libraries separately in the routine gate.

The PM sets `coverage.floor` from the first binary-only measurement and owns subsequent increases. `make unreached` displays line execution counts from the last complete collection. Raw child and harness profiles remain in the coverage target directory for diagnosis until the next collection.

## Inspect a candidate with mutation testing

The release tool manifest pins `cargo-mutants`. The mutation commands run in native jobs. They do not add a runtime dependency. Run `python3 release/mutations.py --help` for a short command list. Each command prints one summary and keeps full native logs beside its plan.

The release owner supplies the previous candidate commit and one shared Unix start time before tool installation. Installation requires a new output directory and keeps its Cargo home and tool prefix there. The output directory must belong to the current job. Add `SETUP/tool/bin` to that job's PATH before planning or running shards. Each separate job needs its own pinned tool environment. A temporary installation ends with its job and cannot supply later shards. Use that same start for `install --started START --output SETUP` and `plan --started START --previous COMMIT --output DIRECTORY`. The plan owns the deadline through `CAP_SECONDS` in the runner. Use a new output directory outside tracked source. Installation, discovery and every shard share the deadline. Queue shards together so queue wait does not consume their budget.

The first plan uses `--measure` to select `CALIBRATION_SIZE` mutants. Candidate plans use `--cost-seconds` from completed viable mutants and `--baseline-seconds` for the measured baseline overhead across their shards on the same machine, toolchain, worker settings and cases suite. The planner reserves elapsed setup time and baseline overhead before choosing the count. The planner considers changed product Rust files before the rule files in `RULE_FILES`. It records the whole inventory and every selected mutant name. Unselected mutants remain in the final unrun list. A cost drawn only from build failures cannot size a candidate run. A surviving mutant may need the whole cases suite even when other mutants fail quickly.

Run `shard --plan DIRECTORY/plan.json --index INDEX` as a separate job for each index declared with the plan's `--shards`. Each shard runs the complete `cargo test -p biomcp --test cases` target with two build and test workers. Each shard also runs its baseline. This command provides no baseline-skip option. It stops the native child process group when the shared deadline expires and keeps partial native outcomes.

Run `collect --plan DIRECTORY/plan.json` after the shard jobs finish. Attach `mutation-report.json` and the raw shard directories to the candidate. The report separates caught mutants, survivors, build failures, native timeouts and unrun mutants. It preserves native phase durations and reports the number that actually completed. Missing shards, baseline failures and tool errors fail collection. Survivors remain advisory. The release owner chooses at most ten survivors for new outside-in cases, starting with rule modules, and lists the rest.

A calibration that hits the cap before all selected mutants finish is incomplete. Do not call it a measured calibration or replace missing results with estimates from booked cores. The full cases duration may prevent the required calibration or assertion-deletion experiment from fitting the cap. The release owner must resolve that limit before running either experiment. A planted proof must use the same exact `status.rs` mutant with one owning assertion deleted and restored. Other assertions may also catch the mutant; check that premise before claiming a survivor.
