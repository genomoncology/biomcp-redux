# 0201 — Check every persisted response field for a reflected key

Status: OPEN.

Milestone: 1

Depends on: none

Priority: first in the next lane set, ahead of new slice work.

## Outcome

No response field that BioMCP records, caches or returns can carry the active request key. The 0197 adversarial review, pinned at 85e6ba86, found that `contain_response` in `crates/biomcp-fetch/src/containment.rs` scans only Content-Type and body, while `crates/biomcp-fetch/src/request.rs` copies the provider's Content-Range into the wire reply, the recording exchange and the cache entry (`cache_request.rs`, `cache.rs`). A provider that reflected the key in Content-Range would bypass the guard and persist the key. No real key was read or observed leaking; the path is unsafe by construction.

- Starts from: `contain_response`, the wire reply built in `request.rs`, and `crates/biomcp/tests/runner/credential_containment.rs`.
- Keeps: the existing body, decoded-JSON and media checks, the BadReply failure, and the rule that a key never appears in a request key, recording, trace, error or envelope.
- Changes: `crates/biomcp-fetch/src/containment.rs`, `crates/biomcp-fetch/src/request.rs`, `crates/biomcp/tests/runner/credential_containment.rs`, and the fetch crate's unit tests.
- Fix: `contain_response` receives the whole wire reply, so every persisted metadata field (Content-Type, Content-Range and any field added later) passes through one check before recording or caching. Replay and cached reads apply the same check to the stored metadata. Retry-After stays covered if it is ever persisted. No second scanner.
- Proof: a stub provider reflects a placeholder key in Content-Range on a keyed source. The request fails with BadReply. No recording, cache entry, trace or envelope contains the placeholder, checked by byte search over the recording directory, the cache file and stdout. The same case with an unrelated Content-Range still records and caches. Offline landing gate passes.
- Defers: the review's other open findings, which arrive with the 0197 report.
