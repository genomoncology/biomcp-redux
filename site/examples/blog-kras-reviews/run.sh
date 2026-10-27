biomcp search article --gene KRAS --variant G12C --drug sotorasib \
  --type review --limit 3 \
  | jq -r 'def cut: if length > 50 then .[:50] + "…" else . end;
    .result.items[] | [.pmid, .header.date, (.header.title | cut)] | @tsv'
