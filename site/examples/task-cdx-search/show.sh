biomcp search diagnostic --source fda-cdx --biomarker KRAS --limit 20 \
  | jq -r 'def cut: if length > 24 then .[:24] + "…" else . end;
    "rows: \(.page.total)",
    (.result.items[]
    | [(.cells["Diagnostic Name (Manufacturer)"].text | cut),
    (.cells["Drug Trade Name (Generic) NDA / BLA"].text | cut),
    (.cells["Indication - Sample Type"].text | cut)] | @tsv)'
