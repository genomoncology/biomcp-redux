# 510(k) search by FDA classification name

The replies were recorded from the public openFDA device 510(k) endpoint on 2026-11-14 by `biomcp --record` without a key: the classification-name request in pm job j40733 and the device-name control in j40739. Git holds each reply byte for byte, with a final newline added. No row or value is invented.

| Request | Status | Committed bytes | SHA-256 of the committed file |
| --- | --- | --- | --- |
| `https://api.fda.gov/device/510k.json?limit=5&skip=0&sort=decision_date%3Adesc&search=openfda.device_name%3A%22cell-free%22` | 200 | 6,243 | `7778c836a2ff5a0ab7b58c0e29a7e41ca35c58dc2339b6756d6d9547b37cbeb9` |
| `https://api.fda.gov/device/510k.json?limit=5&skip=0&sort=decision_date%3Adesc&search=device_name%3A%22cell-free%22` | 200 | 3,769 | `b894694faca57b7de890e6fd3dd28771e4d2a190d1347bd058e1fa4e08edb832` |

The search is the dx-lab-director persona question "which cell-free DNA assays went through FDA" (biomed-agent-bench run of 2026-11-14), whose agent missed the de novo PGDx elio plasma focus Dx (DEN230046, product code SBY). openFDA adds FDA's classification name for each record's product code as `openfda.device_name`; SBY is "High Throughput Sequencing Based Tumor Profiling Test Of Circulating Cell-Free Nucleic Acids". Read live on 2026-11-14: `openfda.device_name:"cell-free"` counts 4 records (K260827 and DEN230046 under SBY, K231776 and DEN200001 Cell-Free DNA BCT blood tubes under QMA), `"cell free"` also 4, and the submitted `device_name:"cell-free"` only the 2 blood tubes.
