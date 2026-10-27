biomcp get drug NDA202429 approvals --source drugsfda --section-limit 20 \
  | jq -r '.result.approvals.data | "history rows: \(.total)", (.records | sort_by(.event.dates[0].date) | .[0:3][] | [.event.kind, .submission_number, .event.status, .event.dates[0].date] | @tsv)'
