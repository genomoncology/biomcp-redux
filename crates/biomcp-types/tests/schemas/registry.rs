//! Register shared value and research query schema contracts.
use super::*;

pub(super) fn common_registry() -> Vec<Contract> {
    contracts! {
        Accession => ("accession", "biodata.accession/2"),
        DeviceRecallNumber => ("device-recall-number", "biodata.device-recall-number/1"),
        Chromosome => ("chromosome", "biodata.chromosome/1"),
        Base => ("base", "biodata.base/1"),
        Sequence => ("sequence", "biodata.sequence/1"),
        SequenceLocation => ("sequence-location", "biodata.sequence-location/2"),
        Allele => ("allele", "biodata.allele/2"),
        HgvsText => ("hgvs-text", "biodata.hgvs-text/3"),
        Expression => ("expression", "biodata.expression/2"),
        ExpressionSyntax => ("expression-syntax", "biodata.expression-syntax/1"),
        Representation => ("representation", "biodata.representation/3"),
        VariantId => ("variant-id", "biodata.variant-id/2"),
        Variant => ("variant", "biodata.variant/5"),
        RsId => ("rs-id", "biodata.rs-id/1"),
        CaId => ("ca-id", "biodata.ca-id/1"),
        PaId => ("pa-id", "biodata.pa-id/1"),
        VcvId => ("vcv-id", "biodata.vcv-id/1"),
        RcvId => ("rcv-id", "biodata.rcv-id/1"),
        ScvId => ("scv-id", "biodata.scv-id/1"),
        ClinVarVariationId => ("clinvar-variation-id", "biodata.clinvar-variation-id/1"),
        ClinVarAlleleId => ("clinvar-allele-id", "biodata.clinvar-allele-id/1"),
        CosmicId => ("cosmic-id", "biodata.cosmic-id/1"),
        MondoId => ("mondo-id", "biodata.mondo-id/1"),
        NcitId => ("ncit-id", "biodata.ncit-id/1"),
        VrsId => ("vrs-id", "biodata.vrs-id/1"),
        Contig => ("contig", "biodata.contig/1"),

    }
}

pub(super) fn research_registry() -> Vec<Contract> {
    contracts! {
        ResearchSearch<ResearchEnvelope> => ("research-search", ResearchSearch::<ResearchEnvelope>::SCHEMA),
        ResearchFetch<ResearchEnvelope> => ("research-fetch", ResearchFetch::<ResearchEnvelope>::SCHEMA),
    }
}
