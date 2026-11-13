# 0879 — ema-label-text-by-section

Status: OPEN.

Milestone: 7

## Outcome

An EMA product's label text can be read by section (therapeutic indications, 4.1, at least), so a US-versus-EU indications side-by-side is one call per side.

## Why

From bench 0041 reg-affairs smoke run on BioMCP b87cb3b3e (pm job j38923), `personas/reg-affairs/runs/2026-11-13/log.md` in biomed-agent-bench (until filed: `~/workspace/data/biomed-agent-bench/0041-persona-reg-affairs/` on the M5), log.md:192-210: the EMA record gives one flat 8,884-character `indication` string with no section heading or product-information text, while the US label is numbered 1.1 to 1.10.

## Proof

- Proof: first check, live, what EMA publishes in machine-readable form for product information (EPAR/SmPC) and record it here; then a routine case reading Keytruda's 4.1 text with its version and URL, or a recorded finding that EMA offers none.

## Starts from

- Starts from: the EMA adapter and its `regulatory` section.

## Keeps

- Keeps: the current `indication` field; attribution and version.

## Defers

- Defers: other SmPC sections beyond the first proved one.

## Changes

- Changes: `crates/biomcp-adapters/src/`, `crates/biomcp-query/src/drug/`, `tests/cases/drug/`

## Progress

### 2026-11-13: live check of what EMA publishes for product information

Checked from the Beelink with curl. EMA's legal notice and robots.txt allow every fetch below.

- **Medicines JSON** (`/en/documents/report/medicines-output-medicines_json-report_en.json`, 6.8 MB, already synced by the adapter). The Keytruda row (EMEA/H/C/003820, `revision_number` 73, `latest_procedure_affecting_product_information` VR/0000336194) has one flat `therapeutic_indication` string of 8,994 characters, with `&nbsp;` entities and no section heading. It is not the SmPC 4.1 text word for word. EMA's web text says "Stage IIB, IIC, or with Stage III melanoma and lymph node involvement who have undergone complete resection". SmPC 4.1 says "Stage IIB, IIC or III melanoma and who have undergone complete resection (see section 5.1)".
- **EPAR documents JSON** (`/en/documents/report/documents-output-epar_documents_json-report_en.json`, 29.3 MB, 20,378 rows, timestamp 2026-11-13T05:50:07Z). It is metadata only: name, type `product-information`, `ema_product_number`, first published and last updated dates, a `document_url` and translation URLs. For Keytruda the entry is id 63196, last updated 2026-10-30T16:10:00Z, at `https://www.ema.europa.eu/en/documents/product-information/keytruda-epar-product-information_en.pdf`. The file holds no text.
- **Product-information PDF** (the URL above). 6,611,178 bytes and 372 pages, tagged PDF 1.7. It holds two SmPCs (25 mg/mL concentrate for infusion; 395 mg and 790 mg solution for injection), then labelling and the package leaflet. Each SmPC has its own "4.1 Therapeutic indications". `pdftotext` gives clean 4.1 text for the first SmPC (pages 2 to 4, 9,048 characters, ending at "4.2 Posology and method of administration"). "DATE OF REVISION OF THE TEXT" (section 10) is blank, so a version would have to come from the document index date or the medicines `revision_number`. BioMCP's PDF reader (`crates/biomcp-adapters/src/pdf/text.rs`) is scoped to FDA documents (ticket 0611 rules). It refuses PDFs over 100 pages (`MAX_PAGES`), so this file would come back `TooLarge`. lopdf's text from this file was not tested.
- **ePI (electronic product information, FHIR)**. The developer portal (`https://epi.developer.ema.europa.eu/api-details`) describes a keyless "EMA.EPI.Consuming" API "for EU human medicines published during the EMA-HMA-EC ePI pilot". Its gateway is `https://epi.ema.europa.eu/consuming/api/fhir/` with `List`, `List/{id}`, `Bundle` and `Bundle/{id}` (parameters `title` and `regulatoryAgency`). Every call returned HTTP 500 with a .NET stack trace from EMA's own code: "The format of value ' ' is invalid" in `RetrievalBusiness.GetConsumerLists`. That includes `List`, `List?title=Keytruda`, `List?regulatoryAgency=EMA`, `Bundle?title=Keytruda` and `List/1`, with and without FHIR Accept headers. The raw FHIR server at `https://epi.ema.europa.eu/fhir/` answers, but `List?_summary=count` and `Bundle?_summary=count` both give total 0. EMA describes the data as the 2023–2024 pilot set, so Keytruda is unlikely to be there even when the API works.
- **Rate limit.** www.ema.europa.eu returned HTTP 429 ("Too Many Requests") after four page fetches within about a minute, and again for WebFetch shortly after. It recovered within a few minutes.
- **Licence.** EMA's legal notice lets anyone reproduce its documents, for commercial use too, "provided that EMA is always acknowledged as the source"; citations need no permission. Content "supplied by third parties" is excluded. The product information is the marketing-authorisation holder's text annexed to the Commission decision, which makes it a minor question, not a blocker, for quoting one section with attribution.

Conclusion: EMA publishes no machine-readable, sectioned product-information text that BioMCP can use today. The structured route (ePI) is broken and covers pilot products only. The only route that works is to download a 6.6 MB, 372-page PDF on each read and cut 4.1 out of it by heading text. That would be a new non-FDA PDF reader, which Ian's decision `sdlc/decisions/2026-10-24-pdfs-are-downloaded-not-read.md` rules out ("BioMCP does not extract text from PDFs"; the FDA reader came later, under ticket 0611), and it would also need the 100-page cap raised. Stopped before building, per the brief; the decision goes back to the PM.

### 2026-11-13: PM decision (option 1), and what changed

The PM chose to record "EMA offers none" and not to extend the PDF reader. A product-information link was allowed only from data EMA publishes and BioMCP can sync cheaply. The only source for it is the EPAR documents index, which is 29.3 MB: 4.3 times the medicines file already synced, and a second daily EMA download from a host that throttles, just for one link and date per medicine. That is not cheap, so no link was added. The medicines file has no product-information URL; `medicine_url` is the EPAR page.

Changed: the description of the `regulatory` field (`DrugRegulatory` and `DrugRegulatoryPart`, `crates/biomcp-types/src/drug/`) now says that EMA's `indication` is the `therapeutic_indication` web summary shown on the EPAR page, not the SmPC 4.1 text, and that the wording can differ. The Keytruda EPAR page shows the same "Stage IIB, IIC, or with Stage III melanoma and lymph node involvement" wording as the JSON. `indication` is defined in the pinned `biodata` crate, so the note sits on the section field, which only EMA serves. This is a description-only schema change.
