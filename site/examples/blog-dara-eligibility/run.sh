biomcp get trial NCT06625320 eligibility \
  | jq -r '.result.items[0].sections.eligibility.data
    | "minimum_age: \(.minimum_age.raw)", "sex: \(.sex)",
    (.criteria | split("\n") | map(select(test("RAS"))) | .[])' \
  | fold -s -w 80
