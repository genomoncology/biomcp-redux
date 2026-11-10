---
title: biomcp search trial
description: "The BioMCP 1.0.0 command biomcp search trial: Finds clinical
  trials in ClinicalTrials.gov, the default, or NCI, by condition, drug, gene,
  location and more."
goal: Look up the arguments, sections and examples of biomcp search trial.
reader: A BioMCP user or agent checking one command
sidebar:
  label: search trial
kind: reference
verified: 2026-11-10
---

`biomcp search trial` is a command of BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources. It finds clinical trials in ClinicalTrials.gov, the default, or NCI, by condition, drug, gene, location and more. This page lists its arguments, examples and reply.

The program's help describes it:

> Search clinical trials. Sources: ctgov, nci.

## Usage

```text
biomcp search trial [OPTIONS] [query]
```

## Arguments

| Argument | Type | Required | Limits | Choices | Description |
| --- | --- | --- | --- | --- | --- |
| `--age` | integer | no | min 0 |  | Age in years. ClinicalTrials.gov only; unstated source bounds count as no limit. Value form: integer. Example: --age 18. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--biomarker-code` | string | no |  |  | NCI NCIt code. Exact NCIt biomarker concept listed by NCI in trial criteria; NCI source only. This listing does not establish biomarker positivity or patient eligibility. Value form: string. Example: --biomarker-code C96022. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--biomarker-use` | string | no |  | `inclusion`, `exclusion` | NCI criterion; help search trial NCI criterion. Filter by NCI's source inclusion or exclusion criterion. Check each result's biomarker entry and reported problems. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --biomarker-use inclusion. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--condition` | array | no | max 5 |  | Disease; NCI needs NCIt. Conditions joined with OR. NCI needs NCIt codes such as C3224; use --condition-id for MONDO or DOID identifiers. Value form: repeatable values (repeat the flag). Example: --condition melanoma. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--condition-id` | string | no |  |  | MONDO/DOID/NCIT:C. Exact MONDO or DOID CURIE resolved by MyDisease; NCI also accepts explicit NCIT:C codes. Value form: string. Example: --condition-id NCIT:C3224. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--count-only` | boolean | no |  |  | Total. Source total only; no returned study rows. Value form: boolean. Example: --count-only. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--country` | array | no | max 5 | `Afghanistan`, `Aland Islands`, `Albania`, `Algeria`, `American Samoa`, `Andorra`, `Angola`, `Anguilla`, `Antarctica`, `Antigua and Barbuda`, `Argentina`, `Armenia`, `Aruba`, `Australia`, `Austria`, `Azerbaijan`, `Bahrain`, `Bangladesh`, `Barbados`, `Belarus`, `Belgium`, `Belize`, `Benin`, `Bermuda`, `Bhutan`, `Bolivia`, `Bonaire, Saint Eustatius and Saba `, `Bosnia and Herzegovina`, `Botswana`, `Brazil`, `Brunei`, `Bulgaria`, `Burkina Faso`, `Burma`, `Burundi`, `Cabo Verde`, `Cambodia`, `Cameroon`, `Canada`, `Cayman Islands`, `Central African Republic`, `Chad`, `Chile`, `China`, `Christmas Island`, `Colombia`, `Comoros`, `Costa Rica`, `Croatia`, `Cuba`, `Curacao`, `Cyprus`, `Czechia`, `Côte d’Ivoire`, `Democratic Republic of the Congo`, `Denmark`, `Djibouti`, `Dominica`, `Dominican Republic`, `Ecuador`, `Egypt`, `El Salvador`, `Equatorial Guinea`, `Eritrea`, `Estonia`, `Eswatini`, `Ethiopia`, `Faroe Islands`, `Federal Republic of Yugoslavia`, `Fiji`, `Finland`, `France`, `French Guiana`, `French Polynesia`, `French Southern and Antarctic Lands`, `Gabon`, `Georgia`, `Germany`, `Ghana`, `Gibraltar`, `Greece`, `Greenland`, `Grenada`, `Guadeloupe`, `Guam`, `Guatemala`, `Guinea`, `Guinea-Bissau`, `Guyana`, `Haiti`, `Holy See`, `Honduras`, `Hong Kong`, `Hungary`, `Iceland`, `India`, `Indonesia`, `Iran`, `Iraq`, `Ireland`, `Israel`, `Italy`, `Jamaica`, `Japan`, `Jersey`, `Jordan`, `Kazakhstan`, `Kenya`, `Kiribati`, `Kosovo`, `Kuwait`, `Kyrgyzstan`, `Laos`, `Latvia`, `Lebanon`, `Lesotho`, `Liberia`, `Libya`, `Liechtenstein`, `Lithuania`, `Luxembourg`, `Macau`, `Madagascar`, `Malawi`, `Malaysia`, `Maldives`, `Mali`, `Malta`, `Martinique`, `Mauritania`, `Mauritius`, `Mayotte`, `Mexico`, `Micronesia`, `Moldova`, `Monaco`, `Mongolia`, `Montenegro`, `Montserrat`, `Morocco`, `Mozambique`, `Namibia`, `Nepal`, `Netherlands`, `Netherlands Antilles`, `New Caledonia`, `New Zealand`, `Nicaragua`, `Niger`, `Nigeria`, `Niue`, `North Korea`, `North Macedonia`, `Northern Mariana Islands`, `Norway`, `Oman`, `Pakistan`, `Palestinian Territories`, `Panama`, `Papua New Guinea`, `Paraguay`, `Peru`, `Philippines`, `Poland`, `Portugal`, `Puerto Rico`, `Qatar`, `Republic of the Congo`, `Reunion`, `Romania`, `Russia`, `Rwanda`, `Saint Kitts and Nevis`, `Saint Lucia`, `Saint Martin`, `Saint Vincent and the Grenadines`, `Samoa`, `San Marino`, `Saudi Arabia`, `Senegal`, `Serbia`, `Serbia and Montenegro`, `Seychelles`, `Sierra Leone`, `Singapore`, `Slovakia`, `Slovenia`, `Solomon Islands`, `Somalia`, `South Africa`, `South Korea`, `South Sudan`, `Spain`, `Sri Lanka`, `Sudan`, `Suriname`, `Sweden`, `Switzerland`, `Syria`, `Taiwan`, `Tajikistan`, `Tanzania`, `Thailand`, `The Bahamas`, `The Gambia`, `Timor-Leste`, `Togo`, `Trinidad and Tobago`, `Tunisia`, `Turkey (Türkiye)`, `Turkmenistan`, `Uganda`, `Ukraine`, `United Arab Emirates`, `United Kingdom`, `United States`, `United States Minor Outlying Islands`, `Uruguay`, `Uzbekistan`, `Vanuatu`, `Venezuela`, `Vietnam`, `Virgin Islands`, `Yemen`, `Zambia`, `Zimbabwe` | Country label; help search trial Country label. Exact ClinicalTrials.gov country label; repeat for OR. Trials must list a site in a named country. NCI uses its source country filter. Listed choices accept any letter case; replies use their listed spelling. Value form: repeatable values (repeat the flag). Example: --country 'United States'. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--criteria` | string | no |  |  | Criteria text. ClinicalTrials.gov only: search exact registry eligibility text for discovery; no eligibility judgment. Value form: string. Example: --criteria 'exact criteria'. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--distance` | string | no |  |  | Radius, mi or km. ClinicalTrials.gov only: source distance with explicit mi or km units. Value form: string. Example: --distance 50mi. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--exact-condition` | boolean | no |  |  | Exact condition. Ask ClinicalTrials.gov to restrict matching to the registered condition phrase; its support is partial. Value form: boolean. Example: --exact-condition. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--exact-term` | boolean | no |  |  | Search typed treatment terms without resolving their other names. Value form: boolean. Example: --exact-term. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--facility` | string | no |  |  | Facility. Facility phrase; ClinicalTrials.gov broadens after zero exact matches. NCI matches exact names. Cannot combine with radius. Value form: string. Example: --facility 'Dana-Farber Cancer Institute'. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--gene` | string | no |  |  | Gene symbol/ID. ClinicalTrials.gov only: resolve one checked gene for registry text discovery; no eligibility judgment. Value form: string. Example: --gene BRAF. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--has-results` | boolean | no |  |  | Posted results. ClinicalTrials.gov only: require posted source results. Value form: boolean. Example: --has-results. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--id` | string | no |  |  | NCT identifier. ClinicalTrials.gov only: exact NCT identity in a conjunction with other filters. Value form: string. Example: --id NCT05273554. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--intervention` | array | no | max 5 |  | Treatment terms; OR. Intervention alternatives joined with OR; supported by ClinicalTrials.gov. Value form: repeatable values (repeat the flag). Example: --intervention pembrolizumab. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--keyword` | array | no | max 5 |  | Text; OR. Repeat for ClinicalTrials.gov general text alternatives joined with OR; different filters combine with AND. NCI accepts one keyword. Value form: repeatable values (repeat the flag). Example: --keyword survival. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--lat` | string | no |  |  | Latitude degrees. ClinicalTrials.gov only: radius center latitude in degrees. Value form: string. Example: --lat 42.3601. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--limit` | integer | no | min 1, max 100 |  | Page size. Headers per page; NCI applies its own lower publisher limit. Value form: integer. Example: --limit 1. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--lon` | string | no |  |  | Longitude degrees. ClinicalTrials.gov only: radius center longitude in degrees. Value form: string. Example: --lon -71.0589. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--next-page` | string | no |  |  | Page token. ClinicalTrials.gov: follow the returned token, selectors and offset unchanged; unbound tokens need a fresh first page. Value form: string. Example: --next-page &lt;next-page&gt; (syntax; replace the placeholder with your retained value). Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--offset` | integer | no | min 0 |  | Page offset. Zero-based offset of the first row in this continuation page. Value form: integer. Example: --offset 0. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--phase` | string | no |  | `early-1`, `1`, `2`, `3`, `4`, `not-applicable`, `1/2`, `2/3`, `NA`, `0`, `I`, `I_II`, `II`, `II_III`, `III`, `IV` | Source phase; help search trial Source phase. Exact source phase. ClinicalTrials.gov: 1, 2, 3, 4, early-1, not-applicable, 1/2, 2/3. NCI: NA, 0, I, I_II, II, II_III, III, IV. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --phase 3. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--sex` | string | no |  | `ALL`, `FEMALE`, `MALE` | ClinicalTrials.gov only: registry sex facet; help search trial ClinicalTrials.gov only: registry sex facet. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --sex ALL. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--source` | string | no |  | `ctgov`, `nci` | clinicaltrials/nci; help search trial clinicaltrials/nci. Explicit source; defaults to ClinicalTrials.gov and never falls back. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --source nci. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--sponsor` | string | no |  |  | Sponsor. ClinicalTrials.gov only: sponsor name; broadens after zero exact matches. Use --sponsor-type for the funding category. Value form: string. Example: --sponsor 'MD Anderson'. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--sponsor-type` | string | no |  | `nih`, `fed`, `industry`, `other` | Funding class; help search trial Funding class. ClinicalTrials.gov only: registry sponsor funding category; use --sponsor for a sponsor name. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --sponsor-type other. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--status` | string | no |  | `ACTIVE_NOT_RECRUITING`, `COMPLETED`, `ENROLLING_BY_INVITATION`, `NOT_YET_RECRUITING`, `RECRUITING`, `SUSPENDED`, `TERMINATED`, `WITHDRAWN`, `AVAILABLE`, `NO_LONGER_AVAILABLE`, `TEMPORARILY_NOT_AVAILABLE`, `APPROVED_FOR_MARKETING`, `WITHHELD`, `UNKNOWN`, `Active`, `Administratively Complete`, `Approved`, `Closed to Accrual`, `Closed to Accrual and Intervention`, `Complete`, `INACTIVE`, `Enrolling by Invitation`, `Temporarily Closed to Accrual`, `Temporarily Closed to Accrual and Intervention`, `Withdrawn` | Source status; help search trial Source status. Exact source status. NCI: Active, Administratively Complete, Approved, Closed to Accrual, Closed to Accrual and Intervention, Complete, INACTIVE, Enrolling by Invitation, Temporarily Closed to Accrual, Temporarily Closed to Accrual and Intervention, Withdrawn. ClinicalTrials.gov uses registry labels. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --status RECRUITING. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--study-type` | string | no |  | `INTERVENTIONAL`, `OBSERVATIONAL`, `EXPANDED_ACCESS` | Study type; help search trial Study type. ClinicalTrials.gov only: exact registry study type. Listed choices accept any letter case; replies use their listed spelling. Value form: string. Example: --study-type INTERVENTIONAL. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--updated-since` | string | no |  |  | First date. ClinicalTrials.gov only: first included registry update date. Value form: string. Example: --updated-since &lt;updated-since&gt; (syntax; replace the placeholder with your retained value). Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--updated-until` | string | no |  |  | Last date. ClinicalTrials.gov only: last included registry update date. Value form: string. Example: --updated-until &lt;updated-until&gt; (syntax; replace the placeholder with your retained value). Command sources: ctgov, nci; source restrictions in the field description apply. |
| `--variant` | string | no |  |  | Variant text. ClinicalTrials.gov only: exact variant text for registry discovery; no eligibility judgment. Value form: string. Example: --variant L576P. Command sources: ctgov, nci; source restrictions in the field description apply. |
| `query` (position 1) | string | no |  |  | Trial text. General registry discovery text. |

