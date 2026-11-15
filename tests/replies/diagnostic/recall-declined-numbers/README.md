# openFDA device recall pages with a malformed recall number

The replies were recorded from the public openFDA device recall and device enforcement endpoints on 2026-11-15 by `biomcp --record` in pm job j41441, without a key. Git holds a projection of each 200 body: address, contact, FEI number, lot (`code_info`, `more_code_info`) and action text are dropped; the 404 body is unchanged. Every other field is kept as recorded. No row or value is invented.

- `--product-code NPR`: two recall records. `Z-1652-2011` is returned; its exact enforcement lookup is a 404, so the row shows no classification. The other record (Roche Molecular Systems Factor II kit, `cfres_id` 86392, event 53708) has `"product_res_number": "Z1006-2010"`, without the first hyphen. On base commit 648240afe this search failed whole as a bad reply (pm job j41440).
- `--classification "Class II" --manufacturer "Valorem Surgical"`: five enforcement records; the third (event 77500) has `"recall_number": "Z-3011-201"`, cut short. The other four are returned and joined to their recall records in one exact request.

openFDA's bulk exports on the recording date held 5,539 device recall records and 11 device enforcement records whose number is not `Z-NNNN-NNNN`, mostly the older `Z-NNNN-YY` form (pm job j41437).

| Request | Status | Committed bytes | SHA-256 of the committed file | Original recording |
| --- | --- | --- | --- | --- |
| `https://api.fda.gov/device/recall.json?search=product_code%3A%22NPR%22&limit=5&skip=0&sort=event_date_initiated%3Adesc` | 200 | 6,327 | `b114729c3bec23406b0347cc3deab82310c0bf23a32477f531105d661444c9cc` | 7,325 bytes, SHA-256 `9e38b7e120526c005b1c7adc8ce027e529d2b43dc3ff8cc15d574f4b4f5606a5`, before projection |
| `https://api.fda.gov/device/enforcement.json?search=recall_number.exact%3A%22Z-1652-2011%22&limit=1&skip=0` | 404 | 512 | `499335f455c8d84afb5231e9aac4e32616f41aff41c113ac22861a483f93f47f` | identical to the committed file |
| `https://api.fda.gov/device/enforcement.json?search=classification%3A%22Class%20II%22%20AND%20recalling_firm%3A%22Valorem%20Surgical%22&limit=5&skip=0&sort=recall_initiation_date%3Adesc` | 200 | 7,024 | `28f6b9b9075ebd15410a3dee0ba8e9f4214c6c0180bf880ee5a9651a00742618` | 7,450 bytes, SHA-256 `49c6e513dc9c5ae89606de006827198be6c1a80ac2cd7e5712004dc7a011d87c`, before projection |
| `https://api.fda.gov/device/recall.json?search=product_res_number.exact%3A%28%22Z-2958-2017%22%20OR%20%22Z-2925-2017%22%20OR%20%22Z-2957-2017%22%20OR%20%22Z-2895-2017%22%29&limit=4&skip=0` | 200 | 129,758 | `f0266c30eb5575230b3f83729451261e0e5186641c1c5856026a561a5bbe5890` | 100,409 bytes, SHA-256 `0373c8149428cd3bd37e08a5581ffebd32df5ad3f8aa8f12310b93210f979961`, before projection |
