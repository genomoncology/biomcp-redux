biomcp search article --gene BRAF --variant V600E --limit 3 | jq -r '.result.items[] | "\(.pmid)  \(.header.title)"'
