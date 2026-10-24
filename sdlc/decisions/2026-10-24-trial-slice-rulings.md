# Trial slice rulings

Date: 2026-10-24

Decided by the PM on review of the trial design at 15ae48b1 (`planning/slices/trial.md`), within `2026-10-24-no-text-heuristics.md`, `2026-10-24-keyed-sources-run-only-with-the-user-s-key.md`, `2026-10-24-pdfs-are-downloaded-not-read.md` and `2026-10-24-biodata-builds-its-own-parts.md`.

The design's recommendations are accepted, with these choices settled:

1. **Molecular text.** `--gene` and `--variant` are discovery inputs that pass checked text to the registry and are echoed with the fields they search. `--criteria` searches the registry's eligibility text. BioMCP never decides that a trial requires or excludes a variant.
2. **Age.** `--age N` sends the native range expression in which a missing bound counts as no limit, the 3,647 form, because the registry defines an absent bound as no registered limit. The query echo shows the expression. A malformed age field is never treated as no limit.
3. **Facility and radius.** `--facility` and `--lat/--lon/--distance` are separate trial-wide filters. Asking for both returns a usage error that says the registry cannot bind them to one site. Site coordinates come back in `locations`.
4. **NCI.** ClinicalTrials.gov is the default. `--source nci` runs only with the user's key, keeps NCI's own statuses and counts, and never folds with or falls back to ClinicalTrials.gov. A disease filter to NCI needs an NCIt identifier; free text goes through an explicit keyword option. No silent switch from one to the other.
5. **Therapy-line options.** `--line-of-therapy`, `--prior-therapies` and `--progression-on` are dropped. Each returns a retirement message that names `--criteria`.
6. **Disclosure.** Header, card, then sections that page by whole records or by character offsets, with continuations. A criterion is never split or classified by recognizing English headings.
7. **Documents.** `get trial <NCT> documents` lists the registry's documents. `trial document <NCT> <FILE> --save PATH` saves one under the save rule. BioMCP never reads a document's text.
8. **One spelling for each option.** 1.0 has no aliases. A 0.9 spelling that changes, such as `--mutation`, `--biomarker`, `--date-from`, `--date-to`, `--results-available` or `--offset`, returns a retirement message naming the 1.0 option. Registry update dates use `--updated-since` and `--updated-until`, because `--since` on articles means publication date.
9. **Exact ID with filters.** A search that combines an NCT ID with other filters builds the `AREA[NCTId]` conjunction, so the registry never ignores the filters. A sole NCT ID belongs to `get trial`.
10. **Positional text.** Positional search text goes to the registry's general text search, and `--condition` searches conditions only. Help and the 0.9 upgrade note say so.
