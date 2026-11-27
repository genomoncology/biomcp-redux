# Read separate pharmacogenomics tables

## Question

How do I inspect gene and diplotype source records without calculating a phenotype?

## 1. Retrieve and inspect

```sh
biomcp search pgx --gene DPYD --source cpic
```

Typed tool: `gene` `{"action":"pgx","pgx":{"gene":"DPYD","source":"cpic"}}`.

Read the source’s returned gene and drug rows and identifiers.

## 2. Retrieve and inspect

```sh
biomcp get pgx --gene DPYD --source cpic
```

Read the native tables independently and retain their identifiers and source attribution.

## 3. Retrieve and inspect

```sh
biomcp get pgx --gene DPYD --source cpic diplotype --diplotype 'c.1905+1G>A (*2A)/Reference'
```

Use CPIC’s spelling for each allele. Either allele order and letter case work. A miss can give examples of CPIC’s spellings for that gene.

## Next

Follow the original guideline and source record for the applicable context. BioMCP retrieves tables; it does not infer a patient’s genotype, phenotype or treatment.

Read response status, problems and source terms at every step. Live replies can change.
