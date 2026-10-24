# 0104 — Reject reflected credentials before recording, caching or output

Status: COMPLETE.
Landed: 0121dc0ddf1f17f738ab19e3e38973530448201f

Milestone: 1

## Outcome

The shared fetch boundary rejects a response that reflects the credential sent with its request before any recording, cache admission, diagnostic or product output can contain that credential.

- Why: local TLS witness j10932 at 6363dc45 authenticated a synthetic key and echoed it in an ordinary OncoKB response field. The actual CLI leaked it into output and its recording. No real key or provider call was used.
- Starts from: `request.rs` exchange calls `record_wire` before typed decoding; 0095's `variant/oncokb-reflected-key-never-leaves-wire` reproducer.
- Changes: enforce credential containment once in the existing shared fetch path, covering recorded body and content type, cache admission, replay/cache reads and safe failure output. Reuse existing credential and response bounds. Preserve ordinary keyed requests and key reload. Do not add source-specific JSON replacement, another transport, new flags or logged credential values.
- Proof: the existing actual CLI synthetic echo fails before the fix and passes after it; no credential bytes appear in output, recording or cache. Cover success and error responses, contaminated replay/cache reads and ordinary keyed continuity. Run affected outside-in cases, formatting, all-target lint and existing ratchets through pm job. A combined landing gate supplies the full suite.
- Keeps: no actual key inspection, provider requests, package publication or rewriting user evidence. Keep the reproducer and sanitized logs; use only synthetic credentials.