## Sources

`--source` picks the source to ask: [ctgov](/sources/clinicaltrials-gov/), [nci](/sources/nci/).

## Examples

The program's help gives these examples.

```sh
biomcp search trial --condition melanoma
```

```sh
biomcp search trial --source nci --facility 'Dana-Farber Cancer Institute' \
  --limit 1
```

```sh
biomcp search trial --condition melanoma --intervention pembrolizumab
```

```sh
biomcp search trial --condition melanoma --condition 'lung cancer' \
  --intervention pembrolizumab --intervention nivolumab --keyword survival \
  --keyword 'quality of life' --limit 1
```

```sh
biomcp search trial --condition melanoma --age 18
```

```sh
biomcp search trial --condition melanoma --lat 42.3601 --lon -71.0589 \
  --distance 50mi
```

```sh
biomcp search trial --gene BRAF --limit 4
```

```sh
biomcp search trial --gene BRAF --count-only
```

```sh
biomcp search trial --condition 'pancreatic cancer' --gene KRAS --phase 3 \
  --status RECRUITING
```

```sh
biomcp search trial --condition melanoma --intervention pembrolizumab \
  --format md
```

```sh
biomcp search trial 'preserve query' --id NCT05273554 --sponsor 'MD Anderson' \
  --facility 'MD Anderson' --country 'United States' --phase 1 \
  --criteria 'literal criteria' --status COMPLETED --sex ALL \
  --sponsor-type other --limit 1
```

```sh
biomcp search trial --condition 'Snyder-Robinson syndrome' --exact-condition \
  --limit 1
```

```sh
biomcp search trial --source nci --biomarker-code C96022 --limit 2 \
  --biomarker-use inclusion
```

```sh
biomcp search trial --source nci --condition-id NCIT:C3224 --limit 1
```

```sh
biomcp search trial --condition melanoma --study-type INTERVENTIONAL
```

```sh
biomcp search trial --gene KIT --variant L576P \
  --condition 'gastrointestinal stromal tumor' --limit 100
```

## Reply

The reply is one JSON object. Its schema is one of [`biomcp.nci-trial-page/10`](/reference/schemas/biomcp-nci-trial-page-10/), [`biomcp.trial-page/14`](/reference/schemas/biomcp-trial-page-14/), whose page lists its fields.

## Next

The [trial guide](/user-guide/trial/) shows every trial command at work. [Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes.
