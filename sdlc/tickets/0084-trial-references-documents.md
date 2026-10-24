# 0084 — List trial references and documents and save one document

Status: OPEN.

Milestone: 2

Depends on: 0082
Depends on: 0052

## Outcome

`get trial NCT… references` returns the registry's references with PMID, reference type and citation, and names `get article PMID` as the next step. `get trial NCT… documents` lists the registry's documents, and `trial document NCT… FILE --save PATH` saves one under the save rule, reusing 0052's save code.

- Why: T21 links VISION to its primary paper; documents keep a 0.9 capability without reading PDFs.
- Starts from: trial.md (T21, documents, budgets); the rulings; `decisions/2026-10-24-pdfs-are-downloaded-not-read.md`; 0052's download boundary and save code.
- Risks the proof must cover: a reference type stays as the registry states it; the document URL comes from the registry's own fields and fixed host, never built from a filename the caller typed; a save never overwrites without `--replace`; HTTP mode refuses the save.
- Proof: T21 asserts PMID 34161051 and its reference type among NCT03511664's references and follows the step to `get article`; a documents manifest on a captured record with documents; a save of a small captured document with path, bytes and SHA-256; a FILE the manifest does not list is refused with no request; the HTTP-mode refusal; MCP parity; measured ceilings. Then make landing-gate runs through pm job after current main.
- Defers: reading document text.
