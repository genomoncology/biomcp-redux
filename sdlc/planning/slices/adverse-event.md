# Adverse event slice

## Basis and evidence boundary

This investigation starts from `d18bfbba511e3852c99228f1a5166a14633e5cee`. The legacy inventory reads `repos/biomcp` revision `4050fdc7dbdf4d713d94cf02a39a147b863665be`; it does not execute that binary. This proposal follows steps 1–4 of [slices](../slices.md). It changes no product code. The PM reviews this branch before filing implementation tickets.

Shared types cannot be accepted before exact-request native captures. The following recipe goes to a separate build delegate. The investigation author made no provider API requests, product executions, cold/warm timings, output token measurements or legacy comparison runs. A separate delegate ran the bounded native recipe; its verified receipts and findings are recorded below. Documentation fields are questions for capture, never proof that all public records carry them.

## Bounded native capture recipe

Run through `pm job` from an exact pushed revision. Retain recipe, exact URL or POST body, start/end times, status, headers, unchanged bytes, SHA-256, byte size and classification for every attempted request. Store raw evidence on Yellow outside Git. Send each listed request once, sequentially, with zero retries, redirects, fallback spellings, extra pages, refreshes or health probes. A failed row stays failed; a 429 stops that source's remaining rows. Never provoke a provider failure. Local synthetic controls are separate evidence. Use HTTPS verification, 30-second request deadlines, 8 MiB openFDA/body cap and 2 MiB WONDER/body cap. These are capture limits, not measured costs.

