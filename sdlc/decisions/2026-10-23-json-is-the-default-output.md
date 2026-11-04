# JSON is the default output

Decided 2026-10-23 by Ian.

- Every command prints compact JSON by default, in the envelope `planning/architecture.md` describes.
- biodata's schemas shape the JSON. Each schema is generated from the Rust types, checked in, versioned and failed on drift.
- The CLI and MCP serialize the same response type. `--format md` prints a Markdown view for people, rendered from the same type and holding nothing the JSON lacks.
- Why: an agent that chains calls needs exact identifiers, statuses, counts and partial results it can read without parsing prose. Size matters more than format, so the envelope omits nulls, empty lists, unrequested sections and repeated commands.
