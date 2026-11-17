# WHO eEML: dabigatran, warfarin, apixaban and imatinib

Recorded on 2026-11-16 by `biomcp --fresh --record` without keys in pm job j42160, one record folder per call, merged; every reply is kept unchanged. The WHO eEML is published under CC BY 3.0 IGO, which allows copying and redistribution, including for commercial purposes, with attribution (https://list.essentialmeds.org/licencing), so real replies are kept.

| Request | Kept as |
| --- | --- |
| `https://list.essentialmeds.org/?query=NAME` for dabigatran, warfarin, apixaban, imatinib and "dabigatran etexilate" | the site's HTML search page, as a byte sidecar |
| `https://list.essentialmeds.org/medicines/{479,128,478,95}` | JSON-LD, asked with `Accept: application/json` |
| `https://list.essentialmeds.org/recommendations/{1004,1022,153,948,108,415,1280}` | JSON-LD, one per listing read |

The eEML has no documented API. Its search page links each medicine as `/medicines/N` under a heading whose `title` is the medicine's name; the medicine and recommendation pages serve schema.org JSON-LD when asked for JSON. "dabigatran etexilate" finds no medicine, so that call reads only the search page.

Attribution: WHO electronic Essential Medicines List (eEML), World Health Organization, 2020. https://list.essentialmeds.org/ (beta version 1.0). Licence: CC BY 3.0 IGO.
