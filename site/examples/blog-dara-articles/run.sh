biomcp search article --drug daraxonrasib --limit 3 \
  | jq -r 'def cut: if length > 28 then .[:28] + "…" else . end;
    .result.items[] | [.pmid, (.header.title | cut)] | @tsv'
echo "--type randomized:"
biomcp search article --drug daraxonrasib --type randomized --limit 3 \
  | jq -r 'def cut: if length > 28 then .[:28] + "…" else . end;
    .result.items[] | [.pmid, (.header.title | cut), (.header.journal | cut)]
      | @tsv'
biomcp get article 42223072 \
  | jq '.result.items[0].answer.card
    | {pmid, doi, journal, published: .date}'
