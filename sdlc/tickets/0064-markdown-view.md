# 0064 — Print the Markdown view for every gene and article command

Status: OPEN.

Milestone: 1

## Outcome

`--format md` (and the config's `format`) prints every gene and article response as Markdown through the one generic view of architecture 8.6, so a person reads a clean document. A full-text part prints its Markdown body directly. Next steps print as commands a person can paste. JSON stays the default and the MCP output.

- Starts from: architecture 8.6 and the `view` module rule in section 2; the hints each response schema needs (a card's title pointer and a page's table columns); the article design's Markdown ruling for full text.
- PM decision: 2026-10-24. The view takes only the serialized JSON and hints, never the typed response, so it cannot show what the JSON lacks. Format branches stay in the one dispatch in `output.rs`.
- Proof: one Markdown case per command shape (gene card, gene section, gene search page, article search page, article card, full-text part, supplement rows, error envelope) asserting headings, the table's columns and the next-step lines, with a byte ceiling. A JSON case for each stays unchanged.
- Defers: HTML for the later wiki.
