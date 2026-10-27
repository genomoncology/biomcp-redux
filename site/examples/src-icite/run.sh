biomcp article citations 33123537 --limit 2 \
  | jq 'def cut: if length > 50 then .[:50] + "…" else . end;
    {status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | {pmid, cited_by, title: (.header.title | cut)})'
