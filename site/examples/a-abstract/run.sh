biomcp get article 33123537 \
  | jq -r '.result.items[0].answer.card
    | "title: \(.title)", "abstract: \(.abstract_parts[0].text)"' \
  | fold -s -w 78 | head -8
