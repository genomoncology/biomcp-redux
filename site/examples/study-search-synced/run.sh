biomcp sync cbioportal-studies | jq -c '{status, sources: [.sources[] | {source, state, reason}]}'
biomcp search study melanoma --limit 3 | jq -r '.result.items[] | [.studyId, .name, .allSampleCount] | @tsv'
