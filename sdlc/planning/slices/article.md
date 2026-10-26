# Article slice

This file holds the article slice's investigation and design, as `planning/slices.md` says. Retrieval strategy is its centre: which sources an agent can search, what each one is built to find, how BioMCP folds several sources into one answer without pretending to rank them, and how pages stay stable. It also covers the 0.9 accounting, the harvested examples, the get layers for abstract, full text and supplementary material, the token budget, the cases and Ian's answers to the design's questions.

Every fact below was observed live on 2026-10-23 unless it says otherwise. NCBI calls ran without an API key, at the 3 per second limit. Semantic Scholar was measured with a key, through the 0.9.3 binary. Section 7 records keyless Semantic Scholar calls made on 2026-10-24, and sections 5 and 8 record the other facts checked that day.

## The rulings this design follows

Ian ruled that BioMCP must not pretend it can merge many sources and search methods into one best order. Each source keeps its own order and meaning. A search over several sources is a fold: BioMCP removes duplicates by identifier, shows which sources found each paper and at what rank, and lays the results out by a plain mechanical rule. The agent picks the sources and the order. BioMCP states what each source is built to do and never claims a best ordering or a best strategy.

Ian also ruled, in `decisions/2026-10-24-article-keeps-every-source-one-option-away.md`, that 1.0 keeps every article source and capability that 0.9 had. Plain PubMed relevance stays the default, and every other source or method sits one option or subcommand away, so an agent working a hard question can walk from one strategy to the next. Help describes each source's job and its known failure modes and never ranks sources against each other.

The slice's bar is the PubMed floor. A plain PubMed search by relevance must never beat BioMCP by BioMCP dropping papers. The default search is that PubMed search, for the exact term BioMCP echoes, so it meets the bar exactly. Any fold that names PubMed holds PubMed's whole result window, and PubMed's own order stays visible.

## Evidence and assumptions in brief

- **The measurements illustrate; they do not rank strategies.** Section 3 counts how many of 20 accepted papers each source puts in its first 10 and first 50. The set is small, picked by one investigator, and favors well-cited papers, which helps citation sorts and Semantic Scholar. Each source does a different job, so a low count on this set says only that the set did not ask for that job. This design therefore recommends no strategy and no recipe beyond the honest default. Claims about which strategy finds more wait for two checks run from outside the repo with `--cases DIR`: a 20-question evaluation set, and floor-only checks on rare variants (CHEK2 G167R, ABCC8 R1437Q, CDKN2A A57V and BRCA1 P1749R) that assert every plain PubMed PMID appears in the walk.
- **Depth.** Each source was read to its first 50 results. Citation sorts read each source's whole list. A paper beyond a source's first 50 counts as missed for that source.
- **Instability.** PubMed's relevance order for the full window moved between runs on the same day: the HER2-low trial 35665782 ranked 1st, then 3rd, then 12th. A later check of "larotrectinib TRK fusion" with windows of 10, 20, 100 and 9,999, twelve calls in all, returned the same top five each time, so the window size did not reorder the top in that check. Counts at 10 carry about one paper of noise per run.
- **0.9.** 0.9 was run with `--limit 50` for the strategy table and with its default `--limit 10` for page one. Its first ten change with `--limit`, as section 1 shows.
- **Original estimates and shipped measurements.** The original investigation built proposed 1.0 outputs with a scratch script. Section 6 separates those historical estimates from offline measurements of the shipped binary. Neither estimate counts model-specific tokenizer tokens.
- **Assumed rate limits.** LitVar2, LitSense2, Europe PMC, iCite and the PMC OA bucket publish no hard limit that this investigation found. Section 5 sets conservative quotas and marks them as assumptions to confirm.

## 1. How BioMCP 0.9.3 does articles

### Commands and options

| 0.9 command | Options | Sources it calls | What it returns | 1.0 |
| --- | --- | --- | --- | --- |
| `search article [QUERY]` | `-g`, `-d`, `--drug`, `-a`, `-k`/`--keyword`/`-q`/`--query`, `--date-from`/`--since`, `--date-to`/`--until`, `--year-min`, `--year-max`, `--type`, `--journal`, `--open-access`, `--no-preprints`, `--exclude-retracted`, `--include-retracted`, `--sort`, `--ranking-mode`, four `--weight-*` flags, `--source`, `--max-per-source`, `--session`, `--limit` 1-50, `--offset`, `--debug-plan`, `--full` | PubTator3, Europe PMC, PubMed and Semantic Scholar by default; LitSense2 only with `--source litsense2`; Semantic Scholar, PubTator3 and Europe PMC enrich rows | Rows of PMID, PMCID, DOI, title, journal, date, citation count, source and retraction flag; no total | `search article [TEXT]`; the option decisions below give every renamed or retired flag. `--source` defaults to PubMed; native source orders replace blends. |
| `get article <ID> [SECTIONS]` | sections `annotations`, `indexing`, `fulltext`, `tldr`, `assets`, `asset <key>`, `all`; `--pdf`, `--outline`, `--lines`, `--output`, `--out`, `--asset-view`, `--asset-limit`, `--asset-offset` | PubTator3 export, Europe PMC, PubMed EFetch and Semantic Scholar for the default card | Card with title, authors, journal, date, one publication type, citations, abstract and a Semantic Scholar block | `get article <INPUT>... [SECTION]...`; `--section WORD` disambiguates section-shaped inputs. PDF uses link metadata and `article pdf <ID> --save PATH`; line selection moves to the text reader below. |
| section `annotations` | | PubTator3 | Entity mentions | `get article <ID> annotations`. |
| section `indexing` | | PubMed EFetch | Authors, affiliations, ORCID, MeSH | `get article <ID> indexing`; no extra source request. |
| section `fulltext` | `--pdf`, `--outline`, `--lines`, `--out` | Europe PMC XML, NCBI EFetch PMC, PMC OA, preprint XML, PMC HTML, then Semantic Scholar PDF with `--pdf` | Writes the text to a cache file and returns its path and coverage | `get article <ID> fulltext` gives the outline; `article text <ID> --part N` reads bounded Markdown. `get article <ID> pdf` reports metadata and `article pdf <ID> --save PATH` saves bytes. `article text <ID> --save PATH` saves JATS Markdown. BioMCP does not read PDF text. |
| section `tldr` | | Semantic Scholar | One-sentence summary and influence counts | `get article <ID> tldr`; source failures leave the card intact. |
| section `assets` | `--asset-view`, `--asset-limit`, `--asset-offset` | PMC OA, Europe PMC supplementary files, JATS links, PMC HTML links, Figshare | JSON-only manifest of figures and supplementary files | `get article <ID> supplements`; lists source-confirmed supplementary files, excludes figure images, and returns license and availability. |
| section `asset <key>` | `--output`, `--out` | Same as `assets` | Raw bytes of one file | `article supplement <ID> <FILE>` reads supported rows; unsupported formats retain the source URL. `article supplement <ID> <FILE> --save PATH` saves original bytes. |
| `article authors <ID>` | | Semantic Scholar | Authors as Semantic Scholar lists them | Retired: `use: get article <id>...`; the card carries authors. |
| `article entities <PMID>` | `--limit`, `--full` | PubTator3 | Annotated entities as suggested commands | Retired: `use: get article <id> annotations`; the section carries normalized entities. |
| `article batch <IDS>...` | | PubTator3, Europe PMC, Semantic Scholar | Compact cards for up to 20 IDs | Retired: `use: get article <id>...`; up to 20 inputs preserve input order. |
| `batch article <IDS>` | `--mode`, `--sections`, `--pdf` | Same as `get article` | Same, in parallel | Retired: `use: get article <id>...`; `--mode` is dropped, `--sections` becomes trailing sections and `--pdf` maps to the explicit `pdf` section and `article pdf --save PATH`. |
| `article citations <ID>` | `--limit` 1-100, `--offset` | Semantic Scholar | Citing papers with citation contexts and intents | `article citations <ID> --source semantic-scholar` keeps contexts and intents; default `icite` gives the PubMed graph ordered by count. Paging uses `--limit`, `--offset`, `--pool`. |
| `article references <ID>` | `--limit` 1-100, `--offset` | Semantic Scholar | Cited papers with contexts and intents | `article references <ID> --source semantic-scholar`; default `icite` gives the PubMed graph. Paging uses the same cursor fields. |
| `article citation-evidence <CITING> <CITED>` | `--fulltext` | Semantic Scholar, Europe PMC XML, OpenCitations | The passage that cites the second paper | `article citation-evidence <CITING> <CITED> [--fulltext]` keeps source contexts and optional citing paragraphs. |
| `article supplement-search <ID> <NEEDLE>` | `--asset` | Same as `assets` | Supplement rows that contain the text, with sheet, row and checksum | Retired: `use: article supplement <id> <file> --find <text>`; `--variant CHANGE` explicitly expands protein-change forms. `--asset` becomes the positional exact file name. |
| `article recommendations <IDS>...` | `--negative`, `--limit` | Semantic Scholar | Related papers | `article recommendations <IDS>... [--negative IDS] [--limit N]`; native recent-paper recommendations, including an explicit empty result. |
| `variant articles [ID]` | `--input`, `--debug-plan`, `--verify-identity`, `--confirmed-only`, `--strategy`, `--limit`, `--offset` | PubMed, Europe PMC, PubTator3, Semantic Scholar, LitVar2, MyVariant citations | Article rows for one to ten variants | Retired: `use: search article --variant <variant>`; `--gene` supplies a protein change's gene. No hidden strategy, verification score, request-file input or local filtering. Paging uses the shared pool. |
| `gene articles`, `disease articles`, `drug articles`, `pathway articles` | `--limit`, `--offset` | Same as `search article` | Article rows for one entity name | `gene articles`: `search article --gene`; `disease articles`: `search article --disease`; `drug articles`: fixed guidance to `search article --drug`; `pathway articles`: moved to the pathway slice. |
| MCP | the shared `search` and `get` tools and a separate `variant_articles` tool | | | One `biomcp` MCP tool accepts the CLI argument array; the old tool names retire. |
The shipped `--pdf` replacement follows Ian's download-only ruling in `decisions/2026-10-24-pdfs-are-downloaded-not-read.md`: `get article <ID> pdf` reports a link, status and license, and `article pdf <ID> --save PATH` downloads it. BioMCP does not extract PDF text. The metadata section and save cases are listed in section 8.

The investigation listed 16 article-family command spellings and 88 arguments, options and section names, excluding global flags. Those are not the historical leaf-command denominator: generic entity choices and retired spellings are separated in the accounting below.


### Leaf-command parity accounting

The historical denominator counts leaves and command-specific long option spellings; short aliases share their option's slot. Long aliases such as `--since` are distinct historical spellings. Sections, positional values, MCP tools and global flags stay outside it. The gene table owns `gene articles`. Generic `batch` owns all its entity choices, so `batch article` is mapped above without inventing another leaf. The historical CLI has no `drug articles` leaf; its fixed migration guidance is still covered above. `pathway articles` is accounted by the pathway slice under its retirement ruling.

<!-- parity-accounting -->
| 0.9 leaf command | Command outcome | Ported or renamed options | Dropped options | Deferred options | Undecided options | Ruling |
| --- | --- | --- | --- | --- | --- | --- |
| `search article` | Ported or renamed | `--gene`, `--disease`, `--drug`, `--author`, `--keyword`, `--query`, `--date-from`, `--since`, `--date-to`, `--until`, `--year-min`, `--year-max`, `--type`, `--journal`, `--open-access`, `--no-preprints`, `--exclude-retracted`, `--include-retracted`, `--sort`, `--source`, `--limit`, `--offset` | `--ranking-mode`, `--weight-semantic`, `--weight-lexical`, `--weight-citations`, `--weight-position`, `--max-per-source`, `--session`, `--debug-plan`, `--full` | — | — | Section 5 maps selectors and paging to typed search, replaces keyword aliases with TEXT, and retires blends and hidden planning under the no-text-heuristics ruling. |
| `get article` | Ported or renamed | `--outline`, `--lines`, `--pdf`, `--output`, `--out` | `--asset-view`, `--asset-limit`, `--asset-offset` | — | — | Section 5 moves parts into the reader. The download-only decision of 2026-10-24 maps `--pdf` to `get article ID pdf` and `article pdf ID --save PATH`; its landed text/supplement save rule maps `--output` and `--out` to `article text ID --save PATH` and `article supplement ID FILE --save PATH`. |
| `article authors` | Ported or renamed | — | — | — | — | Section 1 maps author disclosure to the article card. |
| `article entities` | Ported or renamed | — | `--limit`, `--full` | — | — | Section 5 maps normalized entities to the annotations section with one bounded shape. |
| `article batch` | Ported or renamed | — | — | — | — | Section 1 maps repeated inputs to get article. |
| `article citations` | Ported or renamed | `--limit`, `--offset` | — | — | — | Section 5 keeps the Semantic Scholar graph one source option away and adds iCite as the default. |
| `article references` | Ported or renamed | `--limit`, `--offset` | — | — | — | Section 5 keeps contexts and intents through the explicit graph source. |
| `article citation-evidence` | Ported or renamed | `--fulltext` | — | — | — | Section 5 preserves the pair reader and optional citing paragraphs. |
| `article supplement-search` | Ported or renamed | `--asset` | — | — | — | Section 5 replaces the asset selector with the exact file positional input of article supplement. |
| `article recommendations` | Ported or renamed | `--negative`, `--limit` | — | — | — | Section 5 preserves native positive and negative seed recommendations. |
| `variant articles` | Ported or renamed | `--limit`, `--offset` | `--input`, `--debug-plan`, `--verify-identity`, `--confirmed-only`, `--strategy` | — | — | The no-text-heuristics ruling replaces hidden plans and identity scores with explicit variant and source choices; variant notation belongs to its slice. |
| `disease articles` | Ported or renamed | `--limit`, `--offset` | — | — | — | Section 1 moves the capability to search article --disease. |
<!-- /parity-accounting -->

### Speed and size

Commands ran with `--no-cache` in JSON and then in Markdown. Rows show the JSON time.

| Command | Cold | JSON | Markdown |
| --- | --- | --- | --- |
| `search article "AKT1 E17K"` | 5.8 s | 5.2 KB | 2.7 KB |
| Other searches on the examples | 0.6 to 4.7 s | 2.8 to 5.5 KB | 1.7 to 2.9 KB |
| `variant articles "AKT1 E17K"` | 15.3 s | 45.5 KB | 3.0 KB |
| `get article 17611497` | 0.8 s | 6.0 KB | 3.2 KB |
| `get article 10.1056/NEJMoa1203421` | 0.7 s | 8.1 KB | 4.2 KB |
| `get article 99999999` | 0.6 s | 0.2 KB | error after 5.6 s |
| `get article 34096690 fulltext` | 6.4 s | 6.2 KB, text on disk | 1.0 KB |
| `get article 17611497 indexing` | 0.9 s | 10.8 KB | 3.0 KB |
| `get article 30209399 assets` | 7.8 s | 50.7 KB | refused, exit 2 |
| `article supplement-search 30209399 F1761I` | 6.1 s | 2.6 KB | refused, exit 2 |
| `article citations 17611497 --limit 10` | 1.2 s | 7.2 KB | 2.7 KB |
| `article batch 17611497 34096690 21639808` | 3.7 s | 9.7 KB | 2.9 KB |

### What 0.9 gets wrong

Each finding names the exact command and the observed result.

1. **Plain PubMed's top papers are missing from page one.** PubMed's first result is absent from 0.9's first page for `search article "KRAS G12C sotorasib lung"` (34096690), `"HER2-low trastuzumab deruxtecan"` (35665782) and `"amivantamab lazertinib after osimertinib"` (37879444). For `"AKT1 E17K"`, PubMed's second result 33123537 and the discovery paper 17611497 are both absent. Across nine searches, 0.9's first page shared 0 to 3 papers with PubMed's first ten.
2. **ESearch gets no sort.** 0.9 never sends `sort=relevance` (0.9 `src/sources/pubmed.rs:326-354`), so PubMed returns newest first. With no sort, PubMed's first 25 for every example query held none of the key papers.
3. **Dropped words pass silently.** `search article "xqzvw zzyzxq kinase"` returned ten new kinase papers with status complete. PubMed dropped the two unknown words and named them in `errorlist.phrasesnotfound`. 0.9 shows only a "partial lexical query coverage" warning.
4. **No total and no stable pages.** Every search reports `total: null`, `has_more: true` and `next_page_token: null`. Each page reruns every source.
5. **Rows without a PMID take page-one slots.** Two of ten `"AKT1 E17K"` rows were meeting abstracts known only by a DOI from Semantic Scholar.
6. **`variant articles` misses both key papers.** `variant articles "AKT1 E17K"` took 15.3 s, printed 45.5 KB of JSON, and its first ten rows held neither 17611497 nor 33123537.
7. **The source cap drops rows before ranking.** `--max-per-source` keeps 40% of `--limit` per source by source position before any ranking (0.9 `candidates.rs:290-330`).
8. **The semantic weight is idle.** The default hybrid score gives 0.4 to a LitSense2 signal, and LitSense2 runs only with `--source litsense2`, so every default row scores zero on it.
9. **The card picks one publication type at random.** `get article 17611497` shows the type "research support, non-u.s. gov't". PubMed lists Journal Article and Research Support for it.
10. **Retraction is a type, with no notice.** `get article 17057710` shows "retracted publication" and omits the retraction notice, PMID 21217686, that PubMed links.
11. **Search hides retracted papers by default.** `--exclude-retracted` is the default, so an agent never learns that a paper it already holds was retracted.
12. **Full text is a file path.** `get article 34096690 fulltext` writes the text to a cache file and returns the path. An agent over MCP cannot read that path.
13. **The asset manifest is huge and JSON only.** `get article 30209399 assets` printed 50.7 KB for 25 of 31 files, 15 of them figure images. Markdown refuses with exit 2. `all` leaves assets out.
14. **0.9 does not pace keyless Semantic Scholar calls.** 0.9's card and its citation, reference, recommendation and author commands call Semantic Scholar, and a keyless 429 reaches the agent as a failure. Keyless calls from this investigation got HTTP 429 on 22 of 27 tries on 2026-10-23, which is public issue 225's subject. Section 7 shows that keyless calls work through Semantic Scholar's shared pool, and that search and paper lookups meet 429 more often than citation and recommendation calls.
15. **`article recommendations 17611497` returns nothing.** The seed is a paper with 1,045 citing PubMed papers. Semantic Scholar recommends from a pool of recent papers. On 2026-10-24 it returned no papers for 17611497 and five 2026 KRAS papers for 34096690, so an empty answer is a real state of the source and needs its own status.
16. **Citations come newest first with no total.** `article citations 17611497` lists 2026 papers first and says the endpoint reports no total.
17. **PubTator autocomplete can pick another species.** 0.9 takes the first autocomplete ID with the right type. PubTator3's first variant match for "IDH1 R132H" is `@VARIANT_p.R132H_idh1.L_African_clawed_frog`.
18. **`drug articles --help` calls MyChem.info.** It failed with "API request to MyChem.info failed" instead of printing help.
19. **Two flag families mean one thing.** `--date-from`/`--since` and `--year-min` both bound dates. `--sort`, `--ranking-mode` and four weights all steer order.
20. **A source failure can look empty.** Public issue 97 reports `article_searcher` returning empty output while PubTator3 was unreachable. Public issue 292 reports a whole search failing when local cache locks failed.

