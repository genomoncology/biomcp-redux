---
title: Sources
description: Every data source in BioMCP 1.0.0, with its use group, publisher terms and key.
goal: Find a BioMCP source, its terms, its key and the commands that read it.
reader: A BioMCP user choosing or checking sources
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 registers 56 sources. This table comes from `biomcp --offline health`, which reports the registry without sending requests. The [data sources guide](/sources/) explains use groups and how to turn sources off.

| Source | Use group | Key | Terms |
| --- | --- | --- | --- |
| [cellosaurus](/reference/sources/cellosaurus/) | open |  | [Cellosaurus CC BY 4.0; cite Bairoch (2018)](https://www.cellosaurus.org/description.html) |
| [cpic](/reference/sources/cpic/) | noncommercial |  | [ClinPGx data use policy; noncommercial access restriction; captured policy was a JavaScript shell](https://www.clinpgx.org/page/dataUsagePolicy) |
| [clinpgx](/reference/sources/clinpgx/) | noncommercial |  | [ClinPGx data use policy; noncommercial access restriction; captured policy was a JavaScript shell](https://www.clinpgx.org/page/dataUsagePolicy) |
| [gtr](/reference/sources/gtr/) | open |  | [NCBI usage policies; submitter and third-party rights retained](https://www.ncbi.nlm.nih.gov/home/about/policies/) |
| [who-ivd](/reference/sources/who-ivd/) | noncommercial |  | [WHO terms of use; export has no narrower license; commercial and substantial reproduction reserved](https://www.who.int/about/policies/terms-of-use) |
| [who-pq](/reference/sources/who-pq/) | restricted |  | [WHO copyright policy; export-specific reuse grant unverified](https://www.who.int/about/policies/publishing/copyright) |
| [fda-cdx](/reference/sources/fda-cdx/) | open |  | [FDA website policies; third-party exceptions retained](https://www.fda.gov/about-fda/about-website/website-policies) |
| [fda-pma](/reference/sources/fda-pma/) | open |  | [openFDA terms and license; third-party exceptions](https://open.fda.gov/terms/) |
| [fda-510k](/reference/sources/fda-510k/) | open |  | [openFDA terms and license; third-party exceptions](https://open.fda.gov/terms/) |
| [gwas](/reference/sources/gwas/) | restricted |  | [GWAS Catalog terms and dataset rights](https://www.ebi.ac.uk/about/terms-of-use/) |
| [myvariant](/reference/sources/myvariant/) | restricted |  | [MyVariant.info terms and component data licenses](https://myvariant.info/terms) |
| [car](/reference/sources/car/) | restricted |  | [ClinGen Allele Registry service terms; output rights require review](https://clinicalgenome.org/about/terms-of-use/) |
| [mutalyzer](/reference/sources/mutalyzer/) | restricted |  | [Mutalyzer service output terms require review](https://mutalyzer.nl/) |
| [variantvalidator](/reference/sources/variantvalidator/) | restricted |  | [VariantValidator service output terms require review](https://variantvalidator.org/help/about/) |
| [ctgov](/reference/sources/ctgov/) | restricted |  | [ClinicalTrials.gov attribution and source processing date; publisher terms](https://clinicaltrials.gov/about-site/terms-conditions) |
| [hgnc](/reference/sources/hgnc/) | open |  | [CC0 1.0](https://www.genenames.org/about/license/) |
| [mygene](/reference/sources/mygene/) | restricted |  | [MyGene.info Terms of Use; upstream data terms](https://docs.mygene.info/en/latest/terms.html) |
| [oncokb](/reference/sources/oncokb/) | restricted | `ONCOKB_TOKEN` | [OncoKB license](https://faq.oncokb.org/licensing) |
| [gnomad](/reference/sources/gnomad/) | open |  | [CC0 1.0](https://gnomad.broadinstitute.org/policies) |
| [uniprot](/reference/sources/uniprot/) | attribution |  | [CC BY 4.0](https://www.uniprot.org/help/license) |
| [interpro](/reference/sources/interpro/) | open |  | [CC0](https://interpro-documentation.readthedocs.io/en/latest/license.html) |
| [complex-portal](/reference/sources/complex-portal/) | open |  | [CC0](https://pmc.ncbi.nlm.nih.gov/articles/PMC11701666/) |
| [reactome](/reference/sources/reactome/) | open |  | [CC0 1.0 database data](https://reactome.org/license) |
| [wikipathways](/reference/sources/wikipathways/) | open |  | [CC0 1.0 waiver](https://www.wikipathways.org/help.html) |
| [civic](/reference/sources/civic/) | open |  | [CC0 1.0](https://civicdb.org/releases/licensing) |
| [hotspots](/reference/sources/hotspots/) | restricted |  | [Publisher-specific terms; data reuse unverified](https://www.cancerhotspots.org/) |
| [clinvar](/reference/sources/clinvar/) | restricted |  | [NCBI policy; submission rights remain source-specific](https://www.ncbi.nlm.nih.gov/clinvar/docs/maintenance_use/) |
| [erepo](/reference/sources/erepo/) | restricted |  | [ClinGen sharing policy; record rights unverified](https://clinicalgenome.org/about/terms-of-use/) |
| [clingen](/reference/sources/clingen/) | open |  | [CC0 1.0](https://clinicalgenome.org/docs/terms-of-use/) |
| [hpa](/reference/sources/hpa/) | attribution |  | [CC BY 4.0; third-party conditions](https://www.proteinatlas.org/about/licence) |
| [pubmed](/reference/sources/pubmed/) | restricted | `NCBI_API_KEY` | [NCBI policies; publisher rights in abstracts](https://www.ncbi.nlm.nih.gov/home/about/policies/) |
| [europepmc](/reference/sources/europepmc/) | restricted |  | [Europe PMC copyright; record-specific licenses](https://europepmc.org/help#copyright) |
| [litvar](/reference/sources/litvar/) | open |  | [NCBI public-domain metadata policy](https://www.ncbi.nlm.nih.gov/home/about/policies/) |
| [litsense](/reference/sources/litsense/) | restricted |  | [NCBI policies; publisher rights in input text](https://www.ncbi.nlm.nih.gov/home/about/policies/) |
| [pubtator](/reference/sources/pubtator/) | restricted |  | [NCBI policies; publisher rights in input text](https://www.ncbi.nlm.nih.gov/home/about/policies/) |
| [pmc-oa](/reference/sources/pmc-oa/) | restricted |  | [PMC copyright; article-specific licenses](https://pmc.ncbi.nlm.nih.gov/about/copyright/) |
| [icite](/reference/sources/icite/) | open |  | [NIH public-domain data](https://support.icite.nih.gov/hc/en-us/articles/9066501755291-Citations-Module-Overview) |
| [semantic-scholar](/reference/sources/semantic-scholar/) | noncommercial | `S2_API_KEY` | [Semantic Scholar API License Agreement](https://api.semanticscholar.org/license/) |
| [orcid](/reference/sources/orcid/) | noncommercial | `ORCID_ACCESS_TOKEN` | [ORCID Public API Terms of Service](https://info.orcid.org/public-client-terms-of-service/) |
| [nci](/reference/sources/nci/) | restricted | `NCI_API_KEY` | [NCI Clinical Trials Search publisher terms; user's API key required](https://www.cancer.gov/syndication/api) |
| [cbioportal](/reference/sources/cbioportal/) | restricted |  | [cBioPortal study-specific terms unverified; commercial restrictions may apply](https://docs.cbioportal.org/user-guide/faq/) |
| [mychem](/reference/sources/mychem/) | attribution |  | [BioThings and upstream source terms](https://docs.mychem.info/en/latest/doc/data_source.html) |
| [chembl](/reference/sources/chembl/) | attribution |  | [ChEMBL CC BY-SA 3.0](https://chembl.github.io/chembl-licensing/) |
| [opentargets](/reference/sources/opentargets/) | open |  | [Open Targets CC0; underlying source provenance retained](https://platform-docs.opentargets.org/licence) |
| [openfda-label](/reference/sources/openfda-label/) | open | `OPENFDA_API_KEY` | [openFDA terms; original label rights retained](https://open.fda.gov/terms/) |
| [drugsfda](/reference/sources/drugsfda/) | open |  | [openFDA terms; original label rights retained](https://open.fda.gov/terms/) |
| [ema](/reference/sources/ema/) | attribution |  | [EMA reproduction and attribution terms; third-party exceptions](https://www.ema.europa.eu/en/about-us/about-website/legal-notice) |
| [ddinter](/reference/sources/ddinter/) | noncommercial |  | [DDInter CC BY-NC-SA 4.0](https://ddinter.scbdd.com/terms/) |
| [ols](/reference/sources/ols/) | attribution |  | [EMBL-EBI Terms of Use; ontology-specific rights](https://www.ebi.ac.uk/about/terms-of-use/) |
| [mydisease](/reference/sources/mydisease/) | attribution |  | [MyDisease source-specific terms](https://docs.mydisease.info/en/latest/doc/data.html) |
| [monarch](/reference/sources/monarch/) | attribution |  | [Monarch source-specific licensing](https://monarch-app.monarchinitiative.org/Licensing/) |
| [hpo](/reference/sources/hpo/) | attribution |  | [HPO project custom license: attribution, version and integrity](https://human-phenotype-ontology.github.io/license.html) |
| [seer](/reference/sources/seer/) | open |  | [SEER/NCI Using This Website](https://seer.cancer.gov/about/using-website.html) |
| [faers](/reference/sources/faers/) | open |  | [openFDA terms and license; third-party exceptions](https://open.fda.gov/terms/) |
| [maude](/reference/sources/maude/) | open |  | [openFDA terms and license; third-party exceptions](https://open.fda.gov/terms/) |
| [vaers](/reference/sources/vaers/) | restricted |  | [CDC WONDER data use restrictions](https://wonder.cdc.gov/datause.html) |

## Counts by use group

| Use group | Sources |
| --- | --- |
| `open` | 21 |
| `attribution` | 9 |
| `noncommercial` | 6 |
| `restricted` | 20 |

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
