# One product across markets: dolutegravir, imatinib and lamivudine

Recorded on 2026-11-13 by `biomcp --record` without keys: Drugs@FDA for dolutegravir and imatinib in pm job j39248; EMA and Drugs@FDA for lamivudine in pm job j39400.

| Request | Status | Kept as |
| --- | --- | --- |
| `https://api.fda.gov/drug/drugsfda.json?search=products.brand_name%3A%22dolutegravir%22%20OR%20products.active_ingredients.name%3A%22dolutegravir%22&limit=50` | 200 | unchanged |
| `https://api.fda.gov/drug/drugsfda.json?search=application_number%3A%22NDA204790%22&limit=1` | 200 | unchanged |
| `https://api.fda.gov/drug/drugsfda.json?search=products.brand_name%3A%22imatinib%22%20OR%20products.active_ingredients.name%3A%22imatinib%22&limit=50` | 200 | unchanged |
| `https://api.fda.gov/drug/drugsfda.json?search=application_number%3A%22NDA021335%22&limit=1` | 200 | unchanged |
| `https://api.fda.gov/drug/drugsfda.json?search=products.brand_name%3A%22lamivudine%22%20OR%20products.active_ingredients.name%3A%22lamivudine%22&limit=50` | 200 | unchanged |
| `https://api.fda.gov/drug/drugsfda.json?search=products.brand_name%3A%22lamivudine%22%20OR%20products.active_ingredients.name%3A%22lamivudine%22&limit=50&skip=50` | 200 | unchanged |
| `https://api.fda.gov/drug/drugsfda.json?search=products.brand_name%3A%22lamivudine%22%20OR%20products.active_ingredients.name%3A%22lamivudine%22&limit=50&skip=100` | 200 | unchanged |
| `https://api.fda.gov/drug/drugsfda.json?search=application_number%3A%22NDA020596%22&limit=1` | 200 | unchanged (asked twice in the call, both kept) |
| `https://extranet.who.int/prequal/medicines/prequalified/finished-pharmaceutical-products/export?page&_format=csv` | 200 | synthetic (see below) |
| `https://www.ema.europa.eu/en/documents/report/medicines-output-medicines_json-report_en.json` | 200 | trimmed |
| `https://www.ema.europa.eu/en/documents/report/referrals-output-json-report_en.json` | 200 | trimmed |

The openFDA publisher dataset date was 2026-11-12. The EMA medicines file (timestamp 2026-11-13T18:02:10Z) held 2,765 records and the referrals file 593. Both are trimmed: the medicines file keeps the 22 records whose INN or active substance names dolutegravir, imatinib or lamivudine (Combivir, Delstrigo, Dovato, Dutrebis, Epivir, Glivec, Imatinib Accord, Imatinib Actavis, Imatinib Koanaa, Imatinib Teva, Imatinib Teva B.V., Imatinib medac, Juluca, Kivexa, Lamivudine Teva, Lamivudine Teva Pharma B.V., Lamivudine/Zidovudine Teva, Ruvise, Tivicay, Triumeq, Trizivir, Zeffix), and the referrals file keeps its first record, EMA/REF/0000369747. Each kept record and its fields are unchanged; only `meta.total_records` is set to the kept count.

Checked live the same day: FDA's Drugs@FDA overview page for application 204790, a WHO product page and EMA's Tivicay EPAR page each answered 200. WHO's own list links each product as `/prequal/medicines/` followed by its reference number in lower case with spaces as hyphens. Lamivudine matches 138 Drugs@FDA applications and 4 EMA medicines; the chosen records are NDA020596 (EPIVIR) and EMEA/H/C/001111.

## WHO PQ rows are synthetic

WHO PQ is a noncommercial source, so its export is not kept here (0695, 0910). `who-pq/` holds a synthetic 29-row export, the same file as `tests/replies/drug/who-pq-dates-read/`, in the real export's shape: the same header, quoting (cells with a space or comma quoted, single words and blanks bare), padded date cells such as `22  Sep,  2013`, blank Date of Prequalification cells, and no final newline. Every reference, applicant, strength and date is invented; no date appears in WHO's export of 2026-11-13. It keeps the edge cases the case relies on, so a choice that took the first dated row, or the last of a tie, would fail:

- Dolutegravir: 13 rows; the first five and the last two are undated. The first dated row in export order is SYN202 (9 Nov 2017); the earliest is SYN201 (22 Sep 2013), listed right after it, so the case expects SYN201.
- Imatinib: no row names it.
- Lamivudine: 16 rows. The first dated row is SYN104 (3 Sep 2003); after it SYN101, SYN102 and SYN105 tie on the earliest date (7 Jun 2001), so the case expects SYN101, the first of the tie in export order.
