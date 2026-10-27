biomcp health | jq -r '.result.sources[] | select(.source | IN("hgnc","clinvar","ctgov","pubmed","oncokb")) | [.source, .usage, .state, (.reason // "")] | @tsv'
