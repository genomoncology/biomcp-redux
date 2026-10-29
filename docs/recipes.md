# Worked commands

Generated from harvested case run steps through the shipped help examples. The documentation check executes these commands against recordings through CLI and MCP. Live source replies can differ.

Choose a command below, inspect its source outcomes and follow its returned next commands. Stored sources require the explicit synchronization shown by their command.

## biomcp

```sh
biomcp help
```

## biomcp search biomarker

```sh
biomcp search biomarker --source nci --name PD-L1
```

## biomcp get biomarker

```sh
biomcp get biomarker C96022
```

## biomcp author papers

```sh
biomcp author papers semantic-scholar:123
biomcp author papers semantic-scholar:123 details
```

## biomcp search author

```sh
biomcp search author 'Louis Williams'
biomcp search author 'Louis Williams' --source semanticscholar
```

## biomcp search organization

```sh
biomcp search organization --source nci --name Dana-Farber --limit 5
biomcp search organization --source nci --city Boston --state MA --limit 5
```

## biomcp get author

```sh
biomcp get author semanticscholar:456 semantic-scholar:123 semantic-scholar:999
biomcp get author semantic-scholar:123 semantic-scholar:123
```

## biomcp cell-line assay-count

```sh
biomcp cell-line assay-count CHEMBL3706573
biomcp cell-line assay-count CHEMBL3706573 --fresh
```

## biomcp search cell-line

```sh
biomcp search cell-line MOLM13
biomcp search cell-line --query 'MV4;11'
```

## biomcp get cell-line

```sh
biomcp get cell-line CVCL_2119
biomcp get cell-line CVCL_0004
```

## biomcp get gwas

```sh
biomcp get gwas association:6114
biomcp get gwas GCST000035
```

## biomcp search gwas

```sh
biomcp search gwas --pubmed 17529967
biomcp search gwas --variant rs2981582
```

## biomcp protein features

```sh
biomcp protein features P00533
biomcp protein features P00533 --limit 1
```

## biomcp protein isoforms

```sh
biomcp protein isoforms P00533
biomcp protein isoforms P00533 --limit 1
```

## biomcp protein domains

```sh
biomcp protein domains P00533
biomcp protein domains P00533 --limit 1
```

## biomcp protein complexes

```sh
biomcp protein complexes P00533
biomcp protein complexes P00533 --limit 1
```

## biomcp protein complex-members

```sh
biomcp protein complex-members P00533 --complex CPX-715
biomcp protein complex-members P00533 --complex CPX-715 --limit 1
```

## biomcp protein structures

```sh
biomcp protein structures P00533
biomcp protein structures P00533 --limit 1
```

## biomcp protein sequence

```sh
biomcp protein sequence P00533
biomcp protein sequence P00533 --fresh
```

## biomcp search protein

```sh
biomcp search protein --gene BRAF
biomcp search protein EGFR
```

## biomcp get protein

```sh
biomcp get protein P00533 Q9UIF7 P38398-2
biomcp get protein EGFR
```

## biomcp search phenotype

```sh
biomcp search phenotype synthetic
biomcp search phenotype synthetic --limit 1
```

## biomcp get phenotype

```sh
biomcp get phenotype HP:0000001
biomcp get phenotype HP:0000001 --source monarch
```

## biomcp search pathway

```sh
biomcp search pathway 'Signaling by EGFR'
biomcp search pathway Hemostasis
```

## biomcp get pathway

```sh
biomcp get pathway R-HSA-9652817
biomcp get pathway R-HSA-177929
```

## biomcp pathway genes

```sh
biomcp pathway genes R-HSA-5673001
biomcp pathway genes R-HSA-5673001 details
```

## biomcp pathway events

```sh
biomcp pathway events R-HSA-5673001
biomcp pathway events R-HSA-5673001 --limit 50
```

## biomcp search study

```sh
biomcp search study Atlas
biomcp search study --cancer-type mixed --limit 1
```

## biomcp get study

```sh
biomcp get study synthetic_0
biomcp get study synthetic_1
```

## biomcp search pgx

```sh
biomcp search pgx --gene DPYD --source cpic
biomcp search pgx --gene DPYD --source cpic --limit 1 --cpic-level B/C
```

## biomcp get pgx

```sh
biomcp get pgx --gene DPYD --source cpic
biomcp get pgx --gene DPYD --source cpic diplotype --diplotype '*1/*2A'
```

## biomcp search diagnostic

```sh
biomcp search diagnostic --source fda-device-recall --manufacturer 'Synthetic maker'
biomcp search diagnostic --source fda-device-recall --product-code ABC
biomcp search diagnostic --source gtr --gene BRCA1
biomcp search diagnostic --source gtr --condition-id MedGen:C2676676
```

