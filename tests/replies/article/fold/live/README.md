# Europe PMC metadata search

These replies were recorded through the article command on 2026-10-24. They contain identifiers and citation metadata from PubMed ESearch/ESummary and Europe PMC `resultType=lite`.

The production endpoint is `https://www.ebi.ac.uk/europepmc/webservices/rest/search`. Six bounded checks after 13:24 UTC returned HTTP 200 with `application/json;charset=UTF-8`: the lite query, `sort=FIRST_PDATE_D desc`, `AUTH:Carpten`, `JOURNAL:Nature`, `FIRST_PDATE:[2020 TO 2024]`, and `PUB_TYPE:"clinical trial"`. Their hit counts were 1204, 1204, 12, 93, 24486, and 425. The date range included a row dated 2024-12-26. The publication-type filter included a row whose native type contained `clinical trial, phase iii`.

The lite reply has `hitCount` and `resultList.result`. Citation fields include `id`, `source`, optional `pmid`, `pmcid` and `doi`, `title`, `authorString`, `journalTitle`, `firstPublicationDate`, and `pubType`. The latter is a semicolon-separated string. The adapter preserves unknown clinical types on rows without a PMID. The date-sort check returned source `PMC`, ID `PMC13680221`, without a PMID and with `firstPublicationDate` 2026-10-21.

The recorded relevance window contains 100 Europe PMC rows from 1204 matches. Its fold with 192 PubMed rows has 249 distinct keys. The recorded date window has 100 Europe PMC rows and produces 284 distinct keys with PubMed. The author filter returns 12 Europe PMC rows from 12 matches and three PubMed rows; its fold has 12 keys. Counts and order describe these snapshots.

The checked floors beside the cases come from the native request rows. They preserve first native ranks before duplicate suppression and test the complete stored order. The source quota remains a conservative local policy. The provider documentation does not establish a numeric rate here.

Source: [Europe PMC REST service documentation](https://dev.europepmc.org/RestfulWebService).

The relevance ESearch request has two captured exchanges. PMID 42896970 has native rank 77 in the first and rank 76 in the second. Each stored pool retains the order from its own exchange. Each separate replay case starts from the first captured snapshot. The source-order case uses that snapshot’s ranks.
