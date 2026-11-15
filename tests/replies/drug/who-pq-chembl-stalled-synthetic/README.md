# WHO PQ search widened while ChEMBL stalls: synthetic replies

WHO PQ is a noncommercial source, so this replay holds no real WHO rows (0695). The two export rows are invented: references, medicine names, applicants, strengths and dates. The other replies are authored in the shape each source returns, for the invented research code SW-7 and the invented ChEMBL molecule CHEMBL9100001, so they describe no real drug.

MyChem's free-text name read finds no record for SW-7 and openFDA has no label, so only ChEMBL's record states the WHO name. MyChem's copy of ChEMBL's record answers at once; ChEMBL's own API replies take 30 seconds, longer than the case's command time limit, so a search that still asked ChEMBL's API would stall and stay unwidened (0863).

Synthetic: who-pq
