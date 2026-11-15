# ORCID: synthetic works and person replies

ORCID's public API terms are noncommercial, so these replays hold no real ORCID record (0920). Every work title, translation, source name, put-code, date and external ID (PMIDs 123 and 124, DOIs under 10.1000/synthetic) is authored, and the person reply names "Synthetic Author".

The ORCID iD 0000-0000-0000-0925 is invented: its check digit is valid, but it lies outside the blocks ORCID issues iDs from. Until 0920 these replies used 0000-0002-4299-3018, a real researcher's iD, with the same authored bodies.

Each works reply mixes `public`, `PUBLIC`, `limited` and `private` work summaries.

- `visibility/` also keeps the person reply.
- `refused-member/` adds a malformed work member, so the reply is partial.
- `translated-title/` gives one work a translated title; the Rust test `author-orcid/refused-orcid-work-records-carry-facts` in `crates/biomcp-adapters/tests/mapped_refusals/source_pages.rs` reads it, and no case file does.
