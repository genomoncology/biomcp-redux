biomcp search drug vemurafenib --limit 3 | jq -r '.result.items[] | [.source, .id, .name, .type] | @tsv'
