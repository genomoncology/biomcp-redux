# openFDA drug recall pages with several records that have no usable recall number

The replies were recorded from the public openFDA API on 2026-11-15 by `biomcp --record` in pm job j41440 on base commit 648240afe, without a key; each body is unchanged. They are the default 10-record pages of `search drug --source fda-drug-recall` for recalls initiated on 2013-10-18 and on 2013-02-27. openFDA lists these older records with recall numbers in a form BioMCP does not accept as `D-NNNN-YYYY`, such as `D-404-2014`, so each is declined:

- 2013-10-18: six records; five from event 66568 (`D-401-2014` to `D-405-2014`) are declined and `D-1161-2014` is returned.
- 2013-02-27: four records, all declined: three from event 64727 (`D-398-2014` to `D-400-2014`) and one from event 65061 (`D-623-2013`).

On the base commit both pages answered `partial` with one `recall_number` problem that named no record, so several declined records read as one.

| Request | Status | Recorded bytes | SHA-256 of the recorded file |
| --- | --- | --- | --- |
| `https://api.fda.gov/drug/enforcement.json?search=recall_initiation_date%3A%5B20131018%20TO%2020131018%5D&limit=10&skip=0&sort=recall_initiation_date%3Adesc` | 200 | 8,982 | `6a2a8e2f99cc6ffe2aeb7cf4dca519157692c155ab027beea8bc7eaded67ef94` |
| `https://api.fda.gov/drug/enforcement.json?search=recall_initiation_date%3A%5B20130227%20TO%2020130227%5D&limit=10&skip=0&sort=recall_initiation_date%3Adesc` | 200 | 7,839 | `5cc20228a8d4b99056dbd8bbda3636536fe24c3ed465af5068ae34ec64632a48` |
