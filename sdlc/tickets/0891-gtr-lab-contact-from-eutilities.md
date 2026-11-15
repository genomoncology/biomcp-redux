# 0891 — gtr-lab-contact-from-eutilities

Status: COMPLETE.
Landed: 87f85a0b9a7722819aade72ac9dfed3b296115b4

Milestone: 7

## Outcome

A GTR test card (and, if the cost is measured and small, a search page) carries the lab's city and street address, director, and order or test link, so a counselor can tell a family where to order a test without opening NCBI's page.

## Why

0887 gave GTR rows their gene count, methods and lab state, postcode and country from NCBI's bulk files, but city, street, director and order link are not in those files (0887 Progress). In the rare-disease runs of 2026-11-13, the code-mode agent got all of them for 215 KMT2D tests from E-utilities `esummary db=gtr` in batches of 100 (biomed-agent-bench `personas/rare-disease/runs/2026-11-13-raw/log.md:150-168`). BioMCP's agent had none (`runs/2026-11-13/log.md` Q7).

## Proof

- Proof: a recorded case where a KMT2D test card carries `OffererLocation` address, director and order or test URL from a recorded esummary reply; the fields are absent, not invented, when the source lists none; an E-utilities failure leaves the bulk-file fields intact and says so.

## Starts from

- Starts from: 0887 (`crates/biomcp-adapters/src/gtr/`), BioMCP's existing E-utilities client and its rate limits without an NCBI key.

## Keeps

- Keeps: the bulk files as the source of the test list and counts.

## Defers

- Defers: price, turnaround and CPT codes.

## Changes

- Changes: `crates/biomcp-adapters/src/gtr/`, `crates/biomcp-types/`, `tests/cases/diagnostic/`, `docs/`

## Progress

Design check, 2026-11-15.

- Since this ticket was filed, 0931 added GTR's full XML release (`gtr_ftp.xml.gz`) to the stored GTR set. That file already holds every field this ticket asks for, so the card reads them from the stored copy, not from E-utilities: no per-card network call, no rate limit, and no E-utilities failure mode to handle. The bulk files stay the source of the test list and counts.
- Checked on the 2026-11-14 copy of the XML (the one 0931's cuts came from) against live `esummary db=gtr` on 2026-11-15 for 560508, 500162 and 559466: esummary's `offererlocation` is the lab's `Organization/MailingAddress` (line 1-3, city, state, postcode, country), `directorlist` is the `Person/DisplayName` of each `Personnel` with `IsDirector="true"`, `orderurl` is the test's `Order/URL`, and `testurl` its `TestURL`. Values matched exactly, including both links being empty for 559466.
- Across the whole XML: all 341 labs have a mailing address (none marked non-public) and at least one director (at most 37); the address and director names average 272 bytes per lab, 1,572 at most. Of 63,256 tests, 38,605 list an order link and 38,827 a test link, 63 bytes per test on average (338 at most), about 4 MB in all; every link starts with http(s).
- Search rows: the full set would add about 335 bytes per row (lab address and directors 272, links 63) to rows of about 850-900 bytes, roughly +35-40% on a 50-row page, and every page would need a lab read. That is not small, so search rows stay as they are; the card carries the fields, and the caution says so.

Build, 2026-11-15.

- The XML reader keeps each lab's public `MailingAddress` and its directors' display names (no emails, phones or other personnel), and each test's `Order/URL` and `TestURL`. A lab or test row is kept when it has any kept value.
- Card: optional `laboratory_address` (lines, city, state, postcode, country), `laboratory_directors`, `order_url` and `test_url`; each is absent when GTR lists none. `xml_release_url` now also appears when only these fields came from the XML.
- Cases: `diagnostic/gtr-kmt2d` re-cuts the same 2026-11-14 XML for the KMT2D tests' labs and tests, now also keeping `Organization/MailingAddress`, director `Personnel` (display name only), `Order` and `TestURL`, values unchanged (the old cut rebuilt byte-for-byte from that file first). GTR000500162.7 (University of Chicago) shows the address, two directors and both links; GTR000559466.2 (Athena) has address and six directors but no links, and the card omits both. The synthetic case checks that a non-director and the street address are not read, entity decoding in a director name and an order link, and that a research test's link is not kept.
- Reply size: the synthetic card went from at most 1,858 to 1,888 measured bytes; the KMT2D cards measure 2,811 (500162) and 2,908 (559466, six directors). Caps are measured size plus 15%. Search rows are unchanged. The KMT2D sync stores 617 rows, up from 588 (three more labs with only address and directors, 26 more tests with only links).

Review fixes, 2026-11-15.

- `Public` and `IsDirector` are read as XSD booleans (`true`/`1`, `false`/`0`); an address marked non-public is skipped.
- A certification without a name is skipped alone; the lab row keeps its address, directors and other certifications (before, the whole lab row was refused).
- The "address has no readable part" check now lives only in the card mapping.
- The synthetic case adds a `Public="0"` mailing address (skipped), `IsDirector="1"` and an unnamed certification (skipped while the lab's address and CAP stay).
- Dropped the postcode-difference example from this Progress and the caution: it came from a separate full-file check, not the recorded data.
