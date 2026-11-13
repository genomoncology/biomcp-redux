# Device recall enforcement fields

The replies were recorded from the public openFDA device recall and device enforcement endpoints on 2026-11-13 by `biomcp --record` in pm job j38969, without a key. Git holds a projection of each body: recall and event numbers, product code, FDA status, classification, the enforcement seriousness fields (voluntary or mandated, initial firm notification, product quantity), the initiation, classification and report dates, the root cause category, PMA numbers, the dataset date and the native counts. Firm, address, contact, product, reason, action, distribution and lot text is omitted. No row or value is invented.

| Request | Status | Recorded bytes | SHA-256 of the recorded file |
| --- | --- | --- | --- |
| `https://api.fda.gov/device/recall.json?search=product_res_number.exact%3A%22Z-1747-2025%22&limit=1&skip=0` | 200 | 4,716 | `a7322376a438e8c003c288956317222f110f59cf3836496a9919db652e579b3b` |
| `https://api.fda.gov/device/enforcement.json?search=recall_number.exact%3A%22Z-1747-2025%22&limit=1&skip=0` | 200 | 2,317 | `4f13635e0c9d60bb23a575d77c12a5f75b779ac7559730f081c69be561b81e2d` |

openFDA writes the enforcement dates as `YYYYMMDD`. In 1,000 recent device enforcement records read live on 2026-11-13, `initial_firm_notification` was an empty string in 51 and `product_quantity` in 22; BioMCP reads an empty string as no value.
