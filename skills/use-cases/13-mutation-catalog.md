# Inspect candidates at an explicit genomic position

## Question

Which source variant records overlap this stated region and assembly?

## 1. Retrieve and inspect

```sh
biomcp search variant --region chr7:55259515-55259515 --build GRCh37 --limit 2
```

Read the echoed one-based inclusive region and assembly, source query and every candidate representation. Different alternate alleles can share an rsID.

## 2. Retrieve and inspect

```sh
biomcp search variant --gene EGFR --protein L858R --build GRCh37
```

This separate selector demonstrates a gene and protein query. Inspect each returned candidate and transcript or source representation; do not equate the two searches without checking identity.

## Next

Follow returned pages and explicit variant record commands. Preserve assembly and reference-qualified expressions. These recorded candidate fixtures demonstrate command behavior and do not rank pathogenicity.

Read response status, problems and source terms at every step. Recorded replies demonstrate the commands; live replies can change.
