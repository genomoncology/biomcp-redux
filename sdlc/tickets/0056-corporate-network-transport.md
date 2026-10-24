# 0056 — Work behind corporate proxies and private certificate authorities

Status: OPEN.

Milestone: 1

## Outcome

BioMCP works on hospital and company networks. The config file takes a `ca_bundle` path whose certificates are added to the platform trust roots for every source. The transport reads `HTTPS_PROXY`, `HTTP_PROXY` and `NO_PROXY` in `config.rs` when the config file names no proxy. TLS failures and proxy failures each get their own `Reason`, so they never read as not found or as a source outage. 0.9 had no way to add a corporate root (issues 8 and 250), one source ignored the CA bundle, and a proxy failure was reported as not found.

- Starts from: architecture 4.10 and 13.1; `config.rs`; the transport's agent construction in biomcp-fetch.
- PM decision: 2026-10-24. Environment reads stay in `config.rs`. `ca_bundle` applies to every source with no per-source override. The new reasons are `tls` and `proxy`, each with a fix step in the envelope that names the config key.
- Proof: transport contracts against the local stub serving a certificate from a test CA: refused without `ca_bundle`, accepted with it. A stub proxy proves that `HTTPS_PROXY` routes the request and that `NO_PROXY` bypasses it. A refused proxy and a bad certificate each give their own reason in a runner case. Docs name the keys.
- Defers: client certificates.
