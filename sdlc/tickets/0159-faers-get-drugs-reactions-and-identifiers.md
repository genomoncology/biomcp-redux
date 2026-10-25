# 0159 — Get a FAERS report with its drugs, reactions and identifier pages

Status: OPEN.

Milestone: 3

Depends on: 0156
Depends on: 0157
Depends on: 0158

## Outcome

`get adverse-event ID` with no `--source`, or with `--source faers`, sends `safetyreportid:"ID"` and returns the exact FAERS card: the header, the latest receipt date, the independent seriousness flags, the patient strings, the reporter qualification and country, the drug and reaction counts, `source_url`, `report_url`, the available sections and next steps. `get adverse-event ID drugs` and `reactions` return the independent ordered drug and reaction rows with their absolute source index, paged with `--section-limit` and `--section-offset`. `get adverse-event ID unii|rxcui|product-ndc|package-ndc|spl-id|spl-set-id|application-number --drug-index N` returns one named original identifier array from one native drug row. Paging a checked retained reply makes no new provider request.

- Why: the design keeps exact report get with its growable lists behind named sections, and the 0152 review marks multi-version get unproven.
- Starts from: adverse-event.md's identity paragraphs, card and section JSON shapes and the F21, F08 and I01 findings; 0158's dispatch registration; 0156's types and 0157's registry.
- Rules: the returned ID must equal the requested ID, or the record is dropped with a problem and never enters the cache. A reply with more than one record for the ID returns a problem and never picks one, because multi-version get is unproven, and `--version` does not exist. Each role belongs to its own drug object; a role on one drug never applies to another. Reactions keep the label as a `Concept` with no CURIE, the MedDRA version apart, and the native outcome code. An ingredient UNII never identifies a marketed product, and no identifier offers a drug step in this ticket, because no external identity pivot is proven. The drug index is the absolute position in the retained report. Links are data, and the card has at most five next steps.
- Proof: outside-in cases first, watched to fail. Before section code, one bounded keyless pm job captures an ID-only exact get for 26940567 and for the F08 and I01 reports that carry the longest identifier collections, chosen by native ID from the retained j12641 bodies, plus F18's nonexistent ID as a no-match control. F21 asserts the 26940567 version 1 card with its counts and links; drugs and reactions cases assert the F01 array sizes (2, 9, 6, 5 and 2 drugs, 1, 3, 4, 1 and 2 reactions) and a complete walk over the longest arrays with no gap or duplicate; F08 asserts a long `package-ndc` continuation walk; I01 asserts that the PADCEV drug row's UNII DLE8519RWM stays on its own index and never moves to the KEYTRUDA row; F18 asserts a no-match status distinct from a failure. Synthetic cases cover a wrong returned ID, a second returned version, a bad drug index and missing versus empty fields, each labeled synthetic. The job measures the card, the default drugs and reactions pages, their 19-row and 22-row walks and each identifier page's default and long continuation in CLI and MCP output, and each case gets a ceiling at measured size plus 15 percent, recorded in adverse-event.md. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo; replies that carry MedDRA reaction terms follow 0157's terms reading. Then make landing-gate runs through pm job.
