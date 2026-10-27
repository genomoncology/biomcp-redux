biomcp search article "KRAS G12C sotorasib" --since 2024 --sort date \
  --limit 3 \
  | jq 'def cut: if length > 56 then .[:56] + "…" else . end;
    .result.items[] | {pmid, date: .header.date, title: (.header.title | cut)}'
