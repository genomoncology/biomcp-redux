# openFDA device filters with punctuation

The replies were recorded from the public openFDA API on 2026-11-15 by `biomcp --record` in pm job j41373, without a key; each body is unchanged. Each search was typed with punctuation or a non-ASCII character that openFDA answers with HTTP 400; each URL shows the words BioMCP sent instead. On base commit d61945035 the same typed values failed as source failures (pm job j41358). The values are 510(k) `--name pump!`, `--manufacturer Abbott®` and `--product-code FRN?`; PMA `--name pump;` and `--manufacturer Abbott|`; and device recall `--manufacturer Abbott#` and `--product-code FRN$`, whose rows join their exact enforcement records.

| Request | Status | Recorded bytes | SHA-256 of the recorded file |
| --- | --- | --- | --- |
| `https://api.fda.gov/device/510k.json?limit=1&skip=0&sort=decision_date%3Adesc&search=product_code%3A%22FRN%22` | 200 | 8,889 | `379f28941135094d1c82d62e86e4abcef1a017a5d8b834efcbb8dd035e607cb6` |
| `https://api.fda.gov/device/510k.json?limit=1&skip=0&sort=decision_date%3Adesc&search=applicant%3A%22Abbott%22` | 200 | 3,203 | `ac790e061932e03da11df14de92f0cef4db9d4d2058cedc9ac04bc0a40838e9e` |
| `https://api.fda.gov/device/510k.json?limit=1&skip=0&sort=decision_date%3Adesc&search=device_name%3A%22pump%22` | 200 | 7,620 | `b61300238a75cffa836c692500e5998676845a8fbc2da95ae8723987325d9d71` |
| `https://api.fda.gov/device/recall.json?search=recalling_firm%3A%22Abbott%22&limit=1&skip=0&sort=event_date_initiated%3Adesc` | 200 | 4,458 | `71b0b6c867c1ee895c7fa6355ac7d3af57cc93b634cc34bbfd12d1bc25433346` |
| `https://api.fda.gov/device/recall.json?search=product_code%3A%22FRN%22&limit=1&skip=0&sort=event_date_initiated%3Adesc` | 200 | 27,803 | `0b1fe8066008644683c20a1a805ecb387099fbc8a0f5cb75b63cea71200bef97` |
| `https://api.fda.gov/device/enforcement.json?search=recall_number.exact%3A%22Z-3065-2026%22&limit=1&skip=0` | 200 | 2,306 | `4010be7df9b260a12d2312cea4412e7ab28cdda15723f8350ce16a165d964ebc` |
| `https://api.fda.gov/device/enforcement.json?search=recall_number.exact%3A%22Z-0191-2027%22&limit=1&skip=0` | 200 | 2,301 | `03efd6744ed46b669b86476e1787890091862eba0b92fa2850d8547a604682de` |
| `https://api.fda.gov/device/pma.json?limit=1&skip=0&sort=decision_date%3Adesc&search=trade_name%3A%22pump%22` | 200 | 4,018 | `610952aced49055f1adaef234b9f092ab8bdd4bf2a8d0249664bfef6da294f8a` |
| `https://api.fda.gov/device/pma.json?limit=1&skip=0&sort=decision_date%3Adesc&search=applicant%3A%22Abbott%22` | 200 | 4,815 | `897e6072af268f8e1b1e82aa3a0300e21de31c0c023397a36191d8e8521d1210` |
