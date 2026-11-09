biomcp search drug daraxonrasib --source openfda-label --limit 1 \
  | jq -c '{status, rows: (.result.items | length)}'
biomcp search drug RASONQUE --source openfda-label --limit 1 \
  | jq -r '.result.items[] | [.id, .name] | @tsv'
biomcp get drug 13a65923-d072-4dac-8620-8eabbf8c3641 label \
  --label-section indications_and_usage --source openfda-label \
  | jq -r '.result.label.data | "revised: \(.label.revised)",
    (.text | split(". ")[0] + ".")' \
  | fold -s -w 80
