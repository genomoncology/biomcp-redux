biomcp search gwas --variant rs7903146 --limit 2 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | {study, traits, mapped_genes})'
