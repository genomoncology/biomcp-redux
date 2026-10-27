biomcp search diagnostic --source fda-cdx --biomarker BRAF --limit 2 \
  | jq 'def cut: if length > 56 then .[:56] + "…" else . end;
    {status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | .cells
      | {diagnostic: (.["Diagnostic Name (Manufacturer)"].text | cut),
         indication: (.["Indication - Sample Type"].text | cut)})'
