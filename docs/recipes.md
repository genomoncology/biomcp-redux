# Worked commands

Generated from harvested case run steps through the shipped help examples. The documentation check executes these commands against recordings through CLI and MCP. Live source replies can differ.

Choose a command below, inspect its source outcomes and follow its returned next commands. Stored sources require the explicit synchronization shown by their command.

## 

```sh
biomcp help
```

## author papers

```sh
biomcp author papers semantic-scholar:123
biomcp author papers semantic-scholar:123 details
```

## search author

```sh
biomcp search author 'Louis Williams'
biomcp search author 'Louis Williams' --source semanticscholar
```

## get author

```sh
biomcp get author semanticscholar:456 semantic-scholar:123 semantic-scholar:999
biomcp get author semantic-scholar:123 semantic-scholar:123
```

## cell-line assay-count

```sh
biomcp cell-line assay-count CHEMBL3706573
biomcp cell-line assay-count CHEMBL3706573 --fresh
```

## search cell-line

```sh
biomcp search cell-line MOLM13
biomcp search cell-line --query 'MV4;11'
```

## get cell-line

```sh
biomcp get cell-line CVCL_2119
biomcp get cell-line CVCL_0004
```

## get gwas

```sh
biomcp get gwas association:6114
biomcp get gwas GCST000035
```

## search gwas

```sh
biomcp search gwas --pubmed 17529967
biomcp search gwas --variant rs2981582
```

## protein features

```sh
biomcp protein features P00533
biomcp protein features P00533 --limit 1
```

## protein isoforms

```sh
biomcp protein isoforms P00533
biomcp protein isoforms P00533 --limit 1
```

## protein domains

```sh
biomcp protein domains P00533
biomcp protein domains P00533 --limit 1
```

## protein complexes

```sh
biomcp protein complexes P00533
biomcp protein complexes P00533 --limit 1
```

## protein complex-members

```sh
biomcp protein complex-members P00533 --complex CPX-715
biomcp protein complex-members P00533 --complex CPX-715 --limit 1
```

## protein structures

```sh
biomcp protein structures P00533
biomcp protein structures P00533 --limit 1
```

## protein sequence

```sh
biomcp protein sequence P00533
biomcp protein sequence P00533 --fresh
```

## search protein

```sh
biomcp search protein --gene BRAF
biomcp search protein EGFR
```

## get protein

```sh
biomcp get protein P00533 Q9UIF7 P38398-2
biomcp get protein EGFR
```

## search phenotype

```sh
biomcp search phenotype synthetic
biomcp search phenotype synthetic --limit 1
```

## get phenotype

```sh
biomcp get phenotype HP:0000001
biomcp get phenotype HP:0000001 --source monarch
```

## search pathway

```sh
biomcp search pathway 'Signaling by EGFR'
biomcp search pathway Hemostasis
```

## get pathway

```sh
biomcp get pathway R-HSA-9652817
biomcp get pathway R-HSA-177929
```

## pathway genes

```sh
biomcp pathway genes R-HSA-5673001
biomcp pathway genes R-HSA-5673001 details
```

## pathway events

```sh
biomcp pathway events R-HSA-5673001
biomcp pathway events R-HSA-5673001 --limit 50
```

## search study

```sh
biomcp search study Atlas
biomcp search study --cancer-type mixed --limit 1
```

## get study

```sh
biomcp get study synthetic_0
biomcp get study synthetic_1
```

## search pgx

```sh
biomcp search pgx --gene DPYD --source cpic
biomcp search pgx --gene DPYD --source cpic --limit 1 --cpic-level B/C
```

## get pgx

```sh
biomcp get pgx --gene DPYD --source cpic
biomcp get pgx --gene DPYD --source cpic diplotype --diplotype '*1/*2A'
```

## search diagnostic

```sh
biomcp search diagnostic --source gtr --gene BRCA1
biomcp search diagnostic --source gtr --condition-id MedGen:C2676676
```

## get diagnostic

```sh
biomcp get diagnostic 0466-046-00 --source who-ivd
biomcp get diagnostic GTR000006692.3 --source gtr
```

## search adverse-event

```sh
biomcp search adverse-event pembrolizumab
biomcp search adverse-event pembrolizumab --received-from 2024-01-01 --received-to 2024-12-31
```

## get adverse-event

```sh
biomcp get adverse-event 100
biomcp get adverse-event 100 reactions
```

## search disease

```sh
biomcp search disease --source seer --limit 1
biomcp search disease --source seer --limit 1 --offset 1
```

## get disease

```sh
biomcp get disease 47 incidence --source seer --dataset 4 --period 5 --section-limit 1
biomcp get disease 55 survival --source seer --sex 3 --race 1 --age-range 1 --dataset 4 --interval 5 --section-limit 5
```

## search drug

```sh
biomcp search drug Tibsovo --source ema --limit 1
biomcp search drug Tibsovo --source ema --limit 1 --offset 1 --fresh
```

## get drug

```sh
biomcp get drug EMEA/H/C/000406 --source ema
biomcp get drug EMEA/H/C/000406 regulatory --source ema
```

## get gene

```sh
biomcp get gene KRAS
biomcp get gene BRAF
```

## search gene

```sh
biomcp search gene KRAS
biomcp search gene p16
```

## search article

```sh
biomcp search article 'AKT1 E17K'
biomcp search article 'KRAS G12C sotorasib' --since 2024 --sort date --limit 5
```

## get article

```sh
biomcp get article 33123537
biomcp get article PPR1295425 fulltext
```

## article supplement

```sh
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx --variant P245S
```

## article pdf

```sh
biomcp article pdf 41351423 --save owned.pdf --replace
biomcp article pdf 41351423 --save second.pdf --replace
```

