# Adverse event slice

## Basis and evidence boundary

This investigation starts from `d18bfbba511e3852c99228f1a5166a14633e5cee`. The legacy inventory reads `repos/biomcp` revision `4050fdc7dbdf4d713d94cf02a39a147b863665be`; it does not execute that binary. This proposal follows steps 1–4 of [slices](../slices.md). It changes no product code. The PM reviews this branch before filing implementation tickets.

Shared types cannot be accepted before exact-request native captures. The following recipe goes to a separate build delegate. No provider API requests, product executions, cold/warm timings, output token measurements or legacy comparison runs have occurred here. Documentation fields are questions for capture, never proof that all public records carry them.

## Bounded native capture recipe

Run through `pm job` from an exact pushed revision. Retain recipe, exact URL or POST body, start/end times, status, headers, unchanged bytes, SHA-256, byte size and classification for every attempted request. Store raw evidence on Yellow outside Git. Send each listed request once, sequentially, with zero retries, redirects, fallback spellings, extra pages, refreshes or health probes. A failed row stays failed; a 429 stops that source's remaining rows. Never provoke a provider failure. Local synthetic controls are separate evidence. Use HTTPS verification, 30-second request deadlines, 8 MiB openFDA/body cap and 2 MiB WONDER/body cap. These are capture limits, not measured costs.

