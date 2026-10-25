# 0181 — Get FDA 510(k) clearance records by exact K number

Status: OPEN.

Milestone: 4

Depends on: 0174
Depends on: 0175

## Outcome

`get diagnostic K173492 --source fda-510k` sends openFDA's 510(k) endpoint the exact `k_number` and returns one clearance card with the native K number, decision code and description, decision and received dates, applicant, device name and product code as sent. Companion association pages from 0174 gain one 510(k) pivot per distinct exact K number in their device cells, and the 510(k) card gains its exact `search diagnostic --source fda-cdx --device-id` step. The `fda-510k` atom moves from reserved to implemented in the source registry.

- Why: the single approved K173492 request in 0175's capture j13187 returned one positive native row, and diagnostic.md and 0175 require a positive to open a separate 510(k) source ticket. Companion rows cleared by 510(k) otherwise show a K number with no record behind it.
- Starts from: diagnostic.md's FDA 510(k) record row, source terms row and K/DEN rules; the K173492 row and the K080653 and P000000 absence probes in the captures; Yellow data/biomcp-redux/0175-fda-pma-native-capture/j13187-native-capture.tar.gz (SHA256 072d14d2a8d5ac286139663927dffe40d086474a97669b49d8f0f431366171b1); 0175's PMA adapter as the pattern; 0171's registry.
- Rules: a clearance is never called a PMA approval, and records are never merged by product code or name. The returned K number must equal the requested one, or the record is dropped with a problem and never enters the cache. A 404 `NOT_FOUND` is absence, distinct from failure, and never says the number is invalid everywhere. A non-K identifier fails before I/O with the supported route. DEN numbers stay source facts with no get step. The projection carries no openFDA harmonized field and no licensed GMDN content. No free-text 510(k) search. The source uses `open` terms with no group alias, from the registry.
- Proof: outside-in cases first, watched to fail, replaying j13187's K173492 row and the K080653 absence probe. Cases assert the card's native fields, absence distinct from failure, the identity guard's drop and problem with no cache admission, the companion page's 510(k) pivot and the card's device search step. Recorded 429, 500, timeout and broken-body cases assert a failed outcome with source and terms. CLI and MCP outputs are measured and each case gets a ceiling at measured size plus 15 percent, recorded in diagnostic.md. No new provider call is needed; if the developer finds a field the capture lacks, they ask the PM before any request. No licensed content enters the repo. Then make landing-gate runs through pm job.
