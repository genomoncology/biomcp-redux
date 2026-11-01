# PDFs are downloaded, not read

Date: 2026-10-24

Decided by the PM after Ian's 2026-10-24 remark: "PDF to text is a hard problem. I don't know if this is BioMCP's problem to solve." This replaces the PDF text reader in `2026-10-24-article-keeps-every-source-one-option-away.md` and PM rulings m4389, m4490 and m4506.

BioMCP does not extract text from PDFs. Turning a PDF into text is a hard problem that other tools solve, and BioMCP's clean full text comes from JATS in the PMC open access set and Europe PMC preprints. On the four papers checked on 2026-10-24, one open PDF link gave a PDF: one paper was closed, one publisher refused the download and one link led to an HTML page.

BioMCP reports the open PDF link that Semantic Scholar names, with its open access status and license, and downloads the PDF when the person or agent asks, so they can hand it to a PDF tool of their choice.

The code and its tests hold the save and download rules.
