biomcp get cell-line CVCL_0132 chembl \
  | jq -r '"release: \(.result.release.version)",
    (.result.items.items[] | [.id, .name, .description] | @tsv)'
biomcp cell-line assay-count CHEMBL3308077 | jq '{status, result}'
