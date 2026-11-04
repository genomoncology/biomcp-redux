# PDFs are downloaded, not read

Date: 2026-10-24

Decided by Ian. This replaces the PDF text reader in `2026-10-24-article-keeps-every-source-one-option-away.md` and PM rulings m4389, m4490 and m4506.

BioMCP does not extract text from PDFs. Turning a PDF into text is a hard problem that other tools solve, and BioMCP's clean full text comes from JATS in the PMC open access set and Europe PMC preprints. On the four papers checked on 2026-10-24, one open PDF link gave a PDF: one paper was closed, one publisher refused the download and one link led to an HTML page.

BioMCP reports the open PDF link that Semantic Scholar names, with its open access status and license, and downloads the PDF when the person or agent asks, so they can hand it to a PDF tool of their choice.

Saving any file follows one rule, for a PDF, an article's Markdown text or a supplement file:

- BioMCP writes a file only to a path the caller names. With no path, nothing is saved.
- The reply gives the absolute path, the byte count, the SHA-256, the license and the source URL.
- BioMCP keeps no download folder and no list of saved files. The caller owns the file and deletes it. BioMCP's cache never holds saved files.
- An existing file is never overwritten unless the caller says to replace it.
- The HTTP server mode refuses to save, because the file would land on the server, and returns the source URL instead.

The download keeps its boundary: a public DNS host checked on every redirect, no credential, at most three redirects, at most 25 MB, and a body that starts with `%PDF-`. When a configured proxy applies to the publisher host, the download is refused with a reason. No PDF parser, worker process or audit exception for a PDF crate remains.
