# 0898 — ddinter-pair-lookup-and-who-pq-area-filter

Status: OPEN.

Milestone: 7

## Outcome

Two small lookups in one call each: DDInter interactions for a named pair of drugs (with a plain note that DDInter's published file carries severity levels only, no management text), and WHO prequalified medicines filtered by therapeutic area.

## Why

In the pv-scientist persona run of 2026-11-13 (biomed-agent-bench `personas/pv-scientist/runs/2026-11-13/log.md` and `report.md`, analysis beside them; code-mode run in `runs/2026-11-13-raw/`): proving three DDInter pairs absent took paging 12 + 258 + 550 rows, and the agent then curled DDInter to learn the file has no management column (log.md:175 on, report Q5); finding WHO PQ's 18 oncology rows took 12 pages over 566 rows (report Q10).

## Proof

- Proof: recorded cases where a pembrolizumab–lenvatinib pair lookup returns no row with the levels-only note, a known interacting pair returns its level, and WHO PQ filtered to oncology returns the recorded oncology rows.

## Starts from

- Starts from: `crates/biomcp-adapters/src/ddinter/`, `who_pq/`.

## Keeps

- Keeps: per-drug interaction lists.

## Defers

- Defers: scraping DDInter's web checker for management text.

## Changes

- Changes: `crates/biomcp-adapters/src/ddinter/`, `who_pq/`, `crates/biomcp-types/`, `tests/cases/drug/`, `docs/`

## Progress

- Checked the sources first (2026-11-13). DDInter's eight published CSVs have five columns: `DDInterID_A, Drug_A, DDInterID_B, Drug_B, Level`. There is no management or mechanism column; that text is only on DDInter's interactive checker, which this ticket does not scrape. WHO PQ's finished-product export (657 rows) has its own `Therapeutic Area` column with one value per row (HIV/AIDS 255, Tuberculosis 127, Malaria 103, Reproductive health 62, Covid-19 20, Neglected tropical diseases 19, Oncology 18, Hepatitis B and C 18, and smaller areas; one row blank). It has no ATC field. The filter works on WHO's own area words, so there is real data behind it.
- `get drug X interactions --source ddinter --partner Y` keeps only X's pairs with Y. Y is a DDInter ID or the complete DDInter name, ignoring letter case, with one approved salt word allowed. The reply lists the DDInter identities Y names in `partners`. If DDInter's files name no such drug, `partners` is empty, the hint says so, and the next step is the DDInter name search. Every interactions reply now carries a `hint`: DDInter's files give each listed pair a level and no management or mechanism text, and a pair with no row does not show the drugs are safe together. The `interactions` section help says the same. Per-drug lists without `--partner` are unchanged.
- `search drug --source who-pq --therapeutic-area oncology` keeps rows whose Therapeutic Area cell holds those whole words, ignoring letter case. It works on its own or with a name or dates. With no name, area or dates, WHO PQ search now asks for "a name, --therapeutic-area or a --prequalified-from/--prequalified-to date range". The old message literal stays in the enum because the 1.0 schema candidate lists it.
- Schema changes add only optional fields: `partner` on drug get, `therapeutic_area` on drug search, and `partners`/`hint` on interactions replies. One error-message literal was added and some descriptions changed.
- Live check in job j39568 (built binary, live DDInter and WHO): pembrolizumab with lenvatinib, pembrolizumab with paclitaxel and lenvatinib with paclitaxel each return total 0, the named partner identity and the levels-only hint, one call each. Pembrolizumab with dexamethasone returns one Moderate row with its origin in five DDInter files. A `keytruda` partner says DDInter names no such drug. WHO `--therapeutic-area oncology --limit 50` returns the 18 Oncology rows (rituximab and trastuzumab biotherapeutics, BT-ON001 to BT-ON019) in one page.
- Proof: `tests/cases/drug/ddinter-pair.yaml` uses the existing synthetic DDInter replay. DDInter is CC BY-NC-SA, and 0695 made noncommercial captures synthetic, so this run committed no real DDInter rows; the live pembrolizumab results above are from the job log. `tests/cases/drug/who-pq-area.yaml` reuses the WHO PQ export recorded live for 0878 and asserts the 18 Oncology references, newest first. A private mapper test covers area word matching.
- Review fixes (2026-11-13):
  - DDInter publishes fifteen partitions at the same path, not eight: A B C D G H J L M N P R S U V (the other letters return 404). Sync now reads all fifteen. Before this, 32% of distinct pairs (76,599 of 236,834) were missing, among them sertraline with tramadol (Major, only in N), so a partner lookup could wrongly answer "no row". Synthetic replies cover the seven added parts, and `ddinter-pair.yaml` checks a pair found only in part N.
  - The WHO area case now uses a synthetic export (`drug/who-pq-area-synthetic`: four Oncology rows, one undated, plus HIV/AIDS, Tuberculosis, Hepatitis B and C and Malaria rows). No real WHO rows were added; this ticket leaves the existing WHO recordings alone.
  - The six field titles shortened for the old 2,000-character tool budget are restored, now that 0905's budget is on main.
  - A partner name DDInter does not use is now tried under the exact names MyChem, openFDA labels or ChEMBL state for it, the same way the drug itself is, and a source notice says which name stood in (`ddinter-partner-alias.yaml`, synthetic alias replies). A DDInter ID partner is not widened.
  - A partner whose DDInter entry is refused gets a hint saying it was refused, not that DDInter names no such drug.

