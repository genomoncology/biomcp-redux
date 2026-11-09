biomcp get article 33123537 10.3389/fcell.2020.573599 \
  | jq '.result.items[] | {input, pmid: .answer.card.pmid,
    pmcid: .answer.card.pmcid, doi: .answer.card.doi}'
