biomcp search adverse-event vemurafenib --serious death --limit 3 | jq -r '.result.items[] | [.id, .serious, .first_received] | @tsv'
biomcp get adverse-event 26907716 | jq -c '{status, header: .result.header, next: [.next[]?.args | join(" ")]}'
