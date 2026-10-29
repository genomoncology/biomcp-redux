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
