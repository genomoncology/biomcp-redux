# 0285 — Report replay misses with their reason through one mapping

Status: COMPLETE.
Landed: 42a575bb4b3ed3503ca16c6341b1009c110e09c6

Milestone: 5

## Outcome

A recording miss or an exhausted recording gives the source outcome reason `replay_miss`, as architecture section 4.4 says, through one shared mapping from request errors to reasons in `biomcp-query` that replaces the 24 hand-copied functions. `serve --replay` answers a repeated identical call from the replay cache instead of failing as exhausted.

- Why: the milestone 4 code review found that `Reason::ReplayMiss` is never produced and that a replay server cannot answer the same call twice.
- Starts from: `crates/biomcp-types/src/source.rs` line 181, `request.rs` lines 439 to 456, `status.rs` lines 24 to 27, the copied mappers such as `article/evidence/graph.rs` line 229 and `gene/sections/oncokb.rs` line 85, and `record.rs` lines 252 to 256 and 365 to 374.
- Keeps: every other reason mapping and replay's fail-on-miss rule for new requests.
- Changes: one mapping function, the call sites and the replay server's cache read.
- Proof: a case with a missing recording asserts reason `replay_miss`; a replay server case sends one call twice and gets the same answer; a ratchet or test fails if a new local copy of the mapping appears.
- Defers: nothing.
