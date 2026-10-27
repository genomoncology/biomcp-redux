port=$((20000 + RANDOM % 20000))
timeout 5 biomcp serve --http 127.0.0.1:$port > /dev/null &
sleep 1
curl -s --noproxy '*' -X POST "http://127.0.0.1:$port/mcp" \
  -H 'content-type: application/json' -H 'accept: application/json, text/event-stream' \
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"demo","version":"1"}}}'
echo
for v in 2025-11-25 2025-06-18; do
  curl -s --noproxy '*' -X POST "http://127.0.0.1:$port/mcp" \
    -H 'content-type: application/json' -H 'accept: application/json, text/event-stream' -H "mcp-protocol-version: $v" \
    -d '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' | jq -c --arg v "$v" '{header: $v, tools: (if .result then (.result.tools | length) else null end), error: (.error | if . then {code, supported: .data.supported} else null end)}'
done
curl -s --noproxy '*' -o /dev/null -w 'browser origin: HTTP %{http_code}\n' -X POST "http://127.0.0.1:$port/mcp" \
  -H 'Origin: https://example.org' -H 'content-type: application/json' -d '{}'
wait
