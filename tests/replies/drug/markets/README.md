# One product across markets: dolutegravir, imatinib and lamivudine

Recorded on 2026-11-13 by `biomcp --record` without keys: Drugs@FDA for dolutegravir and imatinib and the WHO PQ export in pm job j39248; EMA and Drugs@FDA for lamivudine in pm job j39400.

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
| `https://extranet.who.int/prequal/medicines/prequalified/finished-pharmaceutical-products/export?page&_format=csv` | 200 | unchanged, 129,537 bytes, SHA-256 `abaeaf032af17e40b6c6f229ca5a4122b04f1093d50a61060ccf6f1b3d6fa481` |
| `https://www.ema.europa.eu/en/documents/report/medicines-output-medicines_json-report_en.json` | 200 | trimmed |
| `https://www.ema.europa.eu/en/documents/report/referrals-output-json-report_en.json` | 200 | trimmed |

The openFDA publisher dataset date was 2026-11-12. The EMA medicines file (timestamp 2026-11-13T18:02:10Z) held 2,765 records and the referrals file 593. Both are trimmed: the medicines file keeps the 22 records whose INN or active substance names dolutegravir, imatinib or lamivudine (Combivir, Delstrigo, Dovato, Dutrebis, Epivir, Glivec, Imatinib Accord, Imatinib Actavis, Imatinib Koanaa, Imatinib Teva, Imatinib Teva B.V., Imatinib medac, Juluca, Kivexa, Lamivudine Teva, Lamivudine Teva Pharma B.V., Lamivudine/Zidovudine Teva, Ruvise, Tivicay, Triumeq, Trizivir, Zeffix), and the referrals file keeps its first record, EMA/REF/0000369747. Each kept record and its fields are unchanged; only `meta.total_records` is set to the kept count.

Checked live the same day: FDA's Drugs@FDA overview page for application 204790, WHO's product page `https://extranet.who.int/prequal/medicines/ha634` and EMA's Tivicay EPAR page each answered 200. WHO's own list links each product as `/prequal/medicines/` followed by its reference number in lower case with spaces as hyphens, such as `anda-077464a-usfda`. WHO's export lists 52 rows naming dolutegravir, 22 with a blank Date of Prequalification; the earliest dated is HA634 (ViiV Healthcare BV, 14 Oct 2014). It lists no row naming imatinib. Lamivudine matches 138 Drugs@FDA applications, 4 EMA medicines and 92 WHO rows; the chosen records are NDA020596 (EPIVIR), EMEA/H/C/001111 and HA110.
