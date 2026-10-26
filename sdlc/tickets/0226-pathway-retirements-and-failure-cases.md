# 0226 — Retire old pathway commands and options and prove pathway failure cases

Status: OPEN.

Milestone: 4

Depends on: 0216
Depends on: 0219
Depends on: 0222

## Outcome

Every 0.9 pathway command, option and section that pathway.md and the 2026-10-26 pathway rulings drop or retire returns a fixed usage error that names the supported route or the reason, and makes zero requests. Reactome and WikiPathways each report a failed outcome, never an empty one, on a 429, 500, timeout, invalid JSON, HTML success body or oversized body, and a foreign returned identity is dropped with a problem and never enters the cache.

- Why: pathway.md's 0.9 accounting retires these spellings, the 2026-10-26 rulings retire the three fanout and title-matching helpers, and the slice rules require failure and identity cases for every adapter.
- Starts from: pathway.md's complete 0.9 accounting, closed public issues and the retirement and fault rows of the case matrix; the 2026-10-26 pathway rulings; the entity boundary ruling; the existing retirement table in `crates/biomcp/src/retired.rs` and its usage errors; the adapters from 0216, 0219 and 0222.
- Rules: `search pathway -q` and `--query` name `search pathway <text>`. `--type` says the accepted pathway class is enforced and needs no filter. A KEGG pathway ID in `get pathway` says KEGG is dropped and names Reactome and WikiPathways IDs, with no request to KEGG. The `enrichment` section and the `enrich` command say gene-set enrichment is dropped and that an agent can call g:Profiler itself, with no provider request and no local calculation. `pathway drugs` with any of its options names `get pathway ID genes` and says that reverse gene-to-drug search is not supported, using the same guidance as the retired `gene drugs` command: select a drug identity and request its explicit source sections. `pathway articles` with any of its options names `search article <text>` and never claims article linkage from a matching title. `pathway trials` with any of its options, including `--source ctgov` and `--source nci`, names `search trial --gene <gene>`. `list pathway` names `help search pathway` and `help get pathway`. A non-human Reactome ID, `events` on a WP ID and any other unsupported source and section pairing fail before I/O. No retirement rewrites the query or falls back to another parser. `-l` and `--limit` keep working.
- Proof: outside-in cases first. A case per retired spelling, per option of each retired helper and per pairing failure asserts its message with zero provider requests. Synthetic 429, 500, timeout, invalid JSON, HTML success body and oversized replies for Reactome search, card, participants, events and the UniProt mapping, labeled synthetic, assert a failed outcome with source and terms; synthetic failures for the WikiPathways sync assert a failed receipt that keeps the old copy. A synthetic wrong returned stable ID, species or class for Reactome and a wrong returned WP ID or species for WikiPathways assert the drop, the problem and no cache admission. A generic 404 and a 404 with a foreign query echo assert failure distinct from the structured empty. A partial multi-source result asserts the successful leg and the named failed leg. Each case has a size ceiling. This ticket adds no filter or exact-ID get and makes no provider call; the filters and exact-ID gets it relies on have known-positive captures through one bounded keyless pm job in 0206, 0216, 0219 and 0222. Then make landing-gate runs through pm job.
