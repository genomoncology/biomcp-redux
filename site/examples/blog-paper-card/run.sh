biomcp get article 12068308 \
  | jq '.result.items[0].answer.card
    | {pmid, pmcid, doi, title, journal, date}'
