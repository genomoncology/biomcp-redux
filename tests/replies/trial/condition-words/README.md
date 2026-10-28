# Condition search witness

The two body files preserve complete ClinicalTrials.gov responses captured on 2026-10-27. Both requests used the standard trial-header projection, `countTotal=true` and `pageSize=25`.

`broad.body` used `query.cond=Snyder-Robinson syndrome`. It returned six trials. Its SHA256 is `87490f718f13653b8526958531f8ad99e55a4e08f96c41c24ae8cdfbedca501b`.

`exact.body` kept that condition query and added `filter.advanced=AREA[Condition]EXPANSION[None]"Snyder-Robinson syndrome"`. It returned zero trials. Its SHA256 is `b54968c05ff881a1ef7ce604b6a340248f8e2aee8637bef06da0f5693f520df8`.

A separate captured request put the expansion operator in `query.cond` alone. Its response was byte-identical to the broad response. That syntax alone did not narrow this witness. These responses do not establish the registry's internal reason for the broad matches or prove an abbreviation collision.

The outside-in test serves these unchanged bodies through local TLS and lets the actual CLI generate transport-owned recordings. Its version reply is synthetic and does not establish the captured registry processing time. Additional condition-list variants are explicitly synthetic. No test contacts the public service.

The [NLM search guide](https://www.nnlm.gov/sites/default/files/2024-11/ClinicalTrialsgov%20Search%20Tools%20-%20Quick%20Guide%20for%20Systematic%20Reviewers.pdf) describes the expansion operator. Source-field matching has partial support; this witness establishes the observed six-to-zero change, without a universal source guarantee or an eligibility conclusion.
