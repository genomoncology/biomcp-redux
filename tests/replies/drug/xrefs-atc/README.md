# Apixaban cross-references with its ATC code

Recorded on 2026-11-16 by `biomcp --record` in pm job j42093, without keys, from the public MyChem.info API. Both replies are kept unchanged.

| Request | Status |
| --- | --- |
| `https://mychem.info/v1/query?q=apixaban&size=50&from=0&fields=...` (the name lookup's fields) | 200 |
| `https://mychem.info/v1/chem/QNZCBYKSOIHPEH-UHFFFAOYSA-N?fields=...` (the card's fields, with `chembl.atc_classifications`) | 200 |

MyChem's ChEMBL component for apixaban (CHEMBL231779) states `atc_classifications` B01AF02, the same code ChEMBL's own molecule record gives. The card request asks for `chembl.atc_classifications`; before 1021 it did not, so a build without the change asks a different URL and the replay has no reply for it.
