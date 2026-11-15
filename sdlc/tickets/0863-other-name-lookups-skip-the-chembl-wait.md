# 0863 — other-name-lookups-skip-the-chembl-wait

Status: OPEN.

Milestone: 7

## Outcome

Every drug-name lookup that still waits on ChEMBL's API (for example WHO prequalification search) uses 0843's MyChem copy of ChEMBL's records instead.

## Why

From the bench 0039 persona review (2026-11-12, runs on BioMCP 58d83b6da), `personas/<slug>/runs/2026-11-12/review.md` and `log.md`: pt-committee log.md:52: a WHO prequalification search timed out waiting on ChEMBL; 0843 covered trial, article and adverse-event lookups only.

## Proof

- Proof: a routine case with ChEMBL stalled shows the WHO search widened and complete.

## Starts from

- Starts from: 0843's `crates/biomcp-adapters/src/mychem/chembl_names.rs` and remaining ChEMBL-API name callers.

## Keeps

- Keeps: `get/search drug --source chembl` still use ChEMBL's API.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `tests/cases/`

## Progress

- Developer note: 2026-11-15. The remaining caller of ChEMBL's API for names was the other-source alias lookup in `crates/biomcp-query/src/drug/aliases.rs` (`with_aliases`). It runs after any drug search that finds nothing under the typed name (WHO PQ, EMA, Drugs@FDA, recalls, openFDA label, Open Targets), after a `get drug NAME` that a source cannot resolve, and for a DDInter `--partner` name. It asked MyChem, openFDA label, then ChEMBL through `get drug NAME --source chembl`: two ChEMBL searches and a molecule read. That is the wait pt-committee log.md:52 shows (`search drug apixaban --source who-pq`, `chembl: failed, deadline-exceeded`).
- Changed: that ChEMBL step now reads ChEMBL's record from MyChem's copy (`search_names::chembl_copy`, shared with 0843's trial, article and adverse-event lookup), one request. ChEMBL's rules stay: one molecule naming the input is used; two give the "candidate records" hint and no alias. The notice still names the ChEMBL record (`chembl record CHEMBLnnn supplies the name ...`). The reply lists MyChem as the source asked and ChEMBL as the record's source. With ChEMBL disabled the step is skipped and listed as disabled; with MyChem disabled it cannot run, as in 0843.
- Other ChEMBL API callers left as they are: `get/search drug --source chembl` (resolve, native search, native sections) and the cell-line ChEMBL sections, which do not look up drug names.
- Proof: `tests/cases/drug/who-pq-chembl-stalled.yaml`. `search drug SW-7 --source who-pq` finds no row, MyChem's free text and openFDA find no record, ChEMBL's record (from MyChem's copy) states "SYNTHETIC MEDICINE S", and the search retries under that name and returns WHO row SYN-CS01, status complete. ChEMBL's own API replies in the replay take 30 s against a 9 s command limit, so the old path would stall. WHO PQ rows are synthetic, with the `Synthetic: who-pq` README line (0920).
- Replays: the alias cases that reached ChEMBL's API now carry MyChem copy replies mirroring their ChEMBL replies (`drug/name-get`, `native-refused-offset`, `recall-empty`, `source-aliases`, `source-aliases-review-*`, and `cache-admission`, whose empty-search alias step for imatinib now asks MyChem's copy). `source-aliases-review-evidence` keeps its failed ChEMBL leg through a malformed copy reply. `source-aliases-ambiguous` disabled MyChem only to avoid recording it; it now disables only openFDA label and records MyChem's empty free-text reply, since the ChEMBL step needs MyChem.
- Cheap job j41393: fmt, docs and schemas unchanged; all `/drug/`, `/search/drug-names/` and `mcp-parity/typed-owned` cases passed except three alias cases and the structural ratchet (two optional-value drops), fixed as above.
- Gate j41396 failed one trial, `drug-native-sources/mychem-search-cache-admission`: its native stub refused the new MyChem copy request for imatinib. Added the empty copy reply beside the empty ChEMBL alias replies it replaces; every other stage and the other 959 trials passed.
