# Compare replies while restructuring

Set `BIOMCP_TRANSCRIPT_DIR` to an empty output directory when running the case runner. It records exact stdout bytes, exit codes, surfaces and arguments before assertions or JSON parsing. Without the variable, tests run normally. Keep transcript directories outside Git.

Run the same selected cases and trials at the base and head commits. Then run `python3 scripts/reply_diff.py BASE HEAD` through `pm job`. A difference exits nonzero and names the step and JSON pointer, or a character offset for text and formatting differences. The default report stops after twenty differences. This comparison is optional and does not run in the routine gate.

Case keys contain the relative case path and zero-based step index. Each case transcript holds its CLI and MCP executions in order. Walk pages append to that same step. MCP records the raw server stdout before parsing. Ordinary trial keys contain the trial name, an argument hash and the number of earlier identical argument lists. The launcher allocates those keys before execution. Isolated environment and checkout paths become placeholders for hashing and comparison.

The comparison masks the version command’s package version line and verbose commit, config and cache lines. It masks the executable command field and launch line from `mcp-config`. It replaces isolated environment and checkout prefixes only in argument path tokens. Source data and other stdout bytes and exit codes remain significant. Calibration uses two commits with unchanged source in separate checkout paths. Every additional mask requires a calibration finding and a comment in the script.

Capture covers YAML steps and trial commands whose existing consumer obtains complete stdout and status. It leaves process transport and lifecycle checks unchanged. The script lists every trial with uncaptured commands or no captured command. A structure change touching these areas must use those trials' own assertions. The architecture work decides whether to extend this coverage.
