biomcp search article "zzqxv qqqzzk" \
  | jq -c '{status, items: (.result.items | length),
    sources: [.sources[] | {source, state}]}'
