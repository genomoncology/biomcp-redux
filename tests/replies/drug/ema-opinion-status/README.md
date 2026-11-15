# EMA opinion status: blank, Positive and Negative

Recorded on 2026-11-15 by `biomcp --record` without keys in pm job j41168.

| Request | Status | Kept as |
| --- | --- | --- |
| `https://www.ema.europa.eu/en/documents/report/medicines-output-medicines_json-report_en.json` | 200 | trimmed |
| `https://www.ema.europa.eu/en/documents/report/referrals-output-json-report_en.json` | 200 | trimmed |
| `https://api.fda.gov/drug/drugsfda.json?search=products.brand_name%3A%22pembrolizumab%22%20OR%20products.active_ingredients.name%3A%22pembrolizumab%22&limit=50` | 500, then 200 on retry | unchanged (both exchanges kept) |
| `https://api.fda.gov/drug/drugsfda.json?search=application_number%3A%22BLA125514%22&limit=1` | 200 | unchanged (asked twice in the call, both kept) |
| `https://extranet.who.int/prequal/medicines/prequalified/finished-pharmaceutical-products/export?page&_format=csv` | 200 | synthetic (see below) |

The EMA medicines file (timestamp 2026-11-15T06:02:09Z) held 2,765 records. Its `opinion_status` values were blank (2,705, including 1,863 authorised medicines), `Positive` (45) and `Negative` (15); no record lacked the key or held another value. The file keeps three records, each unchanged: Keytruda (EMEA/H/C/003820, Authorised, blank opinion status), Itvisma (EMEA/H/C/006498, Authorised, Positive) and Syfovre (EMEA/H/C/005954, Refused, Negative). The referrals file (593 records) keeps its first record. Only `meta.total_records` is set to the kept count.

Syfovre's blank `revision_number` and `latest_procedure_affecting_product_information` still report problems; this case only checks that its opinion status reads.

## WHO PQ rows are synthetic

WHO PQ is a noncommercial source (0695, 0910), so `who-pq/` holds the synthetic 29-row export from `tests/replies/drug/markets/`, unchanged. No synthetic row names pembrolizumab, as no row in WHO's export did on 2026-11-13, so the markets row for WHO says it has no record.
