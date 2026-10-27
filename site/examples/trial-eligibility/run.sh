biomcp get trial NCT06887088 eligibility \
  | jq '.result.items[0].sections.eligibility.data | {minimum_age, maximum_age, sex, healthy_volunteers, total_characters, criteria_start: .criteria[0:200]}'
