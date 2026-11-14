# PMA search by intended-use text

The replies were recorded from the public openFDA device PMA endpoint on 2026-11-14 by `biomcp --record` without a key: the intended-use request in pm job j40449 and the trade-name control in j40467. Git holds the trade-name reply byte for byte. The intended-use reply is a projection: the applicant street, city, state and zip fields and the harmonized `openfda` block are omitted from each row; every other field, including each row's whole `ao_statement`, and the native counts are kept. No row or value is invented. openFDA spells P250029's trade name with the control character U+0099 after `SimpleScreen` (likely a mangled trademark sign); the case expects it as published.

| Request | Status | Recorded bytes | SHA-256 of the recorded file |
| --- | --- | --- | --- |
| `https://api.fda.gov/device/pma.json?limit=10&skip=0&sort=decision_date%3Adesc&search=ao_statement%3A%22cell-free%22` | 200 | 31,010 | `12174d448d051c0d9b2bcf4950e584d876fe8c166c68633540014ebe907cbdd5` |
| `https://api.fda.gov/device/pma.json?limit=5&skip=0&sort=decision_date%3Adesc&search=trade_name%3A%22cell-free%22` | 404 | 509 | `72a871d298b5663119fb2e9716cfb5d60c714eb95641bb6a3fcb07674e20753e` |

The search is the dx-lab-director persona question "which cell-free DNA assays went through FDA" (biomed-agent-bench run of 2026-11-14, log.md:85-92): trade-name guesses found 0 or 1 record, while openFDA's `ao_statement:"cell-free"` counts 12 PMA records, including Shield P230009 and SimpleScreen CRC P250029. Read live on 2026-11-14: `ao_statement:"cell free"` also counts 12, `ao_statement:"cfDNA"` 13, `ao_statement:"liquid biopsy"` none, and `trade_name:"cell-free"` none. openFDA answers HTTP 400 to `ao_statement:"café"` and to `"cell-free!"`. The 510(k) endpoint's records carry `device_name`, `statement_or_summary` (the kind of document, not its text), advisory committee, product code and decision fields, and no intended-use text.
