# 0127 — Search and get drugs through ChEMBL and Open Targets with targets and indications

Status: OPEN.

Milestone: 3

Depends on: 0124
Depends on: 0126
Depends on: 0125
Depends on: 0148

## Outcome

`search drug TEXT --source chembl` and `--source opentargets` return native candidate pages. `get drug CHEMBL941 --source chembl` returns the exact ChEMBL card, and `get drug CHEMBL941 targets` and `indications` with `--source chembl` or `--source opentargets` return source-native target, mechanism and indication pages with `--section-limit` and `--section-offset`. Exact ChEMBL ID equality allows a next step from a ChEMBL card to Open Targets, and nothing else joins them.

- Why: the clinical bank asks a drug's targets for a cancer gene (D01, D03, D21), and drug.md keeps ChEMBL and Open Targets for that. The 0116 recheck marks ChEMBL exact-ID get unproven.
- Starts from: drug.md sections 2 to 4, the ChEMBL and Open Targets rows, and the proven joins table; the three captured Open Targets details for CHEMBL941, CHEMBL1201585 and CHEMBL3989989; 0124 and 0125.
- Rules: ChEMBL's preferred-name and explicit synonym selectors stay distinct, and the literal fam-trastuzumab deruxtecan spelling keeps its actual zero. A cell product never gets a manufactured chemical identity. A target is not an indication. A direct ChEMBL reply without a release never borrows MyChem's ChEMBL_37. Open Targets' clinical stage never reads as an authorization. Assay and structure screening surfaces stay out. `identity_level` comes from the endpoint or record type, never from a field's shape.
- Proof: outside-in cases first. One bounded keyless pm job captures positive exact-ID gets for CHEMBL941, CHEMBL1201585 and CHEMBL3989989 from ChEMBL before adapter code, plus known positives for each search and section paging. Cases S02 (D01, D03, D19 by the Enhertu synonym, D21), S03 for `opentargets` (D01, D03, D17, D19), G01 for ChEMBL IDs and G02 (D01, D03, D21 targets and indications, with source-local paging) replay the recorded replies, with a size ceiling on each. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
- PM ruling 2026-10-25: ChEMBL name search takes `--name-selector preferred|synonym`, default `preferred`, refused before any request for other sources. Each value needs its own captured known-positive case. Changed drug schemas take a version change.
