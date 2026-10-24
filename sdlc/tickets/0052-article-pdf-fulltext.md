# 0052 — Report and download the open PDF that Semantic Scholar names

Status: OPEN.

Milestone: 1

Depends on: 0042
Depends on: 0044

## Outcome

`get article <input> pdf` reports the open PDF that Semantic Scholar's `openAccessPdf` names: its URL, open access status and license, or why there is none. `article pdf <ID> --save PATH` downloads that PDF to PATH and returns where it went. BioMCP never reads text out of the PDF. Both work with or without the user's key, through the CLI and over stdio MCP.

- Why: Ian ruled on 2026-10-24 that BioMCP downloads PDFs and does not parse them (`decisions/2026-10-24-pdfs-are-downloaded-not-read.md`). A person or agent hands the file to a PDF tool of their choice.
- Starts from: the held branch at 8c442619, whose credential-free download boundary, public-address checks, redirect checks, proxy refusal and recorded states carry over; the shared Semantic Scholar paper record call from 0042; 0044's bytes reply; the decision above.
- Removes from the branch: `pdf-extract` and its tree, the `unicode-normalization` and RUSTSEC-2026-0192 exceptions in `deny.toml`, the hidden worker command and its limits, the capped allocator, `TextOrigin::Pdf`, the `pdf_text` notice, `article text --pdf` and stored PDF parts. Revert architecture 2.1, 2.2 and the dependency ratchet to match.
- Section: `Section<PdfLink>` holds `url`, `status` (`OpenAccessStatus`) and `license`, from the paper record with `fields=openAccessPdf`. An empty URL or `closed` status is `Unavailable` with reason `not_open`. The section makes no download. Its `next` names `article pdf <ID> --save <ID>.pdf`. `all` includes `pdf`, because the section costs one paper-record call. A `fulltext` section that is `NotApplicable` or `Unavailable` with reason `not_open` names `get article <ID> pdf`.
- Download: `article pdf <ID> --save PATH [--replace]`. PATH is a file path, or an existing folder, which gets `<ID>.pdf`. The download accepts only `https` or `http` with a DNS host name that resolves to public addresses, sends no credential, follows at most three redirects checked by the same rules, reads at most 25 MB and requires a body starting with `%PDF-`. It writes to a temporary file beside PATH and renames it on success, so a failed download leaves nothing. An existing PATH is refused with a usage error unless `--replace` is given. When a configured proxy applies to the publisher host, the download is `Unavailable` with reason `proxy`. A 4xx is `rejected`, an HTML body is `not_pdf` and an oversize body is refused by size. The reply holds the absolute path, bytes, SHA-256, license, status and source URL. Nothing is cached or tracked.
- HTTP mode: `--save` over `biomcp serve --http` is refused with a usage error that gives the PDF URL.
- Keeps: 0043's JATS full text, 0044's supplement reader and every earlier article case.
- Proof: case 70 keeps its recorded states: `get article 17611497 pdf` asserts `not_open` with no download; `article pdf 22663011 --save` on the recorded 403 asserts `rejected`; `article pdf 33377972 --save` on the recorded HTML asserts `not_pdf`. A small synthetic PDF of BioMCP's own text, starting from a real capture, proves a save with its path, bytes and SHA-256, the folder form, the refusal of an existing file, `--replace`, and no file left after a cut body. Synthetic cases prove the redirect to an IP literal (no request sent), the size refusal, the proxy refusal and the HTTP-mode refusal. The 41351423 PDF case runs from outside the repo through `--cases DIR`. MCP parity covers the section and a save. Then make landing-gate runs through pm job after current main.
- Defers: reading PDF text, which is out of scope by ruling.
