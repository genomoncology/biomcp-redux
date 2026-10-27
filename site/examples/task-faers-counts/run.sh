count() { biomcp search adverse-event vemurafenib --limit 1 "$@" | jq -r \
  --arg f "${*:-none}" '[$f, .page.total] | @tsv'; }
printf 'filter\treports\n'
count
count --serious death
count --serious hospitalization
count --reaction pyrexia
count --received-from 2025-01-01 --received-to 2025-12-31
