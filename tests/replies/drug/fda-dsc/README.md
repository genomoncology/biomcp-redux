# FDA Drug Safety Communications: tofacitinib

Recorded on 2026-11-20 by `biomcp --record` without keys, one record folder per call, merged; every reply is kept unchanged. FDA's website policies: "Unless otherwise noted, the contents of the FDA website (www.fda.gov) — both text and graphics — are not copyrighted. They are in the public domain" (https://www.fda.gov/about-fda/about-website/website-policies). MyChem.info and openFDA name lookups are public.

| Call | Requests kept |
| --- | --- |
| `search drug tofacitinib --source fda-dsc` | the drug-name lookups that widen tofacitinib (Xeljanz and its other names), and FDA's index page; every row FDA lists is in the shipped list, so no communication page is read |
| `search drug --source fda-dsc --limit 3` | FDA's index page |
| `get drug 4ec3359d-b7fe-41ca-9724-9602b68d8ca6 --source fda-dsc` | the 2021-09-01 JAK inhibitor communication's page |

FDA's index lists the 2021-09-01 communication's date as `9-1-21`, which BioMCP does not read, so that row has no `date` and is named in problems; its page states 2021-09-01, which is `issued`.
