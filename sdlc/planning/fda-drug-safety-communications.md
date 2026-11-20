# Design: FDA drug safety communications as a source

Design note for the ticket "fda-drug-safety-communications-source". It records how FDA publishes its Drug Safety Communications, whether an honest client may read them, and the smallest source that meets the ticket's Outcome. Everything below was measured on 2026-11-20 between 15:42 and 15:50 UTC.

## What was fetched

Eleven requests to www.fda.gov, about 30 seconds apart after the first two, each with the client name `biomcp/1.0 (design probe; +https://github.com/genomoncology/biomcp)`. No request was retried and none claimed to be a browser.

| Request | Reply |
|---|---|
| `/robots.txt` | 200, 2,505 bytes |
| `/drugs/drug-safety-and-availability/drug-safety-communications` (the index) | 200, 59,419 bytes, `Last-Modified: Fri, 20 Nov 2026 01:10:01 GMT`, an ETag, and Akamai bot-manager cookies |
| The 2021-09-01 JAK inhibitor communication | 200, 72,203 bytes |
| `/about-fda/contact-fda/subscribe-podcasts-and-news-feeds` | 200 |
| `/about-fda/contact-fda/stay-informed/rss-feeds/drugs/rss.xml` | 200, `application/rss+xml` |
| The old potential-signals address | 301 to `/drugs/fda-adverse-event-monitoring-system-aems/new-safety-information-or-potential-signals-serious-risks-identified-fda-adverse-event-monitoring` |
| That page | 200, 55,606 bytes |
| The April - June 2026 potential-signals page | 200, 40,421 bytes |
| `/sitemap.xml` | 200, an index of 44 site-wide sitemap pages |
| `/about-fda/about-website/website-policies` | 200 |
| `biomcp sync fda-cdx` from the main build (BioMCP's own client, which sends no custom client name) | complete, 244 rows, 180,991 bytes |

Off www.fda.gov: `https://api.fda.gov/download.json` returned 200, and `https://catalog.data.gov/api/3/action/package_search` and `/api/action/package_search` both returned 404 `{"message":"Not Found"}`.

## Access

FDA did not refuse an honest client. Every request above succeeded. The earlier 4xx that one agent saw could not be reproduced at this pace and with these client names; BioMCP already turns an FDA 404 or 406 into a named notice (`tests/cases/diagnostic/fda-document-refused.yaml`).

robots.txt, for all clients:

```
User-agent: *
Crawl-Delay: 30 # wait 30 seconds before starting a new URL request default=30
Disallow: /health  # don't crawl healthcheck
...
Disallow: /node/
Disallow: /search/
```

Nothing there disallows the index or the communication pages. The 30-second crawl delay matters for any design that fetches more than one page per command or per sync.

Website policies: "Unless otherwise noted, the contents of the FDA website (www.fda.gov) — both text and graphics — are not copyrighted. They are in the public domain and may be republished, reprinted and otherwise used freely by anyone without the need to obtain permission from FDA. Credit to the U.S. Food and Drug Administration as the source is appreciated". The policies page says nothing about automated access. `fda-cdx` already cites this page as its terms.

## How FDA publishes the communications

- One HTML page, the index. It holds one table (`summary="Drug Safety Communications"`) with two columns, Date and Title. Each title links to the communication. There is no feed, data file or API for it.
- It goes back only to 2020: 44 rows, from 1-14-2020 to 11-9-2026. Below the table, "Full archived versions of Drug Safety Communications published before 2020" links to an Internet Archive snapshot (`web.archive.org/web/20260114214202/...`). FDA itself no longer lists the earlier ones.
- Dates are typed by hand and inconsistent: `11-9-2026`, `2-4-2021` and `9-1-21` (two-digit year) all appear.
- Each link carries a stable FDA entity id (`data-entity-uuid="4ec3359d-b7fe-41ca-9724-9602b68d8ca6"` for the JAK communication) and the page has a canonical address. Addresses use two prefixes, `/drugs/drug-safety-and-availability/` and `/drugs/drug-safety-communications/`, so the path has moved before; the entity id has not.
- A communication page gives no trustworthy machine-readable issue date. Its schema.org `datePublished` is `Wed, 05/27/2026 - 11:07` (a site re-publish) and `dateModified` and "Content current as of" are `12/07/2021` (the last update). The issue date appears only as text at the top of the body (`09-01-2021 FDA Drug Safety Communication`) and in the index row.
- A page names its drugs as links to Drugs@FDA application numbers. The JAK page links `ApplNo=203214`, `208246`, `207924` and `211675` (Xeljanz, Olumiant, Rinvoq) and names tofacitinib, baricitinib and upadacitinib in its text. Its index title says only "JAK inhibitors", so a search over index titles alone does not find it under tofacitinib.

## Other routes

- openFDA has no such endpoint. `download.json` lists drug: drugsfda, orangebook, label, enforcement, event, shortages, ndc; other: historicaldocument, unii, nsde, substance; nothing for safety communications or potential signals.
- FDA's RSS feeds have no Drug Safety Communications feed. "What's New: Drugs" holds the last 20 page updates of any kind (about two days on the day measured).
- The data.gov catalog API answered 404 at both its old and its documented paths, so data.gov could not be checked by API.
- The sitemap covers the whole site in 44 pages and gives no communication list.

## Recommended design

A stored list, as `fda-cdx` does, read by `search drug` and `get drug` with `--source fda-dsc`, as `fda-drug-recall` is.

- **Sync** reads the index (one request) and then each linked communication page it does not already hold, one request at a time, at FDA's crawl delay. A full first sync is 44 pages, about 22 minutes at 30 seconds; a weekly sync usually fetches the index and zero to two new pages. A page already held is fetched again only when its index row changes. The current bundle machinery has fixed part lists, so this needs one new ability in `biomcp-fetch`: parts discovered from the first part.
- **One row**: FDA's entity id (the record id), title as FDA wrote it, issue date (from the page body, checked against the index date; a disagreement is a named problem, not a guess), last-updated date, address, the Drugs@FDA application numbers and the drug names the page links, and the page's text.
- **Search**: a drug name matches the title, the linked drug names or the text, after the name widening drug search already does (brand to generic); free text matches title and text. Newest first. Each item shows id, title, issue date, drugs and address, never the text.
- **Get**: by entity id or address, from the stored list; it returns the row with FDA's text marked as source text.
- **Size**: about 18 files and 3 to 4 days. The fetch-layer ability is 2 to 3 files; the adapter `crates/biomcp-adapters/src/fda_dsc/` is about 7 (decl, bundle, index table, page reader, date, row, search); types and schemas about 3; query wiring for drug search and get about 2; help, source list and docs about 2; cases and replies about 2.
- **Fixture**: synthetic, response-edited replies in the repo's recorded-reply form under `tests/replies/drug/fda-dsc/`: the index trimmed to the three tofacitinib rows plus one unrelated row, and each linked page trimmed to its body start, drug links and a paragraph. The replayed case runs `sync fda-dsc`, then `search drug tofacitinib --source fda-dsc` and expects the 2021-09-01 and 2021-02-04 items with their addresses, then `get drug 4ec3359d-b7fe-41ca-9724-9602b68d8ca6 --source fda-dsc` and expects Xeljanz's application `NDA203214` and the issue date 2021-09-01.

A smaller design reads only the index (one request, about 12 files, 2 days) and fetches a page live on `get`. It cannot meet the Proof, because the 2021-09-01 title does not name tofacitinib.

## The Proof's 2019 communications

FDA's own list starts in 2020. The 2019 tofacitinib communications are reachable from FDA only through its link to the Internet Archive, which this design did not use. Their pages may still exist on www.fda.gov at their old addresses, but no FDA page lists them.

## Potential signals from adverse event reports

The same reader would cover them with a second row decoder. The index at `/drugs/fda-adverse-event-monitoring-system-aems/new-safety-information-or-potential-signals-serious-risks-identified-fda-adverse-event-monitoring` links one page per quarter (quarters from 2015 to 2026 on that page, older ones on an "Archived Quarterly Reports" page). Each quarter page holds one table with three columns: "Product Name: Trade (Active Ingredient) or Product Class", "New Safety Information or Potential Signal of a Serious Risk" and "Additional Information (as of ...)"; April - June 2026 has 24 rows. The index itself holds no signals, so this needs the same parts-from-the-first-part sync, about 40 pages. It is best filed as its own ticket after this one.
