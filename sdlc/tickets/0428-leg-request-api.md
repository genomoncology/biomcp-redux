# 0428 — Give the transport one request path

Status: OPEN.

Milestone: 5

Depends on: 0427

## Outcome

`Leg` exposes one request function with a small options type in place of 32 combination methods, the generic decoder moves out of `xml.rs`, positional tuples become named structs, and paging cursors share one implementation in biomcp-types.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`.
- Starts from: `crates/biomcp-fetch/src/leg*`, `xml.rs`, `cache_request.rs`, the 19 `at_offset` and 10 `restart` copies and six `page!` macros in biomcp-types.
- Rules: no behavior change; schedule after the 1.0 feature tickets to avoid collisions.
- Proof: existing cases pass; single-caller methods are gone.
- Changes: the files above.
- Defers: nothing.
