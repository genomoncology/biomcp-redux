biomcp get trial NCT06625320 eligibility \
  | jq '.result.items[0].sections.eligibility.data | {minimum_age: .minimum_age.raw, sex, ras_lines: (.criteria | split("\n") | map(select(test("RAS"))))}'
