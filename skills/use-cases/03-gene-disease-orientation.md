# Read gene–disease source assessments

## Question

How can I inspect the source assessments for a named group of genes?

## 1. Retrieve and inspect

```sh
biomcp get gene MLH1 MSH2 MSH6 PMS2 EPCAM clingen
```

Read each resolved gene identity in input order. Inspect the source disease identifier, assessment label and cited record for each returned validity row. Keep dosage and validity records separate.

## 2. Retrieve and inspect

```sh
biomcp get gene HGNC:1097
```

This separate identity example shows an explicit HGNC input. Compare the requested identifier with the resolved gene before using any source section.

## Next

Follow source record links and returned section commands. An assessment about a gene and disease is not a classification of an individual variant or a diagnosis.

Read response status, problems and source terms at every step. Recorded replies demonstrate the commands; live replies can change.
