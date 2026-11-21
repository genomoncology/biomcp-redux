# 1088 — fda-potential-safety-signals-source

Status: COMPLETE.
Landed: 460df3f08cf36b171406516bed8f8935aa1754d2

Milestone: 7

## Outcome

BioMCP holds FDA's quarterly lists of potential signals of serious risks identified from its adverse event reporting system: a caller can search them by drug or text and get each row's product, the potential signal, FDA's additional information, the quarter and the page's address.

## Why

Agents traced on 2026-11-19 hunted for these pages by hand; one run spent 83 calls. The design work for ticket 1067 (FDA drug safety communications) found that the same reader covers them: an index page links one page per quarter, 2015 to 2026 plus an archive page, and each quarter page is a three-column table (product, potential signal, additional information). April to June 2026 has 24 rows.

## Proof

- Proof: a replayed case where a search for a drug returns the rows that name it, each with its quarter and address; it fails on the base commit, where the source does not exist.
- Proof: a first read answers without the caller running a sync.

## Starts from

- Starts from: ticket 1067 as landed, its planning file `sdlc/planning/fda-drug-safety-communications.md`, and the stored-list ability it adds for lists whose pages are found from a first page.
- FDA's robots.txt asks for 30 seconds between requests; about 40 pages. Follow ticket 1067's decisions on the shipped seed, the crawl delay, and naming the client honestly. FDA's site policy says its content is in the public domain; quote it in the source's attribution.

## Keeps

- Keeps: BioMCP reports FDA's wording. A potential signal is not a finding that the drug causes the risk; the reply carries FDA's own statement of that.

## Defers

- Defers: nothing.

## Changes

- Changes: a new source beside the drug safety communications source, the drug search command, help, docs and cases.
