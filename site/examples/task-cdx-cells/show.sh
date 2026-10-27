# Read every row of the list, then count the biomarker cells that name KRAS.
# A no-break space in a cell prints as <U+00A0>.
for offset in 0 50 100 150 200 250; do
  biomcp search diagnostic --source fda-cdx --limit 50 --offset "$offset"
done | jq -r '.result.items[] | select(.kind == "individual") | .cells["Biomarker(s)"].text // empty | select(test("KRAS")) | gsub("\u00a0"; "<U+00A0>") | @json' \
  | sort | uniq -c
