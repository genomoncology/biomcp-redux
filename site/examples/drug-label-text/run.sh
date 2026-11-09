biomcp get drug 38eea320-7e0c-485a-bc30-98c3c45e2763 label \
  --label-section indications_and_usage --source openfda-label \
  | jq -r '.result.label.data | "revised: \(.label.revised)",
    "characters: \(.text | length)", (.text | split(". ")[0] + ".")' \
  | fold -s -w 80