21. **Page one depends on `--limit`.** On nine searches, 0.9's default page of ten carried 1 of 13 key papers. The first ten rows of the same searches with `--limit 50` carried 5, and the two first-ten lists shared only 1 to 5 papers. 0.9 cuts each source to 40% of the limit before ranking, so a larger limit lets other papers into the top.
22. **Each page rebuilds the search.** 0.9 reruns every source and reranks the whole pool on every page, then applies the offset. A walk of 35 pages for `variant articles "AKT1 E17K"` ran discovery 35 times, took about 52 to 57 s per page of 50, and the pool size moved between about 1,500 and 1,700 within one walk. The same paper appeared on two pages, and papers that one strategy returned never appeared in the union walk.
23. **Strict queries cut at 25 rows.** `variant articles` sends only title and abstract queries, keeps 25 rows from each, and sends PubMed no sort. The strict PubMed query for AKT1 E17K matched all 192 floor papers, but its first 25 were all from 2023 to 2026. PubMed's second result 33123537 never reached the union, and the 2007 discovery paper ranked about 388th.
24. **A fused score hid the reasons.** The `variant articles` ranker sorts by identity confirmation, then exact route, then reciprocal rank fusion over route positions. The first key is empty unless identity checks run, so the fused rank decides almost everything, and it reads no title, abstract, date or citation. `search article` uses a different ranker, so the two commands order the same papers differently.
25. **One deadline starves later sources.** Routes run one after another under one 60 s deadline, so a slow early route leaves later routes unattempted. PubMed was reported as timed out after 12 s on queries that ESearch answered in 0.25 s. A failed strict PubTator call was cached and reused as another route's empty answer.
26. **Paging can never finish.** `variant articles "CHEK2 G167R" --limit 50 --offset 110` returned 0 rows with `has_more` true and `complete` false while every work item was ok. Europe PMC paging once ignored its page parameter and returned page 1 every time.
27. **Date sort put future issue dates first.** `--sort date` ordered papers by issue date, so issues dated 2026-12-31 led a search run in October 2026.
28. **Narrow fields missed papers.** Restricting Europe PMC to title and abstract after a Boolean pass dropped papers that only all-field queries found. An exact-name query over title and abstract found 0 papers for BRCA1 F1761I, while a full-text query found 10811118.
29. **A semantic weight that never ran.** As item 8 says, the default blend gave 0.4 to LitSense2, which the default plan never called.
30. **Old gene names were not searched.** Queries for ABCC8 R1437Q left out the old name SUR1. Europe PMC `SUR1 AND "R1437Q"` found 6 PMIDs, including 10685980 and 9312191, against 3 for ABCC8.
31. **A variant named only in a supplementary table was missed.** The exact-name strategy searched titles and abstracts and found 0 papers for BRCA1 F1761I. The fact sits in row 2780 of a supplementary spreadsheet of 30209399, which only `article supplement-search` reached.

What 0.9 does well: `search article -a Carpten -k AKT1` puts 17611497 first; `article supplement-search` finds BRCA1 p.F1761I in `NIHMS1501643-supplement-2.xlsx` Sheet1 row 2780 and AXIN2 p.Pro245Ser in `MGG3-5-553-s002.xlsx` sheet ST2 row 85, each with a SHA-256, and it matches the three-letter form when given P245S; asset discovery finds all 16 supplement files of PMC6181777 through PMC OA; the full-text ladder returns author-manuscript text when Europe PMC answers 500; the card returns in under a second with the whole abstract.

## 2. Harvested examples

Thirty-five examples, each from a public paper or a public issue. Every PMID was checked against PubMed ESummary. "Floor" means the plain PubMed relevance search for the words in the question.

| # | Story | Question | Public source | What a good answer holds | Sources that answer |
| --- | --- | --- | --- | --- | --- |
| 1 | A breast cancer report shows AKT1 E17K | What is the literature on AKT1 E17K, starting with the discovery paper? | PMID 17611497, PMID 33123537 | Floor of 192 PMIDs; 17611497 (Nature 2007) and the 2020 systematic review 33123537 on page one | PubMed, LitVar2, iCite |
| 2 | The same agent asks about treatment | Is there trial evidence for capivasertib in AKT1 E17K tumors? | PMID 33377972 | NCI-MATCH subprotocol Y, JAMA Oncol 2021, PubMed's first result | PubMed |
| 3 | A lung tumor carries KRAS G12C | Which trials led to sotorasib? | PMID 34096690, PMID 32955176 | Phase 2 CodeBreaK 100 (NEJM 2021) and phase 1 (NEJM 2020) on page one | PubMed, iCite |
| 4 | A patient progresses on an EGFR inhibitor with T790M | What is the phase 3 evidence for osimertinib in T790M? | PMID 27959700 | AURA3, NEJM 2017. PubMed ranks it 543rd of 1,338 | PubMed, iCite |
| 5 | A melanoma is BRAF V600E | What is the pivotal vemurafenib trial? | PMID 21639808 | BRIM-3, NEJM 2011, PubMed's first result | PubMed |
| 6 | A lung cancer is BRAF V600E | What did dabrafenib show in BRAF V600E NSCLC? | PMID 27080216 | Lancet Oncol 2016. Its title writes BRAF(V600E) | PubMed, PubTator3 |
| 7 | A glioma is IDH1 R132H | What is the landmark IDH1 and IDH2 glioma paper? | PMID 19228619 | Yan et al., NEJM 2009. It is outside the floor of 644, because its abstract does not spell "R132H" | LitVar2, iCite |
| 8 | A child has adrenocortical carcinoma and a germline TP53 R337H | Which paper linked TP53 R337H to pediatric adrenocortical carcinoma? | PMID 11481490 | PNAS 2001. Outside the floor of 52, because its abstract says "adrenal cortical" | LitVar2, PubTator3 |
| 9 | A family carries BRCA1 185delAG | What is the cancer risk for Ashkenazi founder mutations? | PMID 9145676 | NEJM 1997. PubMed ranks it 38th of 141 | PubMed, LitVar2, iCite |
| 10 | An agent holds only an rsID | Which papers discuss rs121434592? | PMID 17611497 | rs121434592 is AKT1 E17K. PubMed finds 3 papers by the rsID; LitVar2 lists 1,590 including 17611497 | LitVar2 |
| 11 | A breast tumor is HER2-low | What is the trastuzumab deruxtecan trial in HER2-low disease? | PMID 35665782 | DESTINY-Breast04, NEJM 2022 | PubMed |
| 12 | A sarcoma shows an NTRK fusion | What is the larotrectinib efficacy paper? | PMID 29466156 | NEJM 2018 | PubMed |
| 13 | A cholangiocarcinoma has an FGFR2 fusion | What did pemigatinib show? | PMID 32203698 | FIGHT-202, Lancet Oncol 2020 | PubMed |
| 14 | A clinician asks after osimertinib failure | What does MARIPOSA-2 report? | PMID 37879444 | Ann Oncol 2024; ESearch "MARIPOSA-2" returns it first of 19 | PubMed |
| 15 | A urologist asks about PARP inhibition | What is the olaparib trial in mCRPC? | PMID 32343890 | PROfound, NEJM 2020 | PubMed |
| 16 | An agent decides whether to suggest NGS | What does ESMO recommend for NGS in advanced cancer? | PMID 38834388 | ESMO 2024 recommendations, typed Practice Guideline | PubMed, `--type guideline` |
| 17 | A pathologist asks how to test for NTRK fusions | What are the ESMO NTRK detection recommendations? | PMID 31268127 | Ann Oncol 2019 | PubMed |
| 18 | An agent knows the author | Which AKT1 papers did Carpten publish? | PMID 17611497 | 17611497 first | PubMed with `[au]`, Europe PMC |
| 19 | An agent wants recent work only | What KRAS G12C sotorasib papers appeared since 2024? | public issue 69 | Only 2024 and later papers; total stated | PubMed with a date range |
| 20 | An agent wants the newest first | Sort KRAS G12C papers by date | public issue 69 | Newest first, with the sort named in the query echo | PubMed |
| 21 | An agent saves tokens | Return five results per page | public issue 69 | Five headers, a total and a next page step | Pool |
| 22 | A very large topic | Papers on BRAF V600E | PMID 21639808 | Floor of 8,964 PMIDs fits PubMed's 9,999 window; complete | PubMed |
| 23 | A larger topic | Papers on EGFR lung cancer | public issue 69 | 31,816 matches; the first 9,999 by relevance; `truncated` set with the total | PubMed |
| 24 | A typo or nonsense | Search "xqzvw zzyzxq kinase" | public issue 4 | PubMed's results for the word it kept, with a notice naming the dropped words xqzvw and zzyzxq | PubMed `phrasesnotfound` |
| 25 | A source is unreachable | Search while one source fails | public issue 97 | Status partial with the failed source named; never an empty success | Every source |
| 26 | An agent types HER2 as the gene | Papers on HER2-low disease with `--gene HER2` | genomoncology/biomcp issue 113 | ERBB2 resolved and the term `(ERBB2 OR HER2) AND low trastuzumab deruxtecan` sent; 35665782 on page one. The term `ERBB2 low trastuzumab deruxtecan` loses it | HGNC bundle, PubMed |
| 27 | An agent holds a DOI | Get 10.1056/NEJMoa1203421 | PMID 22663011 | METRIC trametinib trial, NEJM 2012, PMID 22663011; not in PMC | PubMed `[doi]` |
| 28 | An agent holds a PMCID | Get PMC6181777 | PMID 30209399 | Saturation genome editing of BRCA1, Nature 2018; full text and 16 supplement files | PubMed, PMC OA |
| 29 | A cited paper was retracted | Get 17057710 | PMID 17057710 | Flagged retracted, with retraction notice PMID 21217686 | PubMed EFetch |
| 30 | A wrong ID | Get 99999999 | none | Not found | PubMed |
| 31 | A variant classification lives in a supplement | How does saturation genome editing classify BRCA1 F1761I? | PMID 30209399 | `NIHMS1501643-supplement-2.xlsx`, Sheet1 row 2780: c.5281T>A, p.F1761I, func.class LOF | PMC OA |
| 32 | A colorectal panel finding sits in a supplement | How was AXIN2 P245S classified? | PMID 28944238 | `MGG3-5-553-s002.xlsx`, sheet ST2 row 85: c.733C>T, p.Pro245Ser, rs62640028, VUS. The file spells it only in three-letter form | PMC OA, CC BY |
| 33 | A fact sits in the body of an author manuscript | Read the CodeBreaK 100 full text | PMID 34096690 | PMC9116274, license TDM; Europe PMC full text answers 500, and the PMC OA bucket holds its JATS | PMC OA bucket |
| 34 | An agent needs normalized entities | Which genes and variants does 17611497 name? | PMID 17611497 | AKT1 (Entrez 207), p.E17K with rs121434592, breast, colorectal and ovarian cancers | PubTator3 |
| 35 | An agent follows citations | Who cites 17611497, and what does 34096690 cite? | PMID 17611497, PMID 34096690 | 1,045 citing PubMed papers by iCite; 34096690's 37 references include the phase 1 paper 32955176 | iCite |

A preprint with no PMID, Europe PMC PPR1064174 (DOI 10.21203/rs.3.rs-7293078/v1), serves as the input case for `get article` on a Europe PMC ID. Its full text is not open. Preprint PPR1295425 (DOI 10.64898/2026.08.05.26359768, CC BY) serves as the preprint full-text case. PMC13020499 serves as a PMCID with no open full text.

Gaps no free source fills: papers that name a variant only in a figure, and conference abstracts without a PMID.

## 3. How the sources answer

### Per source

| Source | Call | Cold | What matters | Quirks | Terms |
| --- | --- | --- | --- | --- | --- |
| PubMed ESearch | `esearch.fcgi?db=pubmed&term=…&sort=relevance&retmax=9999&retmode=json` | 0.2 to 0.4 s; 0.8 to 1.1 s for a full 9,999 window (99 to 111 KB) | Relevance-ordered PMIDs, the count, the query translation, dropped terms | Newest first without `sort=relevance`. At most 9,999 records: `retstart` 10000 answers HTTP 200 with an `ERROR` field that holds a raw control character, so the body is not valid JSON. The same full-window call put 35665782 first in one run and third in another. Twelve calls for "larotrectinib TRK fusion" with windows of 10, 20, 100 and 9,999 returned the same top five. Unknown words are dropped and listed in `errorlist.phrasesnotfound`. `10.1056/NEJMoa1203421[doi]` and `PMC6181777[pmc]` each return their PMID | NLM terms; abstracts may carry publisher copyright |
| PubMed ESummary | `esummary.fcgi` POST, batches of 200 | 0.25 s for 10; 1.6 s for 192 | Header fields for each PMID | About 2.2 KB of JSON per record | As above |
| PubMed EFetch | `efetch.fcgi?db=pubmed&retmode=xml`, batches of 200 | 0.2 s for one; 1.4 to 1.9 s per 200 | Title, abstract with labels, authors, journal, date, all publication types, DOI, PMCID, retraction and erratum links | About 19 KB of XML per record, ten times ESummary. Abstracts carry inline `<i>` and `<sup>` tags | As above |
| NCBI EFetch PMC | `efetch.fcgi?db=pmc&id=<digits>` | 0.3 to 0.4 s | Whole JATS XML for PMC articles, including author manuscripts | Measured and not used: the OA bucket holds the same JATS for every open article | Per-article license; author manuscripts are TDM |
| PMC OA bucket | `pmc-oa-opendata.s3.amazonaws.com/?list-type=2&prefix=PMCID.` then `PMCID.N/PMCID.N.json` | 0.1 to 0.2 s each | `pmid`, `doi`, `license_code`, `is_retracted`, `xml_url` for the JATS, `media_urls` with every supplement file | The old `oa.fcgi` service answered 404. The JATS for PMC9116274 downloaded in 0.2 s, and media files in 0.2 to 0.5 s. PMC13020499 and PMC13020505 are not in the bucket and have no body | Per-article: CC BY, CC0, TDM and others |
| Europe PMC search | `search?query=…&resultType=idlist&pageSize=…` | 0.5 s for 25; 2 to 6 s for 500; 5.7 s for 1,000 | Matches in full text for open articles, and preprints with `PPR` IDs | Default search matches full text, so counts are 3 to 6 times PubMed's. `lite` and `core` result types cost 8 and 70 times more bytes. Title and abstract fields tokenize differently: 28 floor papers that spell "p.E17K" were missing from a title and abstract query | Metadata open; text per article license |
| Europe PMC article | `fullTextXML`, `supplementaryFiles`, `MED/<pmid>/citations`, `references`, core record | 0.4 to 1.8 s; citations 2.5 to 7.3 s | Full text for the open subset and for some preprints; core record with `isOpenAccess`, `hasSuppl`, `license` | `fullTextXML` answers 500 for author manuscripts and for preprints without full text (PPR1064174); PPR1295425 returned 92 KB of JATS in 0.7 s. `supplementaryFiles` for PMC6181777 returned a 176-byte ZIP holding one empty xlsx; for non-open articles it returns an XML error with HTTP 200 | As above |
| PubTator3 | `publications/export/biocjson?pmids=` | 0.1 to 0.3 s | Gene, variant, disease and chemical mentions with Entrez IDs, HGVS, rsIDs and MeSH IDs | Returns an empty document for some PMIDs (17057710) | Public NCBI data |
| PubTator3 search | `entity/autocomplete`, `search/?text=@VARIANT_…&page&size=100` | 0.2 s autocomplete; 1 to 2 s per page of 100 | Variant-normalized PMIDs | First variant match can be another species (frog idh1.L for IDH1 R132H). Over LitVar2 it added 0 to 3 unique papers per example and took 5 to 10 s for five pages | As above |
| LitVar2 | `variant/autocomplete/?query=`, `variant/get/<id>/publications` | 0.2 s each; 0.5 s and 642 KB for BRAF V600E | Every PMID that mentions a variant, keyed by rsID or gene and HGVS | The `#` in IDs must be percent-encoded. Lists can be huge: 32,239 PMIDs for BRAF V600E. The list order carries no stated meaning | Public NCBI data |
| iCite | `icite.od.nih.gov/api/pubs?pmids=…` | 0.1 s for one; 0.14 to 0.56 s per 200 with `fl=` | Citing PMIDs, reference PMIDs, citation count, clinical citations, relative citation ratio, year, title | One of 192 recent PMIDs was missing. With `legacy=false&fl=` the count field is `citedByPmidCount`. A GET with 1,000 PMIDs got HTTP 413, so batches stay at 200 | Public domain, NIH Open Citation Collection |
| Semantic Scholar | `graph/v1/paper/search`, `paper/PMID:…`, `…/citations` | 0.1 to 0.8 s unkeyed; 0.5 to 1.4 s through 0.9 with a key | Relevance search, TLDR, citation contexts and intents | Keyless calls on 2026-10-23 got HTTP 429 on 22 of 27 tries, even 12 s apart. Keyless calls on 2026-10-24 succeeded on 1 of 19 search tries, 6 of 40 paper lookups and 0 of 5 batch lookups, and on 10 of 10 citation, reference and recommendation calls (section 7). 228 of 751 search rows through 0.9 had no PMID | API license agreement |
| LitSense2 | `litsense2-api/api/sentences/?query=…&rerank=true` | 0.4 to 3.5 s; 2 of 17 calls took 21 s | 100 sentences ranked by similarity, each with PMID, section and entity annotations; about 50 distinct papers | One reply with no paging. Returned 50 papers for a nonsense query on 2026-10-23. On 2026-10-24 the same nonsense query answered HTTP 502 after 30 s, and "AKT1 E17K capivasertib breast cancer" returned 100 sentences from 65 papers in 3.0 s, each with `pmid`, `pmcid`, `section`, `text`, `score` and `annotations` | Public NCBI data |
| OpenCitations | `index/v2/citation-count/pmid:…` | 0.5 to 3.3 s | Citation counts by DOI or PMID | 37879444 showed 1 citation where iCite showed 405 | CC0 |

### Search legs on the examples

Each cell gives the hit count, then where each key paper ranked. A dash means the key paper was absent from what the leg returned. PubTator3 was read for its first 50 text-search results. Semantic Scholar without a key answered HTTP 429 on 14 of these 17 queries and is left out here; its keyed results are in the next table.

| Query | PubMed floor | Europe PMC | PubTator3 text | LitVar2 |
| --- | --- | --- | --- | --- |
| AKT1 E17K | 192; 17611497 89th, 33123537 2nd | 1,204; 833rd, 182nd | 784; both absent | 1,590; both present |
| BRAF V600E melanoma vemurafenib | 684; 21639808 1st | 5,966; absent from first 1,000 | 3,656; absent | |
| KRAS G12C sotorasib lung | 390; 34096690 1st, 32955176 5th | 2,550; absent, 2,064th | 1,440; absent | |
| EGFR T790M osimertinib | 1,338; 27959700 543rd | 6,852; absent | 4,758; absent | |
| IDH1 R132H glioma | 644; 19228619 absent | 4,376; absent | 2,637; absent | 4,224; present |
| TP53 R337H adrenocortical | 52; 11481490 absent | 219; absent | 130; absent | 680; present |
| rs121434592 | 3; 17611497 absent | 19; absent | 15; absent | 1,590; present |
| HER2-low trastuzumab deruxtecan | 422; 35665782 1st | 5,516; absent | 3,199; 6th | |
| larotrectinib TRK fusion | 281; 29466156 1st | 2,481; absent | 1,147; absent | |
| FGFR2 fusion pemigatinib cholangiocarcinoma | 107; 32203698 2nd | 820; 507th | 657; absent | |
| amivantamab lazertinib after osimertinib | 22; 37879444 1st | 592; 220th | 311; absent | |
| olaparib metastatic castration-resistant prostate cancer | 286; 32343890 1st | 3,066; absent | 1,991; absent | |
| next-generation sequencing recommendations advanced cancer ESMO | 24; 38834388 1st | 1,597; 174th | 992; 1st | |
| BRCA1 185delAG Ashkenazi | 141; 9145676 38th | 659; 485th | 373; absent | 448; present |
| BRAF V600E | 8,964; 21639808 3,954th | 31,087 | 19,493 | 32,239; present |
| EGFR lung cancer | 31,816 | 165,901 | 109,714 | |
| xqzvw zzyzxq kinase | 1,236,445, after dropping two words | 0 | 0 | |

