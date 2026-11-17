# Drug names with spaces

Recorded on 2026-11-17 by `biomcp --fresh --record` without keys in pm job j42207, one record folder per call, merged; every reply is kept unchanged. The sources are public: openFDA Drugs@FDA and drug labels (US government data), Open Targets Platform (CC0) and MyChem.info.

| Call | Requests kept |
| --- | --- |
| `get drug "metamizole sodium" --source drugsfda` | MyChem's name lookup for metamizole sodium, then one Drugs@FDA brand-or-ingredient query for each of the 21 names it states (Novalgin, Analgin, dipyrone and others); none matches |
| `get drug "metamizole sodium" --source openfda-label` | the same name lookup, then one openFDA label query per name; none matches |
| `get drug "domperidone maleate" --source opentargets` | Open Targets' GraphQL drug search for the name and MyChem's name lookup; Open Targets has no drug by that name |

Metamizole is not marketed in the US, so neither FDA source holds a record for it; each reply is absent and names the ChEMBL record MyChem states. Each reply keeps the requested name, spaces included, in `query.input`.
