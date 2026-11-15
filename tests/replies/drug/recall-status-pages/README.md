# openFDA drug recall pages with a record that has no recall number

The replies were recorded from the public openFDA API on 2026-11-15 by `biomcp --record` in pm job j41420 on base commit 8ac4f1674, without a key; each body is unchanged. They are the default 10-record pages of `search drug --source fda-drug-recall --status Ongoing` and `--reason contamination`. On the `Ongoing` page the third record (event 100178) has `"recall_number": ""`; on the `contamination` page the ninth record (event 99860) has `"recall_number": "N/A"`. On the base commit both searches failed as a bad reply.

| Request | Status | Recorded bytes | SHA-256 of the recorded file |
| --- | --- | --- | --- |
| `https://api.fda.gov/drug/enforcement.json?search=reason_for_recall%3A%22contamination%22&limit=10&skip=0&sort=recall_initiation_date%3Adesc` | 200 | 14,259 | `bb773ad27b5736590e568a71e2b02d7f6cf71f26940b8c02191f0d17ccfdae31` |
| `https://api.fda.gov/drug/enforcement.json?search=status%3A%22Ongoing%22&limit=10&skip=0&sort=recall_initiation_date%3Adesc` | 200 | 23,679 | `8a454b0969a594173125b6ba16f3fc5b68ee305161051871c3e134153630a79d` |
