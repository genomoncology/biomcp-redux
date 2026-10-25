# 0160 — Search and get MAUDE device reports with devices and narrative

Status: COMPLETE.
Landed: 378e9e877b65d3a33a91b85094071ce7b2e1c14b

Milestone: 3

Depends on: 0156
Depends on: 0157
Depends on: 0158

## Outcome

`search adverse-event TEXT --source maude` sends `device.generic_name:"TEXT"`, a phrase match on an analyzed field, to openFDA's device event endpoint sorted by `date_received:desc` and returns a five-row page of report headers with the exact `mdr_report_key`, the native event type, the received date, the native total in reports and the exact query echo, paged with `--limit` and `--offset`. `get adverse-event ID --source maude` returns the exact MAUDE card with the report number, the distinct event, received and report dates, the native device and patient count strings, the device and narrative counts, `source_url`, `report_url`, the available sections and next steps. `get adverse-event ID devices --source maude` and `narrative --source maude` return the native rows, paged with `--section-limit` and `--section-offset`.

- Why: the breast implant questions A18 to A20 need device reports, and the design keeps MAUDE as an explicit source with positional generic-name text.
- Starts from: adverse-event.md's MAUDE rows, card, devices and narrative shapes and the D01 to D05 findings, with 0152 recheck amendments 1 and 2; 0158's dispatch registration; 0156's types and 0157's registry.
- Rules: MAUDE never runs without `--source maude`, and a numeric ID never probes another source. The four FAERS filters fail before I/O with `--source maude`. No wildcard expansion and no narrative classification: implant presence never establishes cancer or reconstruction, and capsule SCC, BIA-ALCL and breast carcinoma stay distinct source text. A present empty string stays distinct from an absent field. A narrative row too large for its page returns a too-large problem and is never cut. Administrative address fields stay out of the consumer projection. The returned key must equal the requested key, or the record is dropped with a problem and never enters the cache. Report counts and device or patient counts are different units.
- Proof: outside-in cases first, watched to fail. Before adapter code, one bounded keyless pm job captures the D01 search and its exact report keys, the exact get for 50952244, and exact gets for one D02 and one D04 report chosen by native key from the retained j12641 bodies. D01 asserts the BREAST IMPLANT page and total (23,863 on 2026-10-25); D05 asserts the 50952244 card and its empty narrative; D02 and D04 reports assert device rows and narrative arrays from zero to five rows; D03 replays the retained exact SCC expression's 404 `NOT_FOUND` through the adapter's no-match path and asserts an empty status distinct from a failure, as an adapter fixture and not a CLI positive. Synthetic cases cover multiple devices, an oversized narrative row and a wrong returned key, each labeled synthetic. The job measures the default search, the card and the devices and narrative pages in CLI and MCP output, and each case gets a ceiling at measured size plus 15 percent, recorded in adverse-event.md. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo; replies that carry GMDN device names follow 0157's terms reading. Then make landing-gate runs through pm job.
