{
  jq -nc '{jsonrpc: "2.0", id: 1, method: "initialize",
    params: {protocolVersion: "2025-06-18", capabilities: {},
      clientInfo: {name: "demo", version: "1"}}}'
  jq -nc '{jsonrpc: "2.0", method: "notifications/initialized"}'
  jq -nc '{jsonrpc: "2.0", id: 2, method: "tools/call",
    params: {name: "biomcp",
      arguments: {args: ["search", "gene", "BRAF", "--limit", "1"]}}}'
} | biomcp serve \
  | jq 'select(.id == 2) | .result.structuredContent
    | {schema, status, symbol: .result.items[0].symbol}'
