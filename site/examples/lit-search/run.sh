biomcp search article --gene BRAF --variant V600E --limit 3 \
  | jq -r 'def cut: if length > 56 then .[:56] + "…" else . end;
    .result.items[] | "\(.pmid)  \((.header.title | cut))"'
