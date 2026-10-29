//! Remaining source-owned identifier families use the same public collection API.
use super::hashable;
use biodata::*;

#[test]
fn source_identifiers_support_hash_collections() {
    hashable::<MondoId>("MONDO:0000001");
    hashable::<NcitId>("C1");
    hashable::<DrugId>("CHEMBL1");
    hashable::<ReportId>("001");
    hashable::<HpId>("HP:0000001");
    hashable::<GwasStudyId>("GCST000001");
    hashable::<GwasAssociationId>("association:1");
    hashable::<GwasTraitId>("EFO_0000001");
    hashable::<ClinPgxAccession>("PA1");
    hashable::<PgxGeneKey>("EGFR");
    hashable::<GtrTestId>("GTR000551465.2");
    hashable::<WhoProductId>("0123-456-00");
    hashable::<PmaNumber>("P000001");
    hashable::<PmaSupplement>("S001");
    hashable::<FdaDrugApplication>("NDA000001");
    hashable::<DiagnosticDeviceId>("P000001/S001");
    hashable::<PmaSupplementCell>("");
    hashable::<DeviceRecallNumber>("Z-0001-2026");
    hashable::<ReactomeId>("R-HSA-1");
    hashable::<WikiPathwaysId>("WP1");
    hashable::<PathwayId>("R-HSA-1");
    hashable::<UniProtIsoform>("P00533-1");
    hashable::<InterProId>("IPR000001");
    hashable::<ComplexPortalId>("CPX-1");
    hashable::<StudyId>("synthetic");
    hashable::<StudyProfileId>("synthetic_profile");
    hashable::<StudySampleListId>("synthetic_samples");
    hashable::<CbioportalCancerTypeId>("synthetic");
    hashable::<StudyResourceId>("synthetic_resource");
    hashable::<StudyBlobSha>("0123456789abcdef0123456789abcdef01234567");
    hashable::<CellosaurusAccession>("CVCL_0001");
    hashable::<CellLineSourceId>("synthetic:1");
    hashable::<ChemblCellId>("CHEMBL1");
    hashable::<SemanticScholarAuthorId>("1");
    hashable::<OrcidId>("0000-0002-1825-0097");
    hashable::<OrcidClientId>("APP-0123456789ABCDEF");
    hashable::<AuthorId>("orcid:0000-0002-1825-0097");
}
