# Device recall line breaks

The replies were recorded from the public openFDA device recall and device enforcement endpoints on 2026-11-14 by `biomcp --record` in pm job j40879, without a key. Git holds a projection of each body: address, contact, FEI number, lot (`code_info`, `more_code_info`) and action text are dropped. Every other field is kept as recorded, including the recall `product_description` with its seven line breaks. No row or value is invented.

| Request | Status | Recorded bytes | SHA-256 of the recorded file |
| --- | --- | --- | --- |
| `https://api.fda.gov/device/recall.json?search=product_res_number.exact%3A%22Z-0884-2026%22&limit=1&skip=0` | 200 | 5,870 | `d3692bdcf14470391002aeedbf1a29ea5f36d736e74196de4485fa0c574ea008` |
| `https://api.fda.gov/device/enforcement.json?search=recall_number.exact%3A%22Z-0884-2026%22&limit=1&skip=0` | 200 | 4,160 | `fe4e2486deb0e9c196cdf965e6f5db90971876eb2723935c1fe10b9b0c12b9b2` |

openFDA sends the line breaks as JSON `\n`. BioMCP joins each run of them with one space, so the reply carries no `\u{000a}` escape (ticket 0947).
