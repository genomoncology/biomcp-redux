# 0116 — Investigate the drug slice and propose its design

Status: OPEN.

Milestone: 3

## Outcome

`sdlc/planning/slices/drug.md` holds the drug slice investigation and proposed design, through steps 1 to 4 of `planning/slices.md`, ready for PM review and Ian's questions. No product code changes.

- Why: milestone 3 serves approval, label, safety and drug identity questions for clinicians, and every slice starts from its own investigation.
- Starts from: `planning/slices.md`; `planning/slices/variant.md` and `trial.md` as worked examples; milestone 3 in `planning/milestones.md`; the drug sources in ROADMAP.md; 0.9's drug commands; decisions `2026-10-24-no-text-heuristics.md`, `2026-10-24-keyed-sources-run-only-with-the-user-s-key.md` and `2026-10-24-sources-are-open-by-default.md`.
- Examples: 20 to 40 real clinical drug questions from papers and public issues, covering a brand or generic name, an approval status and date in the US and EU, a label section such as boxed warnings or dosing, a drug's targets for a cancer gene, a biomarker-linked therapy, an interaction, an orphan designation, an empty result and a failing source, each with its source and accepted answer. Bench chemistry and compound screening questions are out of scope. Clinical judgments go to a clinical reviewer.
- Sources: ask each candidate source in ROADMAP.md directly for each example and record counts, speed, requests, returned identifiers, the fields its public tier carries, and terms. Each bundle candidate records its file, size, update cadence and license. Each source marked investigate ends kept or dropped with a reason.
- Identity: record how each source names the same drug, which identifier joins them, and where they disagree. A changed drug identity answer goes to a clinical reviewer, as milestone 3's done-when says.
- 0.9: measure 0.9's drug commands on the same examples in time, bytes and facts, find any filter mapped to the wrong field, and account for every command, option and section.
- Design: the delight questions, types and schema, a search that returns drug headers with drill-downs into approvals, label, targets, interactions and other sections the examples need, disclosure layers and token budget against 0.9, sources and bundles kept and dropped, harvested case list, and the open questions for Ian with options and a recommendation.
- Proof: the PM reviews the design on its branch. Live captures run through pm job with raw evidence on the evidence store. No licensed content enters the repo.
- PM ruling 2026-10-24 after j11400: the old 0.9 binary bypassed the capture proxy, so its outputs have unknown network provenance and are excluded. The design is reviewed from the verified direct and bundle captures, with the 0.9 comparison marked unmeasured. A later 0.9 baseline runs only behind verified network containment, under a separate ticket.
- PM review 2026-10-24 of 90fa409c, returned with fixes that use the evidence already captured:
  1. Map each question to the kept source that answers it and how completely. Keep WHO prequalified products, FDA orphan designations and DGIdb only with a question that only they answer; otherwise drop them with a reason.
  2. Name the identifier that joins each pair of sources. EMA, Drugs@FDA, WHO, FDA orphan and DDInter rows stay source-scoped sections or searches reached by their own ID or by literal name text that returns candidates, never by a joined card field. Remove authorization events from the header unless a proven identifier join exists.
  3. Give every source and component its use label and group under the open-by-default decision. DDInter is kept and labeled noncommercial. Say that OncoKB therapy levels stay with the variant slice's keyed source, reached by next steps.
  4. Settled by the PM: a drug name returns candidates and never picks a product; the agent chooses a source explicitly, and the design names one default search source with its evidence; the card carries no full label text, which lives in named sections; PharmacoDB and CVX are dropped with retirement messages for `drug cell-lines` and `cvx sync`.
  5. Use one spelling for interactions, `biomcp sync <bundle>` as the bundle decision says, and identifiers in their published form, as variant ruling 1 does, instead of a new prefix grammar. Shortage leaves the section list while it is deferred.
  6. Add header and card field lists, and a case list that maps every command and option to its cases. Label the failing-source case as synthetic.
  7. Replace the PM decisions section with a section for Ian's questions. It says there are none, because the bundles fall under the approved bundle decision, unless a fix raises a cost or commitment.
- PM recheck 2026-10-24 of 36a0aeb1: all seven fixes closed; design accepted with these amendments, applied on the branch before it lands:
  1. DDInter interactions are a section, `get drug DDInter1744 interactions --source ddinter`, with no separate verb.
  2. `search drug --source openfda-label` sends the brand-name field only, named in the design, until a generic-name positive case is captured.
  3. Exact-ID gets for MyChem, ChEMBL, CIViC therapy, EMA and DDInter are marked unproven; each adapter ticket captures a positive case first.
  4. The default MyChem search page budget is marked unmeasured; its adapter ticket measures it.
  5. `identity_level` comes from the endpoint or record type, never from a field's shape.
  6. Text continuation reuses the shipped `section_offset` and `section_limit` spelling, with no new option.
- Defers: tickets, which the PM files from the reviewed design.
