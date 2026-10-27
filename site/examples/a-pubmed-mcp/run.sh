jq -c . messages.json | biomcp serve \
  | jq 'def cut: if length > 56 then .[:56] + "…" else . end;
    select(.id == 2) | .result.structuredContent.result.items[]
    | {pmid, title: (.header.title | cut)}'
