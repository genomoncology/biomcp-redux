# HGNC published files

The two recordings were captured once through the checked bundle transport in recording mode on 2026-10-24 by job j8844. They retain the original request metadata and exchange timing. The complete-set download held 45,277 records and the withdrawn download held 5,291 records.

Each text body is a trimmed public source slice. The complete slice retains every source row whose location starts with `9p21`, the named gene resolution examples, an approved and withdrawn symbol collision, and one source example of each observed locus mapping. The withdrawn slice retains BRCA3, A2MR, CBBM, AK6 and a merge with a withdrawn successor. Band counts assert this captured snapshot only. Tests that mutate these captures label their data synthetic and write only to isolated temporary directories.

Sources: [HGNC complete set](https://storage.googleapis.com/public-download-files/hgnc/tsv/tsv/hgnc_complete_set.txt), [HGNC withdrawn identities](https://storage.googleapis.com/public-download-files/hgnc/tsv/tsv/withdrawn.txt), and [HGNC field definitions](https://hgnc.genenames.org/help/symbol-report/index.html).

These temporary public adapter and resolver contracts move to command cases when the gene command consumes these paths.
