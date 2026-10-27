---
title: Reply schemas
description: Every reply schema in BioMCP 1.0.0, with its result type and the
  commands that return it.
goal: Find the JSON schema of a BioMCP reply and the commands that return it.
reader: A developer or agent parsing BioMCP JSON
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 has 119 reply schemas. Every reply names one in its `schema` field, such as `biomcp.gene_cards/4`. `biomcp schema ID` prints one as JSON Schema, and `biomcp version --verbose` lists every schema ID, including the types the replies share.

| Schema | Result type | Returned by |
| --- | --- | --- |
| [`biomcp.adverse_event_devices/1`](/reference/schemas/biomcp-adverse-event-devices-1/) | AdverseEventDevices | [`get adverse-event`](/reference/commands/get-adverse-event/) |
| [`biomcp.adverse_event_drugs/1`](/reference/schemas/biomcp-adverse-event-drugs-1/) | AdverseEventDrugs | [`get adverse-event`](/reference/commands/get-adverse-event/) |
| [`biomcp.adverse_event_faers_card/1`](/reference/schemas/biomcp-adverse-event-faers-card-1/) | AdverseEventFaersCard | [`get adverse-event`](/reference/commands/get-adverse-event/) |
| [`biomcp.adverse_event_identifiers/1`](/reference/schemas/biomcp-adverse-event-identifiers-1/) | AdverseEventIdentifiers |  |
| [`biomcp.adverse_event_maude_card/1`](/reference/schemas/biomcp-adverse-event-maude-card-1/) | AdverseEventMaudeCard | [`get adverse-event`](/reference/commands/get-adverse-event/) |
| [`biomcp.adverse_event_narrative/1`](/reference/schemas/biomcp-adverse-event-narrative-1/) | AdverseEventNarrative | [`get adverse-event`](/reference/commands/get-adverse-event/) |
| [`biomcp.adverse_event_page/2`](/reference/schemas/biomcp-adverse-event-page-2/) | AdverseEventPage | [`search adverse-event`](/reference/commands/search-adverse-event/) |
| [`biomcp.adverse_event_reactions/1`](/reference/schemas/biomcp-adverse-event-reactions-1/) | AdverseEventReactions | [`get adverse-event`](/reference/commands/get-adverse-event/) |
| [`biomcp.article_cards/3`](/reference/schemas/biomcp-article-cards-3/) | ArticleCards | [`get article`](/reference/commands/get-article/) |
| [`biomcp.article_edges/4`](/reference/schemas/biomcp-article-edges-4/) | ArticleEdgePage | [`article citations`](/reference/commands/article-citations/), [`article references`](/reference/commands/article-references/) |
| [`biomcp.article_evidence/2`](/reference/schemas/biomcp-article-evidence-2/) | ArticleEvidenceResult | [`article citation-evidence`](/reference/commands/article-citation-evidence/) |
| [`biomcp.article_page/3`](/reference/schemas/biomcp-article-page-3/) | ArticlePage | [`search article`](/reference/commands/search-article/) |
| [`biomcp.article_pdf/2`](/reference/schemas/biomcp-article-pdf-2/) | ArticlePdfResult | [`article pdf`](/reference/commands/article-pdf/) |
| [`biomcp.article_recommendations/3`](/reference/schemas/biomcp-article-recommendations-3/) | ArticleRecommendations | [`article recommendations`](/reference/commands/article-recommendations/) |
| [`biomcp.article_supplement/3`](/reference/schemas/biomcp-article-supplement-3/) | ArticleSupplementResult | [`article supplement`](/reference/commands/article-supplement/) |
| [`biomcp.article_text/3`](/reference/schemas/biomcp-article-text-3/) | ArticleTextResult | [`article text`](/reference/commands/article-text/) |
| [`biomcp.author_card/3`](/reference/schemas/biomcp-author-card-3/) | AuthorCards | [`get author`](/reference/commands/get-author/) |
| [`biomcp.author_page/2`](/reference/schemas/biomcp-author-page-2/) | AuthorPage | [`search author`](/reference/commands/search-author/) |
| [`biomcp.author_paper_details/1`](/reference/schemas/biomcp-author-paper-details-1/) | AuthorPaperDetails | [`author papers`](/reference/commands/author-papers/) |
| [`biomcp.author_papers/1`](/reference/schemas/biomcp-author-papers-1/) | AuthorPapers | [`author papers`](/reference/commands/author-papers/) |
| [`biomcp.cell_line_assay_count/2`](/reference/schemas/biomcp-cell-line-assay-count-2/) | CellLineAssayCount | [`cell-line assay-count`](/reference/commands/cell-line-assay-count/) |
| [`biomcp.cell_line_card/3`](/reference/schemas/biomcp-cell-line-card-3/) | CellLineCard | [`get cell-line`](/reference/commands/get-cell-line/) |
| [`biomcp.cell_line_chembl/2`](/reference/schemas/biomcp-cell-line-chembl-2/) | CellLineChembl | [`get cell-line`](/reference/commands/get-cell-line/) |
| [`biomcp.cell_line_identity/1`](/reference/schemas/biomcp-cell-line-identity-1/) | CellLineIdentityPage | [`get cell-line`](/reference/commands/get-cell-line/) |
| [`biomcp.cell_line_names/1`](/reference/schemas/biomcp-cell-line-names-1/) | CellLineNames | [`get cell-line`](/reference/commands/get-cell-line/) |
| [`biomcp.cell_line_page/2`](/reference/schemas/biomcp-cell-line-page-2/) | CellLinePage | [`search cell-line`](/reference/commands/search-cell-line/) |
| [`biomcp.cell_line_variants/2`](/reference/schemas/biomcp-cell-line-variants-2/) | CellLineVariants | [`get cell-line`](/reference/commands/get-cell-line/) |
| [`biomcp.cell_line_xrefs/1`](/reference/schemas/biomcp-cell-line-xrefs-1/) | CellLineXrefs | [`get cell-line`](/reference/commands/get-cell-line/) |
| [`biomcp.diagnostic_card/1`](/reference/schemas/biomcp-diagnostic-card-1/) | DiagnosticCard | [`get diagnostic`](/reference/commands/get-diagnostic/) |
| [`biomcp.diagnostic_companion/1`](/reference/schemas/biomcp-diagnostic-companion-1/) | DiagnosticCompanion | [`search diagnostic`](/reference/commands/search-diagnostic/) |
| [`biomcp.diagnostic_conditions/2`](/reference/schemas/biomcp-diagnostic-conditions-2/) | DiagnosticConditions | [`get diagnostic`](/reference/commands/get-diagnostic/) |
| [`biomcp.diagnostic_genes/2`](/reference/schemas/biomcp-diagnostic-genes-2/) | DiagnosticGenes | [`get diagnostic`](/reference/commands/get-diagnostic/) |
| [`biomcp.diagnostic_methods/2`](/reference/schemas/biomcp-diagnostic-methods-2/) | DiagnosticMethods | [`get diagnostic`](/reference/commands/get-diagnostic/) |
| [`biomcp.diagnostic_page/2`](/reference/schemas/biomcp-diagnostic-page-2/) | DiagnosticPage | [`search diagnostic`](/reference/commands/search-diagnostic/) |
| [`biomcp.diagnostic_regulatory/1`](/reference/schemas/biomcp-diagnostic-regulatory-1/) | DiagnosticRegulatory | [`get diagnostic`](/reference/commands/get-diagnostic/) |
| [`biomcp.diagnostic_supplements/1`](/reference/schemas/biomcp-diagnostic-supplements-1/) | DiagnosticSupplements | [`get diagnostic`](/reference/commands/get-diagnostic/) |
| [`biomcp.disease_card/2`](/reference/schemas/biomcp-disease-card-2/) | DiseaseCard | [`get disease`](/reference/commands/get-disease/) |
| [`biomcp.disease_genes/5`](/reference/schemas/biomcp-disease-genes-5/) | DiseaseGenes | [`get disease`](/reference/commands/get-disease/) |
| [`biomcp.disease_hierarchy/2`](/reference/schemas/biomcp-disease-hierarchy-2/) | DiseaseHierarchy | [`get disease`](/reference/commands/get-disease/) |
| [`biomcp.disease_incidence/3`](/reference/schemas/biomcp-disease-incidence-3/) | DiseaseIncidence | [`get disease`](/reference/commands/get-disease/) |
| [`biomcp.disease_page/1`](/reference/schemas/biomcp-disease-page-1/) | DiseasePage | [`search disease`](/reference/commands/search-disease/) |
| [`biomcp.disease_phenotypes/5`](/reference/schemas/biomcp-disease-phenotypes-5/) | DiseasePhenotypes | [`get disease`](/reference/commands/get-disease/) |
| [`biomcp.disease_sites/2`](/reference/schemas/biomcp-disease-sites-2/) | DiseaseSites | [`search disease`](/reference/commands/search-disease/) |
| [`biomcp.disease_survival/3`](/reference/schemas/biomcp-disease-survival-3/) | DiseaseSurvival | [`get disease`](/reference/commands/get-disease/) |
| [`biomcp.disease_synonyms/2`](/reference/schemas/biomcp-disease-synonyms-2/) | DiseaseSynonyms | [`get disease`](/reference/commands/get-disease/) |
| [`biomcp.disease_xrefs/2`](/reference/schemas/biomcp-disease-xrefs-2/) | DiseaseXrefs | [`get disease`](/reference/commands/get-disease/) |
| [`biomcp.drug_approvals/3`](/reference/schemas/biomcp-drug-approvals-3/) | DrugApprovals | [`get drug`](/reference/commands/get-drug/) |
| [`biomcp.drug_card/2`](/reference/schemas/biomcp-drug-card-2/) | DrugCard | [`get drug`](/reference/commands/get-drug/) |
| [`biomcp.drug_civic/4`](/reference/schemas/biomcp-drug-civic-4/) | DrugCivic | [`get drug`](/reference/commands/get-drug/) |
| [`biomcp.drug_fda_page/1`](/reference/schemas/biomcp-drug-fda-page-1/) | DrugFdaPage | [`search drug`](/reference/commands/search-drug/) |
| [`biomcp.drug_indications/3`](/reference/schemas/biomcp-drug-indications-3/) | DrugIndications | [`get drug`](/reference/commands/get-drug/) |
| [`biomcp.drug_interactions/3`](/reference/schemas/biomcp-drug-interactions-3/) | DrugInteractions | [`get drug`](/reference/commands/get-drug/) |
| [`biomcp.drug_label/3`](/reference/schemas/biomcp-drug-label-3/) | DrugLabel | [`get drug`](/reference/commands/get-drug/) |
| [`biomcp.drug_page/3`](/reference/schemas/biomcp-drug-page-3/) | DrugPage | [`search drug`](/reference/commands/search-drug/) |
| [`biomcp.drug_regulatory/2`](/reference/schemas/biomcp-drug-regulatory-2/) | DrugRegulatory | [`get drug`](/reference/commands/get-drug/) |
| [`biomcp.drug_targets/3`](/reference/schemas/biomcp-drug-targets-3/) | DrugTargets | [`get drug`](/reference/commands/get-drug/) |
| [`biomcp.drug_who_card/1`](/reference/schemas/biomcp-drug-who-card-1/) | DrugWhoCard | [`get drug`](/reference/commands/get-drug/) |
| [`biomcp.drug_who_page/1`](/reference/schemas/biomcp-drug-who-page-1/) | DrugWhoPage | [`search drug`](/reference/commands/search-drug/) |
| [`biomcp.drug_xrefs/1`](/reference/schemas/biomcp-drug-xrefs-1/) | DrugXrefs | [`get drug`](/reference/commands/get-drug/) |
| [`biomcp.gene_cards/4`](/reference/schemas/biomcp-gene-cards-4/) | GeneCards | [`get gene`](/reference/commands/get-gene/) |
| [`biomcp.gene_page/3`](/reference/schemas/biomcp-gene-page-3/) | GenePage | [`search gene`](/reference/commands/search-gene/) |
| [`biomcp.gwas_ancestries/1`](/reference/schemas/biomcp-gwas-ancestries-1/) | GwasAncestries | [`get gwas`](/reference/commands/get-gwas/) |
| [`biomcp.gwas_association/1`](/reference/schemas/biomcp-gwas-association-1/) | GwasAssociationCard | [`get gwas`](/reference/commands/get-gwas/) |
| [`biomcp.gwas_loci/1`](/reference/schemas/biomcp-gwas-loci-1/) | GwasLoci | [`get gwas`](/reference/commands/get-gwas/) |
| [`biomcp.gwas_page/3`](/reference/schemas/biomcp-gwas-page-3/) | GwasPage | [`search gwas`](/reference/commands/search-gwas/), [`get gwas`](/reference/commands/get-gwas/) |
| [`biomcp.gwas_study/1`](/reference/schemas/biomcp-gwas-study-1/) | GwasStudy | [`get gwas`](/reference/commands/get-gwas/) |
| [`biomcp.health/2`](/reference/schemas/biomcp-health-2/) | Health | [`health`](/reference/commands/health/) |
| [`biomcp.help/2`](/reference/schemas/biomcp-help-2/) | Help | [`help`](/reference/commands/help/), [`help network proxy`](/reference/commands/help-network-proxy/), [`help network ca_bundle`](/reference/commands/help-network-ca_bundle/) |
| [`biomcp.key/2`](/reference/schemas/biomcp-key-2/) | KeyHelp | [`help key ncbi`](/reference/commands/help-key-ncbi/), [`help key semantic-scholar`](/reference/commands/help-key-semantic-scholar/), [`help key orcid`](/reference/commands/help-key-orcid/), [`help key oncokb`](/reference/commands/help-key-oncokb/), [`help key alphagenome`](/reference/commands/help-key-alphagenome/), [`help key nci`](/reference/commands/help-key-nci/), [`help key openfda`](/reference/commands/help-key-openfda/) |
| [`biomcp.nci_trial_cards/2`](/reference/schemas/biomcp-nci-trial-cards-2/) | NciTrialCards | [`get trial`](/reference/commands/get-trial/) |
| [`biomcp.nci_trial_page/3`](/reference/schemas/biomcp-nci-trial-page-3/) | NciTrialPage | [`search trial`](/reference/commands/search-trial/) |
| [`biomcp.orcid_works/2`](/reference/schemas/biomcp-orcid-works-2/) | OrcidWorks | [`author papers`](/reference/commands/author-papers/) |
| [`biomcp.pathway_card/5`](/reference/schemas/biomcp-pathway-card-5/) | PathwayCard | [`get pathway`](/reference/commands/get-pathway/) |
| [`biomcp.pathway_events/2`](/reference/schemas/biomcp-pathway-events-2/) | PathwayEvents | [`pathway events`](/reference/commands/pathway-events/) |
| [`biomcp.pathway_genes/4`](/reference/schemas/biomcp-pathway-genes-4/) | PathwayGenes | [`pathway genes`](/reference/commands/pathway-genes/) |
| [`biomcp.pathway_page/3`](/reference/schemas/biomcp-pathway-page-3/) | PathwayPage | [`search pathway`](/reference/commands/search-pathway/) |
| [`biomcp.pgx_annotation/1`](/reference/schemas/biomcp-pgx-annotation-1/) | PgxAnnotation | [`get pgx`](/reference/commands/get-pgx/) |
| [`biomcp.pgx_annotations/1`](/reference/schemas/biomcp-pgx-annotations-1/) | PgxAnnotations | [`get pgx`](/reference/commands/get-pgx/) |
| [`biomcp.pgx_card/1`](/reference/schemas/biomcp-pgx-card-1/) | PgxCard | [`get pgx`](/reference/commands/get-pgx/) |
| [`biomcp.pgx_diplotype/1`](/reference/schemas/biomcp-pgx-diplotype-1/) | PgxDiplotype | [`get pgx`](/reference/commands/get-pgx/) |
| [`biomcp.pgx_guidelines/1`](/reference/schemas/biomcp-pgx-guidelines-1/) | PgxGuidelines | [`get pgx`](/reference/commands/get-pgx/) |
| [`biomcp.pgx_page/2`](/reference/schemas/biomcp-pgx-page-2/) | PgxPage | [`search pgx`](/reference/commands/search-pgx/) |
| [`biomcp.pgx_recommendations/1`](/reference/schemas/biomcp-pgx-recommendations-1/) | PgxRecommendations | [`get pgx`](/reference/commands/get-pgx/) |
| [`biomcp.phenotype-card/1`](/reference/schemas/biomcp-phenotype-card-1/) | PhenotypeTermCard | [`get phenotype`](/reference/commands/get-phenotype/) |
| [`biomcp.phenotype-diseases/3`](/reference/schemas/biomcp-phenotype-diseases-3/) | PhenotypeDiseases | [`get phenotype`](/reference/commands/get-phenotype/) |
| [`biomcp.phenotype-genes/3`](/reference/schemas/biomcp-phenotype-genes-3/) | PhenotypeGenes | [`get phenotype`](/reference/commands/get-phenotype/) |
| [`biomcp.phenotype-hierarchy/1`](/reference/schemas/biomcp-phenotype-hierarchy-1/) | PhenotypeHierarchy | [`get phenotype`](/reference/commands/get-phenotype/) |
| [`biomcp.phenotype-search/1`](/reference/schemas/biomcp-phenotype-search-1/) | PhenotypePage | [`search phenotype`](/reference/commands/search-phenotype/) |
| [`biomcp.phenotype-synonyms/1`](/reference/schemas/biomcp-phenotype-synonyms-1/) | PhenotypeSynonyms | [`get phenotype`](/reference/commands/get-phenotype/) |
| [`biomcp.phenotype-xrefs/1`](/reference/schemas/biomcp-phenotype-xrefs-1/) | PhenotypeXrefs | [`get phenotype`](/reference/commands/get-phenotype/) |
| [`biomcp.protein_card/3`](/reference/schemas/biomcp-protein-card-3/) | ProteinCard | [`get protein`](/reference/commands/get-protein/) |
| [`biomcp.protein_complex_members/1`](/reference/schemas/biomcp-protein-complex-members-1/) | ProteinComplexMembers | [`protein complex-members`](/reference/commands/protein-complex-members/) |
| [`biomcp.protein_complexes/2`](/reference/schemas/biomcp-protein-complexes-2/) | ProteinComplexes | [`protein complexes`](/reference/commands/protein-complexes/) |
| [`biomcp.protein_domains/2`](/reference/schemas/biomcp-protein-domains-2/) | ProteinDomains | [`protein domains`](/reference/commands/protein-domains/) |
| [`biomcp.protein_features/1`](/reference/schemas/biomcp-protein-features-1/) | ProteinFeatures | [`protein features`](/reference/commands/protein-features/) |
| [`biomcp.protein_function/2`](/reference/schemas/biomcp-protein-function-2/) | ProteinFunctionResult |  |
| [`biomcp.protein_isoforms/1`](/reference/schemas/biomcp-protein-isoforms-1/) | ProteinIsoforms | [`protein isoforms`](/reference/commands/protein-isoforms/) |
| [`biomcp.protein_page/1`](/reference/schemas/biomcp-protein-page-1/) | ProteinPage | [`search protein`](/reference/commands/search-protein/) |
| [`biomcp.protein_sequence/1`](/reference/schemas/biomcp-protein-sequence-1/) | ProteinSequenceResult | [`protein sequence`](/reference/commands/protein-sequence/) |
| [`biomcp.protein_structures/1`](/reference/schemas/biomcp-protein-structures-1/) | ProteinStructures | [`protein structures`](/reference/commands/protein-structures/) |
| [`biomcp.registered-keys/2`](/reference/schemas/biomcp-registered-keys-2/) | RegisteredKeys | [`key list`](/reference/commands/key-list/) |
| [`biomcp.store-clean/1`](/reference/schemas/biomcp-store-clean-1/) | StoreClean | [`store clean`](/reference/commands/store-clean/) |
| [`biomcp.study_card/2`](/reference/schemas/biomcp-study-card-2/) | StudyCard | [`get study`](/reference/commands/get-study/) |
| [`biomcp.study_files/1`](/reference/schemas/biomcp-study-files-1/) | StudyFiles | [`get study`](/reference/commands/get-study/) |
| [`biomcp.study_page/1`](/reference/schemas/biomcp-study-page-1/) | StudyPage | [`search study`](/reference/commands/search-study/) |
| [`biomcp.study_profiles/1`](/reference/schemas/biomcp-study-profiles-1/) | StudyProfiles | [`get study`](/reference/commands/get-study/) |
| [`biomcp.study_sample_lists/1`](/reference/schemas/biomcp-study-sample-lists-1/) | StudySampleLists | [`get study`](/reference/commands/get-study/) |
| [`biomcp.sync/4`](/reference/schemas/biomcp-sync-4/) | SyncResult | [`sync`](/reference/commands/sync/), [`sync cbioportal-studies`](/reference/commands/sync-cbioportal-studies/), [`sync ema`](/reference/commands/sync-ema/), [`sync ddinter`](/reference/commands/sync-ddinter/), [`sync who-pq`](/reference/commands/sync-who-pq/), [`sync gtr`](/reference/commands/sync-gtr/), [`sync who-ivd`](/reference/commands/sync-who-ivd/), [`sync fda-cdx`](/reference/commands/sync-fda-cdx/) |
| [`biomcp.trial_cards/2`](/reference/schemas/biomcp-trial-cards-2/) | TrialCards | [`get trial`](/reference/commands/get-trial/) |
| [`biomcp.trial_document/2`](/reference/schemas/biomcp-trial-document-2/) | TrialDocumentResult | [`trial document`](/reference/commands/trial-document/) |
| [`biomcp.trial_page/5`](/reference/schemas/biomcp-trial-page-5/) | TrialPage | [`search trial`](/reference/commands/search-trial/) |
| [`biomcp.vaers_summary/1`](/reference/schemas/biomcp-vaers-summary-1/) | VaersSummary | [`search adverse-event`](/reference/commands/search-adverse-event/) |
| [`biomcp.variant_assertions/3`](/reference/schemas/biomcp-variant-assertions-3/) | VariantAssertions | [`variant assertions`](/reference/commands/variant-assertions/) |
| [`biomcp.variant_card/5`](/reference/schemas/biomcp-variant-card-5/) | VariantCard | [`get variant`](/reference/commands/get-variant/) |
| [`biomcp.variant_normalization/3`](/reference/schemas/biomcp-variant-normalization-3/) | VariantNormalization | [`variant normalize`](/reference/commands/variant-normalize/) |
| [`biomcp.variant_page/6`](/reference/schemas/biomcp-variant-page-6/) | VariantPage | [`search variant`](/reference/commands/search-variant/) |
| [`biomcp.variant_population/4`](/reference/schemas/biomcp-variant-population-4/) | VariantPopulationResult | [`variant population`](/reference/commands/variant-population/) |
| [`biomcp.variant_recurrence/4`](/reference/schemas/biomcp-variant-recurrence-4/) | VariantRecurrence | [`variant recurrence`](/reference/commands/variant-recurrence/) |
| [`biomcp.wikipathways_sync/1`](/reference/schemas/biomcp-wikipathways-sync-1/) | WikiPathwaysSync | [`sync wikipathways`](/reference/commands/sync-wikipathways/) |

The commands come from running each command's help examples with `--offline`, and from live calls of every command the site's examples run, with each section of each `get` (`npm run schema-calls`). Each schema page lists the calls that returned it.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