## article text

```sh
biomcp article text PMC5606870 --find AXIN2
biomcp article text PMC5606870 --part 1
```

## article citations

```sh
biomcp article citations 17611497
biomcp article citations 23000897 --source semantic-scholar
```

## article references

```sh
biomcp article references 34096690
biomcp article references 33123537 --source semantic-scholar
```

## article citation-evidence

```sh
biomcp article citation-evidence 33123537 17611497
biomcp article citation-evidence 33123537 17611497 --fulltext
```

## article recommendations

```sh
biomcp article recommendations 34096690 --limit 5
biomcp article recommendations 34096690 32955176 --negative 33433946 --limit 5
```

## search trial

```sh
biomcp search trial --condition melanoma
biomcp search trial --condition melanoma --intervention pembrolizumab
```

## get trial

```sh
biomcp get trial NCT04303780
biomcp get trial NCT04303780 documents
```

## trial document

```sh
biomcp trial document NCT04303780 SAP_001.pdf --save protocol.pdf --replace
biomcp trial document NCT04303780 SAP_001.pdf --save analysis.pdf --replace
```

## variant assertions

```sh
biomcp variant assertions clinvar:13961 --source clinvar
biomcp variant assertions CA000434 --source erepo
```

## get variant

```sh
biomcp get variant clinvar:13961 clinvar
biomcp get variant CA000434 erepo
```

## variant population

```sh
biomcp variant population CA123643 --detail
biomcp variant population CA123643 --detail --offset 10
```

## search variant

```sh
biomcp search variant --gene EGFR --protein L858R --build GRCh37
biomcp search variant --gene CHEK2 --protein Arg117Gly --build GRCh37
```

## variant recurrence

```sh
biomcp variant recurrence 'NC_000007.13:g.140494253G>A' --source cbioportal --gene BRAF --study msk_impact_2017 --profile msk_impact_2017_mutations --sample-set msk_impact_2017_all --limit 3
biomcp variant recurrence 'NC_000007.13:g.140494253G>A' --source cbioportal --gene BRAF --study msk_impact_2017 --profile msk_impact_2017_mutations --sample-set msk_impact_2017_all --limit 3 --page 0
```

## variant normalize

```sh
biomcp variant normalize --source mutalyzer --hgvs 'NM_004333.4:c.1799T>A'
biomcp variant normalize --source variantvalidator --hgvs 'NM_004333.4:c.1799T>A' --build GRCh38
```

## skill

```sh
biomcp skill list
```

## skill list

```sh
biomcp skill list
```

## skill render

```sh
biomcp skill render
```

## skill status

```sh
biomcp skill status biomcp-guidance
```

## skill install

```sh
biomcp skill install biomcp-guidance
```

## version

```sh
biomcp version
```

## mcp-config

```sh
biomcp mcp-config
```

## store clean

```sh
biomcp store clean
```

## health

```sh
biomcp health --offline
```

## sync

```sh
biomcp sync hgnc
```

## schema

```sh
biomcp schema
```

## help

```sh
biomcp help
```

## key list

```sh
biomcp key list
```

## key set

```sh
biomcp key set oncokb
```

## key remove

```sh
biomcp key remove oncokb
```

## serve

```sh
biomcp serve
biomcp help serve
```

## sync hgnc

```sh
biomcp sync hgnc
```

## sync clingen

```sh
biomcp sync clingen
```

## sync wikipathways

```sh
biomcp sync wikipathways
biomcp sync wikipathways --fresh
```

## sync cbioportal-studies

```sh
biomcp sync cbioportal-studies
biomcp sync cbioportal-studies --fresh
```

## sync ema

```sh
biomcp sync ema
biomcp sync ema --fresh
```

## sync ddinter

```sh
biomcp sync ddinter
biomcp sync ddinter --fresh
```

## sync who-pq

```sh
biomcp sync who-pq
biomcp sync who-pq --fresh
```

## sync gtr

```sh
biomcp sync gtr
biomcp sync gtr --fresh
```

## sync who-ivd

```sh
biomcp sync who-ivd
biomcp sync who-ivd --fresh
```

## sync fda-cdx

```sh
biomcp sync fda-cdx
biomcp sync fda-cdx --fresh
```

## help key ncbi

```sh
biomcp help key ncbi
```

## help key semantic-scholar

```sh
biomcp help key semantic-scholar
```

## help key orcid

```sh
biomcp help key orcid
```

## help key oncokb

```sh
biomcp help key oncokb
```

## help key alphagenome

```sh
biomcp help key alphagenome
```

## help key nci

```sh
biomcp help key nci
```

## help key openfda

```sh
biomcp help key openfda
```

## help network ca_bundle

```sh
biomcp help network ca_bundle
```

## help network proxy

```sh
biomcp help network proxy
```

## help exit

```sh
biomcp help exit
```

## author

```sh
biomcp author papers semantic-scholar:123
```

## search

```sh
biomcp search author 'Louis Williams'
```

## get

```sh
biomcp get author semanticscholar:456 semantic-scholar:123 semantic-scholar:999
```

## cell-line

```sh
biomcp cell-line assay-count CHEMBL3706573
```

## protein

```sh
biomcp protein features P00533
```

## pathway

```sh
biomcp pathway genes R-HSA-5673001
```

## article

```sh
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx
```

## trial

```sh
biomcp trial document NCT04303780 SAP_001.pdf --save protocol.pdf --replace
```

## variant

```sh
biomcp variant assertions clinvar:13961 --source clinvar
```

## store

```sh
biomcp store clean
```

## key

```sh
biomcp key list
```

## help relations

```sh
biomcp help relations
```

