# Pathway search across Reactome and WikiPathways: MAPK

Recorded on 2026-11-15 by `biomcp --record` without keys in pm job j41438 (`sync wikipathways`, then `search pathway MAPK --source reactome --source wikipathways --limit 5`).

| Request | Status | Kept as |
| --- | --- | --- |
| `https://reactome.org/ContentService/search/query?query=MAPK&rows=5&Start%20row=0&species=Homo%20sapiens&types=Pathway&types=TopLevelPathway&Force%20filters=true` | 200 | unchanged |
| `https://www.wikipathways.org/json/findPathwaysByText.json` | 200 | trimmed |
| `https://www.wikipathways.org/json/getPathwayInfo.json` | 200 | trimmed |

Both WikiPathways catalogs held 2,218 rows. Each is trimmed to the 24 rows whose title contains "MAPK" without regard to case (all species; 7 are human). Each kept row and its fields are unchanged and in the catalog's order. The recording stored the catalogs as sidecar files; here each body is the trimmed JSON written inline as text.
