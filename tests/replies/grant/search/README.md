# NIH RePORTER: Pasi Janne's projects and a KRAS G12C text search

Recorded on 2026-11-17 by `biomcp --fresh --record` without keys in pm jobs j42229 and JOB3, one record folder per call, merged; every reply is kept unchanged. RePORTER is public NIH data (RePORT Data Access Policy, https://report.nih.gov/sites/report/files/2020-07/DataAccessPolicy.pdf; https://api.reporter.nih.gov/), so real replies are kept. BioMCP asks only for named fields, so the grantees' abstracts are not in these replies.

| Request | Kept as |
| --- | --- |
| `POST https://api.reporter.nih.gov/v2/projects/search` with `pi_names [{"any_name": "Pasi Janne"}]`, limit 3 and then offset 68 | JSON |
| the same with `advanced_text_search` "KRAS G12C" over title, abstract and terms | JSON |
| the Janne search with `org_names ["DANA-FARBER"]` and `fiscal_years [2024, 2025]` | JSON |
| `pi_names [{"any_name": "Zzqxv Nobody"}]` | JSON, total 0 |
