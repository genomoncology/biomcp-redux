biomcp search article --drug daraxonrasib --limit 3 | jq -r '.result.items[] | [.pmid, .header.title] | @tsv'
echo "--type randomized:"
biomcp search article --drug daraxonrasib --type randomized --limit 3 | jq -r '.result.items[] | [.pmid, .header.title, .header.journal] | @tsv'
biomcp get article 42223072 | jq -c '.result.items[0].answer.card | {pmid, doi, journal, published: .date}'
