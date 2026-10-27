biomcp get article 33123537 | jq -r '.result.items[0].answer.card | .title, (.abstract_parts | tostring | .[0:240])'
