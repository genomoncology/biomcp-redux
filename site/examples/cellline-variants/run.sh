biomcp get cell-line CVCL_0132 variants \
  | jq -r '.result.items.items[] | select(.genes[0].reference.label == "BRAF") | [.genes[0].reference.label, .description, .zygosity] | @tsv'
