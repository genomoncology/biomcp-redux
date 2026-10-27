biomcp search diagnostic --source fda-cdx --biomarker BRAF --limit 3 \
  | jq -r '.result.items[] | .cells["Diagnostic Name (Manufacturer)"].text'