## biomcp get diagnostic

```sh
biomcp get diagnostic Z-0001-2024 --source fda-device-recall
biomcp get diagnostic 0466-046-00 --source who-ivd
biomcp get diagnostic GTR000006692.3 --source gtr
```

## biomcp search adverse-event

```sh
biomcp search adverse-event pembrolizumab
biomcp search adverse-event pembrolizumab --received-from 2024-01-01 --received-to 2024-12-31
biomcp search adverse-event --source maude --manufacturer 'Synthetic maker' --product-code ABC --product-code XYZ --problem 'Synthetic problem'
```

## biomcp get adverse-event

```sh
biomcp get adverse-event 100
biomcp get adverse-event 100 reactions
```

## biomcp search disease

```sh
biomcp search disease --source seer --limit 1
biomcp search disease --source seer --limit 1 --offset 1
```

## biomcp get disease

```sh
biomcp get disease 47 incidence --source seer --dataset 4 --period 5 --section-limit 1
biomcp get disease 55 survival --source seer --sex 3 --race 1 --age-range 1 --dataset 4 --interval 5 --section-limit 5
```

## biomcp search drug

```sh
biomcp search drug Tibsovo --source ema --limit 1
biomcp search drug Tibsovo --source ema --limit 1 --offset 1 --fresh
```

## biomcp get drug

```sh
biomcp get drug EMEA/H/C/000406 --source ema
biomcp get drug EMEA/H/C/000406 regulatory --source ema
```

## biomcp get gene

```sh
biomcp get gene KRAS
biomcp get gene BRAF
```

## biomcp search gene

```sh
biomcp search gene KRAS
biomcp search gene p16
```

## biomcp search article

```sh
biomcp search article 'AKT1 E17K'
biomcp search article 'KRAS G12C sotorasib' --since 2024 --sort date --limit 5
```

## biomcp get article

```sh
biomcp get article 33123537
biomcp get article PPR1295425 fulltext
```

## biomcp article supplement

```sh
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx --variant P245S
```

## biomcp article pdf

```sh
biomcp article pdf 41351423 --save owned.pdf --replace
biomcp article pdf 41351423 --save second.pdf --replace
```

## biomcp article text

```sh
biomcp article text PMC5606870 --find AXIN2
biomcp article text PMC5606870 --part 1
```

## biomcp article citations

```sh
biomcp article citations 17611497
biomcp article citations 23000897 --source semantic-scholar
```

## biomcp article references

```sh
biomcp article references 34096690
biomcp article references 33123537 --source semantic-scholar
```

## biomcp article citation-evidence

```sh
biomcp article citation-evidence 33123537 17611497
biomcp article citation-evidence 33123537 17611497 --fulltext
```

## biomcp article recommendations

```sh
biomcp article recommendations 34096690 --limit 5
biomcp article recommendations 34096690 32955176 --negative 33433946 --limit 5
```

## biomcp search trial

```sh
biomcp search trial --condition melanoma
biomcp search trial --source nci --facility 'Dana-Farber Cancer Institute' --limit 1
biomcp search trial --condition melanoma --intervention pembrolizumab
biomcp search trial --condition melanoma --condition 'lung cancer' --intervention pembrolizumab --intervention nivolumab --keyword survival --keyword 'quality of life' --limit 1
```

## biomcp get trial

```sh
biomcp get trial NCT04303780
biomcp get trial NCT04303780 documents
```

## biomcp trial document

```sh
biomcp trial document NCT04303780 SAP_001.pdf --save protocol.pdf --replace
biomcp trial document NCT04303780 SAP_001.pdf --save analysis.pdf --replace
```

## biomcp variant assertions

```sh
biomcp variant assertions clinvar:13961 --source clinvar
biomcp variant assertions CA000434 --source erepo
```

## biomcp get variant

```sh
biomcp get variant clinvar:13961 clinvar
biomcp get variant CA000434 erepo
```

## biomcp variant population

```sh
biomcp variant population CA123643 --detail
biomcp variant population CA123643 --detail --offset 10
```

## biomcp search variant

```sh
biomcp search variant --gene EGFR --protein L858R --build GRCh37
biomcp search variant --gene CHEK2 --protein Arg117Gly --build GRCh37
```

## biomcp variant recurrence

