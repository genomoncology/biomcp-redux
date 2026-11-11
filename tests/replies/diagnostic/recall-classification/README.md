# Device recall classification

The replies were captured from the public openFDA device recall and device enforcement endpoints on 2026-11-11 with `scripts/source_capture.py`, without a key. Git holds a projection of each body: recall numbers, event numbers, product codes, FDA status, classification and date facts, the dataset date and the native counts. Firm, product, reason and distribution prose is omitted. No row or value is invented.

| Request | Status | Bytes | SHA-256 of the original body |
| --- | --- | --- | --- |
| `https://api.fda.gov/device/recall.json?search=recalling_firm%3A%22Remel%22&limit=5&skip=0&sort=event_date_initiated%3Adesc` | 200 | 17,860 | `ca7a23da8e52a0b32254bd93fd4e7123be5bf88b4f4d9cf7735d64c66f237317` |
| `https://api.fda.gov/device/enforcement.json?search=recall_number.exact%3A%28%22Z-0355-2027%22%20OR%20%22Z-2927-2026%22%20OR%20%22Z-2623-2026%22%20OR%20%22Z-1923-2026%22%20OR%20%22Z-1806-2026%22%29&limit=5&skip=0` | 200 | 5,774 | `c20db13663b6aa2965fbf0c228249af391f74af1b6d9721792d4ec63ab7a9bad` |
| `https://api.fda.gov/device/enforcement.json?search=classification%3A%22Class%20I%22%20AND%20recalling_firm%3A%22Remel%22&limit=3&skip=0&sort=recall_initiation_date%3Adesc` | 200 | 4,571 | `07c5e77dc1be738afd8e0db0d09e589fb56b03324e8b350cf8da800f3054fb5f` |
| `https://api.fda.gov/device/recall.json?search=product_res_number.exact%3A%28%22Z-0334-2023%22%20OR%20%22Z-0329-2023%22%20OR%20%22Z-0330-2023%22%29&limit=3&skip=0` | 200 | 38,726 | `bd2fdde5fe9f7750986eed3b5e8fcd9e2698d091e0aadfe0294b3ba1d16de678` |

The first pair is one search page and its single enforcement join. Enforcement returned four of the five recall numbers; `Z-0355-2027`, initiated 2026-10-19, had no enforcement record on the capture date, so the case asserts a row without a class and the notice that counts it. The second pair is the `Class I` filter: enforcement records select the page and one exact request reads their recall records.

Device recall records hold no classification field, so the source cannot filter recall records by class. The enforcement dataset holds fewer records than the recall dataset (40,481 against 59,705 on the capture date).
