biomcp get gene BRAF \
  | jq 'def cut: if length > 56 then .[:56] + "…" else . end;
    .result.items[0].answer.gene.summary
    | {source, dated, text: (.text | cut)}'
