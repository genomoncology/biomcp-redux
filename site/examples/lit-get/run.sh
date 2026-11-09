biomcp get article 33123537 \
  | jq 'def cut: if length > 50 then .[:50] + "…" else . end;
    .result.items[0].answer.card
      | {pmid, pmcid, doi, title: (.title | cut), journal}'
