biomcp search drug daraxonrasib --source drugsfda --limit 1 \
  | jq '.result.items[]
    | {application: .identity.id,
    products: [.products[]
    | "\(.brand_name) \(.active_ingredients[0].strength) \(.dosage_form)"]}'
biomcp get drug NDA220910 approvals --source drugsfda \
  | jq '.result.approvals.data.records[]
    | {status: .event.status, kind: .event.kind, date: .event.dates[0].date,
    submitter: .event.attribution.submitter, review_priority,
    class: .submission_class_code_description,
    property: [.submission_property_type[].code]}'
