# 1082 — label-history-reads-old-format-labels

Status: OPEN.

Milestone: 7

## Outcome

`label-history` reads labels in the format FDA used before 2006, and originals whose first pages read poorly, well enough to say whether a section holds a phrase, so fewer answers fall back to "on or before" or "could not tell".

## Why

Ticket 1066 made the tool never state a date or an absence it has not established. Its re-review (2026-11-20, job j43458, 30 live questions) found 7 answers that were right before and are now vaguer. Two come from private-use characters (ticket 1079). Five come from versions now counted as unread:

- Lipitor adverse reactions "myasthenia": was 2001, now "on or before 2024-04-02" (old-format labels unread).
- Lipitor "immune-mediated necrotizing myopathy": was exactly S62, now "on or before S64" (reads spent on old-format labels).
- Lipitor warnings "myasthenia": was none, now "could not tell".
- Livalo boxed warning (the label has none): was none, now "could not tell" (a miss now needs every version read).
- Olumiant warnings "thrombosis": was exactly ORIG, now "on or before S1" (the original reads poorly).

Also: when the oldest version is unreadable the search can pass over readable early versions (Tecentriq: truth S1, answer "on or before S18"); the third `--every-version` call for Xeljanz gives an "on or before" for its own five versions that misleads when read without the second call; and seven replies pass 3,000 bytes (largest 3,857).

Added 2026-11-20 from three traced agent runs on main 5db190f8a: `biomcp get drug NDA215309 label-history --source drugsfda --label-section boxed_warning --find "major adverse cardiovascular"` marks three of Opzelura's five versions `section-not-bounded`, the original (2021) and the two newest (2025, 2026), and answers "on or before 2022-07-18". These are current-format labels. All three agents then fetched the original PDF and read it themselves. This ticket covers these too: say why the section could not be bounded, and fix it where the label has the heading.

## Proof

- Proof: the five questions above give their earlier, exact answers, each checked against `pdftotext`.
- Proof: the 30 questions of job j43458 are rerun and none gives a false answer.

## Starts from

- Starts from: the re-review's table and outputs. Old-format labels have unnumbered headings (BOXED WARNING, WARNINGS, ADVERSE REACTIONS) and no contents list.
- PM decision, 2026-11-20: the rule of ticket 1066 stands: "on or before" or "could not tell" before any date or absence that is not established.

## Keeps

- Keeps: label PDFs come only from www.accessdata.fda.gov. The read budget per call stays bounded.

## Defers

- Defers: keeping PDFs between calls (a repeat call fetches everything again; 7 to 8 seconds for Aimovig).

## Changes

- Changes: the label text and history modules, cases, fixtures.
