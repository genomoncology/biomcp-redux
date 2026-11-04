# Ratchets are enforced by lint

Decided 2026-10-23 by Ian.

- The ratchets from the review of BioMCP 0.9 are enforced by tools, not by reviewers' memory: file and function size limits, crate dependency direction and one transport, with the rest of the table in `planning/rust-standards.md`.
- No file over 600 lines, and no adapter file over 300. No function over 80 lines, through `clippy::too_many_lines`.
- Every ratchet starts at its 1.0 target, because the code starts empty, and `make landing-gate` fails on any breach.
- Why: 0.9 grew to 303,616 lines in `src/`, doubling in six months, with a 10,718-line file and a 645-line function threshold that flagged nothing.
