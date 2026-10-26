# Study slice investigation

This proposed design starts from main `85e6ba864c3f13ec50432a58670ec2af13e12857`. It follows steps 1–4 of [the slice process](../slices.md). It adds no product code.

The approved boundary keeps exact study retrieval and counts stated by cBioPortal. Local top-mutated, filter, query, co-occurrence, cohort, survival and comparison calculations retire. A source count never supplies an inferred denominator, prevalence, prognosis or treatment recommendation.

## Investigation method

Capture the public cBioPortal detailed study catalog, twenty exact published study records, molecular profiles and sample-list metadata for representative studies, the DataHub catalog, archive size headers, and current source documentation. Each request has one attempt, a bounded response and a deadline. Keep original bodies and receipts outside Git. No participant records, mutation tables or bulk study archives are needed to decide this metadata design. Preserve source-stated counts without calculating biological summaries.

The 0.9 command inventory is inspected statically. Execute no 0.9 binary until network containment is independently verified. Unmeasured command timing and output remain explicit limits.

## Shared model boundary

The existing [CohortObservation contract](../biodata-variants-and-annotations.md#37-population-frequency-cohort-counts-coverage-and-predictor-scores) requires `subject: VariantRef`. Study-wide and gene-wide counts cannot satisfy that requirement. Reuse it only when a captured source supplies an exact variant subject and the stated counts. Preserve study totals and profile/sample-list counts as source metadata. Request PM review before extending the shared subject type.