```sh
biomcp variant recurrence 'NC_000007.13:g.140494253G>A' --source cbioportal --gene BRAF --study msk_impact_2017 --profile msk_impact_2017_mutations --sample-set msk_impact_2017_all --limit 3
biomcp variant recurrence 'NC_000007.13:g.140494253G>A' --source cbioportal --gene BRAF --study msk_impact_2017 --profile msk_impact_2017_mutations --sample-set msk_impact_2017_all --limit 3 --page 0
```

## biomcp variant normalize

```sh
biomcp variant normalize --source mutalyzer --hgvs 'NM_004333.4:c.1799T>A'
biomcp variant normalize --source variantvalidator --hgvs 'NM_004333.4:c.1799T>A' --build GRCh38
```

## biomcp skill

```sh
biomcp skill list
```

## biomcp skill list

```sh
biomcp skill list
```

## biomcp skill render

```sh
biomcp skill render
```

## biomcp skill status

```sh
biomcp skill status biomcp-guidance
```

## biomcp skill install

```sh
biomcp skill install biomcp-guidance
```

## biomcp version

```sh
biomcp version
```

## biomcp mcp-config

```sh
biomcp mcp-config
```

## biomcp store clean

```sh
biomcp store clean
```

## biomcp health

```sh
biomcp health --offline
```

## biomcp sync

```sh
biomcp sync hgnc
```

## biomcp help

```sh
biomcp help
```

## biomcp key list

```sh
biomcp key list
```

## biomcp key set

```sh
biomcp key set oncokb
```

## biomcp key remove

```sh
biomcp key remove oncokb
```

## biomcp serve

```sh
biomcp serve
biomcp help serve
```

## biomcp help exit

```sh
biomcp help exit
```

## biomcp search

```sh
biomcp search biomarker --source nci --name PD-L1
```

## biomcp get

```sh
biomcp get biomarker C96022
```

## biomcp author

```sh
biomcp author papers semantic-scholar:123
```

## biomcp cell-line

```sh
biomcp cell-line assay-count CHEMBL3706573
```

## biomcp protein

```sh
biomcp protein features P00533
```

## biomcp pathway

```sh
biomcp pathway genes R-HSA-5673001
```

## biomcp article

```sh
biomcp article supplement 28944238 MGG3-5-553-s002.xlsx
```

## biomcp trial

```sh
biomcp trial document NCT04303780 SAP_001.pdf --save protocol.pdf --replace
```

## biomcp variant

```sh
biomcp variant assertions clinvar:13961 --source clinvar
```

## biomcp store

```sh
biomcp store clean
```

## biomcp key

```sh
biomcp key list
```

## biomcp schema

```sh
biomcp schema
biomcp schema biomcp.help/4
```

## biomcp help search

```sh
biomcp help search
```

## biomcp help search biomarker

```sh
biomcp help search biomarker
```

## biomcp help search author

```sh
biomcp help search author
```

## biomcp help search organization

```sh
biomcp help search organization
```

## biomcp help search cell-line

```sh
biomcp help search cell-line
```

## biomcp help search gwas

```sh
biomcp help search gwas
```

## biomcp help search protein

```sh
biomcp help search protein
```

## biomcp help search phenotype

```sh
biomcp help search phenotype
```

## biomcp help search pathway

```sh
biomcp help search pathway
```

## biomcp help search study

```sh
biomcp help search study
```

## biomcp help search pgx

```sh
biomcp help search pgx
```

## biomcp help search diagnostic

```sh
biomcp help search diagnostic
```

## biomcp help search adverse-event

```sh
biomcp help search adverse-event
```

## biomcp help search disease

```sh
biomcp help search disease
```

## biomcp help search drug

```sh
biomcp help search drug
```

## biomcp help search gene

```sh
biomcp help search gene
```

## biomcp help search article

```sh
biomcp help search article
```

## biomcp help search trial

```sh
biomcp help search trial
```

## biomcp help search variant

```sh
biomcp help search variant
```

## biomcp help get

```sh
biomcp help get
```

## biomcp help get biomarker

```sh
biomcp help get biomarker
```

## biomcp help get author

```sh
biomcp help get author
```

## biomcp help get cell-line

```sh
biomcp help get cell-line
```

## biomcp help get gwas

```sh
biomcp help get gwas
```

## biomcp help get protein

```sh
biomcp help get protein
```

## biomcp help get phenotype

```sh
biomcp help get phenotype
```

## biomcp help get pathway

```sh
biomcp help get pathway
```

## biomcp help get study

```sh
biomcp help get study
```

## biomcp help get pgx

```sh
biomcp help get pgx
```

## biomcp help get diagnostic

```sh
biomcp help get diagnostic
```

## biomcp help get adverse-event

```sh
biomcp help get adverse-event
```

## biomcp help get disease

