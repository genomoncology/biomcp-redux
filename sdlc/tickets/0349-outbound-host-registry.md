# 0349 — Declare every outbound host in one reviewed registry

Status: OPEN.

Milestone: 5

## Outcome

Every host BioMCP sends a request to is declared once, beside its source, in one registry that Fetch reads. Fetch refuses a request to an undeclared host before it connects, with a typed local error. The PDF download path keeps its own public-address rules. A test prints the registry, so a new host shows up in review as a one-line addition.

- Why: Ian's security screening request through Jarvis (m10245, 2026-10-27). Request hosts are written today inside each adapter, so a new outbound host is not a single reviewed change.
- Starts from: Fetch request construction, `quotas.rs` and its ratchet test, and each adapter's base URLs.
- Rules: the registry holds request hosts only, not links shown to users. No wildcard host entries. PM ruling 2026-10-27 (m10367): source declarations name primary hosts, known API and CDN download origins are explicit secondary origins, and publisher PDF downloads are one explicit policy row under the existing public-address rules; PDF publishers are not listed one by one. Admission happens before cache, replay and network.
- Proof: a case that builds a request to an undeclared host is refused before connecting; every recorded case still replays; the registry test lists every host.
- Changes: Fetch, adapter base URLs and the ratchet test.
- Defers: nothing. Main only; not cherry-picked onto `release/1.0`; it ships in a 1.0 update.
