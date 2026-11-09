biomcp get trial NCT06887088 eligibility \
  | jq 'def cut: if length > 50 then .[:50] + "…" else . end;
    .result.items[0].sections.eligibility.data
    | {minimum_age, maximum_age, sex, healthy_volunteers, total_characters,
    criteria_start: (.criteria[0:200] | cut)}'
