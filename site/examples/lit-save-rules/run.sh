dir=$(mktemp -d)
trap 'rm -rf "$dir"' EXIT
echo "command line, save the whole text:"
biomcp article text PMC7573235 --save "$dir/whole.md" \
  | jq -c '{status, bytes: .result.content.data.saved.bytes}'
echo "command line, --save with --part:"
biomcp article text PMC7573235 --save "$dir/part.md" --part 3 \
  | jq -c '{status, code: .error.code}'
# mcp ARGS...: call the biomcp tool over stdio with these command words.
mcp() {
  {
    jq -nc '{jsonrpc: "2.0", id: 1, method: "initialize",
      params: {protocolVersion: "2025-06-18", capabilities: {},
        clientInfo: {name: "demo", version: "1"}}}'
    jq -nc '{jsonrpc: "2.0", method: "notifications/initialized"}'
    printf '%s\n' "$@" | jq -Rnc '{jsonrpc: "2.0", id: 2,
      method: "tools/call",
      params: {name: "biomcp", arguments: {args: [inputs]}}}'
  } | biomcp serve \
    | jq -c 'select(.id == 2) | .result.structuredContent
      | {status, code: .error.code, bytes: .result.content.data.saved.bytes}'
}
echo "stdio MCP, save a new file:"
mcp article text PMC7573235 --save "$dir/a.md"
echo "stdio MCP, the same file with --replace:"
mcp article text PMC7573235 --save "$dir/a.md" --replace
