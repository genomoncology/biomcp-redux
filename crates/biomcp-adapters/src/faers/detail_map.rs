use super::detail_raw as raw;
use biodata::{
    FaersDrug, FaersIdentifiers, FaersPatient, FaersReaction, FaersReport, FaersReporter,
    FaersSeriousness,
};
pub(super) fn report(r: raw::Report) -> FaersReport {
    let seriousness = FaersSeriousness {
        death: r.seriousnessdeath,
        lifethreatening: r.seriousnesslifethreatening,
        hospitalization: r.seriousnesshospitalization,
        disabling: r.seriousnessdisabling,
        congenitalanomali: r.seriousnesscongenitalanomali,
        other: r.seriousnessother,
    };
    let has_flags = seriousness.death.is_some()
        || seriousness.lifethreatening.is_some()
        || seriousness.hospitalization.is_some()
        || seriousness.disabling.is_some()
        || seriousness.congenitalanomali.is_some()
        || seriousness.other.is_some();
    let (patient, drugs, reactions) = match r.patient {
        Some(p) => (
            Some(FaersPatient {
                age: p.patientonsetage,
                age_unit: p.patientonsetageunit,
                sex: p.patientsex,
                weight: p.patientweight,
            }),
            p.drug.into_iter().map(drug).collect(),
            p.reaction
                .into_iter()
                .map(|v| FaersReaction {
                    term: biodata::Concept {
                        id: None,
                        label: v.reactionmeddrapt,
                    },
                    vocabulary_version: v.reactionmeddraversionpt,
                    outcome: v.reactionoutcome,
                })
                .collect(),
        ),
        None => (None, Box::default(), Box::default()),
    };
    FaersReport {
        safetyreportid: r.safetyreportid,
        safetyreportversion: r.safetyreportversion,
        receivedate: r.receivedate,
        receivedateformat: r.receivedateformat,
        receiptdate: r.receiptdate,
        receiptdateformat: r.receiptdateformat,
        transmissiondate: r.transmissiondate,
        transmissiondateformat: r.transmissiondateformat,
        serious: r.serious,
        seriousness: has_flags.then_some(seriousness),
        patient,
        reporter: r.primarysource.map(|s| FaersReporter {
            qualification: s.qualification,
            country: s.reportercountry,
        }),
        drugs,
        reactions,
    }
}
fn identifiers(r: raw::Identifiers) -> FaersIdentifiers {
    FaersIdentifiers {
        unii: r.unii,
        rxcui: r.rxcui,
        product_ndc: r.product_ndc,
        package_ndc: r.package_ndc,
        spl_id: r.spl_id,
        spl_set_id: r.spl_set_id,
        application_number: r.application_number,
    }
}
fn drug(r: raw::Drug) -> FaersDrug {
    FaersDrug {
        medicinalproduct: r.medicinalproduct,
        drugcharacterization: r.drugcharacterization,
        drugindication: r.drugindication,
        drugadministrationroute: r.drugadministrationroute,
        drugauthorizationnumb: r.drugauthorizationnumb,
        drugbatchnumb: r.drugbatchnumb,
        drugcumulativedosagenumb: r.drugcumulativedosagenumb,
        drugcumulativedosageunit: r.drugcumulativedosageunit,
        drugdosageform: r.drugdosageform,
        drugdosagetext: r.drugdosagetext,
        drugenddate: r.drugenddate,
        drugenddateformat: r.drugenddateformat,
        drugintervaldosagedefinition: r.drugintervaldosagedefinition,
        drugintervaldosageunitnumb: r.drugintervaldosageunitnumb,
        drugrecurreadministration: r.drugrecurreadministration,
        drugseparatedosagenumb: r.drugseparatedosagenumb,
        drugstartdate: r.drugstartdate,
        drugstartdateformat: r.drugstartdateformat,
        drugstructuredosagenumb: r.drugstructuredosagenumb,
        drugstructuredosageunit: r.drugstructuredosageunit,
        drugtreatmentduration: r.drugtreatmentduration,
        drugtreatmentdurationunit: r.drugtreatmentdurationunit,
        actiondrug: r.actiondrug,
        drugadditional: r.drugadditional,
        openfda: r.openfda.map(identifiers),
    }
}
