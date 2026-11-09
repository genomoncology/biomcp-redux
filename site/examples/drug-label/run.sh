biomcp search drug Zelboraf --source openfda-label --limit 1 \
  | jq -r '.result.items[] | [.id, .name, .identity_level] | @tsv'
biomcp get drug 38eea320-7e0c-485a-bc30-98c3c45e2763 --source openfda-label \
  | jq -r '.next[].args | join(" ")' | fold -s -w 80
