---
title: Data sources
description: Every data source in BioMCP 1.0.0, with its use group, publisher terms and key.
goal: Find a BioMCP source, its terms, its key and the commands that read it.
reader: A BioMCP user choosing or checking sources
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0, the command-line tool and MCP server that reads biomedical data sources, reads 57 sources. Each belongs to one use group, a label for its terms: 21 `open`, 9 `attribution`, 6 `noncommercial`, 21 `restricted`. The table lists every source with its use group, key and publisher terms, as `biomcp --offline health` reports them without sending a request. The [data sources guide](/sources/) explains use groups and how to turn sources off, and each source links to its own page.

| Source | Use group | Key | Terms |
| --- | --- | --- | --- |
| [cellosaurus](/sources/cellosaurus/) | open |  | [Cellosaurus CC BY 4.0; cite Bairoch (2018)](https://www.cellosaurus.org/description.html) |
| [cpic](/sources/cpic/) | noncommercial |  | [ClinPGx data use policy; noncommercial access restriction; captured policy was a JavaScript shell](https://www.clinpgx.org/page/dataUsagePolicy) |
| [clinpgx](/sources/pharmgkb/) | noncommercial |  | [ClinPGx data use policy; noncommercial access restriction; captured policy was a JavaScript shell](https://www.clinpgx.org/page/dataUsagePolicy) |
| [gtr](/sources/gtr/) | open |  | [NCBI usage policies; submitter and third-party rights retained](https://www.ncbi.nlm.nih.gov/home/about/policies/) |
| [who-ivd](/sources/who-ivd/) | noncommercial |  | [WHO terms of use; export has no narrower license; commercial and substantial reproduction reserved](https://www.who.int/about/policies/terms-of-use) |
| [who-pq](/sources/who-prequalification/) | restricted |  | [WHO copyright policy; export-specific reuse grant unverified](https://www.who.int/about/policies/publishing/copyright) |
| [fda-cdx](/sources/fda-cdx/) | open |  | [FDA website policies; third-party exceptions retained](https://www.fda.gov/about-fda/about-website/website-policies) |
| [fda-pma](/sources/fda-pma/) | open | optional, `OPENFDA_API_KEY` | [openFDA terms and license; third-party exceptions](https://open.fda.gov/terms/) |
| [fda-510k](/sources/fda-510k/) | open | optional, `OPENFDA_API_KEY` | [openFDA terms and license; third-party exceptions](https://open.fda.gov/terms/) |
| [gwas](/sources/gwas/) | restricted |  | [GWAS Catalog terms and dataset rights](https://www.ebi.ac.uk/about/terms-of-use/) |
| [myvariant](/sources/myvariant/) | restricted |  | [MyVariant.info terms and component data licenses](https://myvariant.info/terms) |
| [car](/sources/clingen-allele-registry/) | restricted |  | [ClinGen Allele Registry service terms; output rights require review](https://clinicalgenome.org/about/terms-of-use/) |
| [mutalyzer](/sources/mutalyzer/) | restricted |  | [Mutalyzer service output terms require review](https://mutalyzer.nl/) |
| [variantvalidator](/sources/variantvalidator/) | restricted |  | [VariantValidator service output terms require review](https://variantvalidator.org/help/about/) |
| [ctgov](/sources/clinicaltrials-gov/) | restricted |  | [ClinicalTrials.gov attribution and source processing date; publisher terms](https://clinicaltrials.gov/about-site/terms-conditions) |
| [hgnc](/sources/hgnc/) | open |  | [CC0 1.0](https://www.genenames.org/about/license/) |
| [mygene](/sources/mygene/) | restricted |  | [MyGene.info Terms of Use; upstream data terms](https://docs.mygene.info/en/latest/terms.html) |
| [oncokb](/sources/oncokb/) | restricted | required, `ONCOKB_TOKEN` | [OncoKB license](https://faq.oncokb.org/licensing) |
| [alphagenome](/sources/alphagenome/) | restricted | required, `ALPHAGENOME_API_KEY` | [AlphaGenome API terms](https://alphagenome.google/terms) |
| [gnomad](/sources/gnomad/) | open |  | [CC0 1.0](https://gnomad.broadinstitute.org/policies) |
| [uniprot](/sources/uniprot/) | attribution |  | [CC BY 4.0](https://www.uniprot.org/help/license) |
| [interpro](/sources/interpro/) | open |  | [CC0](https://interpro-documentation.readthedocs.io/en/latest/license.html) |
| [complex-portal](/sources/complex-portal/) | open |  | [CC0](https://pmc.ncbi.nlm.nih.gov/articles/PMC11701666/) |
| [reactome](/sources/reactome/) | open |  | [CC0 1.0 database data](https://reactome.org/license) |
| [wikipathways](/sources/wikipathways/) | open |  | [CC0 1.0 waiver](https://www.wikipathways.org/help.html) |
| [civic](/sources/civic/) | open |  | [CC0 1.0](https://civicdb.org/releases/licensing) |
| [hotspots](/sources/hotspots/) | restricted |  | [Publisher-specific terms; data reuse unverified](https://www.cancerhotspots.org/) |
| [clinvar](/sources/clinvar/) | restricted | optional, `NCBI_API_KEY` | [NCBI policy; submission rights remain source-specific](https://www.ncbi.nlm.nih.gov/clinvar/docs/maintenance_use/) |
| [erepo](/sources/clingen-erepo/) | restricted |  | [ClinGen sharing policy; record rights unverified](https://clinicalgenome.org/about/terms-of-use/) |
| [clingen](/sources/clingen/) | open |  | [CC0 1.0](https://clinicalgenome.org/docs/terms-of-use/) |
| [hpa](/sources/human-protein-atlas/) | attribution |  | [CC BY 4.0; third-party conditions](https://www.proteinatlas.org/about/licence) |
| [pubmed](/sources/pubmed/) | restricted | optional, `NCBI_API_KEY` | [NCBI policies; publisher rights in abstracts](https://www.ncbi.nlm.nih.gov/home/about/policies/) |
| [europepmc](/sources/europepmc/) | restricted |  | [Europe PMC copyright; record-specific licenses](https://europepmc.org/help#copyright) |
| [litvar](/sources/litvar/) | open |  | [NCBI public-domain metadata policy](https://www.ncbi.nlm.nih.gov/home/about/policies/) |
| [litsense](/sources/litsense/) | restricted |  | [NCBI policies; publisher rights in input text](https://www.ncbi.nlm.nih.gov/home/about/policies/) |
| [pubtator](/sources/pubtator/) | restricted |  | [NCBI policies; publisher rights in input text](https://www.ncbi.nlm.nih.gov/home/about/policies/) |
| [pmc-oa](/sources/pmc-oa/) | restricted |  | [PMC copyright; article-specific licenses](https://pmc.ncbi.nlm.nih.gov/about/copyright/) |
| [icite](/sources/icite/) | open |  | [NIH public-domain data](https://support.icite.nih.gov/hc/en-us/articles/9066501755291-Citations-Module-Overview) |
| [semantic-scholar](/sources/semantic-scholar/) | noncommercial | optional, `S2_API_KEY` | [Semantic Scholar API License Agreement](https://api.semanticscholar.org/license/) |
| [orcid](/sources/orcid/) | noncommercial | optional, `ORCID_ACCESS_TOKEN` | [ORCID Public API Terms of Service](https://info.orcid.org/public-client-terms-of-service/) |
| [nci](/sources/nci/) | restricted | required, `NCI_API_KEY` | [NCI Clinical Trials Search publisher terms; user's API key required](https://www.cancer.gov/syndication/api) |
| [cbioportal](/sources/cbioportal/) | restricted |  | [cBioPortal study-specific terms unverified; commercial restrictions may apply](https://docs.cbioportal.org/user-guide/faq/) |
| [mychem](/sources/mychem/) | attribution |  | [BioThings and upstream source terms](https://docs.mychem.info/en/latest/doc/data_source.html) |
| [chembl](/sources/chembl/) | attribution |  | [ChEMBL CC BY-SA 3.0](https://chembl.github.io/chembl-licensing/) |
| [opentargets](/sources/opentargets/) | open |  | [Open Targets CC0; underlying source provenance retained](https://platform-docs.opentargets.org/licence) |
| [openfda-label](/sources/openfda/) | open | optional, `OPENFDA_API_KEY` | [openFDA terms; original label rights retained](https://open.fda.gov/terms/) |
| [drugsfda](/sources/drugsfda/) | open | optional, `OPENFDA_API_KEY` | [openFDA terms; original label rights retained](https://open.fda.gov/terms/) |
| [ema](/sources/ema/) | attribution |  | [EMA reproduction and attribution terms; third-party exceptions](https://www.ema.europa.eu/en/about-us/about-website/legal-notice) |
| [ddinter](/sources/ddinter/) | noncommercial |  | [DDInter CC BY-NC-SA 4.0](https://ddinter.scbdd.com/terms/) |
| [ols](/sources/ols/) | attribution |  | [EMBL-EBI Terms of Use; ontology-specific rights](https://www.ebi.ac.uk/about/terms-of-use/) |
| [mydisease](/sources/mydisease/) | attribution |  | [MyDisease source-specific terms](https://docs.mydisease.info/en/latest/doc/data.html) |
| [monarch](/sources/monarch-initiative/) | attribution |  | [Monarch source-specific licensing](https://monarch-app.monarchinitiative.org/Licensing/) |
| [hpo](/sources/hpo/) | attribution |  | [HPO project custom license: attribution, version and integrity](https://human-phenotype-ontology.github.io/license.html) |
| [seer](/sources/seer/) | open |  | [SEER/NCI Using This Website](https://seer.cancer.gov/about/using-website.html) |
| [faers](/sources/faers/) | open | optional, `OPENFDA_API_KEY` | [openFDA terms and license; third-party exceptions](https://open.fda.gov/terms/) |
| [maude](/sources/maude/) | open | optional, `OPENFDA_API_KEY` | [openFDA terms and license; third-party exceptions](https://open.fda.gov/terms/) |
| [vaers](/sources/vaers/) | restricted |  | [CDC WONDER data use restrictions](https://wonder.cdc.gov/datause.html) |

## Next

The [data sources guide](/sources/) shows how to check that sources answer and how to turn a use group off.