Maximum provider budget: **33 requests**: 20 FAERS base rows, four device rows, three conditional exact report/identity rows, two CDC files, three WONDER rows, and one reserved exact FAERS aggregation query defined below. No request replaces a skipped row. No account creation, keys borrowed from another user, production data or licensed curator answers. openFDA runs keyless under its documented public quota. WONDER is keyless, includes `accept_datause_restrictions=true`, observes its [data use restrictions](https://wonder.cdc.gov/datause.html) and keeps the existing legacy gap of at least 16 seconds between requests. No re-identification or suppressed-cell reconstruction.

F01–F20 use `GET https://api.fda.gov/drug/event.json`, `limit=5`, `skip=0`, `sort=receivedate:desc`, except where stated. Encode the exact decoded expressions below. `N(x)` means `(patient.drug.openfda.generic_name:"x" OR patient.drug.openfda.brand_name:"x" OR patient.drug.medicinalproduct:"x")`. Expand it literally in the recipe and retain the final expression. These are explicit text searches, never identity joins.

| Row | Exact search or operation | Native question |
|---|---|---|
| F01 | `N(pembrolizumab)` | Complete drug/reaction arrays, report IDs/versions and optional harmonized identities. |
| F02 | `N(pembrolizumab) AND patient.reaction.reactionmeddrapt:"MYOCARDITIS"` | Co-reporting, never causation. |
| F03 | `N(pembrolizumab) AND patient.reaction.reactionmeddrapt:"PNEUMONITIS"` | Distinct source reaction term. |
| F04 | `N(trastuzumab) AND patient.reaction.reactionmeddrapt:"CARDIOMYOPATHY"` | Distinct products, including conjugates, must remain distinct. |
| F05 | `N(fam-trastuzumab deruxtecan-nxki) AND patient.reaction.reactionmeddrapt:"INTERSTITIAL LUNG DISEASE"` | No Enhertu spelling fallback if empty. |
| F06 | `N(axicabtagene ciloleucel) AND patient.reaction.reactionmeddrapt:"CYTOKINE RELEASE SYNDROME"` | CAR-T report scope. |
| F07 | `N(axicabtagene ciloleucel) AND patient.reaction.reactionmeddrapt:"NEUROTOXICITY"` | Distinct neurologic report scope. |
| F08 | `N(tamoxifen) AND patient.reaction.reactionmeddrapt:"PULMONARY EMBOLISM"` | Native reaction, indication and drug roles. |
| F09 | `N(venetoclax) AND patient.reaction.reactionmeddrapt:"TUMOUR LYSIS SYNDROME"` | Exact term; no spelling retry. |
| F10 | `N(bevacizumab) AND patient.reaction.reactionmeddrapt:"GASTROINTESTINAL PERFORATION"` | Native reporting term. |
| F11 | `N(ibrutinib) AND patient.reaction.reactionmeddrapt:"HAEMORRHAGE"` | Native reporting term. |
| F12 | `N(osimertinib) AND patient.reaction.reactionmeddrapt:"ELECTROCARDIOGRAM QT PROLONGED"` | Report versus label warning. |
| F13 | `N(pembrolizumab) AND seriousnessdeath:1` | Report-level death flag. |
| F14 | `N(pembrolizumab) AND seriousnesshospitalization:1` | Hospitalization is not a reaction-outcome code. |
| F15 | `N(pembrolizumab) AND receivedate:[20240101 TO 20241231]` | Receipt interval is not event onset or exposure interval. |
| F16 | `N(pembrolizumab) AND patient.drug.drugcharacterization:1` | Check roles inside each drug object; cross-object conjunction cannot establish that this drug was suspect. |
| F17 | `N(pembrolizumab) AND patient.patientsex:2 AND patient.patientonsetage:[65 TO 120] AND patient.patientonsetageunit:801 AND primarysource.qualification:1` | Explicit recorded female sex, years and physician; no age-unit assumption. |
| F18 | `safetyreportid:"BIOMCP-NO-SUCH-REPORT-0152"` | Exact-ID negative control; no-match versus transport failure. |
| F19 | F01 expression with `count=patient.reaction.reactionmeddrapt.exact`, `limit=5`, no sort/skip | Buckets and native count unit; no sum-of-buckets report denominator. |
| F20 | F01 expression, `limit=5`, `skip=5`, same sort | Actual next page. |

D01–D04 use `GET https://api.fda.gov/device/event.json`, `limit=5`, `skip=0`, `sort=date_received:desc`.

| Row | Exact decoded search | Native question |
|---|---|---|
| D01 | `device.generic_name:"INFUSION PUMP"` | Complete device list and report identity; query alone does not establish oncology use. |
| D02 | `device.generic_name:"INFUSION PUMP" AND event_type:"Death"` | Device event types are separate from drug seriousness. |
| D03 | `device.generic_name:"IMPLANTED PORT"` | Cancer-care access device candidate; empty remains empty. |
| D04 | `device.generic_name:"INFUSION PUMP" AND date_received:[20240101 TO 20241231]` | Received period and optional narrative. |

F21 conditionally gets the first actual F01 report in native order with `safetyreportid:"<exact native ID>" AND safetyreportversion:"<exact native version>"`, limit 1. If version is absent, query exact ID alone and preserve every returned version. D05 gets the first actual D01 `mdr_report_key` by that exact field, limit 1. I01 selects the first F01 drug object carrying `openfda.unii` and queries `patient.drug.openfda.unii:"<first exact native UNII>"`, limit 5. All native UNIIs remain visible; this one selected query proves no universal product join. Missing prerequisite IDs skip the conditional row. F22 repeats F01 with `count=seriousnessdeath`, limit 5, no sort/skip, to inspect the aggregate unit without calculating a risk.

C01 downloads `https://www2.cdc.gov/vaccines/iis/iisstandards/downloads/cvx.txt` once. C02 downloads `https://www2.cdc.gov/vaccines/iis/iisstandards/downloads/TRADENAME.txt` once. Record whole-file receipts, delimiters, optional columns and notices. Inspect exact CVX 165, 62, 118 and trade rows whose CVX column equals those values. No MVX file or CPT crosswalk is needed. Never infer a CVX-to-WONDER mapping from text or numerals.

V01–V03 POST to `https://wonder.cdc.gov/controller/datarequest/D8` as `request_xml=<XML>&accept_datause_restrictions=true`. Start with the exact committed legacy `spec/fixtures/vaers/reactions-request.xml` at the cited revision. Change only `F_D8.V14` from `MMR` to `HPV9`; change `B_1` respectively to `D8.V13-level2`, `D8.V10`, `D8.V1`. Preserve every other parameter including declared vintage. Archive final XML before sending. Record actual returned dataset/date/version, notices, suppression, row labels/codes, totals and counted unit. This is a caller-selected native code, not an inferred CVX165 join. Rejected HPV9 ends that row; no HPV4 or alternate-code fallback.

Each harvested question will map to these rows or a specific out-of-scope reason. Cancer drug questions do not call VAERS/CVX; vaccine questions do not call FAERS automatically; devices do not call drug endpoints. An API error is never negative clinical evidence.

## Documentation findings and source proposal

| Candidate | Proposed use and official reference | Use and terms | Native questions |
|---|---|---|---|
| openFDA FAERS | Explicit report search, exact report/version get, aggregation; independent drug and reaction arrays. [Overview](https://open.fda.gov/apis/drug/event/), [fields](https://open.fda.gov/apis/drug/event/searchable-fields/). | `open`, [openFDA terms](https://open.fda.gov/terms/), CC0 default with stated third-party exceptions; attribution visible. | Versions, optional UNII/RxCUI/NDC/SPL identities, dates, roles, multiplicity. |
| openFDA MAUDE | Separate device report family with native key/event type. [Overview](https://open.fda.gov/apis/device/event/), [endpoint](https://open.fda.gov/apis/device/event/how-to-use-the-endpoint/). | `open`, same FDA terms and exceptions. | Full device list, product code/UDI availability, report multiplicity, narrative size. |
| CDC WONDER VAERS | Explicit HPV9 aggregates. [API](https://wonder.cdc.gov/wonder/help/wonder-api.html), [help](https://wonder.cdc.gov/wonder/help/vaers.html). | Proposed `restricted` because [data use](https://wonder.cdc.gov/datause.html) limits purpose. Keep CDC citation/notices/suppression; no account key requirement. PM reviews category. | Actual fields, totals/counting unit; no promised individual report get. |
| CDC CVX/trade names | Explicit vaccine identities. [Code sets](https://www.cdc.gov/iis/code-sets/index.html). No token-based name bridge or automatic search-time bundle sync. | Proposed `open` government tables; inspect per-download notices. Exclude CPT tables and their separate AMA terms. | Status/dates, native product-to-CVX/MVX relations, no proved WONDER join. |
| ClinicalTrials.gov results | Drop automatic fallback after FAERS empty. Trial-owned explicit evidence remains separate. | Existing trial source terms. | No capture requested; trial denominators cannot replace spontaneous report counts. |
| FDA recalls | Drop the legacy recall route from this entity; regulatory notices need separate question-driven ownership. | Existing FDA terms. | No capture requested; no claim of recall parity. |

[Authentication](https://open.fda.gov/apis/authentication/) lists public keyless limits and [the event guide](https://open.fda.gov/apis/drug/event/how-to-use-the-endpoint/) permits keyless use. Optional higher quotas do not change source rights. The registry will own use categories, terms and disable policy once; response types will reference that source truth.
