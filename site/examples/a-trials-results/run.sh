biomcp search trial --condition melanoma --has-results --count-only \
  | jq '{status, sources: [.sources[] | {source, count}]}'
