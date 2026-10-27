biomcp get drug NDA202429 approvals --source drugsfda --section-limit 20 \
  | jq '.result.approvals.data
    | {total, first: (.records | sort_by(.event.dates[0].date) | .[0:3]
        | map({kind: .event.kind, submission_number, status: .event.status,
               date: .event.dates[0].date}))}'
