pub(super) fn cases() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("excluded-member", "    expect: {'/enum': {excludes: hgnc}}", "excludes: unexpected member"),
        ("excluded-wrong-type", "    expect: {'/type': {excludes: hgnc}}", "excludes requires an array"),
        (
            "mcp-wrong-value",
            "  - mcp: [schema, biodata.source/1]\n    expect: {'/type': integer}",
            "/type: expected",
        ),
        (
            "mcp-bytes",
            "  - mcp: [schema, biodata.source/1]\n    max_bytes: 0",
            "max_bytes: expected <= 0",
        ),
        (
            "mcp-tokens",
            "  - mcp: [schema, biodata.source/1]\n    max_tokens: 0",
            "max_tokens: expected <= 0",
        ),
        (
            "mcp-pointer",
            "  - mcp: [schema, biodata.source/1]\n    expect: {'/enum': {count: 0}}",
            "count: expected 0",
        ),
        ("mcp-is-error", "  - mcp: []", "MCP is_error mismatch"),
        (
            "unequal-success",
            "    mcp: [schema, biomcp.error/1]",
            "CLI/MCP parity: unequal results",
        ),
        ("no-command", "  - expect: {}", "step requires run or mcp"),
        (
            "unknown",
            "    expect: {'/type': {equal: string}}",
            "unknown assertion operator equal",
        ),
        (
            "empty-operators",
            "    expect: {'/type': {}}",
            "assertion object requires an operator",
        ),
        (
            "object-equality-explicit",
            "    expect: {'/type': {type: string}}",
            "unknown assertion operator type",
        ),
        (
            "nested-wildcard-missing",
            "  - run: [schema, biodata.problem/1]\n    expect: {'/properties/record/anyOf/*/$ref': []}",
            "actual <missing>",
        ),
        (
            "count-fraction",
            "    expect: {'/enum': {count: 1.5}}",
            "count requires a nonnegative integer",
        ),
        (
            "rank-negative",
            "    expect: {'/enum': {rank_of: {hgnc: -1}}}",
            "rank_of requires positive integer ranks",
        ),
        (
            "count-negative",
            "    expect: {'/enum': {count: -1}}",
            "count requires a nonnegative integer",
        ),
        (
            "count-range",
            "    expect: {'/enum': {count: {min: 1}}}",
            "count requires a nonnegative integer",
        ),
        (
            "rank-shape",
            "    expect: {'/enum': {rank_of: [hgnc]}}",
            "rank_of requires a nonempty object",
        ),
        (
            "rank-zero",
            "    expect: {'/enum': {rank_of: {hgnc: 0}}}",
            "rank_of requires positive integer ranks",
        ),
        (
            "rank-range",
            "    expect: {'/enum': {rank_of: {hgnc: '<3'}}}",
            "rank_of requires positive integer ranks",
        ),
        (
            "includes-type",
            "    expect: {'/type': {includes: string}}",
            "includes: expected an array",
        ),
        (
            "count-type",
            "    expect: {'/type': {count: 6}}",
            "count: expected an array",
        ),
        (
            "rank-type",
            "    expect: {'/type': {rank_of: {string: 1}}}",
            "rank_of: expected an array",
        ),
        (
            "count-wrong",
            "    expect: {'/enum': {count: 1}}",
            "count: expected 1, actual 7",
        ),
        (
            "includes-missing",
            "    expect: {'/enum': {includes: absent}}",
            "includes: expected member \"absent\"",
        ),
        (
            "rank-missing",
            "    expect: {'/enum': {rank_of: {absent: 1}}}",
            "rank_of absent: expected 1, actual <missing>",
        ),
        (
            "rank-wrong",
            "    expect: {'/enum': {rank_of: {pubmed: 1}}}",
            "rank_of pubmed: expected 1, actual 3",
        ),
        (
            "wildcard-type",
            "    expect: {'/type/*': []}",
            "wildcard requires an array",
        ),
        (
            "wildcard-missing",
            "    expect: {'/enum/*/missing': null}",
            "actual <missing>",
        ),
        (
            "bad-pointer",
            "    expect: {'enum': []}",
            "expected a JSON pointer",
        ),
        (
            "bad-escape",
            "    expect: {'/~2': []}",
            "invalid JSON pointer escape",
        ),
        (
            "ceiling-shape",
            "    expect: {'/type': {max_bytes: '8'}}",
            "max_bytes requires a nonnegative integer",
        ),
        (
            "validate-all-steps",
            "    expect: {'/type': integer}\n  - run: [schema, biodata.source/1]\n    expect: {'/type': {unknown: null}}",
            "unknown assertion operator unknown",
        ),
    ]
}
