use super::{ArticleSearch, GeneGet, GeneSearch};
use serde::{Deserialize, Serialize};
/// One checked query. Housekeeping actions are separate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Request {
    /// Find NCI organization names without assigning an identity.
    SearchOrganization(crate::OrganizationSearch),
    /// Discover native author candidates without identity reconciliation.
    SearchAuthor(crate::AuthorSearch),
    /// Page one exact provider record’s native papers.
    AuthorPapers(crate::AuthorPapersQuery),
    /// Read exact provider author cards in input order.
    GetAuthor(crate::AuthorGet),
    /// Read the native assay total for one exact ChEMBL cell.
    CellLineAssayCount(crate::CellLineAssayCountQuery),
    /// Discover unchanged literal Cellosaurus candidates.
    SearchCellLine(crate::CellLineSearch),
    /// Read an exact accession or verified qualified source identity.
    GetCellLine(crate::CellLineGet),
    /// Search one native GWAS Catalog association page.
    SearchGwas(crate::GwasSearch),
    /// Read one exact association and its native loci.
    GetGwasAssociation(crate::GwasAssociationGet),
    /// Read one exact study and its native relations.
    GetGwasStudy(crate::GwasStudyGet),
    /// Read native InterPro matches.
    ProteinDomains(crate::ProteinDomainsQuery),
    /// Read exact participant-admitted complex headers.
    ProteinComplexes(crate::ProteinRelationQuery),
    /// Read native participants under an exact protein anchor.
    ProteinComplexMembers(crate::ProteinComplexMembersQuery),
    /// Read exact native feature rows with checked residue selectors.
    ProteinFeatures(crate::ProteinFeaturesQuery),
    /// Read native alternative products without constructing sequences.
    ProteinIsoforms(crate::ProteinRelationQuery),
    /// Read source PDB references without residue coverage inference.
    ProteinStructures(crate::ProteinRelationQuery),
    /// Read one exact source sequence without alias repair.
    ProteinSequence(crate::ProteinSequenceQuery),
    /// Discover reviewed human UniProt candidates without selecting one.
    SearchProtein(crate::ProteinSearch),
    /// Read exact protein cards or all HGNC-listed protein candidates.
    GetProtein(crate::ProteinGet),
    /// Discover literal HP term candidates in publisher order.
    SearchPhenotype(crate::PhenotypeSearch),
    /// Read one independent exact HPO or Monarch term card.
    GetPhenotype(crate::PhenotypeGet),
    /// Discover source-native human pathway headers.
    SearchPathway(crate::PathwaySearch),
    /// Read an exact human pathway card or summary.
    GetPathway(crate::PathwayGet),
    /// Page retained exact source membership references.
    PathwayGenes(crate::PathwayGenesQuery),
    /// Page retained source-contained event references.
    PathwayEvents(crate::PathwayEventsQuery),
    /// Discover literal study catalog metadata.
    SearchStudy(crate::StudySearch),
    /// Read exact study metadata.
    GetStudy(crate::StudyGet),
    /// Discover source-native CPIC gene pairs.
    SearchPgx(crate::PgxSearch),
    /// Read an explicit CPIC gene card or source section.
    GetPgx(crate::PgxGet),
    /// Read one exact ClinPGx clinical annotation accession.
    GetPgxAnnotation(crate::PgxAnnotationGet),
    /// Discover native diagnostic laboratory tests.
    SearchDiagnostic(crate::DiagnosticSearch),
    /// Read an exact source diagnostic record or relation page.
    GetDiagnostic(crate::DiagnosticGet),
    /// Search source-native adverse event reports.
    SearchAdverseEvent(crate::AdverseEventSearch),
    /// Read one exact source report, without source probing.
    GetAdverseEvent(crate::AdverseEventGet),
    /// Discover one source-native disease candidate page.
    SearchDisease(crate::DiseaseSearch),
    /// Read an exact source disease concept or native statistic page.
    GetDisease(crate::DiseaseGet),
    /// Discover literal native drug candidates.
    SearchDrug(crate::DrugSearch),
    /// Read one exact source drug identity.
    GetDrug(crate::DrugGet),
    /// Read native WHO prequalification rows by exact reference.
    GetWhoMedicine(crate::DrugWhoGet),
    /// Read source-native variant assertions.
    VariantAssertions(crate::VariantAssertionsGet),
    /// Read source ancestry observations.
    VariantPopulation(crate::VariantPopulationQuery),
    /// Observe an exact allele in an explicitly selected cohort.
    VariantRecurrence(crate::VariantRecurrenceQuery),
    /// Send unchanged HGVS to explicit normalization services.
    VariantNormalize(crate::VariantNormalize),
    /// Read a published variant identifier.
    GetVariant(crate::VariantGet),
    /// Discover source-native variant candidates.
    SearchVariant(crate::VariantSearch),
    /// Save an exact registry document to an explicit destination.
    TrialDocument(crate::TrialDocumentGet),
    /// Retrieve exact trial cards and source protocol sections.
    GetTrial(crate::TrialGet),
    /// Search ClinicalTrials.gov without local screening.
    SearchTrial(crate::TrialSearch),
    /// Save a publisher PDF to an explicit destination.
    ArticlePdf(crate::ArticlePdf),
    /// Read source citation evidence.
    ArticleEvidence(crate::ArticleEvidence),
    /// Source-native paper recommendations.
    ArticleRecommend(crate::ArticleRecommend),
    /// Read a licensed supplementary file.
    ArticleSupplement(crate::ArticleSupplement),
    /// Read licensed PMC body text.
    ArticleText(crate::ArticleText),
    /// Papers citing one PMID.
    ArticleCitations(crate::ArticleEdges),
    /// Papers referenced by one PMID.
    ArticleReferences(crate::ArticleEdges),
    /// Retrieve PubMed article cards.
    GetArticle(crate::ArticleGet),
    /// Search PubMed article identities.
    SearchArticle(ArticleSearch),
    /// Get gene identities and annotations.
    GetGene(GeneGet),
    /// Search gene headers.
    SearchGene(GeneSearch),
}
