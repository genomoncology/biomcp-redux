# openFDA adverse event filters with punctuation

The replies were recorded from the public openFDA API on 2026-11-15 by `biomcp --record` in pm job j41373, without a key; each body is unchanged. Each search was typed with punctuation or a non-ASCII character that openFDA answers with HTTP 400; each URL shows the words BioMCP sent instead. On base commit d61945035 the same typed values failed as source failures (pm job j41358). The values are FAERS `aspirin$` and `--reaction` ``Reye`s syndrome`` (a backtick for the apostrophe), and MAUDE `infusion pump!`, `--manufacturer Medtronic®`, `--product-code FRN?` and `--problem leak;`.

| Request | Status | Recorded bytes | SHA-256 of the recorded file |
| --- | --- | --- | --- |
| `https://api.fda.gov/drug/event.json?search=%28patient.drug.openfda.generic_name%3A%22aspirin%22%20OR%20patient.drug.openfda.brand_name%3A%22aspirin%22%20OR%20patient.drug.medicinalproduct%3A%22aspirin%22%29&limit=1&skip=0&sort=receivedate%3Adesc` | 200 | 254,187 | `2dee504691581b8f4f3c7aae3cbd7fcc1494790d8fa5f2f1a49bcfa4cba6be58` |
| `https://api.fda.gov/drug/event.json?search=%28patient.drug.openfda.generic_name%3A%22aspirin%22%20OR%20patient.drug.openfda.brand_name%3A%22aspirin%22%20OR%20patient.drug.medicinalproduct%3A%22aspirin%22%29%20AND%20patient.reaction.reactionmeddrapt%3A%22Reye%20s%20syndrome%22&limit=1&skip=0&sort=receivedate%3Adesc` | 200 | 78,166 | `74451217de3a1b88536a475550b756859520fd0817606beccfff2c318ebf6a92` |
| `https://api.fda.gov/device/event.json?search=product_problems%3A%22leak%22&limit=1&skip=0&sort=date_received%3Adesc` | 200 | 7,688 | `eef71244537238dbb10862ad456d739590b04456e703aa576684194dc619265f` |
| `https://api.fda.gov/device/event.json?search=device.generic_name%3A%22infusion%20pump%22&limit=1&skip=0&sort=date_received%3Adesc` | 200 | 7,842 | `71fd51db4334224c1670a9c8541b5bf318048bbd7a95c33ac033db16c07af18c` |
| `https://api.fda.gov/device/event.json?search=device.manufacturer_d_name%3A%22Medtronic%22&limit=1&skip=0&sort=date_received%3Adesc` | 200 | 8,470 | `c98b46be91ba16d65008b19b691b5eeb0adfa21f0709d1c35dc8f9bd89e1d295` |
| `https://api.fda.gov/device/event.json?search=device.device_report_product_code.exact%3A%22FRN%22&limit=1&skip=0&sort=date_received%3Adesc` | 200 | 7,711 | `7e5b1505344c61075b389c19551da45bfeab641cdb8a9587d978b80f65bb73a2` |
