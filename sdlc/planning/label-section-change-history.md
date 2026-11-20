# Design: label section change history

Design note for the ticket "label-section-change-history". The question it answers: in which FDA-approved label version does this text first appear in this section? Everything below was measured on 2026-11-19 against live sources. The sample was the five Proof drugs plus ten drugs picked by name before any of their documents were opened: Keytruda, Humira, Eliquis, Januvia, Lipitor, Ozempic, Opdivo, Entresto, Zoloft and Tagrisso.

## What was fetched

- 81 archived Drugs@FDA labels and 15 approval letters. For the Proof drugs, every label up to the target date plus the next one (51). For the blind drugs, the oldest, a middle and the newest label (30).
- 96 requests to accessdata.fda.gov over 7 minutes, one every 4 seconds, with a plain non-browser client name. All returned HTTP 200. No "excessive requests" page appeared, so the blocking could not be reproduced at this pace. BioMCP already turns an FDA refusal (404 or 406) into a named notice.
- 205 MB in all. A label PDF has a median size of 1.6 MB; 90% are under 4.7 MB and the largest is 20 MB. A letter has a median size of 67 KB. A fetch took a median of 0.35 s and at most 3.7 s.
- Also 13 DailyMed version histories, 10 archived DailyMed versions, and the current openFDA label for each application.

## Sources

| Source | Gives | Cannot give | How far back |
|---|---|---|---|
| Drugs@FDA label PDFs (links `approvals` already returns) | One label per approved supplement, with FDA's date and supplement number. All five Proof answers. | Section structure, which has to be found from headings. Some labels are scans or HTML, and some "Label" links hold only the Instructions for Use. | The original approval in every case sampled (Zoloft 1999, Humira 2002, Lipitor 2001). |
| Drugs@FDA letters | Why the supplement was filed. `approvals` already returns this as `supplement_purpose`. | The sentence that was added. | The same as labels. One 2007 letter URL contains spaces and commas and has to be encoded. |
| DailyMed SPL versions (`history.json`; `getFile.cfm?type=zip&version=N` fetches any past version) | Exact section boundaries, because each section is coded. Smaller files. | FDA's date or supplement number: its dates are when the company published. Livalo's DailyMed version of 2024-02-02 already lists myasthenia gravis, six weeks before FDA approved the change. | It restarts when the company changes: Lipitor from 2024, Zoloft from 2023, Livalo from 2013, Keytruda from 2016. Looking it up by application also finds repackagers (9 set IDs for Eliquis; only repackagers for Ozempic). |
| openFDA labels | The current label, split into fields. | Any earlier version. | The current label only. |

Only Drugs@FDA PDFs give what the ticket's Outcome asks for: FDA's date, the supplement, and the document the text came from. DailyMed answers a different question, namely when the company published the text. It can be added later as a cross-check.

## Finding a section in a PDF

The method takes the PDF's text and finds the heading line, for example `5 WARNINGS AND PRECAUTIONS`. The section ends at the next top-level heading. The last heading match that has real body text under it is the section itself, which skips the Highlights and the table of contents. The boxed warning is the last `WARNING:` block before section 1. Labels in the format used before 2006 use plain headings (WARNINGS, PRECAUTIONS, ADVERSE REACTIONS).

Out of 81 label documents, measured with poppler's text extraction, which was used only for this measurement. A second reader, pypdf, found text in the same documents and the same five Proof answers. Poppler took a median of 0.07 s per label; pypdf took 2.4 s, and up to 25 s.

- 74 were prescribing information, and their sections were found, including both labels in the format used before 2006. One boxed warning was missed, in the original Eliquis label, where the heading that ends the box was not found. Every missing postmarketing subsection was checked and is genuinely absent from that version.
- 7 were unreadable as prescribing information. Three Aimovig "Labels" hold only the Instructions for Use. Two PDFs are images with no text: Zoloft 1999, and Ozempic's newest "Label", a 3-page scan. One label, Humira 2002, is an HTML page.
- The first version of the rules failed on four layout quirks, which the text has to be cleaned of: a form-feed before a heading, a section number on its own line, a trailing colon, and FDA's "This label may not be the latest approved by FDA" stamp plus "Reference ID" and "Page x of y" lines inside sentences. The rules were tuned on this sample, so expect new quirks.
- In five blind drugs whose newest PDF was the current openFDA version, or within two weeks of it, 94 to 97% of the PDF section's six-word phrases appear in openFDA's text for that section. Tables and column order explain the rest.
- Proof: all five answers were found, and each version before them has no match. Major adverse cardiovascular events enter the boxed warning on 2021-12-02 in Olumiant supplement 4, Xeljanz supplement 31 and Rinvoq supplement 8. "5.4 Raynaud's Phenomenon" enters Aimovig's warnings in supplement 26 on 2025-03-21. Myasthenia gravis enters Livalo's 6.2 Postmarketing Experience in supplement 22 on 2024-03-14.

