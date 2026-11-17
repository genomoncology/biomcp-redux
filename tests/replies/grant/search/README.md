# NIH RePORTER: Pasi Janne's awards, a KRAS G12C text search and the Hua Zhang name clash

Recorded on 2026-11-17 by `biomcp --fresh --record` without keys in pm job j42259, one record folder per call, merged; every reply is kept unchanged. RePORTER is public NIH data (RePORT Data Access Policy, https://report.nih.gov/sites/report/files/2020-07/DataAccessPolicy.pdf; https://api.reporter.nih.gov/), so real replies are kept. BioMCP asks only for named fields, so the grantees' abstracts are not in these replies.

| Request | Kept as |
| --- | --- |
| `POST https://api.reporter.nih.gov/v2/projects/search` with `pi_names [{"any_name": "Pasi Janne"}]`, limit 3 and then offset 68 | JSON |
| the same with `advanced_text_search` "KRAS G12C" over title, abstract and terms | JSON |
| the Janne search with `org_names ["DANA-FARBER"]` and `fiscal_years [2025]` | JSON |
| `pi_names [{"any_name": "Zzqxv Nobody"}]` | JSON, total 0 |
| `pi_names [{"any_name": "Hua Zhang"}]`, limit 10 | JSON, total 273, many different people |
| `pi_profile_ids [14892358]`, limit 3, and `pi_profile_ids [1861387]` with `fiscal_years [2026]` | JSON |

Every request names `SubprojectId` among its fields, so rows from multi-part awards carry their component ID.
