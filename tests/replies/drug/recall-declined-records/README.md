# openFDA drug recall pages with older-form and malformed recall numbers

The replies were recorded from the public openFDA API by `biomcp --record`, without a key; each body is unchanged. The two 2013 day pages were recorded on 2026-11-15 in pm job j41440 (base commit 648240afe); the Olympia, Qualitest and `D-502-2013` replies on 2026-11-15 in pm job j41472.

- 2013-10-18 and 2013-02-27: the default 10-record pages of `search drug --source fda-drug-recall` for recalls initiated that day. openFDA numbers most of them in FDA's older `D-NNN-YYYY` form (`D-401-2014` to `D-405-2014`, event 66568; `D-398-2014` to `D-400-2014`, event 64727; `D-623-2013`, event 65061). Before 0978 BioMCP declined each of them; now every record is a row.
- `--firm Olympia`, initiated 2013-05-29: ten of 105 records from event 65360. Nine are numbered `D-NNN-2013` and are rows; `D-751-013` is cut short and is declined.
- `--firm Qualitest`, initiated 2015-03-25: one record, `D00432-2015` (event 70852), missing its first hyphen; it is declined.
- `get drug D-502-2013`: the exact lookup returns that one record (Hospira, event 65225).

openFDA's drug enforcement export on 2026-11-15 held 2,541 records numbered `D-NNN-YYYY`, initiated 2006 to 2015 (pm job j41468).

| Request | Status | Recorded bytes | SHA-256 of the recorded file |
| --- | --- | --- | --- |
| `https://api.fda.gov/drug/enforcement.json?search=recall_initiation_date%3A%5B20131018%20TO%2020131018%5D&limit=10&skip=0&sort=recall_initiation_date%3Adesc` | 200 | 8,982 | `6a2a8e2f99cc6ffe2aeb7cf4dca519157692c155ab027beea8bc7eaded67ef94` |
| `https://api.fda.gov/drug/enforcement.json?search=recall_initiation_date%3A%5B20130227%20TO%2020130227%5D&limit=10&skip=0&sort=recall_initiation_date%3Adesc` | 200 | 7,839 | `5cc20228a8d4b99056dbd8bbda3636536fe24c3ed465af5068ae34ec64632a48` |
| `https://api.fda.gov/drug/enforcement.json?search=recalling_firm%3A%22Olympia%22%20AND%20recall_initiation_date%3A%5B20130529%20TO%2020130529%5D&limit=10&skip=0&sort=recall_initiation_date%3Adesc` | 200 | 12,501 | `42c35c9b01b7d32598099f50a1f13e181232214d0fc4b0bee8acd708111a0497` |
| `https://api.fda.gov/drug/enforcement.json?search=recalling_firm%3A%22Qualitest%22%20AND%20recall_initiation_date%3A%5B20150325%20TO%2020150325%5D&limit=10&skip=0&sort=recall_initiation_date%3Adesc` | 200 | 2,530 | `6dc8b563438000644dc049ede3b118b0420a7a7ea9b9266cb71d5f586b9eaf5d` |
| `https://api.fda.gov/drug/enforcement.json?search=recall_number.exact%3A%22D-502-2013%22&limit=1&skip=0` | 200 | 2,061 | `8cec1002ee8ce9fcd60aa0db0369b4411ad6d8d694bc97aa417431d65db46e9a` |