PDF library: BioMCP already reads FDA letters and device PDFs with `lopdf` (MIT) and its own word assembler (`biomcp-adapters/src/pdf`). That needs no new library or outside tool. It refuses PDFs over 100 pages, and the newest Keytruda (202 pages) and Opdivo (191 pages) labels are over that, so the reader should stop once the section ends. Its text order and speed on label PDFs are untested and are the build's first check. Speed matters because a pure-code reader like pypdf took up to 25 s, and each source call has a 15 s deadline.

## Reader check (PM ruling 2, 2026-11-19)

BioMCP's own `lopdf` reader was run, in release mode in a pm job, over the same 80 label PDFs plus 14 letters. A second Rust library, `pdf-extract` 0.12.1 (MIT), was run beside it. Each reader's text was compared with poppler's, using shared six-word phrases. Sections were found with heading rules that work inside a line, because BioMCP's reader runs headings into the surrounding text.

| | poppler (design) | BioMCP `lopdf` reader | `pdf-extract` |
|---|---|---|---|
| Time per label: median, maximum | 0.07 s, about 1 s | 0.014 s, 0.15 s (202 pages: 0.11 s) | 0.05 s, 0.76 s |
| Labels with text whose text is mostly lost (of 78) | 0 | 14: 7 with the body inside form objects the reader skips (all four Rinvoq versions before 2021-12-02 among them), 7 with fonts decoded as gibberish (7 Xeljanz and Olumiant versions) | 10, all with gibberish fonts (the same Xeljanz and Olumiant versions) |
| Warnings and Precautions found (of 72) | 72 | 57 | 61 |
| Adverse Reactions found (of 72) | 72 | 49 | 53 |
| Boxed warning found (of 36) | 36 | 23 | 26 |
| Proof first matches found (of 5) | 5 | 4: Livalo's postmarketing match was missed | 4: Livalo's match was missed |

Speed is not a concern. Accuracy is: about one label in six cannot be read. Because many of those unread labels sit just before a Proof match, three of the five Proof answers would degrade to "on or before" the date. `pdf-extract` also adds 279 lines to `Cargo.lock`, including a second version of `lopdf`. The build is stopped here for a PM ruling.

## Command and reply

`get drug NDA207924 label-history --source drugsfda --label-section boxed_warning --find "major adverse cardiovascular"`

- Section names are openFDA's: `boxed_warning`, `indications_and_usage`, `contraindications`, `warnings_and_cautions`, `adverse_reactions`, `drug_interactions`, `use_in_specific_populations`. Postmarketing is searched within `adverse_reactions`, and the reply names the subsection that matched, because many labels have no postmarketing subsection.
- `--find` matches literal text, ignoring case, line breaks and hyphenation.
- The reply gives:
  - the first version with a match: its date, supplement, label and letter links, heading, and at most two matching sentences of up to 400 characters each;
  - the version before it, which was read and does not match;
  - how many versions were read and how many were not;
  - each unread version and the reason: no text, not prescribing information, refused, or too large.
- A mock Olumiant reply is 1.5 KB, about 380 tokens.

Search order: label versions by approval date, then bisection. That reads ⌈log₂ n⌉+1 versions: 5 or 6 per Proof drug (4 to 24 MB, a few seconds), or 8 of Keytruda's 102, about 60 MB with nothing stored. A linear scan to the first match would read 34 MB for Xeljanz and 55 MB for Aimovig. If the newest version has no match, the reply says so and names the versions read. `--every-version` reads them all, up to a cap, for text that may have been removed.

## What is stored

Each supplement's label URL is fixed, so the command stores each section's text by URL once it has read it: 5 to 180 KB per version, against a median PDF of 1.6 MB. The build checks whether the PDFs also enter the 2 GiB reply cache, which this sample alone (205 MB) would crowd. The Proof cases' replay fixtures need the PDFs that bisection reads, about 50 MB, mostly Aimovig, on top of the repo's 120 MB of replies. The PM should rule on that, or drop the Aimovig case.

## Where it will be wrong or silent

- **Text added, removed, then added again:** bisection reports the last time it was added. `--every-version` avoids this.
- **A change made before FDA approved it** (Livalo above): the date given is FDA's approval date.
- **Different wording:** "MACE" does not match "major adverse cardiovascular events", and the reply says nothing about near matches.
- **A version that cannot be read** (a scan, HTML, Instructions for Use only): it is named as unread. If it falls just before the first match, the answer is "on or before" that date.
- **Labels in the format used before 2006:** their boxed warnings have no heading to anchor on, so the boxed warning is reported as unread.
- **One application, several products:** the command reads the PDF that FDA lists for each supplement. Xeljanz's three forms share one PDF, although openFDA splits them into two set IDs.

## Recommendation and size

Build on the Drugs@FDA label PDFs that `approvals` already lists, with BioMCP's existing `lopdf` reader, the heading rules above, bisection, and a reply limited to sentences. The first version reads neither DailyMed nor the letters; it returns the letter's link. Size: medium, about 600 to 800 lines of Rust:

- a section finder that cleans the text (about 250 lines);
- bisection and the reply schema (about 250 lines);
- help;
- five replayed Proof cases, plus one "no match" case and one "unread version" case.

The first step: run BioMCP's own reader over this sample's 80 PDFs and compare it with the figures above.
