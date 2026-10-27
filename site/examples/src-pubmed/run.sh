biomcp search article --gene BRAF --variant V600E --limit 2 \
  | jq 'def cut: if length > 56 then .[:56] + "…" else . end;
    {status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | {pmid, title: (.header.title | cut)})'