```sh
biomcp help get disease
```

## biomcp help get drug

```sh
biomcp help get drug
```

## biomcp help get gene

```sh
biomcp help get gene
```

## biomcp help get article

```sh
biomcp help get article
```

## biomcp help get trial

```sh
biomcp help get trial
```

## biomcp help get variant

```sh
biomcp help get variant
```

## biomcp help author

```sh
biomcp help author
```

## biomcp help author papers

```sh
biomcp help author papers
```

## biomcp help cell-line

```sh
biomcp help cell-line
```

## biomcp help cell-line assay-count

```sh
biomcp help cell-line assay-count
```

## biomcp help protein

```sh
biomcp help protein
```

## biomcp help protein features

```sh
biomcp help protein features
```

## biomcp help protein isoforms

```sh
biomcp help protein isoforms
```

## biomcp help protein domains

```sh
biomcp help protein domains
```

## biomcp help protein complexes

```sh
biomcp help protein complexes
```

## biomcp help protein complex-members

```sh
biomcp help protein complex-members
```

## biomcp help protein structures

```sh
biomcp help protein structures
```

## biomcp help protein sequence

```sh
biomcp help protein sequence
```

## biomcp help pathway

```sh
biomcp help pathway
```

## biomcp help pathway genes

```sh
biomcp help pathway genes
```

## biomcp help pathway events

```sh
biomcp help pathway events
```

## biomcp help article

```sh
biomcp help article
```

## biomcp help article supplement

```sh
biomcp help article supplement
```

## biomcp help article pdf

```sh
biomcp help article pdf
```

## biomcp help article text

```sh
biomcp help article text
```

## biomcp help article citations

```sh
biomcp help article citations
```

## biomcp help article references

```sh
biomcp help article references
```

## biomcp help article citation-evidence

```sh
biomcp help article citation-evidence
```

## biomcp help article recommendations

```sh
biomcp help article recommendations
```

## biomcp help trial

```sh
biomcp help trial
```

## biomcp help trial document

```sh
biomcp help trial document
```

## biomcp help variant

```sh
biomcp help variant
```

## biomcp help variant assertions

```sh
biomcp help variant assertions
```

## biomcp help variant population

```sh
biomcp help variant population
```

## biomcp help variant recurrence

```sh
biomcp help variant recurrence
```

## biomcp help variant normalize

```sh
biomcp help variant normalize
```

## biomcp help sync

```sh
biomcp help sync
```

## biomcp help sync hgnc

```sh
biomcp help sync hgnc
```

## biomcp help sync clingen

```sh
biomcp help sync clingen
```

## biomcp help sync gencc

```sh
biomcp help sync gencc
```

## biomcp help sync wikipathways

```sh
biomcp help sync wikipathways
```

## biomcp help sync cbioportal-studies

```sh
biomcp help sync cbioportal-studies
```

## biomcp help sync ema

```sh
biomcp help sync ema
```

## biomcp help sync ddinter

```sh
biomcp help sync ddinter
```

## biomcp help sync who-pq

```sh
biomcp help sync who-pq
```

## biomcp help sync gtr

```sh
biomcp help sync gtr
```

## biomcp help sync who-ivd

```sh
biomcp help sync who-ivd
```

## biomcp help sync fda-cdx

```sh
biomcp help sync fda-cdx
```

## biomcp help skill

```sh
biomcp help skill
```

## biomcp help skill list

```sh
biomcp help skill list
```

## biomcp help skill render

```sh
biomcp help skill render
```

## biomcp help skill status

```sh
biomcp help skill status
```

## biomcp help skill install

```sh
biomcp help skill install
```

## biomcp help version

```sh
biomcp help version
```

## biomcp help mcp-config

```sh
biomcp help mcp-config
```

## biomcp help health

```sh
biomcp help health
```

## biomcp help store

```sh
biomcp help store
```

## biomcp help store clean

```sh
biomcp help store clean
```

## biomcp help key

```sh
biomcp help key
```

## biomcp help key list

```sh
biomcp help key list
```

## biomcp help key set

```sh
biomcp help key set
```

## biomcp help key remove

```sh
biomcp help key remove
```

## biomcp help serve

```sh
biomcp help serve
```

## biomcp help schema

```sh
biomcp help schema
```

## biomcp help help

```sh
biomcp help help
```

## biomcp help network

```sh
biomcp help network
```

## biomcp help sources

```sh
biomcp help sources
```

## biomcp help config

```sh
biomcp help config
```

## biomcp help relations

```sh
biomcp help relations
```

## biomcp help relations

```sh
biomcp help relations
```

