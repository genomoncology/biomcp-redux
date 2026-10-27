# Call the biomcp tool with three commands over stdio, then over HTTP,
# and print each reply's status. The sync call works like biomcp sync hgnc.
cmds='[["store","clean"],["sync","hgnc"],["key","list"]]'
calls() {
  echo "$cmds" | jq -c 'to_entries[] | {jsonrpc: "2.0", id: (.key + 2),
    method: "tools/call", params: {name: "biomcp", arguments: {args: .value}}}'
}
show='{command: ($cmds | fromjson)[.id - 2] | join(" "),
  status: .result.structuredContent.status,
  error: .result.structuredContent.error.code}'
echo "stdio:"
{
  jq -nc '{jsonrpc: "2.0", id: 1, method: "initialize",
    params: {protocolVersion: "2025-11-25", capabilities: {},
      clientInfo: {name: "demo", version: "1"}}}'
  jq -nc '{jsonrpc: "2.0", method: "notifications/initialized"}'
  calls
} | biomcp serve \
  | jq -s -c --arg cmds "$cmds" "map(select(.id > 1)) | sort_by(.id)[] | $show"
echo "http:"
port=$((20000 + RANDOM % 20000))
timeout 20 biomcp serve --http 127.0.0.1:$port 2> /dev/null &
sleep 1
calls | while read -r call; do
  curl -s --noproxy '*' -X POST "http://127.0.0.1:$port/mcp" -d "$call" \
    -H 'content-type: application/json' \
    -H 'accept: application/json, text/event-stream' \
    -H 'mcp-protocol-version: 2025-11-25'
  echo
done | jq -c --arg cmds "$cmds" "$show"
kill %1
