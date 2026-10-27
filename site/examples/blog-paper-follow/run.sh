biomcp get article 21639808 | jq -c '.result.items[0].answer.card | {pmid, pmcid, journal, date}'
biomcp article text PMC3549296 --find V600E \
  | jq -c '{source: .sources[0].source, hits: .page.total, first: [.result.content.data.hits[0:4][] | .heading_path | join(" > ")]}'
