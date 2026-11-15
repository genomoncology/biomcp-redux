# WHO PQ search widened while ChEMBL stalls

WHO PQ is a noncommercial source, so the WHO export here is synthetic (0695): both rows are invented, references, applicants, strengths and dates included. Real WHO lists rifampicin under its INN; the invented row SYN-RF01 lists it as "Rifampin", the preferred name ChEMBL's record states, so only ChEMBL's record can lead the search to it.

The MyChem, MyChem copy of ChEMBL and openFDA label replies are real, recorded live for `search drug rifampicin --source civic --limit 1` in pm job j41412 (2026-11-15), which reached the same alias step. MyChem's free-text read finds two stereo records stating rifampicin, so its own name resolution names no single record; openFDA's label (Rifadin, RIFAMPICIN) gives no other name; MyChem's copy of ChEMBL finds one molecule, CHEMBL374478, preferred name RIFAMPIN.

The ChEMBL API replies are real, recorded live in the same job for `get drug rifampicin --source chembl`; only their delay is authored, set to 30 seconds against the case's 15-second command limit, so a search that still asked ChEMBL's API would stall and stay unwidened (0863).

Synthetic: who-pq
