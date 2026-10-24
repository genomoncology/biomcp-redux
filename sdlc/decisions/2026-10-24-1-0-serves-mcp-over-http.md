# BioMCP 1.0 serves MCP over HTTP

Date: 2026-10-24

Decided by Ian, after the review of 0.9's issues showed hosted users on notebooks, agent frameworks and reverse proxies depend on 0.9's HTTP endpoint.

BioMCP 1.0 keeps `biomcp serve --http` with MCP's streamable HTTP transport at `/mcp` and a `/health` endpoint, and adds `biomcp health`. It also supports corporate networks through added certificate authorities and the standard proxy settings. These land in milestone 1, before the release, so hosted users can move from 0.9 to 1.0 without losing access. The browsable wiki stays later.