Maximum provider budget: **34 requests**: 21 FAERS base rows, four device rows, three conditional exact report/identity rows, two CDC files, three WONDER rows, and one reserved exact FAERS aggregation query defined below. No request replaces a skipped row. No account creation, keys borrowed from another user, production data or licensed curator answers. openFDA runs keyless under its documented public quota. WONDER is keyless, includes `accept_datause_restrictions=true`, observes its [data use restrictions](https://wonder.cdc.gov/datause.html) and keeps the existing legacy gap of at least 16 seconds between requests. No re-identification or suppressed-cell reconstruction.

F01–F20 and F23 use `GET https://api.fda.gov/drug/event.json`, `limit=5`, `skip=0`, `sort=receivedate:desc`, except where stated. Encode the exact decoded expressions below. `N(x)` means `(patient.drug.openfda.generic_name:"x" OR patient.drug.openfda.brand_name:"x" OR patient.drug.medicinalproduct:"x")`. Expand it literally in the recipe and retain the final expression. These are explicit text searches, never identity joins.

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
| F23 | `N(osimertinib) AND patient.reaction.reactionmeddrapt:"PNEUMONITIS"` | Separate ILD/pneumonitis question. |

D01–D04 use `GET https://api.fda.gov/device/event.json`, `limit=5`, `skip=0`, `sort=date_received:desc`.

| Row | Exact decoded search | Native question |
|---|---|---|
| D01 | `device.generic_name:"BREAST IMPLANT"` | Complete device list and report identity; implant presence does not establish cancer or reconstruction. |
| D02 | `device.generic_name:"BREAST IMPLANT" AND event_type:"Death"` | Device event types are separate from drug seriousness. |
| D03 | `mdr_text.text:"SQUAMOUS CELL CARCINOMA" AND device.generic_name:"BREAST IMPLANT"` | Literal capsule-SCC discovery query; no automatic narrative classification or BIA-ALCL equivalence. |
| D04 | `device.generic_name:"BREAST IMPLANT" AND date_received:[20240101 TO 20241231]` | Received period and optional narrative; no inference of event onset. |

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

## Native capture finding

A separate build delegate ran `j12641` from landed `d18bfbba511e3852c99228f1a5166a14633e5cee` with the exact proposed 34-row ledger. It made 34 physical provider requests: FAERS 23 HTTP200 plus the intentional exact-ID HTTP404; devices four HTTP200 plus the literal SCC query HTTP404; both CDC files HTTP200; WONDER three HTTP403 HTML denials. There were no retries, redirects, refreshes or fallbacks. This is source evidence, not product acceptance or legacy execution.

The unchanged evidence is on Yellow at `data/biomcp-redux/0152-adverse-event-native-proof/j12641-native-capture.tar.gz`, SHA-256 `b96dcbbfb7349f79596eaea262674a9711f9968c605572241a2c191caabf101a`, 1,198,872 bytes. The delegate independently verified all 225 manifest files. Archive root `2293-adverse-event-native-capture` contains unchanged bodies, request/receipt/headers, both job logs, exact original/final XML, `ledger.json`, `summary.json`, `native-field-inventory.json` and explicitly non-product `offline-investigation.json`. The independent verification JSON sits beside the archive. Original source dates remain separate from local investigation dates.

F01 returned native total 105020 and report 26940567 version 1 first. F21 exact get returned that native report. F20 returned native skip 5 and total 105020. The five F01 report objects contained respectively 2/9/6/5/2 drug rows and 1/3/4/1/2 reaction rows. The default page must not select one drug as the report's sole drug. I01 selected the first actual drug object's UNII DLE8519RWM from PADCEV, not the later KEYTRUDA object's UNII, and returned total 7224. This proves one exact native-ID query and no pembrolizumab identity join. F19 string reaction terms and F22 numeric terms are separate native bucket shapes, not a common clinical count vocabulary or denominator.

The first device key 50952244 returned unchanged through exact D05 get. Device event, received and reporting dates are distinct. Optional source strings can be present and empty; empty must not silently become absent without an explicit reviewed rule. C01 carried 290 seven-column pipe-delimited rows. C02 carried 225 nine-column rows, including its trailing empty column. Exact CVX 118/62/165 and their trade rows were selected by the native CVX column while preserving original whitespace. These native code relations do not prove a WONDER relation.

All three WONDER XML requests returned403 Access Denied HTML. No aggregate schema, code acceptance, vintage, total, suppression or report-level identifier was proved. Do not accept VAERS response types or advertise a working summary from these replies. Recommended disposition is to retain the source as a proposed future explicit option and defer runtime implementation pending a separately approved bounded native positive capture. No alternate HPV code, user-agent workaround or repeated attempt is authorized by this evidence. The HPV questions remain unanswered by this tier. The literal SCC device query's404 is a verified no-match for that exact expression, not evidence that capsule SCC does not occur.

### Observed native fields and disclosure consequences

The JSON field inventory contains strings, integer metadata/counts, objects and arrays. No null-valued field was observed in this sample; this does not establish a universal no-null schema. Age/outcome/role/date values are source strings, not inferred JSON integers. Report drug arrays reached 19 rows and reaction arrays 22. Harmonized `package_ndc` arrays reached 2,087 entries and `spl_set_id` arrays 841. Those many identifiers do not establish which marketed product was administered. Device arrays happened to contain one row per captured report; narrative arrays ranged from empty to five. A single captured device never justifies a scalar schema.

The proposed drug row therefore carries native medicinal text/role/indication and identity collection counts. Full harmonized arrays move behind `get adverse-event ID identifiers --source faers --drug-index N --field unii|rxcui|product-ndc|package-ndc|spl-id|spl-set-id|application-number --section-limit N --section-offset N`. The index is the exact absolute native drug-array position in the checked retained report/version, never a name search. Other captured harmonization collections such as names/routes/substances/manufacturers/pharmacologic classifications need a bounded attributed component page before advertisement. The inventory determines supported selectors during type review. Arrays never silently collapse into one value.

The largest native five-report response was F08 at 498,926 bytes. It motivates bounded layers but supplies no measured envelope size or approved allocation. Identity-list options need native long-list/full-walk cases with exact report snapshot binding and no duplicate/gap assertions. Paging a checked retained response makes no new provider request. Type review must account explicitly for administrative/address fields no harvested case needs; this proposal does not label a legacy struct complete.

F19 native reaction buckets were malignant neoplasm progression 12016, death 5857, diarrhoea 5740, fatigue 5131 and off-label use 4758. F22 numeric term 2 had 59345 and term 1 had 18819. A numeric term is not a Boolean without its field codebook; their sum is not the F01 total. No risk or denominator is inferred. The JSON negative controls F18/D03 carried native `NOT_FOUND` and `No matches found!`; the denied HTML WONDER replies are different failures.

## Clinical rulings and harvested questions

A fresh clinical reviewer read the existing drug/disease rulings and official sources on October 25, 2026. These are faithful reformulations of published safety concerns, not invented verbatim clinician quotations or licensed curator answers. Each accepted answer below is an evidence boundary. Actual source counts and report IDs remain pending the bounded capture. A label can establish its own warning; a report query cannot reproduce a trial incidence or establish causality.

The reviewer confirmed with high confidence: FAERS, MAUDE and VAERS counts describe reports, not exposed populations or causal events; report drug/reaction arrays supply no pairwise causal attribution; source suspect/concomitant roles remain unvalidated reporter classifications; seriousness is separate from reaction outcome and CTCAE grade; empty and unavailable sources provide no clinical reassurance. Harm from violating these boundaries is high. The contrary reading, that more reports prove a more dangerous product, lacks an exposure denominator and ignores reporting bias/duplicates. A separately attributed suitable clinical study could support a risk estimate; a larger report count would not change this ruling.

HPV vaccine questions belong here only as cancer prevention. The reviewer confirmed that boundary from CDC guidance with high confidence. General vaccine research stays outside scope. Implant presence alone establishes neither breast cancer nor reconstruction. Capsule SCC, BIA-ALCL and breast carcinoma remain distinct source claims. A report narrative is not a source-coded diagnosis established by our software.

| Case | Question and published concern | Accepted answer and direct-source coverage |
|---|---|---|
| A01 | What reports mention pembrolizumab and myocarditis? L1/P1. | Preserve matching reports, terms and drug roles; label myocarditis warning is independent. F01/F02. |
| A02 | What reports mention pembrolizumab and pneumonitis? L1. | Preserve pneumonitis without merging pneumonia; no trial/report denominator mixture. F03. |
| A03 | What cardiomyopathy reports mention trastuzumab? L2. | Keep product identities distinct, including antibody conjugates. F04. |
| A04 | What fatal ILD/pneumonitis reports mention trastuzumab deruxtecan? L3. | Preserve reaction and death flags independently; do not substitute trastuzumab. F05 may be empty and does not yet filter fatality server-side. |
| A05 | What serious CRS reports mention axicabtagene ciloleucel? L4. | Keep named product and native seriousness; label warning does not establish reporting incidence. F06. |
| A06 | What neurologic toxicity reports mention axicabtagene ciloleucel? L4. | Preserve native terms; do not require simultaneous CRS. F07. |
| A07 | What pulmonary embolism reports mention tamoxifen in breast cancer care or risk reduction? L5. | Preserve stated indication; missing indication prevents assigning a population. F08. |
| A08 | What TLS reports mention venetoclax, including fatal outcomes? L6. | Preserve TLS/outcomes; do not infer dialysis, phase or malignancy. F09. |
| A09 | What gastrointestinal perforation reports mention bevacizumab? L7. | Return reported terms/outcomes without causal attribution. F10. |
| A10 | What hemorrhage reports mention ibrutinib, including reports marked as deaths? L8. | Preserve other drugs and roles; co-listing does not prove an interaction. F11. |
| A11 | What QT-prolongation reports mention osimertinib? L9. | Do not translate QT prolongation into torsades, arrhythmia or death. F12. |
| A12 | What ILD/pneumonitis reports mention osimertinib? L9. | Preserve chemotherapy/radiation context when stated; no pooling label populations. F23. |
| A13 | Which pembrolizumab–myocarditis reports are serious? F3/F4/L1. | Preserve native overlapping flags; no CTCAE grade assignment. Inspect F02, compare F13 only as a broader report query. |
| A14 | Which venetoclax–TLS reports record hospitalization, death or both? F3/F4/L6. | Preserve independent flags; no forced mutually exclusive outcome. Inspect F09; F13/F14 prove field forms on a different named query. |
| A15 | Which pembrolizumab–pneumonitis reports were first received during 2024? F3. | First FDA receipt is `receivedate`, not event onset or `receiptdate`. F15 measures the date filter on the broader drug query; the exact combined reaction/date query remains unmeasured. |
| A16 | Which ibrutinib–hemorrhage reports identify ibrutinib as suspect? F3/F4. | Role must belong to that drug object; source conjunction across arrays is insufficient. Inspect F11; F16 tests broader query mechanics only. |
| A17 | What matching trastuzumab/cardiomyopathy count and denominator support a percentage? F1–F3/L2. | Query/snapshot/unit required; no patient-risk percentage. F04 has a report total; F19/F22 measure aggregation on pembrolizumab, not this exact pair. |
| A18 | What breast-implant lymphoma reports record death in reconstruction? M1/M2. | Keep event type and stated indication; generic implant query does not establish lymphoma or reconstruction. D01/D02 supply candidates, not a complete answer. |
| A19 | What reports describe SCC in the capsule around breast implants? M3. | Preserve narrative text as such, distinct from BIA-ALCL and breast carcinoma. D03 is explicit text discovery; no locally inferred diagnosis. |
| A20 | Does a breast-implant report total represent unique patients/events? M1/M2. | State native report counting unit and duplicates; FDA curated BIA-ALCL cases differ from broad text matches. D01/D05. |
| A21 | What syncope reports follow HPV vaccination in cancer prevention? V1/V2. | Preserve native vaccine code/reaction rows; no causal claim or incidence. V01, C01/C02; HPV9 aggregate cannot answer every HPV formulation. |
| A22 | What serious allergic-reaction reports follow Gardasil 9? V1/V2. | Do not diagnose anaphylaxis from nonspecific symptoms or combine vaccines. V01/V02 are independent marginal tables, not a joint reaction-seriousness answer. |
| A23 | What can an empty exact cancer-drug/reaction/date query establish? F1/V1. | Only no matching records under those selectors/snapshot. F18 proves a deliberate nonexistent ID, not this exact clinical query. |
| A24 | Can timeout/refusal/unreadable data display zero reports? F1/M1/V1. | No; unavailable/incomplete is separate from a verified empty. Local controlled failures can prove product handling later; no provider failure is manufactured. |

### Clinical source catalog

Pinned label versions establish the cited warning. They are not claims that each linked label is the newest approved revision. Undated documentation was accessed October 25, 2026. The reviewer reread the cited passages and rejected causal or incidence interpretations.

| Key | Source, date/version and passage relied upon |
|---|---|
| F1 | [FDA FAERS dashboard presentation](https://www.fda.gov/files/drugs/published/%22The-FAERS-Public-Dashboard-and-its-Value-to-the-Pharmaceutical-Industry-%22-July-10--2018-Issue-PDF.pdf), July 10, 2018, p2: FAERS cannot calculate incidence. |
| F2 | [FDA postmarket surveillance explanation](https://www.fda.gov/drugs/cder-conversations/understanding-cders-postmarket-safety-surveillance-programs-and-public-data), undated: reports do not establish drug causation. |
| F3 | [openFDA field reference](https://open.fda.gov/fields/drugevent_reference.pdf), undated; [official field definitions](https://raw.githubusercontent.com/FDA/open.fda.gov/master/static/fields/drugevent.yaml): latest report version, unvalidated roles, distinct receipt dates and reaction outcome codes. |
| F4 | [FDA drug dashboard FAQ](https://fis.fda.gov/extensions/fpdwidgets/2e01da82-13fe-40e0-8c38-4da505737e36.html), undated: quarterly extract concomitant drugs differ from dashboard suspect-only view. |
| L1 | [KEYTRUDA label](https://www.accessdata.fda.gov/drugsatfda_docs/label/2025/125514s173lbl.pdf), June 2025, §§5.1/6.1: immune-mediated pneumonitis and fatal myocarditis. |
| L2 | [HERCEPTIN label](https://www.gene.com/download/pdf/herceptin_prescribing.pdf), May 2026, boxed warning: clinical/subclinical cardiac failure. |
| L3 | [ENHERTU label](https://www.accessdata.fda.gov/drugsatfda_docs/label/2025/761139s038s042lbl.pdf), December 2025, boxed warning: ILD/pneumonitis including fatal cases. |
| L4 | [YESCARTA label](https://www.fda.gov/media/193167/download), June 2026, boxed warning: fatal/life-threatening CRS and neurologic toxicity with or after CRS. |
| L5 | [Tamoxifen DailyMed label](https://dailymed.nlm.nih.gov/dailymed/lookup.cfm?setid=509f8ba3-214d-aec8-e063-6294a90af498&version=2), version 2, April 30, 2026, boxed warning: fatal pulmonary emboli in the stated risk-reduction/DCIS context. |
| L6 | [VENCLEXTA label](https://www.rxabbvie.com/pdf/venclexta.pdf), May 2026, §5.1: TLS including fatal events and dialysis. |
| L7 | [AVASTIN label](https://www.gene.com/download/pdf/avastin_prescribing.pdf), September 2022, §5.1: serious/fatal gastrointestinal perforation. |
| L8 | [IMBRUVICA label](https://www.rxabbvie.com/pdf/imbruvica_pi.pdf), October 2025, §5.1: fatal bleeding. |
| L9 | [TAGRISSO label](https://www.dailymed.nlm.nih.gov/dailymed/drugInfo.cfm?audience=professional&setid=5e81b4a7-b971-45e1-9c31-29cea8c87ce7), September 14, 2026, §§5.1–5.2: severe/fatal ILD/pneumonitis and QTc prolongation. |
| M1 | [FDA About MAUDE](https://www.fda.gov/medical-devices/mandatory-reporting-requirements-manufacturers-importers-and-device-user-facilities/about-manufacturer-and-user-facility-device-experience-maude-database), undated: MDR data cannot evaluate adverse-event rates. |
| M2 | [FDA BIA-ALCL reports](https://www.fda.gov/medical-devices/breast-implants/medical-device-reports-breast-implant-associated-anaplastic-large-cell-lymphoma), tables through June 30, 2024: incomplete information can prevent duplicate removal. |
| M3 | [FDA implant risks](https://www.fda.gov/medical-devices/breast-implants/risks-and-complications-breast-implants), December 14, 2023: literature and MDR reports describe capsule SCC. |
| V1 | [VAERS interpretation guide](https://vaers.hhs.gov/data/dataguide.html), May 8, 2025 public-data expansion: added secondary reports concern already reported events; reporting does not document vaccine causation. Current data/guide pages differ on exact secondary-file inclusion. Native tier/version must settle the applicable count unit. |
| V2 | [CDC HPV safety/effectiveness](https://www.cdc.gov/hpv/hcp/vaccination-considerations/safety-and-effectiveness-data.html), July 9, 2024: vaccination prevents cancer-causing infections and cervical precancers. |
| P1 | [Salem et al.](https://escholarship.org/content/qt66j3w519/qt66j3w519.pdf), Lancet Oncology 19(12), December 2018, author manuscript p9: VigiBase cannot establish cardiovascular immune-event incidence. Disproportionate reporting and fatality among reported cases are not exposed-patient risk. |

## Identity and native semantics

Keep FAERS `safetyreportid` as an opaque string and its source `safetyreportversion` separately. Do not parse a checksum or fabricate a case-family ID from inconsistent overview prose. Exact get verifies the returned ID and, when requested, version before cache admission. MAUDE uses its own opaque `mdr_report_key`, with `report_number` retained separately. Numeric report IDs never trigger cross-source probing. Require `--source faers|maude` for get; no speculative FAERS-then-MAUDE resolver.

Each FAERS drug object retains the original `medicinalproduct`, role code, indication, administration fields and optional harmonized identity arrays. Distinct objects do not merge by name. An exact source-provided UNII, RxCUI, NDC or SPL cross-reference may offer the corresponding existing drug command only after that command's accepted namespace and granularity are checked. An ingredient UNII does not identify a marketed product, application or package. Missing/multiple mappings remain missing/candidates. Literal name search returns reports/candidates, never a chosen drug identity. The source report title must not choose the first suspect or first listed drug as the whole report's drug.

FAERS reactions remain an independent ordered array. A role filter must validate the selected matching drug object rather than assuming an indexed conjunction binds nested objects. Captures must settle whether an exact identity query supports that check. Name-only queries cannot acquire a trustworthy suspect-only promise from a report-level search expression.

Official reaction outcomes at last observation are `1 recovered/resolved`, `2 recovering/resolving`, `3 not recovered/not resolved`, `4 recovered/resolved with sequelae`, `5 fatal`, `6 unknown`. Unknown/missing are distinct. Report seriousness flags can overlap. The legacy mapping of hospitalization to reactionoutcome1 and disability to3 is wrong. Use report seriousness hospitalization/disabling instead. Neither field is a CTCAE grade. `receivedate` is first FDA receipt; `receiptdate` is receipt of the most recent information; neither is onset. Preserve dates with their field roles and native formatting evidence.

A count row is a native aggregation bucket, not a distinct person. Never sum reaction buckets into a report denominator or add seriousness marginals into a total. WONDER reaction/seriousness/age tables stay independent; never use the maximum total from separate queries as a fabricated common denominator. Preserve each native table's own total, vintage and caveats. A CVX code is a vaccine administration code, not proof of a WONDER family mapping. Native exact mappings need explicit provenance. No regexes, token extraction, synonym rewriting, inferred ontology categories or hidden label lookup occurs.

## Command proposal and disclosure

This command tree is proposed for PM review. Shared `--format`, `--disable-sources`, cache, record/replay and MCP behavior stay in the existing shared contract. No command silently fans out across sources. `--source` is required while this entity spans report and aggregate types. Explicit source text is sent unchanged after source-literal escaping; the reply echoes the exact resulting source expression. Exact native report identity is separate from a text query.

| Command and options | Header/default result | Drill-down and next actions | Harvest/case obligations |
|---|---|---|---|
| `search adverse-event TEXT --source faers --limit N --offset N` | One bounded report page; source/query/date, total/unit, report IDs/versions, native seriousness, short drug/reaction headers and explicit truncation. No chosen causal drug. | Exact get for every report; next source page preserves selectors. | A01–A17, A23; explicit positive/empty/page/window and CLI/MCP parity cases. |
| `search adverse-event --query TEXT --source faers|maude --limit N --offset N` | Caller-written source expression unchanged; no semantic classification. Positional TEXT and raw query are mutually exclusive. | Exact report get/page; no cross-provider fallback. | F01–F23/D01–D05. Raw query permits source-native filtering without a new local filter vocabulary. |
| `search adverse-event TEXT --source maude --limit N --offset N` | Literal device-name candidate reports; native event type, key, dates, complete reachable device list. No diagnosis extracted from narrative. | Exact get and next report page. | A18–A20; positive device identity and source field mapping need captures. |
| `get adverse-event ID --source faers` | Exact report/version, receipt dates by role, overlapping seriousness flags, patient fields only if supplied, collection counts and section actions. `--version` selects explicit source version only if endpoint proves it. | `drugs`, `reactions`, `reporter`; every array remains reachable by named section. | F21, A01–A17; wrong ID/version before cache, missing versus empty, multiple report versions. |
| `get adverse-event ID drugs|reactions --source faers --section-limit N --section-offset N` | Independent ordered drug objects or reaction objects. Drug role/indication and identity collection counts stay with their own row; reaction outcome stays with its reaction. Full identity arrays use the separate bounded identifiers section. | All distinct verified drug-ID pivots before independent next section page. No name-derived medical advice. | A04/A07/A08/A10/A13/A14/A16; preserve optionality, multiplicity and collection scope. |
| `get adverse-event ID identifiers --source faers --drug-index N --field FIELD --section-limit N --section-offset N` | One exact drug row and named harmonized identity collection, with native total and no implied administered-product equivalence. | Next collection page retains report/version/drug index/field; exact external action only if namespace/granularity is checked. | Long native package-NDC/SPL lists; all field selectors, wrong drug index, absent/multiple mappings and complete walk. |
| `get adverse-event ID reporter --source faers` | Native reporter qualification/country and source fields, no identifying reconstruction. | Source report link only where source proves its URL format. | A24 plus native available-field proof; unproved fields do not create sections. |
| `get adverse-event ID --source maude` | Exact key, report number, source dates by role, native event type, device/patient counts with stated units. | `devices`, `narrative`; source link if proved. | D05/A18–A20; duplicate reports, multiple devices and exact-key refusal. |
| `get adverse-event ID devices|narrative --source maude --section-limit N --section-offset N` | Full native device rows or narrative entries; no silent excerpt loss. Keep optional native UDI/product code and text type. | Next independent row page. Long individual text requires an explicit bounded part mechanism before acceptance, never silent truncation. | A18/A19; narrative size and fields must be source-proved before advertised. |
| `adverse-event count --source faers --query TEXT --field reaction|seriousnessdeath --limit N` | Native bucket terms/counts and exact query; no invented total or calculated percentage. | Source query link; no fake offset continuation for a top-N-only endpoint. | F19/F22/A17; count-unit and bucket-overlap cases. Other legacy aggregation fields are not approved by this harvest. |
| `adverse-event summary HPV9 --source vaers --group reactions|seriousness|age --limit N --offset N` | One explicit aggregate table, native total, vintage, code/labels and notices. Does not join marginal tables or select a vaccine by text. | Page within one retained complete source table; no invented server cursor. | A21/A22/V01–V03; note these marginals do not answer the joint serious-allergy query. |
| `search adverse-event TEXT --source cvx --limit N --offset N`; `get adverse-event CVX --source cvx` | Explicit vaccine candidate code table or exact code; original names/status/date and source-related trade rows. Never a report. | Exact CVX card and relation pages only; no VAERS pivot until a source gives a code relation. | C01/C02/A21/A22; ambiguity, inactive code, absent mapping, no hidden download. PM should consider a distinct vaccine entity name instead. |
| `sync cvx` | Explicit bounded atomic publication of the two approved code files; counts, revision and publisher dates retained distinctly. | No automatic source fetch from a read; failed replacement retains previous complete bundle. | C01/C02; publication after code acceptance through a separate build delegate, never part of normal per-query work. |

The CVX placement is an explicit unresolved naming choice: vaccine identity is not an adverse report. Preferred PM default is to defer CVX runtime commands until an observed native mapping answers a harvested question. Keep its source investigation and exact legacy accounting here. Do not implement the provisional CVX tree merely because a bundle exists. Similarly, the VAERS summary remains proposed only if the public aggregate tier answers a retained question without name inference. No three-table all-in-one default is proposed.

### Proposed type boundaries

`biodata` holds neutral no-I/O identifiers, coded values, dates with source roles, native counts and attributed records. `biomcp-types` owns queries, source selectors, cards/pages and typed next actions. Schema derives from those response types behind the existing `schema` feature. No schema is hand-written in this investigation. Proposed names below identify distinct response families; they are not generated schemas or executed JSON.

| Type/schema family | Proposed fields and limits of meaning |
|---|---|
| `AdverseReportId` | Tagged FAERS ID/version or MAUDE key; opaque original string, no numeric source inference. |
| `ReportedDrug` | Native row index, medicinal product text, optional reported role/indication/administration fields, attributed harmonized identity arrays. No inferred cross-source product equivalence. |
| `ReportedReaction` | Native row index, source term, optional outcome code/label and source vocabulary/version when returned. No local reaction normalization or invented MedDRA ID. |
| `ReportedSeriousness` | Optional native serious code plus independent optional death/hospitalization/life-threatening/disabling/congenital/other flags. Absent does not default to false. |
| `ReportedDevice` | Native row index, optional names/manufacturer/product code/UDI and source-supplied relations. No inferred cancer indication. |
| `ReportDate` | Exact native value plus field role; parsing only with source format proof. First receipt, latest receipt and event onset remain separate. |
| `adverse_report_page/1` | Source-specific report headers, native total/count unit, query selectors, page window and all exact get actions. No report becomes a unique patient. |
| `adverse_faers_card/1`, `adverse_maude_card/1` | Distinct exact-identity cards and section counts; reuse existing Envelope/Section/PageInfo/Problem/SourceOutcome. No union of incompatible clinical meanings. |
| `adverse_drug_page/1`, `adverse_reaction_page/1`, `adverse_device_page/1`, `adverse_narrative_page/1`, `adverse_identifier_page/1` | Complete reachable independent source rows, optional fields, collection totals and typed section continuation. No unbounded default arrays. |
| `adverse_count/1` | Exact query, native count field, terms/counts and available source metadata. No bucket sum promoted to report count. |
| `adverse_vaers_table/1` | Exact native vaccine selector and one grouping, native rows/total/unit/vintage/caveats/suppression. No combined denominator or individual-report claim. |

Type review must inventory every captured field as retained, behind a section, or intentionally omitted with a reason. It must compare missing/null/scalar/list, preserve ordered multiplicity, and verify the public tier actually carries every advertised section. Legacy structs are incomplete source evidence and do not define the new schema. Clinical interpretation belongs to no adapter or response type. Optional unknown codes retain native strings rather than being mapped into an invented closed clinical set.

### Delight questions

1. The first question names a drug and reported reaction. One explicit FAERS query returns source reports and exact report actions. A label warning remains a separate explicit drug section, never hidden extra work.
2. The smallest useful header states source, exact query, report ID/version, receipt dates, seriousness and counted unit. Full drug/reaction/narrative collections stay behind sections with counts.
3. Next actions expose every report and every checked identity choice, independent page continuation, and available source sections. A global action cap cannot hide later reports or verified identity choices.
4. Exact report IDs, versions, harmonized drug IDs and device keys return without namespace invention. A native ingredient ID cannot silently become a product identity.
5. Sources disagree by remaining separate source facts. Report counts and label trial percentages never share an unlabeled numeric field or denominator.
6. Empty, disabled, missing key, malformed, timeout and unavailable remain separate source outcomes. Stale accepted data with a failed refresh carries the existing Problem contract; failure never becomes zero.
7. Cold/warm costs are unmeasured until actual product commands exist. The capture ledger measures source HTTP duration and requests only; cache behavior requires later outside-in command cases.
8. Legacy name bridges, automatic trial fallback, synthetic guidance, wrong outcome aliases and calculated count percentages do not answer this bank safely. Exact identities, all row collections and honest source failure states improve its answers.
9. The agent can stop when a source report answers the stated report question, with query/date/unit/source link. It cannot stop with a clinical safety conclusion from an empty search or a count.
10. Clinicians need source and retrieval date, report/version identity, exact terms and limits. A source-coded suspect role and a label warning retain different provenance.

### Budget and comparison

No product output has been executed or synthesized. Therefore no default/section wire budget is measured or approved here. No timings, tokens or clinical parity are invented. Native raw-body bytes and timing from the bounded ledger will be reported separately and cannot substitute for wire output measurements. The allocation decision must follow the first captured command candidates; implementation must measure compact UTF-8 JSON for the complete envelope, separately show any exact source-terms comparison deduction, then bind each case to its measured size plus the approved margin. Estimated tokens use an explicitly stated estimator; no estimated figure is called tokenizer output.

| Proposed layer | Measurement state | Required before command acceptance |
|---|---|---|
| FAERS/MAUDE default report page | Unmeasured; no implementation | Full envelope at ordinary default and explicit page, real native cases, independent source totals. |
| Exact FAERS/MAUDE default card | Unmeasured | Complete header, overlapping flags, optional fields and all section actions. |
| Drugs/reactions/devices/narrative/identifiers section | Unmeasured | Ordinary default, explicit page and largest native row; no silent text loss. |
| FAERS count | Unmeasured | Both approved native bucket shapes and explicit top-N bound. |
| VAERS one-group table | Unmeasured | Three independent tables, native caveats and no combined totals. |
| CVX candidate/card/sync | Deferred runtime choice; unmeasured | First prove an observed native relation and unique harvested need. |
| Legacy 0.9 same-question JSON | Unmeasured | Execute only behind the verified network containment used by 0136, with exact old executable revision/hash and recorded physical attempts. No containment proof is present for this assignment. |

Do not run legacy commands directly: they may retry, follow alternate names, auto-download CVX or fall back to ClinicalTrials.gov. Source captures alone do not establish containment. If the separate delegate cannot reuse and verify the existing isolation recipe with all egress denied except the recorder, the comparison remains unmeasured with this reason. Old fixture outputs are not new runs. No retrospective timing or fabricated byte count is permitted.

## Complete legacy command and option accounting

This is source inspection at the pinned legacy revision. Dispositions below are proposals for PM review, not approvals or completed ports. Both search and `drug adverse-events` repeat many filters; the shared rows account for each explicitly. A dropped option returns an actionable usage error in future retirement cases and makes no source request.

| Legacy surface | Proposal and reason | Cases/limits |
|---|---|---|
| `search adverse-event` | Keep bounded one-source report search. | A01–A24; sources never auto-merge. |
| Positional `QUERY`, `-d/--drug` | Keep positional literal report search; retire duplicate drug flag aliases if typed tree uses one input. | A01/F01; explicit query is not identity. |
| `--reaction` in search/helper | Replace with unchanged explicit `--query` source expression until exact known-positive source cases justify a convenience filter. | A01–A12; no synonym rewriting. |
| `--outcome death/fatal` in search/helper | Replace with explicit native reactionoutcome5 or report seriousnessdeath, chosen by caller. | A04/A08/A10; roles remain distinct. |
| `--outcome hospitalization/hospitalisation/disability` in search/helper | Drop incorrect mappings. Legacy maps these to reactionoutcome1/3, which mean recovered/resolved and not recovered/not resolved. | A14; native report hospitalization/disabling fields only. |
| `--serious [any]` in search/helper | Replace with explicit source serious1 expression, no severity/grade implication. | A13. |
| Seriousness death, hospitalization/hospitalisation, lifethreatening/life-threatening, disability/disabling, congenital/congenital_anomaly, other | Preserve source-native fields via raw query; no new aliases until each convenience filter has a positive case. | F13/F14 prove only two source forms; other values remain unmeasured. |
| `--date-from`, alias `--since`, in search/helper | Replace with explicit source range; future convenience name must say first receipt. | A15/F15. Year expansion is retired unless separately justified. |
| `--date-to`, alias `--until`, in search/helper | Same, independent inclusive endpoint; reject reversed interval before I/O. | A15; MAUDE legacy rejected upper bound, new raw query can express it explicitly. |
| `--suspect-only` in search/helper | Drop unsafe cross-object promise; retain each drug's own role. | A16/F16; exact identity and nested scope need proof. |
| `--sex m/male/f/female` in search/helper | Raw native patientsex expression; no convenience aliases initially. | F17 mechanics only, not an invented clinical demographic question. |
| `--age-min`, `--age-max` in search/helper | Drop unitless ages. Raw native query can explicitly constrain patientonsetageunit. | F17; years must never be assumed. |
| `--reporter` physician/doctor/pharmacist/other_health_professional/health-professional/lawyer/consumer/patient | Raw native qualification expression; no convenience aliases initially. | F17; no locally inferred reporter profession. |
| `--count reaction/reactionmeddrapt/patient.reaction.reactionmeddrapt[.exact]` in search/helper | Rename to count command, field reaction, exact native bucket. | F19/A17. |
| Count medicinalproduct[.exact], openfda.generic_name[.exact], openfda.brand_name[.exact] | Drop initial convenience buckets; no harvested identity question needs bucketed names. Raw source query does not add arbitrary count fields. | Exact original full fields include `patient.drug.` prefix. |
| Count patientsex, patientonsetage, serious, seriousnessdeath/hospitalization/lifethreatening/disabling/congenitalanomali/other, reactionoutcome, primarysource.qualification | Keep only seriousnessdeath as second measured count shape; defer other buckets absent a unique harvested question and unit proof. | F22; percentages never calculated. |
| Fake count total/meta.total/results.total/meta.results.total | Preserve explicit refusal. Native `meta.results.total` is metadata, not an aggregation field. | Retirement usage case, zero I/O. |
| `--type faers` | Rename to explicit `--source faers`. | A01–A17. |
| `--type device/devices/maude` | Rename to explicit `--source maude`. | A18–A20. |
| `--type recall/recalls/enforcement`, `--classification` | Drop from adverse-event entity; recall ownership needs separate PM decision. | No recall capture or parity claim. |
| `--source faers/vaers/all`, default all | Keep explicit separate sources; drop all and automatic vaccine detection. | No static name bridge, merged output or hidden fan-out. |
| `--device` | Replace with literal MAUDE search; original wildcard expansion is dropped. | D01–D04. |
| `--manufacturer`, `--product-code` | Raw exact source fields initially; no name wildcard or inferred identity. | Captures may expose exact code/manufacturer; no positive filter promise yet. |
| Device serious any/death/injury | Raw event_type expression; no borrowing FAERS code values. | D02 proves death only; any=death-or-injury is not a universal seriousness meaning. |
| `-l/--limit`, `--offset` in search/helper | Keep long page selectors; no short alias proposed. VAERS pages a retained complete table, not source skip. Count has no offset. | F20 and local bounds/window tests. |
| `get adverse-event REPORT_ID` | Keep with required source and optional proved version. Drop speculative numeric-source resolution. | F21/D05. |
| Get `reactions` | Keep independent reaction page including native outcome. | A01–A17. |
| Get `outcomes` | Rename to card seriousness plus each reaction's own outcome; do not mislabel flags as reaction outcomes. | A14. |
| Get `concomitant` | Rename to complete drugs page preserving every role; no name selection. | A10/A16. |
| Get `guidance` | Drop generated text-derived drug/indication guidance. Existing drug actions need exact source IDs. | No diagnostic/treatment inference. |
| Get `all` | Drop unbounded all-sections output. | Named pages retain complete collections. |
| `drug adverse-events NAME` | Drop duplicate helper and its FAERS404→ClinicalTrials.gov fallback. Typed exact drug-ID pivot may be added only after native identity join proof. | Existing drug ledger already marks adverse-event ownership; no duplicate leaf. |
| Drug card `top_adverse_events` and `faers_query` enrichment | Drop hidden name-based aggregate calls from a default drug card; explicit source-ID pivot requires proved mapping. | Existing drug ledger owns the card; A17 does not justify unconditional enrichment. |
| Helper `--type`, `--count`, all filter/page flags above | Same explicit dispositions as their search counterparts, including unused/misleading type routes. | No orphan option retained because helper existed. |
| `cvx sync` | Investigate explicit two-file sync; defer runtime pending unique need. No automatic reads/downloads or MVX fetch. | C01/C02. |
| VAERS summary total/seriousness/age/top reactions and source outcomes | Keep separate native tables if adopted; drop maximum-of-three-totals denominator, local percentages, name matching and automatic combined branch. | V01–V03/A21/A22; no per-report get advertised. |
| Global `--json/-j`, Markdown sections and section-parser acceptance of trailing json flags | Use existing `--format json|md`; schema/CLI/MCP share typed tree. No section parser accepts format flags as clinical sections. | Existing shared formatting contract plus generated help cases. |

## Structural risks and case requirements

The main risks are restated source semantics, nested collection scope, isolated source evidence and unbounded output. Define source facts once in raw/checked types; generate schema/help; never maintain separate CLI/MCP interpretation. Use existing transport, cache, Problem and source registry rather than new per-adapter clients. Test cases use isolated homes/config/caches and retained exact recordings. A wrong report identity fails before cache admission. No shared temp folder owns durable evidence. Retain raw CLI stdout/stderr, raw MCP JSON-RPC stdout and real recorder files outside temporary test environments before validation can fail.

Every proposed command and option needs an outside-in CLI/MCP case: source-positive literal/raw/exact get; missing optional fields; repeated drugs/reactions/devices; overlapping seriousness; role on another drug; wrong ID/version; missing/multiple joins; no-match; 429/500/timeout/broken200; disabled source; stale accepted data plus rejected refresh; ordinary page/full walk/window; count without fake total; separate vaccine marginals; explicit missing code mapping; no read-triggered sync; failed publication retains old bundle. Synthetic malformed/failure replies are labeled controls, never source observations. Known-positive filters must assert direct captured facts on the capture date. The bounded sample does not currently prove every possible filter or complete clinical parity.

## Ian's questions

No clinical or configuration question goes to Ian. PM should choose the command surface, source categories and retained scope within the approved slice outcome. The working recommendation is one-source report search/get plus exact native fields, with CVX runtime deferred until a source-proved mapping answers a question. The alternative adds an explicit vaccine identity tree now; it adds bundle publication/schema/case ownership without a proved report join. A joint VAERS serious-allergy query is another separate proposal requiring a new bounded capture, not an inferred answer from marginal tables. No paid service, publication, external commitment or production data change is proposed. If any becomes necessary, the PM must escalate the concrete options and costs under the workspace approval rule.
