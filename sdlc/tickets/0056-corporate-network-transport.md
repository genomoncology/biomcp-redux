# 0056 — Work behind corporate proxies and private certificate authorities

Status: COMPLETE.
Landed: f5566aab77e08192a91db3c68d88962122885237

Milestone: 1

Changes: Cargo.lock, Cargo.toml, README.md, crates/biodata/, crates/biomcp-fetch/, crates/biomcp/, sdlc/planning/architecture.md

## Outcome

BioMCP works on hospital and company networks. The config file takes a `ca_bundle` path whose certificates are added to the platform trust roots for every source. The transport reads `HTTPS_PROXY`, `HTTP_PROXY` and `NO_PROXY` in `config.rs` when the config file names no proxy. TLS failures and proxy failures each get their own `Reason`, so they never read as not found or as a source outage. 0.9 had no way to add a corporate root (issues 8 and 250), one source ignored the CA bundle, and a proxy failure was reported as not found.

- Starts from: architecture 4.10 and 13.1; `config.rs`; the transport's agent construction in biomcp-fetch.
- PM decision: 2026-10-24. Environment reads stay in `config.rs`. `ca_bundle` applies to every source with no per-source override. The new reasons are `tls` and `proxy`, each with a fix step in the envelope that names the config key.
- Proof: transport contracts against the local stub serving a certificate from a test CA: refused without `ca_bundle`, accepted with it. A stub proxy proves that `HTTPS_PROXY` routes the request and that `NO_PROXY` bypasses it. A refused proxy and a bad certificate each give their own reason in a runner case. Docs name the keys.
- Defers: client certificates.

- Reviewed reliability boundary, 2026-10-24: Prove additive CA capability at pinned ureq3.5.0 before implementation. Fetch selects PlatformVerifier today; Specific roots would replace platform trust. rustls-platform-verifier0.6.1 exposes new_with_extra_roots, but exact ureq integration needs a checked connector proposal if unavailable. Config owns environment and PEM I/O; Fetch receives explicit checked settings. Missing/malformed/unreadable configured bundle is local configuration failure. Specify proxy precedence and exact NO_PROXY host/domain/port boundaries; transport must not read environment. Route/trust configuration must participate in connection reuse while reply identities omit proxy credentials and CA paths. Map TLS/proxy failures consistently across ordinary and bundle/binary paths using route/stage facts; destination refusal is not inherently proxy failure. Local TLS/proxy stubs, not HTTP response recordings, prove failure classes and Proxy-Authorization isolation. Correct architecture file-only proxy claims.
