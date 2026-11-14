# WHO PQ date cells read plainly: synthetic export

WHO PQ is a noncommercial source, so this replay holds no real WHO rows (0695, 0910). `who-pq/` holds a synthetic 29-row finished-product export, the same file as `tests/replies/drug/markets/who-pq/`, in the real export's shape: the same header and quoting, padded date cells such as `22  Sep,  2013`, blank Date of Prequalification cells and no final newline. Every reference, applicant, strength and date is invented; no date appears in WHO's export of 2026-11-13.

The case relies on its 13 dolutegravir rows in export order: the first five have a blank date (`SYN ALT 0201 LISTING` first), the sixth is SYN202 dated `9  Nov,  2017`, the seventh SYN201 dated `22  Sep,  2013`, and the last two are undated again. `tests/replies/drug/markets/README.md` says which rows make the markets case's choice.
