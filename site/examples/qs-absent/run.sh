reply=$(biomcp get gene NOTAREALGENEXYZ)
echo "exit code $?"
echo "$reply" | jq -c '{status, items: [.result.items[] | {input, status}]}'
