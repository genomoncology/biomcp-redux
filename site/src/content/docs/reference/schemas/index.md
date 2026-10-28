---
title: Reply schemas
description: Every reply schema in BioMCP 1.0.0, with the commands that return it.
goal: Find the JSON schema of a BioMCP reply and the commands that return it.
reader: A developer or agent parsing BioMCP JSON
kind: reference
verified: 2026-10-28
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. Its `schema` field names the reply schema, such as `biomcp.gene_cards/4`, so code that parses replies can pick the right fields. Each schema below has a page with a real reply and its fields. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/). `biomcp schema ID` prints one schema as JSON Schema.

| Schema | Returned by |
| --- | --- |
| [`biomcp.adverse_event_devices/1`](/reference/schemas/biomcp-adverse-event-devices-1/) | [`get adverse-event devices`](/reference/commands/get-adverse-event/) |
| [`biomcp.adverse_event_drugs/1`](/reference/schemas/biomcp-adverse-event-drugs-1/) | [`get adverse-event drugs`](/reference/commands/get-adverse-event/) |
| [`biomcp.adverse_event_faers_card/1`](/reference/schemas/biomcp-adverse-event-faers-card-1/) | [`get adverse-event`](/reference/commands/get-adverse-event/), [`get adverse-event drugs`](/reference/commands/get-adverse-event/) |
| [`biomcp.adverse_event_identifiers/1`](/reference/schemas/biomcp-adverse-event-identifiers-1/) |  |
| [`biomcp.adverse_event_maude_card/1`](/reference/schemas/biomcp-adverse-event-maude-card-1/) | [`get adverse-event`](/reference/commands/get-adverse-event/) |
| [`biomcp.adverse_event_narrative/1`](/reference/schemas/biomcp-adverse-event-narrative-1/) | [`get adverse-event narrative`](/reference/commands/get-adverse-event/) |
| [`biomcp.adverse_event_page/2`](/reference/schemas/biomcp-adverse-event-page-2/) | [`search adverse-event`](/reference/commands/search-adverse-event/) |
| [`biomcp.adverse_event_reactions/1`](/reference/schemas/biomcp-adverse-event-reactions-1/) | [`get adverse-event reactions`](/reference/commands/get-adverse-event/) |
| [`biomcp.article_cards/3`](/reference/schemas/biomcp-article-cards-3/) | [`get article`](/reference/commands/get-article/), [`get article indexing`](/reference/commands/get-article/) |
| [`biomcp.article_edges/4`](/reference/schemas/biomcp-article-edges-4/) | [`article references`](/reference/commands/article-references/), [`article citations`](/reference/commands/article-citations/) |
| [`biomcp.article_evidence/2`](/reference/schemas/biomcp-article-evidence-2/) | [`article citation-evidence`](/reference/commands/article-citation-evidence/) |
| [`biomcp.article_page/3`](/reference/schemas/biomcp-article-page-3/) | [`search article`](/reference/commands/search-article/) |
| [`biomcp.article_pdf/2`](/reference/schemas/biomcp-article-pdf-2/) | [`article pdf`](/reference/commands/article-pdf/) |
| [`biomcp.article_recommendations/3`](/reference/schemas/biomcp-article-recommendations-3/) | [`article recommendations`](/reference/commands/article-recommendations/) |
| [`biomcp.article_supplement/3`](/reference/schemas/biomcp-article-supplement-3/) | [`article supplement`](/reference/commands/article-supplement/) |
| [`biomcp.article_text/3`](/reference/schemas/biomcp-article-text-3/) | [`article text`](/reference/commands/article-text/) |
| [`biomcp.author_card/3`](/reference/schemas/biomcp-author-card-3/) | [`get author`](/reference/commands/get-author/) |
| [`biomcp.author_page/2`](/reference/schemas/biomcp-author-page-2/) | [`search author`](/reference/commands/search-author/) |
| [`biomcp.author_paper_details/1`](/reference/schemas/biomcp-author-paper-details-1/) | [`author papers`](/reference/commands/author-papers/) |
| [`biomcp.author_papers/1`](/reference/schemas/biomcp-author-papers-1/) | [`author papers`](/reference/commands/author-papers/) |
| [`biomcp.cell_line_assay_count/2`](/reference/schemas/biomcp-cell-line-assay-count-2/) | [`cell-line assay-count`](/reference/commands/cell-line-assay-count/) |
| [`biomcp.cell_line_card/3`](/reference/schemas/biomcp-cell-line-card-3/) | [`get cell-line`](/reference/commands/get-cell-line/) |
| [`biomcp.cell_line_chembl/2`](/reference/schemas/biomcp-cell-line-chembl-2/) | [`get cell-line chembl`](/reference/commands/get-cell-line/) |
| [`biomcp.cell_line_identity/1`](/reference/schemas/biomcp-cell-line-identity-1/) | [`get cell-line identity`](/reference/commands/get-cell-line/) |
| [`biomcp.cell_line_names/1`](/reference/schemas/biomcp-cell-line-names-1/) | [`get cell-line names`](/reference/commands/get-cell-line/) |
| [`biomcp.cell_line_page/2`](/reference/schemas/biomcp-cell-line-page-2/) | [`search cell-line`](/reference/commands/search-cell-line/) |
| [`biomcp.cell_line_variants/2`](/reference/schemas/biomcp-cell-line-variants-2/) | [`get cell-line variants`](/reference/commands/get-cell-line/) |
| [`biomcp.cell_line_xrefs/1`](/reference/schemas/biomcp-cell-line-xrefs-1/) | [`get cell-line xrefs`](/reference/commands/get-cell-line/) |
| [`biomcp.diagnostic_card/1`](/reference/schemas/biomcp-diagnostic-card-1/) | [`get diagnostic`](/reference/commands/get-diagnostic/) |
| [`biomcp.diagnostic_companion/1`](/reference/schemas/biomcp-diagnostic-companion-1/) | [`search diagnostic`](/reference/commands/search-diagnostic/) |
| [`biomcp.diagnostic_conditions/2`](/reference/schemas/biomcp-diagnostic-conditions-2/) | [`get diagnostic conditions`](/reference/commands/get-diagnostic/) |
| [`biomcp.diagnostic_genes/2`](/reference/schemas/biomcp-diagnostic-genes-2/) | [`get diagnostic genes`](/reference/commands/get-diagnostic/) |
| [`biomcp.diagnostic_methods/2`](/reference/schemas/biomcp-diagnostic-methods-2/) | [`get diagnostic methods`](/reference/commands/get-diagnostic/) |
| [`biomcp.diagnostic_page/2`](/reference/schemas/biomcp-diagnostic-page-2/) | [`search diagnostic`](/reference/commands/search-diagnostic/) |
| [`biomcp.diagnostic_regulatory/1`](/reference/schemas/biomcp-diagnostic-regulatory-1/) | [`get diagnostic regulatory`](/reference/commands/get-diagnostic/) |
| [`biomcp.diagnostic_supplements/1`](/reference/schemas/biomcp-diagnostic-supplements-1/) | [`get diagnostic supplements`](/reference/commands/get-diagnostic/) |
| [`biomcp.disease_card/2`](/reference/schemas/biomcp-disease-card-2/) | [`get disease`](/reference/commands/get-disease/) |
| [`biomcp.disease_genes/5`](/reference/schemas/biomcp-disease-genes-5/) | [`get disease genes`](/reference/commands/get-disease/) |
| [`biomcp.disease_hierarchy/2`](/reference/schemas/biomcp-disease-hierarchy-2/) | [`get disease hierarchy`](/reference/commands/get-disease/) |
| [`biomcp.disease_incidence/3`](/reference/schemas/biomcp-disease-incidence-3/) | [`get disease incidence`](/reference/commands/get-disease/), [`get disease`](/reference/commands/get-disease/) |
| [`biomcp.disease_page/1`](/reference/schemas/biomcp-disease-page-1/) | [`search disease`](/reference/commands/search-disease/) |
| [`biomcp.disease_phenotypes/5`](/reference/schemas/biomcp-disease-phenotypes-5/) | [`get disease phenotypes`](/reference/commands/get-disease/) |
| [`biomcp.disease_sites/2`](/reference/schemas/biomcp-disease-sites-2/) | [`search disease`](/reference/commands/search-disease/) |
| [`biomcp.disease_survival/3`](/reference/schemas/biomcp-disease-survival-3/) | [`get disease survival`](/reference/commands/get-disease/), [`get disease`](/reference/commands/get-disease/) |
| [`biomcp.disease_synonyms/2`](/reference/schemas/biomcp-disease-synonyms-2/) | [`get disease synonyms`](/reference/commands/get-disease/) |
| [`biomcp.disease_xrefs/2`](/reference/schemas/biomcp-disease-xrefs-2/) | [`get disease xrefs`](/reference/commands/get-disease/) |
| [`biomcp.drug_approvals/3`](/reference/schemas/biomcp-drug-approvals-3/) | [`get drug approvals`](/reference/commands/get-drug/) |
| [`biomcp.drug_card/2`](/reference/schemas/biomcp-drug-card-2/) | [`get drug`](/reference/commands/get-drug/) |
| [`biomcp.drug_civic/4`](/reference/schemas/biomcp-drug-civic-4/) | [`get drug civic`](/reference/commands/get-drug/) |
| [`biomcp.drug_fda_page/1`](/reference/schemas/biomcp-drug-fda-page-1/) | [`search drug`](/reference/commands/search-drug/) |
| [`biomcp.drug_indications/3`](/reference/schemas/biomcp-drug-indications-3/) | [`get drug indications`](/reference/commands/get-drug/) |
| [`biomcp.drug_interactions/3`](/reference/schemas/biomcp-drug-interactions-3/) | [`get drug interactions`](/reference/commands/get-drug/) |
| [`biomcp.drug_label/3`](/reference/schemas/biomcp-drug-label-3/) | [`get drug label`](/reference/commands/get-drug/) |
| [`biomcp.drug_page/3`](/reference/schemas/biomcp-drug-page-3/) | [`search drug`](/reference/commands/search-drug/) |
| [`biomcp.drug_regulatory/2`](/reference/schemas/biomcp-drug-regulatory-2/) | [`get drug regulatory`](/reference/commands/get-drug/), [`get drug`](/reference/commands/get-drug/) |
| [`biomcp.drug_targets/3`](/reference/schemas/biomcp-drug-targets-3/) | [`get drug targets`](/reference/commands/get-drug/) |
| [`biomcp.drug_who_card/1`](/reference/schemas/biomcp-drug-who-card-1/) | [`get drug`](/reference/commands/get-drug/) |
| [`biomcp.drug_who_page/1`](/reference/schemas/biomcp-drug-who-page-1/) | [`search drug`](/reference/commands/search-drug/) |
| [`biomcp.drug_xrefs/1`](/reference/schemas/biomcp-drug-xrefs-1/) | [`get drug xrefs`](/reference/commands/get-drug/) |
| [`biomcp.gene_cards/4`](/reference/schemas/biomcp-gene-cards-4/) | [`get gene constraint`](/reference/commands/get-gene/), [`get gene`](/reference/commands/get-gene/) |
| [`biomcp.gene_page/3`](/reference/schemas/biomcp-gene-page-3/) | [`search gene`](/reference/commands/search-gene/) |
| [`biomcp.gwas_ancestries/1`](/reference/schemas/biomcp-gwas-ancestries-1/) | [`get gwas ancestries`](/reference/commands/get-gwas/) |
| [`biomcp.gwas_association/1`](/reference/schemas/biomcp-gwas-association-1/) | [`get gwas`](/reference/commands/get-gwas/) |
| [`biomcp.gwas_loci/1`](/reference/schemas/biomcp-gwas-loci-1/) | [`get gwas loci`](/reference/commands/get-gwas/) |
| [`biomcp.gwas_page/3`](/reference/schemas/biomcp-gwas-page-3/) | [`search gwas`](/reference/commands/search-gwas/), [`get gwas associations`](/reference/commands/get-gwas/) |
| [`biomcp.gwas_study/1`](/reference/schemas/biomcp-gwas-study-1/) | [`get gwas`](/reference/commands/get-gwas/) |
| [`biomcp.health/2`](/reference/schemas/biomcp-health-2/) | [`health`](/reference/commands/health/) |
| [`biomcp.help/2`](/reference/schemas/biomcp-help-2/) | [`help`](/reference/commands/help/), [`help network proxy`](/reference/commands/help-network-proxy/), [`help network ca_bundle`](/reference/commands/help-network-ca_bundle/) |
| [`biomcp.key/2`](/reference/schemas/biomcp-key-2/) | [`help key oncokb`](/reference/commands/help-key-oncokb/), [`help key ncbi`](/reference/commands/help-key-ncbi/), [`help key semantic-scholar`](/reference/commands/help-key-semantic-scholar/), [`help key orcid`](/reference/commands/help-key-orcid/), [`help key alphagenome`](/reference/commands/help-key-alphagenome/), [`help key nci`](/reference/commands/help-key-nci/), [`help key openfda`](/reference/commands/help-key-openfda/) |
| [`biomcp.nci_trial_cards/2`](/reference/schemas/biomcp-nci-trial-cards-2/) | [`get trial`](/reference/commands/get-trial/) |
| [`biomcp.nci_trial_page/3`](/reference/schemas/biomcp-nci-trial-page-3/) | [`search trial`](/reference/commands/search-trial/) |
| [`biomcp.orcid_works/2`](/reference/schemas/biomcp-orcid-works-2/) | [`author papers`](/reference/commands/author-papers/) |
| [`biomcp.pathway_card/5`](/reference/schemas/biomcp-pathway-card-5/) | [`get pathway summary`](/reference/commands/get-pathway/), [`get pathway`](/reference/commands/get-pathway/) |
| [`biomcp.pathway_events/2`](/reference/schemas/biomcp-pathway-events-2/) | [`pathway events`](/reference/commands/pathway-events/) |
| [`biomcp.pathway_genes/4`](/reference/schemas/biomcp-pathway-genes-4/) | [`pathway genes`](/reference/commands/pathway-genes/) |
| [`biomcp.pathway_page/3`](/reference/schemas/biomcp-pathway-page-3/) | [`search pathway`](/reference/commands/search-pathway/) |
| [`biomcp.pgx_annotation/1`](/reference/schemas/biomcp-pgx-annotation-1/) | [`get pgx`](/reference/commands/get-pgx/) |
| [`biomcp.pgx_annotations/1`](/reference/schemas/biomcp-pgx-annotations-1/) | [`get pgx annotations`](/reference/commands/get-pgx/) |
| [`biomcp.pgx_card/1`](/reference/schemas/biomcp-pgx-card-1/) | [`get pgx`](/reference/commands/get-pgx/) |
| [`biomcp.pgx_diplotype/1`](/reference/schemas/biomcp-pgx-diplotype-1/) | [`get pgx diplotype`](/reference/commands/get-pgx/), [`get pgx`](/reference/commands/get-pgx/) |
| [`biomcp.pgx_guidelines/1`](/reference/schemas/biomcp-pgx-guidelines-1/) | [`get pgx guidelines`](/reference/commands/get-pgx/) |
| [`biomcp.pgx_page/2`](/reference/schemas/biomcp-pgx-page-2/) | [`search pgx`](/reference/commands/search-pgx/) |
| [`biomcp.pgx_recommendations/1`](/reference/schemas/biomcp-pgx-recommendations-1/) | [`get pgx recommendations`](/reference/commands/get-pgx/) |
| [`biomcp.phenotype-card/1`](/reference/schemas/biomcp-phenotype-card-1/) | [`get phenotype`](/reference/commands/get-phenotype/) |
| [`biomcp.phenotype-diseases/3`](/reference/schemas/biomcp-phenotype-diseases-3/) | [`get phenotype diseases`](/reference/commands/get-phenotype/) |
| [`biomcp.phenotype-genes/3`](/reference/schemas/biomcp-phenotype-genes-3/) | [`get phenotype genes`](/reference/commands/get-phenotype/) |
| [`biomcp.phenotype-hierarchy/1`](/reference/schemas/biomcp-phenotype-hierarchy-1/) | [`get phenotype hierarchy`](/reference/commands/get-phenotype/) |
| [`biomcp.phenotype-search/1`](/reference/schemas/biomcp-phenotype-search-1/) | [`search phenotype`](/reference/commands/search-phenotype/) |
| [`biomcp.phenotype-synonyms/1`](/reference/schemas/biomcp-phenotype-synonyms-1/) | [`get phenotype synonyms`](/reference/commands/get-phenotype/) |
| [`biomcp.phenotype-xrefs/1`](/reference/schemas/biomcp-phenotype-xrefs-1/) | [`get phenotype xrefs`](/reference/commands/get-phenotype/) |
| [`biomcp.protein_card/3`](/reference/schemas/biomcp-protein-card-3/) | [`get protein`](/reference/commands/get-protein/) |
| [`biomcp.protein_complex_members/1`](/reference/schemas/biomcp-protein-complex-members-1/) | [`protein complex-members`](/reference/commands/protein-complex-members/) |
| [`biomcp.protein_complexes/2`](/reference/schemas/biomcp-protein-complexes-2/) | [`protein complexes`](/reference/commands/protein-complexes/) |
| [`biomcp.protein_domains/2`](/reference/schemas/biomcp-protein-domains-2/) | [`protein domains`](/reference/commands/protein-domains/) |
| [`biomcp.protein_features/1`](/reference/schemas/biomcp-protein-features-1/) | [`protein features`](/reference/commands/protein-features/) |
| [`biomcp.protein_function/2`](/reference/schemas/biomcp-protein-function-2/) |  |
| [`biomcp.protein_isoforms/1`](/reference/schemas/biomcp-protein-isoforms-1/) | [`protein isoforms`](/reference/commands/protein-isoforms/) |
| [`biomcp.protein_page/1`](/reference/schemas/biomcp-protein-page-1/) | [`search protein`](/reference/commands/search-protein/) |
| [`biomcp.protein_sequence/1`](/reference/schemas/biomcp-protein-sequence-1/) | [`protein sequence`](/reference/commands/protein-sequence/) |
| [`biomcp.protein_structures/1`](/reference/schemas/biomcp-protein-structures-1/) | [`protein structures`](/reference/commands/protein-structures/) |
| [`biomcp.registered-keys/2`](/reference/schemas/biomcp-registered-keys-2/) | [`key list`](/reference/commands/key-list/) |
| [`biomcp.store-clean/1`](/reference/schemas/biomcp-store-clean-1/) | [`store clean`](/reference/commands/store-clean/) |
| [`biomcp.study_card/2`](/reference/schemas/biomcp-study-card-2/) | [`get study`](/reference/commands/get-study/) |
| [`biomcp.study_files/1`](/reference/schemas/biomcp-study-files-1/) | [`get study files`](/reference/commands/get-study/) |
| [`biomcp.study_page/1`](/reference/schemas/biomcp-study-page-1/) | [`search study`](/reference/commands/search-study/) |
| [`biomcp.study_profiles/1`](/reference/schemas/biomcp-study-profiles-1/) | [`get study profiles`](/reference/commands/get-study/) |
| [`biomcp.study_sample_lists/1`](/reference/schemas/biomcp-study-sample-lists-1/) | [`get study sample-lists`](/reference/commands/get-study/) |
| [`biomcp.sync/4`](/reference/schemas/biomcp-sync-4/) | [`sync`](/reference/commands/sync/), [`sync cbioportal-studies`](/reference/commands/sync-cbioportal-studies/), [`sync ema`](/reference/commands/sync-ema/), [`sync ddinter`](/reference/commands/sync-ddinter/), [`sync who-pq`](/reference/commands/sync-who-pq/), [`sync gtr`](/reference/commands/sync-gtr/), [`sync who-ivd`](/reference/commands/sync-who-ivd/), [`sync fda-cdx`](/reference/commands/sync-fda-cdx/) |
| [`biomcp.trial_cards/2`](/reference/schemas/biomcp-trial-cards-2/) | [`get trial eligibility`](/reference/commands/get-trial/), [`get trial`](/reference/commands/get-trial/) |
| [`biomcp.trial_document/2`](/reference/schemas/biomcp-trial-document-2/) | [`trial document`](/reference/commands/trial-document/) |
| [`biomcp.trial_page/5`](/reference/schemas/biomcp-trial-page-5/) | [`search trial`](/reference/commands/search-trial/) |
| [`biomcp.vaers_summary/1`](/reference/schemas/biomcp-vaers-summary-1/) | [`search adverse-event`](/reference/commands/search-adverse-event/) |
| [`biomcp.variant_assertions/3`](/reference/schemas/biomcp-variant-assertions-3/) | [`variant assertions`](/reference/commands/variant-assertions/) |
| [`biomcp.variant_card/5`](/reference/schemas/biomcp-variant-card-5/) | [`get variant`](/reference/commands/get-variant/), [`get variant civic`](/reference/commands/get-variant/) |
| [`biomcp.variant_normalization/3`](/reference/schemas/biomcp-variant-normalization-3/) | [`variant normalize`](/reference/commands/variant-normalize/) |
| [`biomcp.variant_page/6`](/reference/schemas/biomcp-variant-page-6/) | [`search variant`](/reference/commands/search-variant/) |
| [`biomcp.variant_population/4`](/reference/schemas/biomcp-variant-population-4/) | [`variant population`](/reference/commands/variant-population/) |
| [`biomcp.variant_recurrence/4`](/reference/schemas/biomcp-variant-recurrence-4/) | [`variant recurrence`](/reference/commands/variant-recurrence/) |
| [`biomcp.wikipathways_sync/1`](/reference/schemas/biomcp-wikipathways-sync-1/) | [`sync wikipathways`](/reference/commands/sync-wikipathways/) |

## Next

[Results](/results/) explains how to read a reply. [Commands](/reference/commands/) lists every command.
