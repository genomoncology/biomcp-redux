# 0346 — Explain the two response export sites where a developer adds a type

Status: COMPLETE.
Landed: ddf056e349706ae0c3a6eaa18db36553c40252f9

Milestone: 5

## Outcome

A short comment beside the private response module in the types crate says that a new public response type needs both its export inside the response module and its re-export at the crate root, and that `unreachable_pub` reports a missing one.

- Why: the coordinator's RCA (m10243, 2026-10-27) found that 0340, 0341 and 0342 each added an inner response export without the crate-root export, which caused three integration failures. Logged under 0060.
- Starts from: the types crate's `lib.rs` and its private response module.
- Rules: comment only. Ordinary compilation stays the enforcement; no new checker, job, dependency or blanket export.
- Proof: the landing gate passes; the comment sits at both export sites.
- Changes: the types crate's `lib.rs` and response module root.
- Defers: nothing.
