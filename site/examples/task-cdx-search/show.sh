biomcp search diagnostic --source fda-cdx --biomarker KRAS --limit 20 \
  | jq -r '"rows: \(.page.total)", (.result.items[] | [.cells["Diagnostic Name (Manufacturer)"].text, .cells["Drug Trade Name (Generic) NDA / BLA"].text, .cells["Indication - Sample Type"].text] | @tsv)'
