# 0096 — Port each 0.9 variant search filter that has a known-positive case

Status: OPEN.

Milestone: 2

Depends on: 0088
Depends on: 0092

## Outcome

Each 0.9 `search variant` filter that maps to a MyVariant field, such as `--significance`, `--max-frequency`, `--min-cadd`, `--consequence`, `--review-status` and `--revel-min`, is ported with one spelling only after a case whose captured facts match MyVariant's own query on the capture date. A filter without such a case is dropped with a reason or stays undecided.

- Starts from: variant.md section 6 and its parity table; the rulings; slices.md "Every filter has a known-positive case".
- Risks the proof must cover: a filter mapped to the wrong field returns a valid zero; a filter that needs a source BioMCP does not call, such as CGI, is never offered.
- Proof: one known-positive case per ported filter with its direct MyVariant query recorded beside it; the parity table updated per filter; MCP parity. Then make landing-gate runs through pm job after current main.