Overlap with the floor, from full lists: for AKT1 E17K, Europe PMC's PubMed-indexed results held 167 of PubMed's 192, and LitVar2 held 187. For FGFR2 pemigatinib, Europe PMC held 78 of 107. For TP53 R337H, it held 41 of 52. No other source held every floor PMID, so the floor needs PubMed itself.

### Retrieval strategies measured on the examples

These counts illustrate how differently the sources behave. They are not grounds for advising a strategy, for the reasons in the evidence section at the top. Each strategy below was run on the 16 search questions of section 2 (examples 1 to 15 and 22, with example 23's words as a 16th). The table counts how many of the 20 accepted answer papers appear in the first 10 and the first 50 results. Strategies that need a variant count only the variant questions. "Outside the floor" is the share of a source's first 50 PMIDs that the plain PubMed search for the same words does not return at any depth, measured on the ten questions whose full floor was stored.

| Strategy | First 10 | First 50 | Cold time | Outside the floor | Rows with no PMID |
| --- | --- | --- | --- | --- | --- |
| PubMed by relevance (the plain floor) | 9 of 20 | 12 of 20 | 0.2 to 1.0 s | 0% | 0% |
| PubMed newest first | 0 of 20 | 2 of 20 | 0.2 to 0.3 s | 0% | 0% |
| PubMed's whole floor, most cited first (iCite counts) | 13 of 20 | 14 of 20 | ESearch plus 0.1 to 24 s of iCite calls in series | 0% | 0% |
| Europe PMC by relevance | 0 of 20 | 1 of 20 | 0.5 to 2.2 s | 58% | 8% |
| PubTator3 text search | 2 of 20 | 3 of 20 | 0.3 to 1.0 s | 43% | 0% |
| PubTator3 variant entity, native order | 0 of 13 | 0 of 13 | 0.3 to 1.7 s | 58% | 0% |
| LitVar2 variant list, native order | 0 of 10 | 0 of 10 | 0.2 to 0.5 s | 82% | 0% |
| LitVar2 variant list, most cited first | 5 of 13 | 10 of 13 | 0.2 to 0.5 s, plus 2 to 150 s of iCite calls in series for 450 to 32,000 PMIDs | 82% | 0% |
| PubMed and LitVar2 together, most cited first | 5 of 13 | 10 of 13 | as above | | |
| LitSense2 sentence similarity | 1 of 20 | 5 of 20 | 0.4 to 3.5 s; 2 of 17 calls took 21 s | 63% | 0% |
| Semantic Scholar by relevance, with a key | 10 of 20 | 12 of 20 | 0.5 to 1.4 s | 11% | 39% |
| 0.9 default, `--limit 50` | 10 of 20 | 12 of 20 | 3.4 to 19 s | | |
| 0.9 default page, `--limit 10`, nine questions | 1 of 13 | | 0.6 to 5.8 s | | |
| Fold of all sources, round-robin by native rank | 11 of 20 | 14 of 20 | slowest source | | |
| Fold of PubMed and Semantic Scholar, round-robin | 13 of 20 | 15 of 20 | slowest source | | |
| Fold of all sources except Semantic Scholar, round-robin | 8 of 20 | 9 of 20 | slowest source | | |
| Fold of PubMed with any one of Europe PMC, PubTator3, LitSense2 or LitVar2, round-robin | 9 of 20 | 10 of 20 | slowest source | | |

What the table shows, as observations on this set:

- **No source wins everywhere.** PubMed by relevance and Semantic Scholar each found about half the answers in their first ten, and they found different halves. Semantic Scholar found the AURA3 trial 27959700 first, where PubMed ranked it 543rd. PubMed found the KRAS phase 1 paper 32955176 fourth, where Semantic Scholar's first 50 missed it.
- **Entity sources found papers PubMed never returns.** LitVar2 and PubTator3 entity search returned such papers, and their native order put no answer in the first 50. LitVar2 lists PMIDs in an order it does not explain. Sorted by citation count, LitVar2 put the IDH1 glioma paper 19228619 first, the TP53 R337H paper 11481490 18th and the AKT1 discovery paper 22nd for the rsID query. No words-based source returned any of these three in its first 50.
- **Europe PMC and LitSense2 reach furthest beyond the floor.** 58% and 63% of their first 50 were papers that the plain PubMed search does not return. That is their job: Europe PMC reads open full text, and LitSense2 matches sentences. The accepted answers on this set are mostly landmark trials that PubMed already finds, so few of them appeared in those first 50. LitSense2 returned 50 papers for the nonsense query "xqzvw zzyzxq kinase", because a similarity search always finds a nearest sentence.
- **Most cited first favors old papers.** Sorting PubMed's floor by citation count found 13 answers in the first ten, on a set that favors landmark papers. It buries new ones: the 2020 AKT1 review 33123537 fell from PubMed's 2nd to 46th.
- **Folding costs depth.** A round-robin of two sources gives each about half the page. PubMed alone found 12 answers in its first 50; PubMed in a round-robin with Europe PMC found 10, because PubMed's results 26 to 50 moved past position 50.
- **Fold layouts.** The round-robin fold of all sources found 11 and 14. Showing each source's first ten side by side found 9, in pages of 36 to 67 papers, which costs 3.6 to 6.7 times the tokens. Listing PubMed's results first and appending the others found 9 and 12. Putting papers found by more sources first found only 4 in the first ten, because broad reviews that every source returns crowd out trials. Sorting the whole fold by citation count found 13 and 13.
- **0.9's good numbers come from Semantic Scholar.** With `--limit 50`, 0.9's first ten held 10 answers, and 0.9 credited 8 of them to Semantic Scholar rows. With the default `--limit 10`, the per-source cut leaves room for fewer of those rows, and page one carried 1 of 13 answers on nine questions.
- **PubMed's own order shifts.** In this run the larotrectinib paper 29466156 ranked 19th, the HER2-low trial 3rd and the PROfound trial 1st. Earlier the same day the first two ranked 1st and 1st. A case can assert that a paper is in PubMed's floor, never its exact rank.

Rank of each accepted answer, by strategy. A dash means absent from the first 50, or from the whole list for the citation sorts.

| Question | Answer | PubMed relevance | PubMed most cited | LitVar2 most cited | Semantic Scholar | 0.9, limit 50 |
| --- | --- | --- | --- | --- | --- | --- |
| AKT1 E17K | 17611497 | 90 | 1 | 22 | - | - |
| AKT1 E17K | 33123537 | 2 | 46 | 520 | 13 | - |
| AKT1 E17K | 18783292 | 32 | 89 | 937 | 16 | 20 |
| BRAF V600E melanoma vemurafenib | 21639808 | 1 | 1 | 5 | 1 | 1 |
| KRAS G12C sotorasib lung | 34096690 | 1 | 2 | 22 | 1 | 1 |
| KRAS G12C sotorasib lung | 32955176 | 4 | 3 | 27 | - | - |
| EGFR T790M osimertinib | 27959700 | 543 | 2 | 5 | 1 | 1 |
| IDH1 R132H glioma | 19228619 | - | - | 1 | - | - |
| TP53 R337H adrenocortical | 11481490 | - | - | 18 | - | - |
| rs121434592 | 17611497 | - | - | 22 | - | - |
| HER2-low trastuzumab deruxtecan | 35665782 | 3 | 1 | | - | 28 |
| larotrectinib TRK fusion | 29466156 | 19 | 1 | | 1 | 1 |
| FGFR2 fusion pemigatinib cholangiocarcinoma | 32203698 | 2 | 1 | | - | - |
| amivantamab lazertinib after osimertinib | 37879444 | 1 | 1 | | 1 | 1 |
| olaparib metastatic castration-resistant prostate cancer | 32343890 | 1 | 1 | | 1 | 1 |
| next-generation sequencing recommendations advanced cancer ESMO | 38834388 | 3 | 2 | | 1 | 1 |
| BRCA1 185delAG Ashkenazi | 9145676 | 38 | 1 | 2 | 7 | 10 |
| BRAF V600E | 21639808 | 3,954 | 1 | 5 | 1 | 1 |
| BRAF V600E | 27080216 | 3,531 | 112 | 602 | - | - |
| EGFR lung cancer | 27959700 | outside the window | - | | 7 | 7 |

For KRAS G12C the LitVar2 column uses rs121913530, LitVar2's entry for p.G12C, with 8,915 PMIDs. Its native order put the two trials at 4,897th and 6,343rd. An earlier run took LitVar2's first autocomplete hit, rs121913529, which is p.G12D, and missed both; section 5 makes BioMCP take only a hit whose gene and change match.

### Where sources disagree

- **Citation counts.** For 17611497, iCite counts 1,045 citing PubMed papers, Europe PMC 1,012, OpenCitations 1,165 and Semantic Scholar 1,332 (from 0.9's card). For 37879444, iCite counts 405 and OpenCitations 1.
- **Variant spelling.** PubMed matches the token "E17K" inside "p.E17K"; Europe PMC's title and abstract fields do not. 19228619 never writes "R132H" in its abstract, and 11481490 writes "adrenal cortical", so only annotation sources find them.
- **Order.** PubMed's relevance order can shift between runs for the top few results. LitVar2's order has no stated meaning.
- **Supplement files.** PMC OA lists 16 supplement files for PMC6181777, and Europe PMC's supplement ZIP held one empty file.
- **Full text.** Europe PMC's full-text service answers 500 for author manuscripts that the PMC OA bucket holds. A PMCID alone does not mean either holds a body.
- **Publication type.** PubMed lists several types per paper. 0.9 shows one, and Europe PMC adds "research-article".

### Which source owns which field

| Field | Owner | Why |
| --- | --- | --- |
| The floor: PMIDs and their relevance order | PubMed ESearch | The bar itself |
| Header fields for any PMID | PubMed ESummary | One small JSON record per PMID |
| Card fields: abstract, all authors, all publication types, retraction and erratum links | PubMed EFetch | One request returns all of them |
| Records with no PMID (preprints) | Europe PMC core record | The only source here that holds them |
| Variant-annotated PMIDs | LitVar2 | Finds 19228619, 11481490 and 9145676 that the floor misses or buries, in 0.2 s |
| Full-text-only and preprint matches | Europe PMC search | The only full-text search here |
| Citation counts, citing and cited PMIDs | iCite | Public domain, 0.1 s per paper, PMIDs ready to chain |
| Full text and supplement list, with license | PMC OA bucket | One 0.15 s call lists the license, the JATS and every file, including author manuscripts |
| Preprint full text | Europe PMC `fullTextXML` | The only source here that holds it |
| Normalized entities | PubTator3 export | Gene, variant and disease IDs that chain to other commands |
| Papers by normalized entity | PubTator3 search | Finds other spellings of a gene, disease or chemical by its ID |
| Sentences that resemble a question | LitSense2 | The only sentence-similarity search here |
| Machine-written summary, citation contexts and intents, recommendations, open PDF links | Semantic Scholar | The only source here that holds them |
| MeSH headings, keywords, affiliations, ORCID iDs | PubMed EFetch | Already in the card's reply |

### Requests and speed for the design below

- `get article <PMID>`: one EFetch call of about 0.2 s. A DOI adds one ESearch call. A PMCID adds one ESearch call.
- Each card section adds requests in parallel: `annotations` 0.2 s, `citations` 0.1 s, `fulltext` and `supplements` one OA bucket listing and record of 0.15 s each, plus the JATS download of about 0.2 s for `fulltext` and for the captions in `supplements`. `indexing` reads the card's reply and adds none. `tldr` adds one Semantic Scholar call of 0.1 to 0.2 s, plus any wait for the shared pool. `pdf` adds the same call. `article pdf --save` adds the PDF download, 4.8 MB for 41351423.
- `search article` with the default: one ESearch of 0.2 to 1.0 s for the whole window, then ESummary for the page of ten at 0.25 s. A fold adds each named source in parallel, so a cold first page takes the time of the slowest named source: 0.2 to 0.5 s for LitVar2, 0.5 to 2.2 s for Europe PMC, up to 21 s when LitSense2 stalls. `--sort citations` adds iCite for the whole pool, about 1 s per 1,000 PMIDs at four batches at a time. A warm page reads the store and makes no network request.

## 4. Delight questions

1. **What does the agent ask first, and which command answers it?** "What does the literature say about X?", where X is a variant, a drug in a biomarker group, a trial name or a guideline topic. `search article <words>` answers it with plain PubMed relevance. The agent may pick another strategy by name: another order with `--sort`, another source with `--source`, or a fold of several. Help says what each is built to find and what it costs. "Show me this paper" comes second, and `get article <ID>` answers it.
2. **What is the smallest header that lets the agent choose its next step?** PMID, DOI, PMCID when present, the whole title, journal, date, the first author with the author count, and the clinical publication types (trial phase, randomized, review, guideline, case report, retracted). On a fold, the item adds the sources that found it with each one's native rank. The citation count joins the item when the request sorts by citations.
3. **Which drill-downs does the agent need next, and does `next` name each one?** A search page's `next` names the next page with its pool and `get article` for the first header. It also names the most-cited order over the same term, with its cost. A card's `next` names the sections that have data behind them, `article citations` with each source, and `article recommendations`. A `supplements` section names `article supplement <ID> <FILE>`.
4. **Which identifiers must come back exact?** The PMID for every next call, the PMCID for full text and supplements, the DOI and Europe PMC ID for papers outside PubMed, and the supplement file name as PMC lists it. Annotations return Entrez gene IDs, rsIDs and HGVS for the gene and variant slices.
5. **Where do sources disagree, and how does the output show it?** On a fold, each item lists every source that found it with that source's own rank, and the order rule is named in the query echo. BioMCP never blends the ranks into a score. Citation counts come from iCite alone and say so.
6. **What can fail or come back empty, and how does the status tell them apart?** Search: complete with zero hits; complete and truncated at PubMed's 9,999 window; PubMed dropped some words, named in a notice on its source outcome while its state stays ok; a named source failed, so the status is partial with that source named and a `fresh` step. LitSense2 almost never returns zero, and help says its results are resembling sentences, not matches. A Semantic Scholar call that meets 429 past its retries fails with reason rate_limited and a `key` step. Recommendations and citation evidence can be complete and empty. Get: resolved, not found, or a section that is data, empty, not applicable (no PMCID), unavailable (for example `not_open`) or skipped.
7. **How fast is a cold and a warm call?** A default search cold in 0.2 to 1.0 s, set by PubMed. A fold cold in the time of its slowest source, 1 to 4 s for most and up to 21 s when LitSense2 stalls. A citation sort adds about 1 s per 1,000 PMIDs. A card cold in about 0.2 s. A warm page or card reads the store and makes no network request.
8. **What does 0.9 return that no example needs, and what does it miss?** It returns ranking weights, a semantic signal that is always zero, a session loop breaker, a debug plan and a 50 KB asset manifest of figure images. It misses the floor, totals, stable pages, a page one that does not depend on `--limit`, retraction notices and full text an agent can read. It hides which source found what and blends their ranks into one order.
9. **What lets the agent stop early with the right answer?** A first page that is exactly what the agent asked for. If it asked PubMed, it gets PubMed's first ten. If it asked for the most cited first, it gets that. The header's types tell a review from a trial without a get.
10. **What would a clinician need to trust the answer?** The PMID, journal, date and publication type, a retraction flag with the notice's PMID and the date the record was fetched, the strategy that found the paper, and for supplement facts the file name, sheet, row, checksum and license.

## 5. Design

### Retrieval strategy

The agent chooses the strategy. BioMCP runs only the sources the agent named, keeps each source's own order, folds several sources by a stated rule, and tells the agent what each source is built to find and what it costs. It never suggests that one strategy is better.

#### Sources an agent can name

| `--source` value | What it is built to find | How it reads the request | Native order | Cold time | What to know |
| --- | --- | --- | --- | --- | --- |
| `pubmed` | Keyword matches in MEDLINE and PubMed records, with PubMed's own mapping of words to MeSH terms and gene concepts | The term BioMCP builds and echoes, with filters as PubMed field tags | Best Match relevance, or most recently added with `--sort date` | 0.2 to 1.0 s | The floor. It misses papers that spell a variant another way or name it only in full text. Its top order can move between runs |
| `europepmc` | Words anywhere in the full text of open articles, and preprints | The words, with Europe PMC field filters | Europe PMC relevance, or newest first with `--sort date` | 0.5 to 2.2 s | Reads open full text, so it finds papers that name a term only in the body. Its counts run 3 to 6 times PubMed's, and many hits mention the words in passing |
| `pubtator` | Papers whose text names a normalized gene, variant, disease or chemical | The words, with `--gene` as `@GENE_<Entrez ID>` from HGNC resolution | PubTator3's own order | 0.3 to 1.7 s | Catches other spellings of an entity. Its autocomplete can pick another species: on 2026-10-24 its first gene match for AKT1 was the rat gene 24185. BioMCP therefore sends the Entrez ID that HGNC resolution gives, such as `@GENE_207` for AKT1 |
| `litvar` | Every paper that names a variant, in any spelling, including rsIDs | `--variant`, required: an rsID alone, or a protein change with `--gene` | The order LitVar2 returns, which it does not explain | 0.2 to 0.5 s for the whole list | Lists can be very long: 32,239 PMIDs for BRAF V600E. Its native order carries no stated meaning, so `--sort citations` is the usual companion |
| `litsense` | Sentences that resemble the words, from abstracts and open full text | The words as a sentence | Sentence similarity | 0.4 to 3.5 s; 2 of 17 calls took 21 s | A hit is a resembling sentence, not a match: it returned 50 papers for a nonsense query on 2026-10-23. Some calls stall: a nonsense query answered HTTP 502 after 30 s on 2026-10-24. Each item carries the sentence that matched |
| `semantic-scholar` | Relevance ranking over a large multidisciplinary index | The words, with year filters | Semantic Scholar relevance | 0.1 to 0.9 s per call, plus any wait for the shared pool | Runs without a key through a shared pool that often answers 429 for search; the user's optional key raises the limit. 39% of rows carry no PMID until resolved |

There is no `all` value. The agent lists the sources it wants. iCite is not a search source; it supplies citation counts for `--sort citations`, the `citations` section and the citation graph. LitSense2, PubTator3 and Semantic Scholar stay as named sources by Ian's ruling, and none of them is ever part of the default.

A LitSense2 item carries `sentence`, the first sentence LitSense2 returned for that paper in native order, with its section, because the sentence is what LitSense2 found. Other sources' items carry no sentence.

#### The default

With no `--source`, `search article` runs PubMed by relevance alone and returns PubMed's order unchanged, for the exact term echoed in the response. The reasons:

- It is the floor itself, so the default can never be beaten by plain PubMed for that term.
- Clinicians and agents already know it, and its results can be checked on pubmed.gov by pasting the echoed term.
- It is the fastest source, at 0.2 to 1.0 s.

The query echo names the strategy: `sources: ["pubmed"]`, `order: "rank"` and `term`, the exact ESearch term sent. A default page carries no `found_by`, because every item has one source and its rank is its position.

`next` on a default page holds the page step, `get article` for the first item, and one alternative order over the same term, `--sort citations`, with its cost: "most cited first by iCite counts; adds about 1 s per 1,000 PMIDs". When the agent passed `--gene` and `--variant`, `next` also holds the same search with `--source pubmed,litvar`. These steps are choices, not advice, and BioMCP runs none of them until the agent asks.

#### How `--gene` builds the PubMed term

`--gene` resolves the input through the HGNC bundle, then sends PubMed `(SYMBOL OR input-as-typed) AND words`, with one name when the two are equal. For `--gene HER2 low trastuzumab deruxtecan` the term is `(ERBB2 OR HER2) AND low trastuzumab deruxtecan`. The plain term `ERBB2 low trastuzumab deruxtecan` loses the DESTINY-Breast04 trial 35665782, and `(ERBB2 OR HER2) low trastuzumab deruxtecan` ranks it first. Previous symbols stay out of the term, as `decisions/2026-10-24-no-text-heuristics.md` rules: a bare previous symbol drags in unrelated papers, and SUR, a previous symbol of ABCC8, matches the French word and grew the ABCC8 group from 2,216 to 171,977 records. An old name such as SUR1 reaches PubMed only when the agent types it, which 0.9 failed to do even then. The floor for a `--gene` search is the plain PubMed result for the exact echoed term. A case checks the term.

#### How `--variant` is recognized

The article reader holds a private `ProteinChange`, until the shared variant notation crate replaces it, a typed parse of an input the agent marked with `--variant`, using the standard one-letter and three-letter amino-acid codes, such as P245S, Pro245Ser and p.P245S. BioMCP never looks for variant shapes in free text, as `decisions/2026-10-24-no-text-heuristics.md` rules. The readers use the parse to search a protein change's forms. When `litvar` is named, BioMCP sends LitVar2 autocomplete the gene and the change, or the rsID, and takes only a hit whose gene and protein change, or rsID, match. For "KRAS G12C" the first autocomplete hit is rs121913529, which LitVar2 names p.G12D; the matching hit is rs121913530, p.G12C, with 8,915 PMIDs. No match, or several, returns the candidates with how each matched, and BioMCP picks none, as the gene slice ruled for aliases. For word sources, `--variant` joins the words as typed.

#### Several sources: a fold

`--source pubmed,litvar` runs each named source in parallel with its own deadline, then folds the results at build:

1. **Resolve identifiers.** Rows with a DOI and no PMID retain their raw DOI until they are resolved to PMIDs, by Semantic Scholar's own PubMed ID when it gives one, or by one ESearch per 100 DOIs with `[doi]`. A resolved PMID's record must carry the same DOI, or the row keeps its DOI alone.
2. **Deduplicate, within each source and across sources.** The key is the PMID, then the PMCID, then the DOI, then the Europe PMC ID. Distinct PMIDs never merge. A source that returns one paper twice keeps its first rank. A citation edge with no usable identifier has no key, is never deduplicated against another anonymous edge, and has no fabricated get action. Such rows still count and keep their source contexts.
3. **Show who found it.** Each item keeps its source identity and may omit its header when hydration fails. A missing title never discards a DOI needed for a join. Each item carries `found_by`, a map from source to that source's native rank, such as `{"pubmed": 2, "semantic-scholar": 13}`.
4. **Lay it out by a stated rule.**

| `--sort` | Rule | Applies to |
| --- | --- | --- |
| `rank` (default) | Each source's own order. With one source, that order. With several, round-robin by native rank in the order the agent listed the sources; a paper already shown is skipped | Every source |
| `date` | Each named source's own newest-first order, as the source applies it. With several, the same round-robin over those orders, so the page is newest first within each source, not across sources. PubMed's order is most recently added to PubMed | PubMed and Europe PMC. Naming another source with `--sort date` is a usage error |
| `citations` | Most cited first, by iCite's count of citing PubMed papers, over every item the pool holds. Ties keep the `rank` order. Items with no PMID, or missing from iCite, come last with no `cited_by` | Every source |

Round-robin bound: when n sources are listed, the k-th result of the i-th listed source sits at position n(k−1)+i or earlier, because skipped duplicates only move items up.

Layouts measured on the examples and rejected:

- **A fused score.** Reciprocal rank fusion or weights blend incomparable ranks into one claimed order. 0.9's fused rank put the AKT1 discovery paper near 388th and hid why.
- **Agreement first.** Papers found by more sources first. It implies a consensus the sources do not claim, and on the examples broad reviews that every source returns crowded out trials.
- **Side by side.** Each source's first ten in its own block makes pages of 36 to 67 papers, 3.6 to 6.7 times the tokens of a page of ten.
- **One source first, then the rest.** It hides the other sources behind the first source's whole list.

#### Filters per source

A source applies a filter itself or not at all. BioMCP never filters a source's rows locally to fake a filter, which is how 0.9's `-d` let a lung paper rank third for rectal cancer. A filter that a named source cannot apply is a usage error with code `unsupported_filter`, naming the source and the filter, before any request.

| Filter | pubmed | europepmc | pubtator | litvar | litsense | semantic-scholar |
| --- | --- | --- | --- | --- | --- | --- |
| words | yes | yes | yes | no | yes | yes |
| `--gene` | in the term, as above | as words | entity ID | required with a protein change | as words | as words |
| `--variant` | as words | as words | as words | required | as words | as words |
| `--disease`, `--drug` | as words | as words | as words, until the disease and drug slices give IDs | no | as words | as words |
| `--author`, `--journal` | `[au]`, `[ta]` | `AUTH:`, `JOURNAL:` | no | no | no | no |
| `--since`, `--until` | `[dp]` | `FIRST_PDATE` | no | no | no | `year=A-B` |
| `--type` | `[pt]` | `PUB_TYPE:` | no | no | no | no |
| `--sort date` | most recently added | newest first | no | no | no | no |
| `--open-access` | `"pubmed pmc open access"[filter]` | `OPEN_ACCESS:y` | no | no | no | `openAccessPdf`, which means a public PDF |
| `--no-preprints` | `NOT preprint[pt]` | `NOT SRC:PPR` | no | no | no | no |
| `--exclude-retracted` | `NOT retracted publication[pt]` | `NOT PUB_TYPE:"retracted publication"` | no | no | no | no |
| `--include-retracted` (the default) | no clause | no clause | yes | yes | yes | yes |

Semantic Scholar documents `publicationTypes`, `venue` and `publicationDateOrYear` filters. Ticket 0042 maps only the year range and refuses the rest until a recorded reply shows each one's behavior.

#### Access, preprint and retraction filters

1.0 keeps 0.9's four filters under their 0.9 spellings, as Ian's ruling keeps every capability. Each one runs only where a source applies it natively, and BioMCP never removes rows after retrieval. 0.9 removed them locally from rows that carried a flag.

- `--open-access` keeps papers that a source marks open. PubMed's filter `"pubmed pmc open access"[filter]` keeps papers in the PMC open access subset: "AKT1 E17K" gave 66 of 192 on 2026-10-24, where `free full text[sb]` gave 123. Europe PMC's field is `OPEN_ACCESS:y`. Semantic Scholar's `openAccessPdf` keeps papers with a public PDF, and help says so.
- `--no-preprints` drops preprints. PubMed types them `preprint[pt]`: "AKT1 E17K" gave 189 with `NOT preprint[pt]`, leaving out 41415395, 40585122 and 38328122. Europe PMC marks preprints with source `PPR`.
- `--exclude-retracted` leaves out retracted papers. It is an explicit filter like the others. PubMed types a retracted paper `retracted publication[pt]`: "AKT1 E17K" gave 190 with `NOT retracted publication[pt]`, leaving out 22505016 and 20068047. A retraction notice carries another type and stays.
- `--include-retracted` is accepted and does nothing, because search includes retracted papers by default.

Search includes retracted papers by default and sends no retraction clause, so the default term stays the plain words and the floor stays plain PubMed. 0.9 left them out by default (0.9 `src/cli/article/dispatch.rs:187` sets `exclude_retracted` unless `--include-retracted` is given). That default hid from the agent that a paper it holds was retracted, and it moved the default away from the plain PubMed floor. 1.0 keeps retracted papers visible instead: each header and card types a retracted paper `retracted_publication`, and the card names its retraction notice in `corrections`.

An explicit filter that a named source cannot apply is `unsupported_filter`, as for every filter, and `--exclude-retracted` follows the same rule. Europe PMC's fields were not checked live on 2026-10-24, because its search answered 503 all day; ticket 0053 checks them before landing.

#### Pools and pages

Every search builds a pool once and pages over it, as `architecture.md` section 6.7 says. The pool is fixed at build and never extended.

- **Pool key.** The request key includes the ordered source list and the sort, because the round-robin depends on the order.
- **Depth at build.** PubMed's whole window of up to 9,999 PMIDs, in one call of about 1 s and 100 KB. Fetching the whole window once makes every page a slice of one snapshot and puts the complete floor in the pool, so a walk can prove it holds every PubMed PMID. LitVar2's whole list in one call, capped at 9,999 PMIDs with `page.truncated` and LitVar2's total when the list is longer. Europe PMC, PubTator3 and Semantic Scholar: their first 100. LitSense2: its one reply of 100 sentences.
- **Citation sort.** iCite runs in batches of 200, four at a time, over every PMID the pool holds. 8,915 PMIDs took 9.3 s, about 1 s per 1,000. Above 10,000 PMIDs the search refuses with a usage error that names the count and asks the agent to narrow the search. Help and the `--sort citations` step in `next` state the cost.
- **Headers.** Each page's headers come from one ESummary call for its PMIDs, about 0.25 s for ten, and are stored with the pool, so a page read twice makes no second call.
- **Totals.** `page.total` counts the distinct items the pool holds. Each source outcome carries its own total and the depth fetched, such as PubMed's 192 and Europe PMC's 1,204 of which 100 were fetched for AKT1 E17K. `page.truncated` is set when a source's total exceeds what the pool holds, such as PubMed's 31,816 for "EGFR lung cancer" against its window of 9,999.
- **End of a walk.** A walk ends when the pool's items are exhausted. The last page has no page step.
- **Failure.** A named source that fails leaves the pool partial with that source named and a `fresh` step. When PubMed is named and fails, the status is failed, because the floor cannot be met.

#### What the response says about the strategy

The query echo holds the ordered sources, the sort and the exact term each source was sent. Each source outcome carries its state, reason, total, depth fetched and notices. PubMed's dropped words from `errorlist.phrasesnotfound` are a notice, and its state stays ok, because PubMed answered the words it kept. The shipped source outcomes include publisher terms; section 6 measures their full size separately from comparable payload ceilings.

`biomcp search article --help` carries the source table in short form:

```
--source LIST   Sources to search, comma separated. Default: pubmed.
                Several sources are folded: duplicates removed, each item shows every
                source and its rank. BioMCP never blends sources into one score.
                pubmed            keyword search with PubMed's MeSH mapping; the plain PubMed result
                europepmc         words in open full text, and preprints
                pubtator          normalized genes, diseases and chemicals; catches other spellings
                litvar            every paper naming a variant; needs --variant (an rsID,
                                  or --gene with a protein change);
                                  its own order has no stated meaning
                litsense          sentences that resemble the words; a hit is a resemblance,
                                  not a match, and it answers even nonsense; slow calls happen
                semantic-scholar  relevance ranking over a large multidisciplinary index; runs
                                  without a key through a shared pool that often answers 429;
                                  your optional key raises the limit; some rows lack a PMID
--sort RULE     rank (default): each source's own order; several sources take turns
                date: newest first within each source; pubmed means date added to PubMed;
                      pubmed and europepmc only
                citations: most cited first by iCite counts; about 1 s per 1,000 PMIDs;
                      refuses above 10,000 PMIDs
--open-access   only papers a source marks open; pubmed: PMC open access subset;
                europepmc: OPEN_ACCESS; semantic-scholar: has a public PDF
--no-preprints  leave out preprints; pubmed and europepmc only
--exclude-retracted  leave out retracted papers; sources that cannot are a usage error
--include-retracted  the default; headers and cards flag retracted papers
```

#### How the design prevents each 0.9 retrieval failure

| 0.9 failure (section 1) | Design element that prevents it |
| --- | --- |
| Plain PubMed's top papers missing from page one (1) | The default is PubMed's own order. A fold naming PubMed holds its whole window, and the round-robin bound keeps its results near the top |
| ESearch sent with no sort (2) | The PubMed adapter sends `sort=relevance`, or no sort only for `--sort date`, and a case asserts the parameter |
| Dropped words pass silently (3) | PubMed's outcome carries the dropped words as a notice |
| No total, no stable pages (4) | Pools with `page.total`, each source's own total, and `--pool` on every next step |
| Rows without a PMID take slots (5) | They appear only from a source the agent named, are resolved to PMIDs where possible, and sort last by citations |
| `variant articles` misses key papers (6), strict queries cut at 25 (23) | No hidden sub-queries and no 25-row cut. `--source litvar` returns LitVar2's whole list |
| Source cap before ranking (7), page one depends on `--limit` (21) | No per-source cap. A page is a slice of a stored order, so `--limit` changes only the slice length |
| Idle semantic weight (8, 29), fused score (24) | No weights and no fused score. LitSense2 runs only when named, with its own order |
| Each page rebuilds the search (22) | One fixed pool per request. A warm page reads the store and makes no network request |
| One deadline starves later sources (25) | Parallel legs, each with its own deadline, as section 6.4 says. Sources share no state, so one failure never becomes another source's empty answer |
| Paging never finishes (26) | The pool is fixed, so the walk ends at its last item |
| Date sort puts future issues first (27) | PubMed's date order is most recently added. Other sources apply their own date sort or refuse it |
| Narrow fields missed papers (28) | Each source receives the agent's words in its own default fields. Europe PMC searches full text by default |
| Old gene names not searched (30) | `--gene` sends the input as typed with the approved symbol, so an old name the agent types reaches PubMed. BioMCP adds no previous symbol by itself. LitVar2 and Europe PMC full text find other spellings from the source side |
| A variant named only in a supplementary table (31) | `article supplement --find` and `article text --find` search the open full text and its supplementary files |
| PubTator autocomplete picks another species (17), LitVar2's first hit is another change | LitVar2 hits must match gene and protein change; PubTator3 entity IDs come from human gene resolution |
| A source failure looks empty (20) | Every named source reports its own state, and the status is partial or failed |
| Search hides retracted papers by default (11) | Search includes retracted papers by default, and headers and cards flag them with the retraction notice. `--exclude-retracted` leaves them out only when the agent asks |

### biodata types

Identifier newtypes, each with a checked constructor, `FromStr` and `Display`:

| Type | Text form | Rule |
| --- | --- | --- |
| `Pmid` | `17611497` | `NonZeroU32`, already in `architecture.md` |
| `Pmcid` | `PMC6181777` | Prefix `PMC` and a nonzero number. Parses bare digits only where the caller says the namespace is PMC |
| `Doi` | `10.1056/NEJMoa1203421` | Prefix `10.`, a registrant, a slash and a suffix. Compares and hashes without ASCII case, prints as given |
| `EuropePmcId` | `PPR1064174` | Europe PMC source prefix and digits, for records with no PMID |

Closed sets: `PublicationType` (non-exhaustive: `randomized_controlled_trial`, `clinical_trial`, `clinical_trial_phase_1` to `clinical_trial_phase_4`, `meta_analysis`, `systematic_review`, `review`, `practice_guideline`, `guideline`, `case_report`, `observational_study`, `multicenter_study`, `letter`, `comment`, `editorial`, `preprint`, `retracted_publication`, `retraction_notice`, `erratum`). PubMed's funding types such as "Research Support, N.I.H., Extramural" and "Journal Article" map to no value and are dropped without a problem, because they carry no clinical meaning. `CorrectionKind` (`retraction`, `erratum`, `expression_of_concern`). `License` (non-exhaustive: `cc0`, `cc_by`, `cc_by_sa`, `cc_by_nc`, `cc_by_nc_sa`, `cc_by_nd`, `cc_by_nc_nd`, `public_domain`, `tdm`). `FileKind` (non-exhaustive: `xlsx`, `xls`, `csv`, `tsv`, `docx`, `doc`, `pdf`, `zip`, `txt`, `image`, `video`). `ArticleMatchKind` (`pmid`, `pmcid`, `doi`, `europepmc_id`). `Order` (`rank`, `date`, `citations`). `EdgeSource` (`icite`, `semantic-scholar`). `CitationIntent` (non-exhaustive: `background`, `methodology`, `result`), Semantic Scholar's three intents. `OpenAccessStatus` (non-exhaustive: `gold`, `green`, `hybrid`, `bronze`, `closed`), as Semantic Scholar reports it.

`ArticleInput` is free text that the edge parses into a `Pmid` (digits), a `Pmcid`, a `Doi` (starts `10.`, or a `doi.org` URL) or a `EuropePmcId`.

`Article` (schema `biodata.article/1`), from PubMed EFetch, or from Europe PMC for a record with no PMID:

- `pmid: Option<Pmid>`, `pmcid: Option<Pmcid>`, `doi: Option<Doi>`, `europepmc_id: Option<EuropePmcId>`; at least one is present
- `title: Text`, `journal: Option<Text>`, `date: Option<PartialDate>`
- `types: Box<[PublicationType]>`
- `corrections: Box<[Correction]>`, each `{ kind: CorrectionKind, relation: CorrectionRelation, pmid: Option<Pmid> }`, where `relation` is `notice` when the PMID names the correction notice and `original` when it names the corrected paper
- `published_as: Option<ArticleRef>`, for a preprint whose Europe PMC record links its published version
- `authors: Box<[Text]>`, whole
- `abstract_parts: Box<[AbstractPart]>`, each `{ label: Option<Text>, text: Markdown }`, where the JATS converter below turns italics, bold, superscripts and subscripts into Markdown

`ArticleHeader` (schema `biodata.article_header/1`): `pmid`, `doi`, `pmcid`, `europepmc_id`, `title`, `journal`, `date`, `types`, `first_author` and `author_count`. A header for a row with no PMID carries `types_unknown: true`, because no PubMed record says whether it was retracted.

`ArticleItem` holds an optional checked `key`, an optional `pmid`, an optional hydrated `header`, `found_by`, optional iCite `cited_by`, and an optional source `sentence`. A source row survives missing header metadata; DOI joins retain the raw secondary DOI even when another identifier wins key priority. `found_by` records native ranks only for a multi-source fold. The sentence is the first matching sentence returned in LitSense2 order. These source facts belong to the item, not the hydrated header.

`ArticleEdgeItem` shares that item shape. With Semantic Scholar, optional `intents`, optional `influential` and optional capped `contexts` distinguish an unreported value from an empty list or false. A page gives the first context and the source total; the pair reader pages every exact source context. Anonymous edges keep no key, receive no fabricated action and never deduplicate against other anonymous edges.

The iCite `citations` section carries optional `cited_by`, `cited_by_clinical`, `references` and `rcr`. Clinical and reference counts count distinct IDs from present source lists. An absent list leaves its count absent, while a present empty list gives zero. The relative citation ratio appears only when the source confirms it is not provisional.

`Markdown` is a `Text` newtype whose content is Markdown. It tells the Markdown view to print the field as it is.

`ArticleResolution`: `resolved { card }` or `not_found`. An article input is never ambiguous: each identifier names one record. A DOI input is looked up with ESearch `[doi]`, and the returned record's DOI must equal the input, compared without ASCII case, or the result is `not_found`. A DOI shared by two PMIDs resolves to both as two items, never merged, as `architecture.md` section 6.5 says.

### Get article: the disclosure layers

`get article` follows the four layers of `architecture.md` section 3.10.

| Layer | What it holds | Source | When it costs a request |
| --- | --- | --- | --- |
| 1. Header | PMID, DOI, PMCID, title, journal, date, clinical types, first author and author count | PubMed ESummary, or Europe PMC for a record with no PMID | Shown on search pages; never requested alone |
| 2. Card | The header fields plus all clinical types, corrections with notice PMIDs, the published version of a preprint, ten authors with the total, the whole abstract with its labels, and the date the record was fetched | PubMed EFetch, one call; Europe PMC core for a record with no PMID | Every `get article` |
| 3. Sections, each a `Section<T>` with five states | `fulltext`: license and the outline of the body. `supplements`: license and each supplementary file. `annotations`: normalized entities. `citations`: counts. `tldr`: Semantic Scholar's one-sentence summary and its citation counts. `pdf`: the open PDF link's status, license and URL. `indexing`: MeSH headings, keywords, affiliations and ORCID iDs | `fulltext` and `supplements`: the PMC OA bucket alone, or Europe PMC full text for a preprint. `annotations`: PubTator3. `citations`: iCite. `tldr`: Semantic Scholar's paper record. `pdf`: Semantic Scholar's `openAccessPdf` and the PDF it names. `indexing`: the card's EFetch reply | One request each, in parallel. `indexing` reads the card's reply and costs none. `pdf` only reports the source link; `article pdf <ID> --save PATH` adds the download |
| 4. Steps | `article text <ID> --part N`, `article pdf <ID> --save PATH`, `article supplement <ID> <FILE>`, `article citations`, `article references`, `article citation-evidence`, `article recommendations` | As the section | One request per step, plus the file download for a supplement; PDF saves require an explicit path |

**Abstract.** The abstract is the card's main content, 60 to 75% of its tokens.

**Full text and supplements come from one source each.** For a PMC article, `fulltext` and `supplements` come only from the PMC OA bucket. Its JSON record gives the license, `xml_url` for the JATS full text and `media_urls` for every supplementary file, in one call of about 0.15 s; the JATS file for PMC9116274 downloaded in 0.2 s. A PMCID does not mean open full text: PMC13020499 and PMC13020505 have no body and no OA files. A PMCID that the bucket does not hold gives both sections `unavailable` with reason `not_open`. A paper with no PMCID gives `not_applicable`. For a preprint, `fulltext` comes from Europe PMC `fullTextXML` through the same JATS reader: PPR1295425 returned 92 KB of JATS under CC BY in 0.7 s, and PPR1064174 answered 500, which gives `unavailable` with the retained source-failure reason. NCBI EFetch for PMC and Europe PMC's supplement files are not used, so each section has one source, as rule 2 of section 3.10 asks.

**Licenses.** Text under a text-mining license or a non-commercial license is served only by the local CLI and MCP, for the user's own use, and never by a later web mode. Each section and reader reply names the license.

**Retractions.** The card shows `fetched`, the time its EFetch record was read. Records from ESummary and EFetch, which carry retraction status, use a freshness of one day in the cache, shorter than the default.

**Indexing.** 0.9's `indexing` section stays. PubMed EFetch already returns MeSH headings with their major-topic flags, author keywords, affiliations and ORCID iDs in the reply the card reads, so the section is built from that reply and makes no request. MeSH headings help an agent restate a question in PubMed's own vocabulary. The section caps affiliations at ten with the total.

**TLDR.** `tldr` holds Semantic Scholar's machine-written one-sentence summary with its model name, and Semantic Scholar's `cited_by` and `influential_cited_by` counts, which 0.9 showed as influence counts. For 17611497 on 2026-10-24 it returned "The identification of a somatic mutation in human breast, colorectal and ovarian cancers that results in a glutamic acid to lysine substitution at amino acid 17 (E17K) in the lipid-binding pocket of AKT1 is reported." with model `tldr@v2.0.0`, 1,332 citations and 82 influential ones. The section says the summary is machine-written. A paper Semantic Scholar holds without a summary gives `empty`. A paper it does not know gives `unavailable` with reason `not_found`. A 429 that outlasts the retries gives `unavailable` with reason `rate_limited` and a `key` step, because the optional key raises the limit. The section asks `GET /graph/v1/paper/PMID:<pmid>?fields=tldr,citationCount,influentialCitationCount`, or `DOI:<doi>` for a paper with no PMID.

**PDF metadata and download.** The download-only ruling replaces the proposed extractor. The shipped `pdf` section reads Semantic Scholar's `openAccessPdf` and returns the URL, status and license without downloading. `article pdf <ID> --save PATH [--replace]` downloads only to the caller's explicit path. Saved-file replies carry the absolute path, bytes, SHA-256, license, status and source URL. Existing files need explicit replacement, and failures leave no partial file. HTTP mode refuses saves and returns the URL. BioMCP keeps no download folder, saved-file inventory or saved-file cache.

The downloader preserves the public-DNS-host boundary on every redirect, sends no credential, limits redirects and bytes, refuses a configured publisher proxy, and requires `%PDF-`. It does not parse, decrypt, count pages, extract text, create a worker or store PDF parts. The canonical limits and states belong to the download-only decision and the implementing cases.

`next` on a card names only sections that can have data behind them: `fulltext` when the paper has a PMCID or is a preprint with full text, and `supplements` only when it has a PMCID. A `fulltext` section that is `not_applicable` or `unavailable` with reason `not_open` names `get article <ID> pdf`. A card always names `tldr` as a choice, because whether Semantic Scholar holds a summary is unknown until asked.

### Readers for full text and supplementary files

The save inputs below describe the shipped explicit-path commands. PDF saves download bytes; text saves render JATS Markdown; supplement saves preserve original bytes.

| Reader | What it does | Limits |
| --- | --- | --- |
| `article text <ID> --part N` | Returns one part of the outline as Markdown | A part is split at 4,000 tokens, and a split part carries a step to its continuation. Each table is its own part |
| `article text <ID> --find TEXT` | Returns the paragraphs and tables that contain the text, as Markdown, each with its part number | 25 hits per call with a page step |
| `article text <ID> --save PATH` (pending 0076) | Writes every JATS part in order as one Markdown file with a header block | Explicit path; a folder PATH gets `<ID>.md` |
| `article supplement <ID> <FILE>` | Without `--find`, returns the file's sheet list with each sheet's row count, and pages of rows by range with `--rows A-B` | 200 rows per page |
| `article supplement <ID> <FILE> --save PATH` (pending 0076) | Writes the file's original bytes, including pdf and docx files the reader marks `unsupported` | 50 MB; a folder PATH gets the file's own name |
| `article supplement <ID> <FILE> --find TEXT` | Returns matching rows with the sheet, row number, header row and the file's SHA-256. The header row is the sheet's first row | 25 hits per call. The match is literal and ignores case |
| `article supplement <ID> <FILE> --variant CHANGE`, `article text <ID> --variant CHANGE` | The same hits for any form of a protein change | `ProteinChange` parses the change, and the reply prints the forms it searched: for P245S, `P245S`, `Pro245Ser`, `p.P245S` and `p.Pro245Ser`. Each form matches literally, ignoring case |

Binary reply recordings use ordered per-exchange `.bin` sidecars. Each path stays inside its recording directory; replay rejects escapes and symlinks and consumes exchanges in order. Disabled sources are refused before replay or cache reads. Workbook validation precedes cache admission. CSV, TSV, TXT and XLSX readers share bounded row output, preserve original row and column positions, and treat only original row one as a header. PDF, DOCX and image supplements remain unsupported and expose the source URL.

#### Body text as Markdown

The JATS reader turns each part of the body into Markdown, so the structure an agent needs to read a paper survives. One small converter serves the PMC OA bucket's JATS, Europe PMC's preprint JATS and the labelled abstract parts from PubMed EFetch.

- **What survives.** Section titles become headings at their depth. Lists become Markdown lists. `bold` and `italic` become `**` and `*`. `sup` and `sub` stay as `<sup>` and `<sub>`, so IC<sub>50</sub> keeps its meaning. A citation `xref` prints its marker in square brackets, such as `[11]` or `[9,11,13]`, and a `sup` that holds only citation markers prints the brackets alone. Figure captions print as `**Figure 2.** caption`. Everything else prints as its text.
- **A text part** is `{ part, heading_path, markdown }`, where `heading_path` lists the section titles above the part, such as `["Results", "Identification of candidate driver mutations in AKT1, AKT2, and AKT3"]`.
- **A table part** appears once, as Markdown: its label and caption, then a Markdown table. A cell that spans columns repeats its text in each column it spans. There are no separate row arrays for body tables.
- **`--find`.** A hit in a paragraph quotes that paragraph's Markdown. A hit inside a table quotes the table's caption, its header row and the matching rows as a small Markdown table, with the part number, so the agent can read the whole table with `--part`.
- **`--format md`.** The Markdown view prints a `Markdown` field as it is instead of escaping it.
- **Supplementary spreadsheets** keep their sheet and row numbers, because a spreadsheet's row number is its natural address and `--rows` pages through it.

On 2026-10-24, PMC9018718 (35440569, CC BY) held nested section titles under Results, 82 numbered citation markers, 58 `sup` and 2 `sub` elements; its first Results subsection writes *AKT1* in italics and cites `<sup>11</sup>`. PMC5606870 (28944238, CC BY) held 9 tables; Table 1, "Sample characteristics", has a header row of sites and a row "Age at diagnosis of cases (range)". None of the open papers checked held a JATS `list`, so lists are proved by a hand-built synthetic JATS file.

The first version of the supplement reader reads xlsx, csv, tsv and txt. A pdf or docx supplement returns `unsupported` with the file's URL, and `--save` still writes it. Downloads stop at 50 MB; the largest example file was 2.3 MB. These readers are drill-downs under `get article`, within `decisions/2026-10-23-search-and-get-is-the-1-0-scope.md`.

New dependencies, added to `architecture.md` section 2.2 by the stage that builds the readers: `calamine` for xlsx and `csv` for csv and tsv. JATS uses the planned `quick-xml`. Both crates are MIT licensed; the stage runs `cargo deny` and brings any transitive license outside the allowlist through the license exception process.

### Semantic Scholar's citation graph and recommendations

These commands restore 0.9's Semantic Scholar capabilities as pivots. Each takes a PMID, a PMCID or a DOI, sent to Semantic Scholar as `PMID:`, `PMCID:` or `DOI:`. Every request runs without a key at the shared-pool quota of section 7, or with the user's optional key at the keyed rate. None of them is part of a default command.

**`article citations --source semantic-scholar` and `article references --source semantic-scholar`.** iCite stays the default source, because it gives PMIDs and a total in one call. With `--source semantic-scholar`, BioMCP asks `GET /graph/v1/paper/<ID>/citations` (or `/references`) with `fields=contexts,intents,isInfluential,externalIds,title,year` and `limit=1000`, and follows `next` for at most five calls, so the pool holds up to 5,000 rows. The total is the row count when Semantic Scholar's reply holds no `next`. When rows remain after five calls, `page.truncated` is set and a notice names iCite's `article citations` for the full count. The order is Semantic Scholar's own, which lists newer citing papers first. Rows with a PubMed ID take ESummary headers; other rows keep Semantic Scholar's title, year and DOI with `types_unknown`.

On 2026-10-24, the first call for 17611497 returned 1,000 rows in 0.6 s and 559 KB, with `next` 1,000; the second returned the last 332, so Semantic Scholar lists 1,332 citing papers. 501 of the first 1,000 carried contexts, 65 were marked influential and 885 carried a PubMed ID. Intents counted 398 `background`, 13 `result` and 8 `methodology`. References for 33123537 (CC BY) returned 116 rows, 80 with contexts. References for 34096690 returned 38 rows and no contexts, and its phase 1 reference 32955176 carried no PubMed ID in Semantic Scholar, only a title, so a publisher can withhold contexts.

**`article citation-evidence <CITING> <CITED>`.** BioMCP reads the citing paper's references from Semantic Scholar, as above, and keeps the row whose PubMed ID equals the cited PMID, or whose DOI equals the cited paper's DOI from ESummary. The reply preserves every context, the intents and the influential flag across bounded pages. Oversized source strings split into exact UTF-8 fragments, with their original positions and character offsets; concatenating them reconstructs the original string without rewriting source Markdown. Stored continuation positions preserve the checked source scan and make the warm continuation network-free. For 33123537 citing 17611497 on 2026-10-24, Semantic Scholar gave 6 contexts, intent `background` and `isInfluential` true. A pair with no matching row is `empty` with the notice `not_in_references`. A matching row with no contexts is `empty` with the notice `no_contexts`, as for 34096690 and 32955176. With `--fulltext`, BioMCP also reads the citing paper's JATS from the PMC OA bucket, finds the `ref` whose `pub-id` equals the cited PMID or DOI, and returns each paragraph that holds an `xref` to it, as Markdown with its part number. PMC7573235, the JATS of 33123537, cites 17611497 as reference B17 in 7 paragraphs. A citing paper with no PMCID or outside the bucket gives that part `not_applicable` or `unavailable` with reason `not_open`, beside Semantic Scholar's contexts.

**`article recommendations <ID>... [--negative ID]...`.** One seed and no negatives asks `GET /recommendations/v1/papers/forpaper/<ID>`; several seeds or any negative asks `POST /recommendations/v1/papers` with `positivePaperIds` and `negativePaperIds`. Both send `fields=externalIds,title,year` and `limit`. The page keeps Semantic Scholar's order, and its query echo says recommendations come from Semantic Scholar's pool of recent papers. As Ian ruled, a recommended paper may not share the seed's gene or topic; it serves discovery at the edges. On 2026-10-24, `forpaper/PMID:34096690` returned five 2026 papers, among them 42745055 and 42685317, and the POST with 34096690 and 32955176 against 33433946 returned the same first two. `forpaper/PMID:17611497` returned `{"recommendedPapers": []}`, which gives a complete page with total 0 and the notice `no_recommendations`.

**Failure.** A 429 that outlasts the retries makes the command failed with reason `rate_limited` and adds a `key` step, `biomcp help key semantic-scholar`. A seed Semantic Scholar does not know answers 404, which gives `not_found` for that input.

### Command tree

| Command | Returns | Notes |
| --- | --- | --- |
| `biomcp search article [TEXT]` | `ArticleItem` page from the pool | Words as the agent would type them into PubMed |
| `--source LIST` | Which sources run, in round-robin order | `pubmed` (default), `europepmc`, `pubtator`, `litvar`, `litsense`, `semantic-scholar` |
| `--sort RULE` | The stated order | `rank` (default), `date`, `citations` |
| `--gene INPUT` | Resolved through the HGNC bundle | PubMed term `(SYMBOL OR input) AND words`; entity ID for `pubtator`; gene for `litvar` |
| `--variant TEXT` | Protein change or rsID | Required by `litvar`, matched through LitVar2 autocomplete; words for the others. `variant articles` becomes this flag |
| `--disease TEXT`, `--drug TEXT` | Words | PubMed maps words to MeSH itself |
| `--author TEXT`, `--journal TEXT` | Native field filters | Usage error for a named source that cannot apply them |
| `--since DATE`, `--until DATE` | Native date filters | PubMed, Europe PMC and Semantic Scholar only |
| `--open-access`, `--no-preprints`, `--exclude-retracted`, `--include-retracted` | Native access, preprint and retraction filters | Retracted papers are included by default and flagged. `--exclude-retracted` leaves them out where the source can |
| `--type TYPE` | Native publication type filters | `trial`, `randomized`, `review`, `systematic_review`, `meta_analysis`, `guideline`, `case_report` |
| `--limit N`, `--offset N`, `--pool ID` | The shared cursor | Limit 1 to 50, default 10 |
| `biomcp get article <INPUT>... [SECTION]...` | One card per input, in input order | PMIDs, PMCIDs, DOIs and Europe PMC IDs, up to 20. Sections `fulltext`, `supplements`, `annotations`, `citations`, `indexing`, `tldr`, `pdf`, `all`. `all` includes PDF metadata because it makes no download |
| `biomcp article text <ID> (--part N \| --find TEXT \| --variant CHANGE)` | One JATS part, or matching passages | The outline names the parts; PDF text extraction is out of scope |
| `biomcp article supplement <ID> <FILE> [--rows A-B \| --find TEXT \| --variant CHANGE]` | Sheet list, a page of rows, or matching rows | xlsx, csv, tsv and txt |
| `biomcp article citations <ID> [--source icite\|semantic-scholar]` | Citing papers with the total | `icite` (default): headers, most cited first. `semantic-scholar`: Semantic Scholar's order, with intents, the influential flag and the first context |
| `biomcp article references <ID> [--source icite\|semantic-scholar]` | Cited papers with the total | As `article citations` |
| `biomcp article citation-evidence <CITING> <CITED> [--fulltext]` | Every context where one paper cites another, with intents | Semantic Scholar; `--fulltext` adds the citing paper's paragraphs from the PMC OA bucket |
| `biomcp article recommendations <ID>... [--negative ID]... [--limit N]` | Papers Semantic Scholar recommends | Up to 10 seeds and 10 negatives; limit 1 to 100, default 10 |

`next` on a search page: the next page with `--pool`, `get article` for the first item, and the alternative orders and sources described above. On a card: one `get article <PMID>` step listing the sections that can have data, then `article citations <PMID>`, `article citations <PMID> --source semantic-scholar` and `article recommendations <PMID>`. On a `fulltext` section: `article text <ID> --part 1`. On a `pdf` section: `article pdf <ID> --save <ID>.pdf`. On a `supplements` section: `article supplement <ID> <FILE>` for the first readable file. On a Semantic Scholar edge item with more contexts: `article citation-evidence <CITING> <CITED>`.

### Keep, change or drop each 0.9 feature

| 0.9 feature | Ruling | Reason |
| --- | --- | --- |
| `search article [QUERY]` | Keep, change | One positional `TEXT`. Default is plain PubMed relevance |
| `-k`/`--keyword` | Rename to positional `TEXT` | `-k` returns fixed migration guidance; `--keyword` is refused by checked grammar. The words remain available as TEXT |
| `-q`/`--query` | Rename to positional `TEXT` | Both return `use: search article <text>` |
| `-g`/`--gene` | `--gene`; short flag retired | Resolved through HGNC and sent as the approved symbol and the input as typed. Example 26 |
| `-d`/`--disease` | `--disease`; short flag retired | Words for PubMed's MeSH mapping |
| `--drug` | Keep | Same |
| `-a`/`--author` | `--author`; short flag retired | Example 18 |
| `--journal` | Keep | Native field filter |
| `--date-from`/`--since` | Rename `--since` | One spelling per idea. Example 19 |
| `--date-to`/`--until` | Rename `--until` | Same |
| `--year-min` | Drop | `--since` covers it |
| `--year-max` | Drop | `--until` covers it |
| `--type` | Keep, change | Values from `PublicationType`, including `guideline` and `trial`. Example 16 |
| `--open-access` | Keep, change | Native filters only: PubMed's PMC open access subset, Europe PMC `OPEN_ACCESS:y`, Semantic Scholar's public PDF. Other sources refuse it |
| `--no-preprints` | Keep, change | Native filters only: PubMed `NOT preprint[pt]`, Europe PMC `NOT SRC:PPR` |
| `--exclude-retracted` | Keep, change | No longer the default. An explicit filter, applied natively, and `unsupported_filter` where a source cannot apply it |
| `--include-retracted` | Keep, change | The default. Accepted and does nothing |
| `--sort` | Keep, change | `rank`, `date` and `citations`. Issue 69 |
| `--ranking-mode` | Retired; maps to no blend | Fixed usage error: 1.0 has no score blends, as `decisions/2026-10-24-no-text-heuristics.md` rules. Each source keeps its own order |
| `--weight-semantic` | Retired; maps to no blend | Maps to no blend. The weight was always zero |
| `--weight-lexical` | Retired; maps to no blend | Maps to no blend |
| `--weight-citations` | Retired; maps to no blend | Maps to no blend. The message names `--sort citations`, a sort the agent asks for |
| `--weight-position` | Retired; maps to no blend | Maps to no blend |
| `--source` | Keep, change | The agent's strategy choice: one source or several, in round-robin order. No `all` |
| `--max-per-source` | Drop | It dropped rows before ranking |
| `--session` | Drop | No example. Next steps replace loop-breaker suggestions |
| `--debug-plan` | Drop | Retired as `dropped in 1.0`; the command has no hidden planner |
| `--full` | Drop | One header shape. The abstract lives on the card |
| `--limit`, `--offset` | Change | The shared cursor with `--pool` |
| `get article <ID>` | Keep, change | Many inputs at once. All publication types and corrections. The default card makes no Semantic Scholar call; `tldr` and `pdf` are sections |
| `annotations` | Keep | Example 34 |
| `indexing` | Keep | MeSH headings, keywords, affiliations and ORCID iDs from the card's EFetch reply, with no extra request |
| `fulltext` | Change | The section returns the outline from the OA bucket. No file writes |
| `--outline` | Drop | The section is the outline |
| `--lines` | Change to `article text --part N` | Parts replace line ranges |
| `--pdf` | Replaced by explicit section and save | `get article <ID> pdf` reports Semantic Scholar's `openAccessPdf`; `article pdf <ID> --save PATH` downloads it. The download-only ruling removes PDF extraction |
| `--out` | Replaced by `--save PATH` | Explicit-path text, supplement and PDF saves are shipped |
| `--output` | Replaced by `--save PATH` | Same |
| `tldr` | Keep | Section from Semantic Scholar's paper record, with its citation and influential citation counts |
| `assets` | Rename `supplements` | Supplementary files only, with license. Figures stay in the full-text outline |
| `--asset-view` | Drop | One view |
| `--asset-limit` | Drop | Files are few; the section lists them all |
| `--asset-offset` | Drop | Same |
| `asset <key>` | Change to `article supplement <ID> <FILE>` | Returns extracted rows. Original-byte saves await 0076; the current reader supplies the file's URL |
| `article supplement-search` | Fold into `article supplement --find` and `--variant` | Example 31 by `--find`, example 32 by `--variant` |
| `article authors` | Drop | Authors are on the card |
| `article entities` | Fold into `annotations` | One spelling |
| `article batch` | Fold into `get article` with several inputs | One spelling |
| `batch article` | Fold into `get article` with several inputs | One spelling |
| `article citations` | Keep, change | iCite PMIDs, most cited first, with a total, by default. `--source semantic-scholar` gives Semantic Scholar's list with citation contexts, intents and the influential flag |
| `article references` | Keep, change | Same |
| `article citation-evidence` | Keep | Semantic Scholar's contexts and intents for one citing and one cited paper; `--fulltext` adds the citing paper's own paragraphs from the PMC OA bucket |
| `article recommendations` | Keep | Semantic Scholar's recommendations for one or more papers, with `--negative`. An empty answer is a stated state |
| `variant articles` | Move to `search article --variant` | The variant slice owns variant parsing |
| MCP `variant_articles` | Move to the one MCP tool's `search article --variant` | One MCP tool |
| `gene articles` | Move to `search article --gene` | The gene slice ruled it |
| `disease articles` | Move to `search article --disease` | One spelling per idea |
| `drug articles` | Move to `search article --drug` | One spelling per idea |
| `pathway articles` | Move to the pathway slice | Its examples include a KEGG ID |

### Sources kept and dropped

Search sources the agent names: PubMed (the default), Europe PMC, PubTator3, LitVar2, LitSense2 and Semantic Scholar, which runs without a key and uses the user's optional key for a higher limit. Get and support sources: PubMed ESummary and EFetch, the PMC OA bucket, Europe PMC core records and preprint full text, PubTator3 export, iCite, and Semantic Scholar's paper record, citation graph, recommendations and open PDF links.

Replaced, with each capability kept through another source: NCBI EFetch for PMC (the OA bucket holds the JATS for every open article, including the example author manuscripts), Europe PMC supplement files (its ZIP for PMC6181777 held one empty file), OpenCitations (iCite covers PMIDs, and its count for 37879444 was 1 against iCite's 405), Figshare (no example), PMC article HTML pages (bot challenges), the PMC ID converter (ESearch `[doi]` and `[pmc]` and the OA JSON cover it) and the bioRxiv API (Europe PMC holds the preprints the examples need).

Every E-utilities request sends `tool=biomcp`, and sends `email` only when the user configures one.

### Changes to `architecture.md`

| Section | What it says now | Change |
| --- | --- | --- |
| 3.6 `Source` | One variant per source, including `pubmed`, `europepmc`, `pubtator` | Add `litvar`, `litsense`, `semantic-scholar`, `icite` and `pmc-oa`. `semantic-scholar` is the one spelling everywhere |
| 3.8 `SourceOutcome` | `source`, `state`, `reason`, `count` | Add `term` (the exact query sent), `fetched` (depth held) and `notices` (such as PubMed's dropped words) |
| 3.8 `Reason` | Eleven reasons | Add `unsupported_filter`, carried by the usage error, and `not_open` for a PMC article outside the OA bucket or a preprint without full text |
| 3.10 rule 2 | One section has one source and one request | State that `fulltext` and `supplements` each have one source, the PMC OA bucket, with Europe PMC full text for preprints, and that the readers are steps |
| 4.5 Quotas | One limiter per source key | Add keys: `ncbi-eutils` (3 per second, 10 with a key, already planned), `europepmc` (assumed 10 per second), `pubtator` (3 per second), `litvar` (assumed 3 per second), `litsense` (assumed 3 per second), `semantic-scholar` (1 request every 2 seconds without a key, 1 per second with the user's optional key, one in flight), `icite` (assumed 5 per second) and `pmc-oa` (assumed 10 per second). Assumed rates are confirmed against each provider before the adapter lands |
| 5.7 Size | Estimates per adapter kind; the article slice measures first | Estimate for this slice: PubMed 400 lines, Europe PMC 350, PubTator3 300, LitVar2 150, LitSense2 120, Semantic Scholar 350 with its paper record, citation graph and recommendations, iCite 150, PMC OA bucket 200, the JATS reader with its Markdown converter 400, the pending credential-free PDF downloader and the supplement readers 250, about 2,800 lines in all |
| 6.4 Fan-out | One job per source, each with its own deadline | Keep. Article jobs run only for named sources |
| 6.5 Merge | Pure union by canonical ID, `found_by` kept | Keep. Resolve DOI-only rows to PMIDs at build; key on PMID, PMCID, DOI, then Europe PMC ID; dedupe within a source too. Print `found_by` as a map, and only on pages with several sources |
| 6.6 Rank | One pure ranker over title, abstract, curation, families and best rank | Replace for articles with the closed `Order` enum `rank`, `date`, `citations`. `Features` shrinks to the iCite count, filled only for `citations`. Gene ranking is unchanged |
| 6.7 Pools and paging | An immutable snapshot keyed without order; headers in the pool | Key on the ordered source list and the sort. Keep the pool immutable: each source is fetched to a fixed depth at build and never extended. Page headers come from one ESummary call per page and are stored with the pool |
| 6.8 Article pool | Fan out to every article source; one mandatory PubMed leg; enrich headers and abstracts | Run only the named sources; the default names PubMed alone. Keep the rule that a cap never drops a PubMed item. Drop abstract enrichment, which served a ranker this design removes. iCite runs for the whole pool only for `citations` |
| 8.2 Trees | `search article` placeholder flags | Add `--source LIST`, `--sort RULE`, `--journal`, `--open-access`, `--no-preprints`, `--exclude-retracted`, `--include-retracted`, the sections `tldr`, `pdf` and `indexing`, and the commands `article text`, `article supplement`, `article citation-evidence` and `article recommendations`; PDF metadata/download is shipped, with `--source` on `article citations` and `article references` |
| 4.3 Keyed sources | A source whose terms need the user's key declares `KeyUse::Required` | State that Semantic Scholar declares the existing `KeyUse::Optional`, as NCBI E-utilities does, so a missing key never skips its leg. A 429 that outlasts the retries on an optional-key source adds one `key` step |
| 8.6 Markdown view | Renders the JSON by structure | A `Markdown` field prints as it is, unescaped |

## 6. Progressive disclosure and token budget

### How it was measured

Tokens are estimated as bytes divided by 4. The original investigation used a scratch script to build each estimated 1.0 output in the shape section 5 gives, as compact JSON with nulls and empty lists left out, from live replies on 2026-10-23: PubMed EFetch and ESearch, LitVar2, Europe PMC, iCite, PubTator3, the PMC OA bucket and NCBI EFetch PMC, whose JATS matches the OA bucket's. Search pages hold the first ten of each strategy's order as section 3 measured them, in the header shape of section 5, so their sizes are estimates from pages of that shape. The 0.9.3 numbers are the real outputs of the same questions, in `--json` and in Markdown, with `--no-cache`.

"Facts" counts what a careful answer needs, checked as exact text in each output. For a search these are the key PMIDs on page one and a total. For a card they are the identifiers, date, first author, a key abstract word and the journal.

### Layers, densest and highest priority first

| Layer | What it holds | Tokens | Why it sits here | Enforcing case |
| --- | --- | --- | --- | --- |
| 1. `search article` item | PMID, DOI, PMCID, whole title, journal, date, clinical types, first author with count; on a fold, sources with ranks; the cited-by count when sorted by citations | 80 to 120 per item | Lets the agent pick a trial, guideline or landmark paper without a get | search/pubmed-floor.yaml: whole page max_tokens; per-item size remains an estimate. |
| 2. `get article` card | Identifiers, title, journal, date, all clinical types, corrections, ten authors with total, whole abstract, one `next` step listing sections | 470 to 730 | Answers "what did this paper find" alone. The abstract is 60 to 75% of it | get/card.yaml: whole card max_bytes. |
| 3. `citations` | Cited-by, clinical cited-by, references, relative citation ratio | about 35 | Smallest section; marks landmark papers | citations/section.yaml: section max_tokens. |
| 4. `supplements` | License and each supplement file's name, kind, label and caption | 95 for 2 files, 290 for 16 | Example 31 and 32 start here | supplements/file-list.yaml: whole output max_bytes; per-file size remains an estimate. |
| 5. `annotations` | Up to 20 normalized entities with IDs and counts | 200 to 230 | Chains to gene and variant cards | sections/annotations.yaml: section max_tokens. |
| 6. `fulltext` | License, part outline, table and figure captions | 220 to 380 | Tells the agent which part to read | fulltext/outline.yaml: section max_tokens. |
| 7. `article supplement --find` | Matching rows with sheet, row, header and checksum | about 165 per hit | One fact from a large file | supplements/protein-change.yaml: whole output max_bytes. |
| 8. `article citations`, `article references` | Ten headers, most cited first, and a total | about 780 per page | Pivots that grow without bound | citations/citing.yaml and citations/references.yaml: page max_bytes and max_tokens; references preserve all headers within its measured bound. |
| 9. `article text --part N` | One part of the body, split at 4,000 tokens | up to 4,000 per reply; 5,800 to 14,100 for a whole body | Bulky prose, read only on request | fulltext/continuations.yaml: each response max_tokens. |
| 10. `tldr` | Semantic Scholar's one-sentence summary, its model, citation and influential citation counts | about 80 | A shorter digest than the abstract, asked for by name | tldr/data.yaml: section max_tokens on synthetic source text; positive original-paper size remains external. |
| 11. `indexing` | MeSH headings with major flags, keywords, up to ten affiliations with the total, ORCID iDs | 200 to 400 | Restates the paper in PubMed's vocabulary | indexing/own-record.yaml: whole multi-card batch max_bytes; the per-section size remains an estimate. |
| 12. `pdf` | URL, open access status and license | Synthetic section: 186 bytes, 47 tokens; full envelope: 1,628 bytes, 407 tokens | Reports the source link without a download | pdf/synthetic.yaml: PDF section max_tokens. Original-paper metadata uses the external example 69. |
| 13. `article citations --source semantic-scholar` | Ten edge items with intents, the influential flag and the first context with the total | up to 1,800 per page | Why a paper is cited, one pivot away | edges/references.yaml: measured page max_bytes and max_tokens; whole headers and contexts exceed the initial estimate. |
| 14. `article citation-evidence` | Every context and the intents for one pair; with `--fulltext`, the citing paragraphs as Markdown | up to 900; up to 4,000 with `--fulltext` | The passage behind one citation | evidence/source.yaml and runner/evidence.rs: output max_bytes and bounded exact-fragment continuations. |
| 15. `article recommendations` | Ten headers in Semantic Scholar's order | up to 1,300 per page | Discovery at the edges | recommendations/single.yaml and recommendations/multi.yaml: output max_bytes. |
| 16. `article pdf <ID> --save PATH` | Absolute path, bytes, SHA-256, license, status and source URL | Synthetic receipt: 663 bytes, 166 tokens at the measured temporary path | Saves only to the caller's explicit path | pdf/synthetic.yaml: save max_tokens; absolute path length varies. |
| Options | `--gene`, `--variant`, `--disease`, `--drug`, `--author`, `--journal`, `--since`, `--until`, `--type`, `--open-access`, `--no-preprints`, `--exclude-retracted`, `--include-retracted`, `--sort`, cursor | per item | Each option narrows or reorders; none adds fields | filters/*.yaml and the owning source cases enforce selector behavior and page ceilings. |


### Token table, 1.0 against 0.9.3

| Case | Facts needed | 0.9 JSON tokens | 0.9 JSON facts | 0.9 Markdown tokens | 0.9 Markdown facts | 1.0 tokens | 1.0 facts | Shipped full tokens (compact bytes); comparable tokens; enforcing case or missing request |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `search article "AKT1 E17K"` (default, PubMed) | 3 | 1307 | 0 | 675 | 0 | about 1000 | 2 | 1,168 (4,671); comparable 1,134; search/pubmed-floor.yaml |
| `search article "AKT1 E17K" --sort citations` | 3 | | | | | about 1050 | 2 | 1,199 (4,793); comparable 1,127; citations/akt1.yaml, citation-sort step |
| `search article "KRAS G12C sotorasib lung"` | 3 | 1305 | 0 | 656 | 0 | about 940 | 3 | 1,183 (4,729); comparable 1,148; external budget/kras-default.yaml |
| `search article "HER2-low trastuzumab deruxtecan"` | 2 | 1252 | 0 | 651 | 0 | about 1020 | 2 | 1,280 (5,120); comparable 1,246; external budget/her2-default.yaml |
| `search article "IDH1 R132H glioma"` (default) | 2 | 1253 | 0 | 673 | 0 | about 1000 | 1 | 1,198 (4,792); comparable 1,164; external budget/idh1-default.yaml |
| `search article --gene IDH1 --variant R132H --source litvar --sort citations` | 2 | | | | | about 1050 | 2 | 1,231 (4,921); comparable 1,107; litvar/idh1-citations.yaml; HGNC subset is synthetic |
| `search article "TP53 R337H adrenocortical"` (default) | 2 | 1363 | 0 | 678 | 0 | about 1150 | 1 | 1,208 (4,832); comparable 1,174; external budget/tp53-default.yaml |
| `search article "BRCA1 185delAG Ashkenazi"` (default) | 2 | 1230 | 0 | 655 | 0 | about 1050 | 1 | 1,178 (4,712); comparable 1,144; external budget/brca1-default.yaml |
| `search article "BRCA1 185delAG Ashkenazi" --sort citations` | 2 | | | | | about 1100 | 2 | 1,166 (4,664); comparable 1,095; citations/brca1.yaml |
| `search article "next-generation sequencing recommendations advanced cancer ESMO"` | 2 | 1373 | 1 | 690 | 1 | about 1060 | 2 | 1,337 (5,345); comparable 1,302; search/esmo-budget.yaml |
| `search article "amivantamab lazertinib after osimertinib"` | 2 | 1354 | 0 | 712 | 0 | about 1170 | 2 | 1,576 (6,302); comparable 1,541; external budget/amivantamab-default.yaml; new comparable ceiling 7,089 bytes / 1,773 tokens under PM m4678 |
| `search article "xqzvw zzyzxq kinase"` | 2 | 1168 | 0 | 706 | 0 | about 1000 | 2 | 1,009 (4,036); comparable 975; search/not-found-words.yaml |
| `variant articles "AKT1 E17K"` against `search article --gene AKT1 --variant E17K --source pubmed,litvar --sort citations` | 3 | 11378 | 0 | 760 | 0 | about 1100 | 2 | 1,249 (4,996); comparable 1,126; external budget/akt1-fold-citations.yaml |
| `get article 17611497` | 6 | 1498 | 6 | 805 | 6 | 480 | 6 | 587 (2,346); comparable 552; external budget/akt1-card.yaml |
| `get article 10.1056/NEJMoa1203421` | 4 | 2020 | 4 | 1045 | 4 | 733 | 4 | 858 (3,430); comparable 823; external budget/doi-card.yaml |
| `get article PMC6181777` | 4 | 1247 | 3 | 796 | 3 | 484 | 4 | 600 (2,397); comparable 565; external budget/pmcid-card.yaml |
| `get article 17057710` (retracted) | 2 | 872 | 1 | 669 | 1 | 469 | 2 | 582 (2,328); comparable 548; external budget/retracted-card.yaml |
| `get article 99999999` | 1 | 47 | 1 | error | 0 | 57 | 1 | 91 (363); comparable 57; get/not-found.yaml; synthetic absent response, not a live absence claim |
| `get article 30209399` with supplements (0.9 `assets`) | 3 | 12685 | 3 | refused | 0 | 771 | 3 | 962 (3,846); comparable 895; external budget/brca1-supplements.yaml |
| Supplement find, BRCA1 F1761I (0.9 `supplement-search`) | 4 | 649 | 4 | refused | 0 | 165 | 4 | 248 (991); comparable 181 (723 bytes); 0122 external actual-workbook replay, j11426; literal match first at Sheet1 row 2780, column 14; original 2026-10-24 capture: 930 (3,718), comparable 863 (3,450 bytes) |
| `article citations 17611497`, first page | 2 | 1811 | 1 | 666 | 1 | 779 | 2 | 1,065 (4,258); comparable 993; citations/citing.yaml |
| `get article 17611497 annotations` | 3 | 1340 | 3 | 390 | 3 | 679 | 3 | 829 (3,315); comparable 760; external budget/akt1-annotations.yaml; partial: PubTator supplied a variant without HGVS or rsID; retained annotations are data |
| `get article 34096690 fulltext` | 2 | 1562, text on disk | 1 | 259 | 1 | 1037 with outline | 2 | 1,300 (5,199); comparable 1,233; external budget/kras-fulltext.yaml |

The earlier measurements use the shipped binary at `ecef1096239f8aeee8fcc6cbefc5720e89506035`, built and replayed offline in pm job j11370. For those commands, `--replay tests/replies/article/<case replay>` and a fresh `--case-cache` select only the named existing recordings. Full tokens are compact UTF-8 JSON bytes divided by four, rounded up. Raw stdout has one additional newline byte. Job j11379 measured the not-found case against its actual synthetic absent-response replay. The initial probe covered all 23 questions; a missing request returned a replay failure and does not establish the successful answer's size.

The comparable column removes only the three required registry-derived terms fields from each top-level source outcome. Existing case ceilings apply to that comparable output. Full output retains those fields. The exact additional bytes are 138 for one PubMed outcome, 286 for PubMed plus iCite, and 494 for the IDH1 HGNC/LitVar/iCite/PubMed request. This is the existing source-terms exception, not permission to increase the payload limits. The historical `1.0 tokens` and `1.0 facts` columns remain investigation estimates; they are not shipped measurements.

The earlier six original-question rows retain their measured real source text. The additional citations-pivot row retains measured real source text, and the not-found row measures a separately labeled synthetic absent response. PDF sizes above use synthetic content. The fifteen newly captured rows use their exact original questions; fourteen commands returned complete and the annotations command returned partial with retained data and a structured source-value problem.

The original cases remain under `tests/cases/article/`. New exact-question cases and original source bodies remain outside Git under Yellow `data/biomcp-redux/0121-article-token-budget/biomcp-redux-0121-2qoxr6zi/`; case names above are relative to its `cases/` directory. Job j11408 built `0c25e43ae66c83f6a42a8f3d26845943061df27b` and ran exactly fifteen ordinary keyless commands once each with separate empty recording, configuration, home and cache directories. Existing retries and default pagination remained unchanged. There were no timeouts, provider failures or unexecuted rows. No historical binary ran, and no provider refresh followed this capture job. The capture archive SHA-256 is `5aeea5debaa1883ffe1118038bf777c6bd454d681588c19664db4f28825eacaf`; its manifest checks every original file. The separate case/proof archive SHA-256 is `dee068ce79404c7899945eeab998c56009aabb6e5e25b907fd8f6d9fc339ec31`; `proof-manifest.json` verifies its cases, original capture script and offline logs without replacing the original capture manifest. Offline job j11413 passed fourteen cases and exposed the amivantamab ceiling conflict. After the approved exception, j11414 passed that one changed case; the other definitions were unchanged. Every passing case checks CLI/MCP parity, schema and its comparable ceiling. Setup job j11411 ran no tests because of an unsupported Python archive API. Job j11412 reached case validation but executed no replay because object equality lacked the existing assertion operator. These setup mistakes do not count as source failures or successful replay proof. New cases enforce comparable ceilings under the existing rules. PM m4678 approves a new comparable ceiling of 7,089 bytes / 1,773 tokens for the amivantamab case. The current result set differs from the historical capture and carries long whole titles; the historical comparison remains visible. Existing case ceilings remain unchanged. The landing commit ticks the article token budget checkbox under PM m4678 ask 3.

The annotations reply is partial because PubTator returned a variant named `#207#` with neither HGVS nor rsID. Both source outcomes are `ok`; the reply retains seven normalized annotations with a `value.unknown` problem. This is not a provider failure or a complete annotation answer. The original 2026-10-24 supplement capture returned the first 25 of 124 row fragments at row 2780, beginning with header fragments. Its first page did not show the requested matching cells. That original measurement was 3,718 full bytes / 930 tokens and 3,450 comparable bytes / 863 tokens. PM m4678 approved a temporary new comparable ceiling of 3,968 bytes / 992 tokens. Ticket 0122 replaces that behavior with literal matching cells first and preserves row context, coordinates and continuations. The independently verified actual-workbook replay in j11426 shows native `p.F1761I` at Sheet1 row 2780, column 14 on the first page. Its full output measures 991 bytes / 248 tokens. Removing exactly 268 required terms bytes gives 723 comparable bytes / 181 tokens, below the historical 649-token target. The external 0122 case enforces the historical 649-token ceiling; the temporary original-capture ceiling does not justify further growth. The verified case and output stay outside Git under Yellow `data/biomcp-redux/0122-supplement-match-first/biomcp-0122-actual-4lnz8ped/`: `actual-match-first.yaml` and `captured/3ff5bf1e196d9976476292758b66f9d2ddb1cf24d3f97bcbb1ef9cd1d856aaed.json`. This reply retains schema `biomcp.article_supplement/2` with an optional page-scoped absence field. The existing CC BY workbook cases keep their ceilings.

No question was replaced with another paper, a rewritten Boolean query, synthetic positive content or an invented request identity. The original licensed/TDM bodies stay outside Git. Representative CC BY cases retain their own behavior and budgets.

The citations facts are the total and the count source. 0.9 states neither total.

### Where 1.0 costs more, and why

- **Search pages against 0.9 Markdown.** The investigation estimated 940 to 1,170 tokens against 650 to 710. 0.9 Markdown cuts titles to 60 characters and omits DOI, PMCID, authors, totals and source ranks, and it carried a key paper in 1 of 9 cases. The 1.0 default carries PubMed's own first ten and the total; where PubMed misses a paper, the row below names the strategy that finds it, at the same page cost. The measured exact-question default pages cost 1,168 versus 1,307 JSON tokens for AKT1, 1,337 versus 1,373 for ESMO, and 1,009 versus 1,168 for the nonsense query. The new exact-question rows now have measured sizes. The amivantamab page uses 1,541 comparable tokens against the historical 1,354 JSON tokens; its ten complete source titles alone occupy 1,697 UTF-8 bytes. The recorded historical JSON field breakdown is unavailable here, so this observation does not attribute the difference to a particular omitted historical field. `found_by` costs about 50 tokens per page as a map and 160 to 180 as a list of objects, so the schema prints it as a map, and only on fold pages. Required source terms now add 138 bytes to each PubMed outcome; the shipped table includes them. A first author with a count costs less than the three authors the mock pages carried, so that observation explains the original mock-page estimates rather than bounding shipped pages.
- **`get article 17611497 annotations` against 0.9 Markdown.** The original estimate was 679 against 390; the captured original-paper reply costs 829 full tokens and 760 comparable tokens and is partial for the source-value problem described above. 1.0 prints the card with the section; 0.9 Markdown prints the section alone. The earlier section-only estimate was 200 tokens.
- **`fulltext` against 0.9 Markdown.** The original estimate was 1,037 against 259; the captured original-paper outline is 1,300 full tokens and 1,233 comparable tokens. 0.9 returns a file path the agent cannot read over MCP. 1.0 returns the card and the outline from the OA bucket, and the text through `article text`.
- **`get article 99999999`** (91 full tokens; 57 comparable, against 47). The synthetic absent-response case preserves the input and resolution, which account for the original increase. Required publisher terms add 138 bytes to the full envelope. This measures the absence shape; it does not claim current live absence.

### Budget

Article page cases use the recorded real output plus 15 percent, capped by the recorded 0.9 JSON tokens for the same question (PM m4341). The 1,300-token page figure is a design target. Provisional page maxima in the investigation table are replaced with exact measured case ceilings before handoff; exceptions explain the responsible facts on the owning ticket. Other command ceilings remain as ruled. Per-command budgets: a search item 130 tokens and a page of ten 1,300; a card 900; `citations` 60; `supplements` 40 per file; `annotations` 350; `fulltext` 500; a supplement find 250 per hit and 25 hits per call; a citations or references page 1,100; an `article text` reply 4,000; a not-found item 80; `tldr` 120; `indexing` 400; `pdf` 120; a save reply 200; a Semantic Scholar citations or references page initially estimated at 1,800, with the shipped bounds enforced by the owning edge cases; `article citation-evidence` 900, or 4,000 with `--fulltext`; a recommendations page 1,300. 0.9 printed no recorded JSON for citation evidence, recommendations or `tldr` on these examples, so those budgets make no 0.9 comparison claim. The shipped column names the exact-question measurements and their cases, preserves the partial-annotation limitation, and identifies the approved narrow amivantamab exception.

## 7. Keyed and optional-key sources

`decisions/2026-10-24-keyed-sources-run-only-with-the-user-s-key.md` governs sources whose terms need the user's own key, and no article source is one. Semantic Scholar is free, and its key is optional and only raises the request limit, as `decisions/2026-10-24-article-keeps-every-source-one-option-away.md` records. It therefore runs like NCBI E-utilities: without a key at a conservative quota, and with the user's key at the higher limit.

| Source | Key | Serves | Rule |
| --- | --- | --- | --- |
| NCBI E-utilities | `ncbi_api_key`, optional | Raises PubMed and EFetch from 3 to 10 requests per second | Runs without a key at the lower limit |
| Semantic Scholar | `semantic_scholar_api_key`, optional, in header `x-api-key` | The `semantic-scholar` search source, `tldr`, `pdf`, the Semantic Scholar edges, citation evidence and recommendations | Runs without a key at 1 request every 2 seconds with one in flight, and with the user's key at 1 per second. Never part of a default command. A 429 that outlasts the retries fails the leg with reason `rate_limited` and adds a `key` step |
| PubTator3, LitVar2, LitSense2, iCite, Europe PMC, PMC OA bucket | none | | Public, no key |

The key is config key `keys.semantic_scholar_api_key`, or environment variable `S2_API_KEY`, its 0.9 name. It is outgoing only: it never appears in a request key, a recording, a trace, an error or an envelope, and BioMCP never prints it. `biomcp help key semantic-scholar` names both places and says the key is free and raises the limit.

### Keyless access, checked 2026-10-24

Semantic Scholar's API page (https://www.semanticscholar.org/product/api) says: "Most Semantic Scholar endpoints are available to the public without authentication. They are rate-limited to 1000 requests per second shared among all unauthenticated users." It says an API key gets higher limits, and that "the introductory rate limit for an API key is 1 RPS on all endpoints". The Graph API description at https://api.semanticscholar.org/graph/v1/swagger.json documents the `x-api-key` header, `/paper/search` (at most 100 per page and 1,000 relevance-ranked results), `/paper/{paper_id}` with `tldr` and `openAccessPdf`, `/paper/{paper_id}/citations` and `/references` (at most 1,000 per page, with `contexts`, `intents`, `contextsWithIntent` and `isInfluential`) and `/paper/batch`. The recommendations description at https://api.semanticscholar.org/recommendations/v1/swagger.json documents `GET /papers/forpaper/{paper_id}` and `POST /papers`, with a `from` pool of `recent` (the default) or `all-cs`, and at most 500 results.

Keyless calls from one machine, 3 or more seconds apart, retrying a 429 after 5, 10, 15, 20 and 25 seconds:

| Call | Tries | HTTP 200 | What came back |
| --- | --- | --- | --- |
| `/paper/search` | 19 | 1 | "EGFR T790M osimertinib": total 10,282; first row 27959700 with PubMed, PMC and DOI IDs |
| `/paper/{PMID:...}` with `tldr` or `openAccessPdf` | 40 | 6 | The TLDR and counts for 17611497; `openAccessPdf` for 22663011, 38834388, 37879444 and 33377972 |
| `/paper/batch` | 5 | 0 | |
| `/paper/{PMID:...}/citations` and `/references` | 6 | 6 | Contexts, intents and influential flags, as section 5 says |
| `/recommendations/v1/papers/forpaper/...` and `POST /papers` | 4 | 4 | As section 5 says |

Every keyless endpoint answered without a key. Search and paper lookups met the shared pool's 429 most of the time, and citation, reference and recommendation calls never did on this day. The design therefore paces keyless calls at 1 every 2 seconds, retries 429 within the leg's deadline as `architecture.md` section 4.6 says, and tells the agent that the optional key raises the limit. No call in this investigation used a key, and none of these facts depends on one.

Cases: no case calls Semantic Scholar live. Its cases replay keyless replies recorded once, with a 429 reply for the failure cases. Contexts are sentences from the citing papers, so recordings in the repo hold contexts only from CC BY citing papers, such as 33123537's references; other context recordings and the TLDR recordings run from outside the repo through `--cases DIR`, as abstracts do. The 41351423 PDF recording also runs from outside the repo.

## 8. Outside-in cases

Each case was checked on 2026-10-23 against live PubMed, LitVar2, LitSense2, Europe PMC, iCite, PubTator3 or the PMC OA bucket, and runs on recorded replies. Each asserts identifiers, order rules, sets, states and a byte ceiling. Ranks from a source's own order assert "within the first N" with room for the shifts section 3 measured, so a re-recording does not break them. Ceilings are about 1.5 times the measured size in section 6.

| # | Story | Run | Expect | Ceiling | 1.0 case or external proof boundary |
| --- | --- | --- | --- | --- | --- |
| 1 | The default is plain PubMed (PMID 33123537) | `search article "AKT1 E17K"`, walked to the end | `term` is `AKT1 E17K`; the walk equals the recorded PubMed relevance list for that term, 192 PMIDs on 2026-10-24, in order, with no duplicates; `sources` is pubmed and `order` is rank; no item carries `found_by`; 33123537 in the first 10 | 7,000 bytes per page of 10 | search/pubmed-floor.yaml; citations/akt1.yaml |
| 2 | PubMed is asked the right way | the recorded requests of case 1 | one ESearch with `sort=relevance`, `retmax=9999` and `tool=biomcp`, and no `email` when none is configured; one ESummary per page read; no second ESearch | none | search/configured-contact.yaml |
| 3 | PubMed's window | `search article "EGFR lung cancer"` | `page.truncated` true; PubMed's outcome total above 30,000; 9,999 items in the pool; status complete | 7,000 bytes | search/window.yaml |
| 4 | Dropped words (issue 4) | `search article "xqzvw zzyzxq kinase"` | PubMed's outcome state ok with a notice naming xqzvw and zzyzxq as dropped; status complete | 7,000 bytes | search/not-found-words.yaml |
| 5 | ESearch error inside HTTP 200 | any search with a recorded 200 reply holding `ERROR` | PubMed's outcome failed with reason bad_reply; status failed | 600 bytes | search/synthetic-search-error.yaml |
| 6 | Gene term (issue 113, PMID 35665782) | `search article --gene HER2 low trastuzumab deruxtecan` | `term` is `(ERBB2 OR HER2) AND low trastuzumab deruxtecan`; 35665782 in the first 10; the walk equals the recorded plain PubMed result for that term | 7,000 bytes | search/gene-terms.yaml |
| 7 | An old gene name the agent types | `search article --gene SUR1 R1437Q` | `term` is `(ABCC8 OR SUR1) AND R1437Q`, with no other previous symbol; the walk equals the recorded plain PubMed result for that term | 7,000 bytes | search/gene-terms.yaml |
| 8 | LitVar2 takes the matching variant | `search article --gene KRAS --variant G12C --source litvar` | LitVar2's outcome `term` is rs121913530, not rs121913529; total 8,915 | 7,000 bytes | litvar/kras-exact.yaml |
| 9 | LitVar2 finds no match | `search article --gene KRAS --variant G12Q --source litvar` with a recorded autocomplete reply that holds no KRAS G12Q hit | candidates returned with how each matched; no search runs | 2,000 bytes | litvar/kras-no-match.yaml |
| 10 | A variant source beyond PubMed (PMID 19228619) | `search article --gene IDH1 --variant R132H --source litvar --sort citations` | 19228619 in the first 3; order echo citations; items carry `cited_by` | 7,500 bytes | litvar/idh1-citations.yaml |
| 11 | Annotation-only paper (PMID 11481490) | `search article --gene TP53 --variant R337H --source litvar --sort citations` | 11481490 in the first 25 | 7,500 bytes per page | litvar/tp53-citations.yaml |
| 12 | rsID input (PMID 17611497) | `search article --variant rs121434592 --source litvar --sort citations` | 17611497 in the first 30 | 7,500 bytes per page | litvar/akt1-rsid-citations.yaml |
| 13 | Most cited first (PMID 9145676) | `search article "BRCA1 185delAG Ashkenazi" --sort citations` | 9145676 in the first 3; counts never increase down the walk; items missing from iCite come last with no `cited_by` | 7,500 bytes | citations/brca1.yaml |
| 14 | Citation sort above its cap | `search article --gene BRAF --variant V600E --source pubmed,litvar --sort citations` | usage error naming the PMID count above 10,000 and asking to narrow; no iCite request | 600 bytes | litvar/braf-cap.yaml |
| 15 | Round-robin rule (PMID 33123537) | `search article "AKT1 E17K" --source pubmed,europepmc`, walked | the k-th result of the i-th listed source sits at position 2(k−1)+i or earlier; no PMID appears twice; each item's `found_by` holds its recorded native ranks; Europe PMC's outcome shows the actual captured depth (at most 100) and its recorded total | 8,000 bytes per page | fold/live-rank.yaml; fold/live-reverse.yaml |
| 16 | DOI-only rows (PMID 27959700) | `search article "EGFR T790M osimertinib" --source pubmed,semantic-scholar` with a recorded keyless reply | rows with only a DOI resolve to their PMIDs where a record with the same DOI exists; 27959700 appears once, with both sources in `found_by` | 8,000 bytes | semantic-scholar/synthetic-fold.yaml (synthetic; successful live search remains outside the repo) |
| 17 | A source repeats a paper | a recorded Europe PMC reply that lists one PMID twice | the item appears once, at the source's first rank | none | fold/synthetic-fold.yaml |
| 18 | The pool is fixed | case 15's walk run twice, then a page read with the network disabled | identical order; the last page has no page step; the offline page makes no request | none | fold/live-rank.yaml; fold/live-reverse.yaml |
| 19 | A named source fails (issue 97) | case 15 with a recorded Europe PMC HTTP 500 | status partial; europepmc named with reason; the walk still holds all 192 PubMed PMIDs; `next` holds a `fresh` step | 8,000 bytes | fold/synthetic-epmc-500.yaml |
| 20 | PubMed fails in a fold | case 15 with a recorded PubMed timeout | status failed; the error names pubmed | 600 bytes | fold/synthetic-pubmed-deadline.yaml |
| 21 | Similarity is not a match | `search article "AKT1 E17K capivasertib breast cancer" --source litsense` | items returned in LitSense2's order, each with `sentence` holding a section and text; status complete; help text for `litsense` says a hit is a resemblance, not a match | 7,000 bytes | Outside the repo: AKT1 successful sentence capture remains unproved. litsense/licensed-subset.yaml tests a marked CC BY subset of the nonsense response. |
| 22 | A filter a source cannot apply | `search article --gene AKT1 --variant E17K --author Carpten --source pubmed,litvar` | usage error with code unsupported_filter naming litvar and `--author`; no request | 400 bytes | litvar/unsupported.yaml |
| 23 | A sort a source cannot apply | `search article --gene AKT1 --variant E17K --source litvar --sort date` | usage error with code unsupported_filter naming litvar and `--sort date` | 400 bytes | litvar/unsupported.yaml |
| 24 | LitVar2 without a variant | `search article AKT1 --source litvar` | usage error naming `--variant` | 400 bytes | litvar/unsupported.yaml |
| 25 | Semantic Scholar without a key | `search article "EGFR T790M osimertinib" --source semantic-scholar` with no key configured, once on the recorded keyless reply and once on a recorded 429 | the first runs and returns 27959700 first; the second fails with reason rate_limited and a `key` step naming `semantic-scholar`, and the request held no `x-api-key` header | 600 bytes for the failure | semantic-scholar/live-rate-limit.yaml; semantic-scholar/synthetic-429.yaml (synthetic success in semantic-scholar/synthetic-minimal.yaml; successful live search remains outside the repo) |
| 26 | Date filter and date order (issue 69) | `search article "KRAS G12C sotorasib" --since 2024 --sort date --limit 5` | five items, each dated 2024 or later, in PubMed's most recently added order; `next` holds a page step | 3,500 bytes | filters/since.yaml |
| 27 | Guideline filter (PMID 38834388) | `search article "next-generation sequencing advanced cancer ESMO" --type guideline` | 38834388 in the first 3; every item typed practice_guideline or guideline | 7,000 bytes | filters/guideline.yaml |
| 28 | Both AKT1 papers (PMIDs 33123537 and 17611497) | `search article "AKT1 E17K"`, then the `--sort citations` step from its `next` | the default page holds 33123537 and a `next` step for the same term with `--sort citations` and its cost; that step's page holds 17611497 in the first 3 | 7,500 bytes per page | citations/akt1.yaml |
| 29 | Author filter (PMID 17611497) | `search article AKT1 --author Carpten` | 17611497 in the first 3 | 7,000 bytes | filters/author.yaml |
| 30 | Expired pool | a page step with a recorded pool ID that no longer exists | error `pool.expired` with a `restart` step | 500 bytes | search/checked-request.yaml |
| 31 | Card by PMID (PMID 17611497) | `get article 17611497` | `doi` 10.1038/nature05933; `date` 2007-07-26; first author Carpten JD; `authors.total` 24; abstract contains "E17K"; `fetched` present; `next` names no `fulltext` or `supplements` | 2,900 bytes | Outside the repo: original publisher abstract. get/card.yaml covers a CC BY card for 33123537. |
| 32 | Card by DOI (PMID 22663011) | `get article 10.1056/NEJMoa1203421` | `pmid` 22663011; `match.kind` doi; the record's DOI equals the input; types include randomized_controlled_trial and clinical_trial_phase_3 | 4,400 bytes | Outside the repo: original publisher abstract. get/doi.yaml covers a CC BY DOI card for 28944238. |
| 33 | A DOI lookup returns another record | `get article 10.1056/NEJMoa1203421` with a recorded ESearch reply whose record carries a different DOI | resolution not_found | 400 bytes | get/synthetic-doi-mismatch.yaml |
| 34 | Card by PMCID (PMID 30209399) | `get article PMC6181777` | `pmid` 30209399; `match.kind` pmcid; `next` names `fulltext` and `supplements` | 2,900 bytes | Outside the repo: original publisher abstract. get/pmcid.yaml covers a CC BY PMCID card for 28944238. |
| 35 | Retracted paper (PMID 17057710) | `get article 17057710` | types include retracted_publication; `corrections` holds retraction with pmid 21217686; `fetched` present | 2,800 bytes | Outside the repo: original paper; get/retraction.yaml covers the retained retraction behavior for 35061708. |
| 36 | Not found | `get article 99999999` | resolution not_found; status absent; the input never appears in an error message | 400 bytes | get/not-found.yaml |
| 37 | Preprint (Europe PMC PPR1064174) | `get article PPR1064174` | `doi` 10.21203/rs.3.rs-7293078/v1; types include preprint; no `pmid`; `fulltext` unavailable with the retained source-failure reason | 2,500 bytes | preprint/card.yaml; preprint/unavailable.yaml |
| 38 | Preprint full text (Europe PMC PPR1295425, CC BY) | `get article PPR1295425 fulltext` | `fulltext` state data with license cc_by and an outline of parts | 4,500 bytes | preprint/reader.yaml |
| 39 | Several inputs (replaces `article batch`) | `get article 17611497 34096690 21639808` | three cards in input order | 9,000 bytes | Outside the repo: original publisher abstracts. get/input-order.yaml covers input order with retained CC BY cards. |
| 40 | Supplements listed (PMID 28944238, CC BY) | `get article 28944238 supplements` | license cc_by; files exactly {MGG3-5-553-s001.pdf, MGG3-5-553-s002.xlsx} | 3,800 bytes | supplements/file-list.yaml |
| 41 | Supplement sheets (PMID 28944238) | `article supplement 28944238 MGG3-5-553-s002.xlsx` | the sheet list includes ST2 with its row count; a page step for rows | 1,500 bytes | supplements/workbook.yaml |
| 42 | Supplement fact by protein change (PMID 28944238) | `article supplement 28944238 MGG3-5-553-s002.xlsx --variant P245S`, then the same with `--find P245S` | the first prints the forms it searched, including Pro245Ser, and holds one hit at sheet ST2 row 85 with cells AXIN2, c.733C>T, p.Pro245Ser and VUS and `sha256`; the literal `--find P245S` is complete with zero hits, because the file spells only Pro245Ser | 1,500 bytes | supplements/protein-change.yaml |
| 43 | A pdf supplement | `article supplement 28944238 MGG3-5-553-s001.pdf` | `unsupported` with the file's URL | 400 bytes | supplements/usage-before-download.yaml |
| 44 | Supplement fact in a TDM manuscript, outside the repo (PMID 30209399) | `article supplement 30209399 NIHMS1501643-supplement-2.xlsx --find F1761I` through `--cases DIR` | one hit at Sheet1 row 2780; cells include c.5281T>A and LOF | 1,500 bytes | Outside the repo: licensed TDM workbook proof through --cases DIR; no licensed text in Git. |
| 45 | A PMCID that is not open | `get article PMC13020499 fulltext supplements` | both sections unavailable with reason not_open; no request beyond the OA bucket listing | 3,000 bytes | fulltext/not-open.yaml |
| 46 | Full text outline for an author manuscript, outside the repo (PMID 34096690) | `get article 34096690 fulltext` | `fulltext` state data with license tdm, from the OA bucket | 4,500 bytes | Outside the repo: licensed TDM JATS through --cases DIR. |
| 47 | A long part splits (PMID 34096690), outside the repo | `article text 34096690 --part N` for a part over 4,000 tokens | the reply holds at most 4,000 tokens and a continuation step | 24,000 bytes | Outside the repo: original TDM body; fulltext/continuations.yaml proves bounded continuation with CC BY text. |
| 48 | No full text (PMID 17611497) | `get article 17611497 fulltext supplements` | both sections not_applicable | 3,000 bytes | fulltext/no-pmcid.yaml |
| 49 | Annotations (PMID 17611497) | `get article 17611497 annotations` | a gene with id 207; a variant with hgvs p.E17K and rsid rs121434592 | 4,000 bytes | Outside the repo: original publisher card; sections/annotations.yaml proves normalized annotations on the retained card. |
| 50 | Citations (PMID 17611497) | `article citations 17611497` | `page.total` at least 1,000; items ordered by cited_by, highest first | 4,700 bytes | citations/citing.yaml |
| 51 | References (PMID 34096690) | `article references 34096690`, walked | the walk holds 32955176; total 37 | 4,700 bytes per page | citations/references.yaml |
| 52 | Retired spelling | `variant articles "AKT1 E17K"` | usage error naming `search article --variant` | 400 bytes | retired.yaml |
| 53 | PubTator3 text search (PMID 38834388) | `search article "next-generation sequencing recommendations advanced cancer ESMO" --source pubtator` | 38834388 in the first 3; PubTator3's outcome carries its total, 992 on 2026-10-24 | 7,000 bytes | pubtator/pubtator-words.yaml |
| 54 | PubTator3 entity search | `search article --gene AKT1 --variant E17K --source pubtator` | PubTator3's `term` is `@GENE_207 AND E17K`, from HGNC's Entrez ID and never from PubTator3's autocomplete; total 750 on 2026-10-24 | 7,000 bytes | pubtator/pubtator-gene.yaml (metadata-only source projection) |
| 55 | LitSense2 stalls in a fold | `search article "xqzvw zzyzxq kinase" --source pubmed,litsense` with the recorded HTTP 502 that LitSense2 sent after 30 s | status partial; litsense failed with its reason; PubMed's items and dropped-word notice intact; `next` holds a `fresh` step | 7,000 bytes | Outside the repo: captured timeout; runner/named_search.rs proves partial failures with response-only synthetic mutations. |
| 56 | TLDR, outside the repo (PMID 17611497) | `get article 17611497 tldr` | `tldr` state data; text contains "E17K"; model `tldr@v2.0.0`; `cited_by` 1,332 and `influential_cited_by` 82 as recorded; the section under 120 tokens | 3,400 bytes | Outside the repo: positive17611497 TLDR remains unproved after bounded429. tldr/data.yaml is explicitly synthetic. |
| 57 | TLDR states | `get article 33123537 tldr` on a synthetic reply with `"tldr": null`, then on a recorded 429 | the first gives `empty`; the second gives `unavailable` with reason rate_limited, a `key` step and an intact card | 3,000 bytes | tldr/empty.yaml; tldr/failure-429.yaml |
| 58 | Indexing (PMID 35440569) | `get article 35440569 indexing` | MeSH headings include Proto-Oncogene Proteins c-akt and Neoplasms as major topics; 8 headings; affiliations capped at 10 with total 36; 6 ORCID iDs; no request beyond the card's EFetch | 4,500 bytes | indexing/own-record.yaml |
| 59 | Citation contexts (PMIDs 33123537, 17611497) | `article references 33123537 --source semantic-scholar`, walked | total 116; the item for 17611497 carries intents [background], influential true, one context and `contexts.total` 6; items with no PubMed ID carry `types_unknown`; each page within the page budget | 7,200 bytes per page | edges/references.yaml |
| 60 | Many citing papers, outside the repo (PMID 17611497) | `article citations 17611497 --source semantic-scholar` | two recorded calls; total 1,332; the first item's year is 2026; `query.source` semantic-scholar | 7,200 bytes per page | Outside the repo: original licensed contexts; edges/window.yaml proves bounded paging with synthetic source text. |
| 61 | A publisher withholds contexts (PMID 34096690) | `article references 34096690 --source semantic-scholar` | total 38; no item carries contexts | 7,200 bytes per page | edges/no-contexts.yaml |
| 62 | Citation evidence (PMIDs 33123537, 17611497) | `article citation-evidence 33123537 17611497` | 6 contexts; intents [background]; influential true | 3,600 bytes | evidence/source.yaml |
| 63 | Citation evidence in the full text | `article citation-evidence 33123537 17611497 --fulltext` | Semantic Scholar's 6 contexts, and 7 paragraphs from PMC7573235 that cite reference B17, each as Markdown with its part number | 16,000 bytes | evidence/source.yaml; runner/evidence.rs |
| 64 | Citation evidence with no contexts (PMIDs 34096690, 32955176) | `article citation-evidence 34096690 32955176` | the row matches by DOI, 10.1056/NEJMoa1917239 against Semantic Scholar's 10.1056/NEJMOA1917239, compared without ASCII case; state empty with notice no_contexts | 1,200 bytes | evidence/doi.yaml |
| 65 | Recommendations (PMID 34096690) | `article recommendations 34096690` | five items in the recorded order, 42745055 first; the query echo says the pool is Semantic Scholar's recent papers | 5,200 bytes | recommendations/single.yaml |
| 66 | Recommendations with a negative seed | `article recommendations 34096690 32955176 --negative 33433946` | one POST whose body holds both positive IDs and the negative one; 42745055 first | 5,200 bytes | recommendations/multi.yaml |
| 67 | No recommendations (PMID 17611497) | `article recommendations 17611497` | status complete; total 0; notice no_recommendations | 600 bytes | recommendations/source.yaml |
| 68 | Semantic Scholar rate limit on a pivot | `article citations 33123537 --source semantic-scholar` with a recorded 429 on every try | failed with reason rate_limited; a `key` step naming semantic-scholar; no key appears anywhere in the output | 600 bytes | edges/failure-429.yaml |
| 69 | Open PDF download, outside the repo (41351423, CC BY) | `get article 41351423 pdf`, then `article pdf 41351423 --save PATH` | Metadata only until the explicit save; saved path, bytes, SHA-256, license, status and URL; no extracted text | External case ceilings; synthetic metadata section and save also have token ceilings | External `cases/article/pdf/open.yaml` retained with the accepted PDF evidence; pdf/synthetic.yaml proves the explicit-save shape with BioMCP text, not the original paper. |
| 70 | PDF source states | Closed metadata, blocked download and HTML body | `not_open` sends no download; blocked download is `rejected`; HTML is `not_pdf` | Enforced case assertions | pdf/closed.yaml, pdf/rejected.yaml, pdf/not-pdf.yaml and pdf/html333.yaml; synthetic fixtures retain their label. |
| 71 | PDF save boundaries | Explicit path, existing file, replace, redirect, size, proxy and HTTP-mode cases | No implicit save; no overwrite without `--replace`; failures leave no file; HTTP mode refuses saves and gives the source URL | pdf/synthetic.yaml save max_tokens and the existing boundary contracts | pdf/synthetic.yaml and runner/pdf.rs: atomic-explicit-saves, hostile-existing-paths, size-cut-and-proxy-boundaries, failed-download-leaves-nothing and http-save-refused-with-link. No PDF parser or worker. |
| 72 | A heading survives (PMID 35440569, CC BY) | `article text 35440569 --part N` for the first Results subsection | `heading_path` is ["Results", "Identification of candidate driver mutations in AKT1, AKT2, and AKT3"]; `markdown` starts with a level-3 heading, writes `*AKT1*`, and cites `[11]` | 17,000 bytes | fulltext/outline.yaml; fulltext/synthetic-reader.yaml (original35440569 detail remains outside the repo) |
| 73 | A table survives (PMID 28944238, CC BY) | `article text 28944238 --part N` for Table 1 | `markdown` starts with `**Table 1.** Sample characteristics`, then a Markdown table whose header row holds Ontario and Mayo Clinic and whose rows hold "Age at diagnosis of cases (range)" | 6,000 bytes | fulltext/continuations.yaml |
| 74 | A find inside a table (PMID 28944238) | `article text 28944238 --find "Age at diagnosis"` | the hit names Table 1's part number and quotes its caption, its header row and the matching row as a small Markdown table | 2,000 bytes | fulltext/continuations.yaml |
| 75 | Lists, superscripts and `--format md` | `article text <ID> --part 1` on a synthetic JATS file with a list, `IC<sub>50</sub>` and a figure caption, once as JSON and once with `--format md` | list items print as `- ` lines; `IC<sub>50</sub>` survives; the caption prints as `**Figure 1.** …`; the Markdown view prints the `markdown` field unescaped | 3,000 bytes | fulltext/synthetic-reader.yaml; runner/markdown_shapes.rs |
| 76 | An abstract in Markdown (PMID 35440569) | `get article 35440569` | an abstract part's text holds `metabolism<sup>1-8</sup>` | 4,000 bytes | get/card.yaml (original35440569 markup lives in its licensed source card) |
| 77 | Open access only | `search article "AKT1 E17K" --open-access` | `term` is `AKT1 E17K AND "pubmed pmc open access"[filter]`; total 66 on 2026-10-24; 33123537 in the first 10 | 7,000 bytes | filters/open-access.yaml (synthetic; positive live filter remains outside the repo) |
| 78 | No preprints | `search article "AKT1 E17K" --no-preprints` | `term` is `AKT1 E17K NOT preprint[pt]`; total 189 on 2026-10-24; 41415395, 40585122 and 38328122 absent from the walk | 7,000 bytes per page | filters/no-preprints.yaml (synthetic; positive live filter remains outside the repo) |
| 79 | Retracted papers excluded on request | `search article "AKT1 E17K" --exclude-retracted`, then `search article "AKT1 E17K" --include-retracted` | the first has `term` `AKT1 E17K NOT retracted publication[pt]`, total 190 on 2026-10-24, and 22505016 and 20068047 absent from the walk; the second has `term` `AKT1 E17K`, total 192, and holds both, each typed retracted_publication | 7,000 bytes per page | filters/exclude-retracted.yaml; filters/include-retracted.yaml (synthetic; live counts remain outside the repo) |
| 80 | Filters a source cannot apply | `search article --gene AKT1 --variant E17K --source pubmed,litvar --open-access`, then the same with `--exclude-retracted` in place of `--open-access` | each is a usage error with code unsupported_filter naming litvar and the filter, with no request | 400 bytes each | filters/access-capabilities.yaml |
| 81 | Europe PMC filters, checked before landing | `search article "AKT1 E17K" --source europepmc --open-access --no-preprints --exclude-retracted` | Europe PMC's `term` holds `OPEN_ACCESS:y`, `NOT SRC:PPR` and `NOT PUB_TYPE:"retracted publication"`; the recorded total matches a live check | 7,000 bytes | filters/access-europepmc.yaml (synthetic; positive live filter remains outside the repo) |
| 82 | A protein change that does not parse | `article supplement 28944238 MGG3-5-553-s002.xlsx --variant P245` | usage error naming the expected form; no download | 400 bytes | supplements/usage-before-download.yaml; supplements/protein-change.yaml |

Case paths in the last column are relative to `tests/cases/article/`; `runner/` paths are relative to `crates/biomcp/tests/`. A behavioral substitute does not prove the original paper or live source count. Synthetic source bodies prove replay behavior and retain their synthetic label. The budget column retains investigation targets; the cases' `max_bytes` and `max_tokens` fields enforce the reviewed shipped ceilings.

Cases 44, 46 and 47 hold text under the TDM license, so their recordings stay outside the repo and run through `--cases DIR`. Cases 56, 60 and 69 also run from outside the repo: 56 and 60 hold Semantic Scholar text from papers under other licenses, and 69's recording holds a 4.8 MB PDF under its publisher's terms. Synthetic recordings are marked synthetic. Recordings that hold PubMed abstracts carry publisher text and follow the same rule unless the PM rules otherwise; the cases that need them are the card cases 31 to 35 and 39.



## 9. Ian's answers

Ian answered the design's three questions on 2026-10-24. `decisions/2026-10-24-article-keeps-every-source-one-option-away.md` records the answers.

- **A. The AKT1 E17K bar.** Option 2: the default page one holds 33123537, and 17611497 is first on the most-cited order of the same term, which the default page's `next` names with its cost. Cases 1 and 28 assert it.
- **B. LitSense2 and PubTator3 search.** Both stay as named sources, never defaults. They do a different job from PubMed, and help describes each job and its failure modes.
- **C. Features that 0.9 had.** All stay: `--pdf` as the pending `pdf` section and `article pdf --save` download, as the later download-only ruling requires, `tldr`, citation contexts and intents on `article citations` and `article references` with `--source semantic-scholar`, `article citation-evidence` and `article recommendations`. Semantic Scholar runs without a key, and its optional key raises the limit.

The later download-only ruling supersedes the earlier PDF extraction design. Saved files belong to the caller; BioMCP never keeps a download folder or saved-file inventory. Text and supplement saves await 0076 and do not change today's reader views.

The same ruling keeps every other 0.9 capability, so this design also keeps the `indexing` section and the `--open-access`, `--no-preprints`, `--exclude-retracted` and `--include-retracted` filters. `decisions/2026-10-24-no-text-heuristics.md`, decided the same day, removes previous symbols from the `--gene` term, keeps `--find` literal, adds `--variant` to the readers and retires 0.9's ranking flags as usage errors.
