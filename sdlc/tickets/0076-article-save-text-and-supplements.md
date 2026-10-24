# 0076 — Save an article's Markdown text and supplement files to a named path

Status: COMPLETE.
Landed: 3201516457ebe79021e1477e641711373a57843a

Milestone: 1

Depends on: 0052

## Outcome

`article text <ID> --save PATH` writes the article's whole JATS Markdown to PATH, and `article supplement <ID> <FILE> --save PATH` writes the supplement file's original bytes to PATH. Each follows the save rule in `decisions/2026-10-24-pdfs-are-downloaded-not-read.md` and reuses 0052's save code.

- Why: an agent that wants the whole paper or a whole spreadsheet in another tool should get a file, not page through parts.
- Starts from: 0052's save path, temporary file and rename, refusal of an existing file, `--replace` and HTTP-mode refusal; 0043's Markdown parts; 0044's supplement download.
- Text: the file holds every part in order, joined as one Markdown document with the article's title, IDs, license and source as a header block. PATH that is a folder gets `<ID>.md`. The reply holds the absolute path, bytes, SHA-256, license, part count and source.
- Supplement: the file holds the bytes the source served, unchanged, within 0044's 50 MB limit. PATH that is a folder gets the supplement's own file name. A `pdf` or `docx` supplement, which the reader marks `unsupported`, can still be saved.
- Keeps: `article text --part`, `--find`, `--variant` and the supplement reader's views unchanged.
- Proof: cases on recorded CC BY replies: save text for an article with JATS (path, bytes, SHA-256, header block, part count), save an xlsx supplement whose SHA-256 equals the recorded body, save the pdf supplement of 28944238, refuse an existing file, `--replace`, the folder forms, no file left after a cut body, and the HTTP-mode refusal. MCP parity covers one save of each. Then make landing-gate runs through pm job after current main.
- Defers: saving any other entity's data.
