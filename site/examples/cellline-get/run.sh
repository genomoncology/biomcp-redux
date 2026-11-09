biomcp get cell-line CVCL_0132 \
  | jq -r '.result.header | [.accession, .name, .category] | @tsv'
biomcp get cell-line CVCL_0132 | jq -r '.next[].args | join(" ")'
