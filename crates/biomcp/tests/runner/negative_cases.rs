pub(super) fn cases() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("unknown", "    expect: {'/type': {equal: string}}", "unknown assertion operator equal"),
        ("empty-operators", "    expect: {'/type': {}}", "assertion object requires an operator"),
        ("count-negative", "    expect: {'/enum': {count: -1}}", "count requires a nonnegative integer"),
        ("count-range", "    expect: {'/enum': {count: {min: 1}}}", "count requires a nonnegative integer"),
        ("rank-shape", "    expect: {'/enum': {rank_of: [hgnc]}}", "rank_of requires a nonempty object"),
        ("rank-zero", "    expect: {'/enum': {rank_of: {hgnc: 0}}}", "rank_of requires positive integer ranks"),
        ("rank-range", "    expect: {'/enum': {rank_of: {hgnc: '<3'}}}", "rank_of requires positive integer ranks"),
        ("includes-type", "    expect: {'/type': {includes: string}}", "includes: expected an array"),
        ("count-type", "    expect: {'/type': {count: 6}}", "count: expected an array"),
        ("rank-type", "    expect: {'/type': {rank_of: {string: 1}}}", "rank_of: expected an array"),
        ("count-wrong", "    expect: {'/enum': {count: 1}}", "count: expected 1, actual 7"),
        ("includes-missing", "    expect: {'/enum': {includes: absent}}", "includes: expected member \"absent\""),
        ("rank-missing", "    expect: {'/enum': {rank_of: {absent: 1}}}", "rank_of absent: expected 1, actual <missing>"),
        ("rank-wrong", "    expect: {'/enum': {rank_of: {pubmed: 1}}}", "rank_of pubmed: expected 1, actual 3"),
        ("wildcard-type", "    expect: {'/type/*': []}", "wildcard requires an array"),
        ("wildcard-missing", "    expect: {'/enum/*/missing': null}", "actual <missing>"),
        ("bad-pointer", "    expect: {'enum': []}", "expected a JSON pointer"),
        ("bad-escape", "    expect: {'/~2': []}", "invalid JSON pointer escape"),
        ("ceiling-shape", "    expect: {'/type': {max_bytes: '8'}}", "max_bytes requires a nonnegative integer"),
        ("validate-all-steps", "    expect: {'/type': integer}\n  - run: [schema, biodata.source/1]\n    expect: {'/type': {unknown: null}}", "unknown assertion operator unknown"),
    ]
}
