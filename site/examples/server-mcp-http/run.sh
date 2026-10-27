port=$((20000 + RANDOM % 20000))
timeout 5 biomcp serve --http 127.0.0.1:$port > /dev/null &
sleep 1
curl -s --noproxy '*' -X POST "http://127.0.0.1:$port/mcp" \
  -H 'content-type: application/json' -H 'accept: application/json, text/event-stream' \
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"demo","version":"1"}}}'
echo
curl -s --noproxy '*' -o /dev/null -w 'browser origin: HTTP %{http_code}\n' -X POST "http://127.0.0.1:$port/mcp" \
  -H 'Origin: https://example.org' -H 'content-type: application/json' -d '{}'
wait
