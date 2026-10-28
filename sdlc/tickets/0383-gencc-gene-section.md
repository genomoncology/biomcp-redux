# 0383 — Add the GenCC gene section

Status: OPEN.

Milestone: 5

## Outcome

`get gene NAME gencc` returns GenCC's submission-level gene-disease validity records from a synced GenCC bundle, keyed by HGNC ID, with each submitter, classification, disease and date as GenCC publishes them. `sync gencc` downloads the CC0 submissions export from the current new-format CSV endpoint (`submissions-export-csv?format=new`, its 31-column contract) through the shared CSV bundle reader (PM ruling m10791 ask 7) and handles 429 with the shared bounded retry.

- Why: the gene slice deferred GenCC until a case found a gene ClinGen has not curated. experiment 499's rerun of six 0.9.3 rare disease witnesses and one published case on release/1.0 at ff2e9b64 (Jarvis m10854, 2026-10-27); public identifiers only reached that case with ODC1 (Bachmann-Bupp syndrome), where 0.9.3 had a GenCC command. Ian's ruling that 1.0 ships every known fix.
- Starts from: the gene slice's GenCC row (about 26.5 MB, CC0), the existing GenCC CSV contract, the bundle sync framework, the ClinGen section and the source registry.
- Rules: GenCC is `open`; records stay per submitter, with no merged or ranked classification. Health reports the bundle by 0342's rules.
- Proof: a bundle fixture case for ODC1 shows its submissions; a 429 then success sync case; a gene with no GenCC rows says so.
- Changes: a GenCC bundle, adapter, gene section, registry entry, cases and the baseline.
- Defers: nothing.
