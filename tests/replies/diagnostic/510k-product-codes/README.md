# 510(k) records counted by product code

The replies were recorded from the public openFDA device 510(k) endpoint on 2026-11-16 by `biomcp --record` without a key in pm job j42077. Git holds each reply as recorded. No row or value is invented.

| Request | Status | Bytes | SHA-256 |
| --- | --- | --- | --- |
| `https://api.fda.gov/device/510k.json?limit=5&skip=0&sort=decision_date%3Adesc&search=decision_date%3A%5B2024-01-01%20TO%202024-12-31%5D` | 200 | 46961 | `b3edfb6347cd0ea69732544586e59a3a245b0a549d5e1c973a2c6daa5b6b24ed` |
| `https://api.fda.gov/device/510k.json?count=product_code&limit=1000&search=decision_date%3A%5B2024-01-01%20TO%202024-12-31%5D` | 200 | 41351 | `9b31722ba5f0e1d846545a7c20d4b9e4727b0c2b83c4c792c1e2f6408f867951` |

The search is the life-sciences-ci persona's question of 510(k) decisions in 2024 by product code (biomed-agent-bench run of 2026-11-16), whose agent used openFDA's facet without a limit and got only its 100 largest codes. With `limit=1000` the facet returns 792 terms summing to 3,129, the search total; one term is an empty product code with 1 record.
