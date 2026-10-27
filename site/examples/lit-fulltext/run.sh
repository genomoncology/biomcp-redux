biomcp get article 33123537 fulltext \
  | jq -r 'def cut: if length > 56 then .[:56] + "…" else . end;
    .result.items[0].sections.fulltext
    | "source: \(.source)  license: \(.data.license)",
    (.data.parts[] | "\(.number)\t\(.kind)\t\((.title | cut))")'
