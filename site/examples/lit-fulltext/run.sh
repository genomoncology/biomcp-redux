biomcp get article 33123537 fulltext \
  | jq -r '.result.items[0].sections.fulltext | "source: \(.source)  license: \(.data.license)", (.data.parts[] | "\(.number)\t\(.kind)\t\(.title)")'
