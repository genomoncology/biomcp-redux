# 0101 — Diagnose Windows TLS in installed key-reload checks

Status: OPEN.

Milestone: 1

## Outcome

The installed Windows key-reload checks complete their local synthetic TLS requests through stdio and HTTP, with certificate verification and credential isolation intact.

- Why: native job j10615 passed the download installer, byte identity, replay, cache concurrency and all installed HTTP cases, then actually ran all three key-reload checks. Each failed with reason `tls` on its first local synthetic Semantic Scholar request. No product defect is established until the certificate and environment boundary is diagnosed.
- Starts from: j10615's exact retained Windows candidate, fixed runner, sanitized output and local certificate/configuration witness. Reuse the owned artifacts and tools documented by 0066 and 0100; no new product build during diagnosis.
- Changes: identify the concrete TLS error and fix its owning local proof, certificate setup or product boundary only after evidence proves the cause. Preserve configured CA roots, certificate verification, required Windows environment, proxy rules, per-call credential reload and outgoing-only synthetic keys. Do not disable TLS checks or change source policy to pass a test.
- Proof: run only the three source-free installed key-reload cases against retained candidate and runner, first reproducing the exact failure. Inspect bounded certificate/path/environment and underlying TLS diagnostics without logging key values or unrelated host configuration. Prove no-key, registered, replaced, removed and environment-fallback requests on their existing checked wire assertions, plus stdio/HTTP continuity. If product code changes, build the fixed candidate once and run appropriate routine checks; do not repeat successful package channels.
- Keeps: previously passed Windows download, installer, HTTP, replay and concurrency evidence. All probes use booked pm jobs, an owned Windows folder and the existing approved compiler read-only if required. Each run stays within an hour and restores the VM to stopped.
- Defers: public source calls, real credentials, package publication and aggregate platform completion owned by 0066.
