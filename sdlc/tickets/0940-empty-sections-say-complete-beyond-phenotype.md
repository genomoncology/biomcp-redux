# 0940 — empty-sections-say-complete-beyond-phenotype

Status: OPEN.

Milestone: 7

## Outcome

An entity section asked for an identity the source holds but whose section is empty comes back `complete` with an empty list, not `absent`, for disease sections read from Monarch and pathway relation pages, as 0929 did for phenotypes. `absent` stays for an identity the source doesn't hold (`crates/biomcp-types/src/response/status.rs:8-9`).

## Why

0929's developer (2026-11-14) found the same mix-up outside phenotypes: `crates/biomcp-query/src/disease/monarch.rs:49` and `crates/biomcp-query/src/pathway/relation.rs:218`, `:240` call an empty page `absent`. `crates/biomcp-query/src/drug/who.rs:36` may be a third case, or may be how it decides whether the drug exists; check before changing it. An agent then can't tell "no rows" from "no such disease or pathway".

## Proof

- Proof: recorded cases for a real disease and a real pathway with an empty section show `complete` and an empty list; an unknown ID still shows `absent`.

## Starts from

- Starts from: 0929's change (`git log origin/main --grep 0929`), its use of a term lookup when the source answers an empty page the same way for both, and the lines above.

## Keeps

- Keeps: `partial` for refused rows; the 1.0.0 schema rules (status values unchanged).

## Defers

- Defers: other entities; list any further drift in Progress.

## Changes

- Changes: the disease, pathway (and if confirmed, WHO drug) section status code and cases.

## Progress

- Measured live 2026-11-14. Monarch's gene association page for MONDO:0005812 (influenza) has total 0 while `/entity/MONDO:0005812` answers 200; MONDO:9999999 has gene and phenotype totals 0 and `/entity` 404, so, as for phenotypes, the page alone can't tell them apart. Reactome: `/data/query/R-HSA-9999999` is 404 and `/data/participants/R-HSA-9999999` is `[]` with 200; every human Reactome pathway in `/data/eventsHierarchy/9606` has contained events, and a scan of the 653 smallest leaf pathways found none with empty participants, so no real Reactome pathway with an empty section was found. WikiPathways WP5034 (Mitochondrial complex inhibition leading to liver injury) is in the human catalog and its GPML has 25 DataNodes, all Event or Pathway, so no gene rows.
- Changed: a disease Monarch genes or phenotypes page with total 0 now reads Monarch's entity once more (`monarch::held`): held is `complete`, 404 is `absent`, any other failure is `failed` with the source marked. Pages with rows make no extra call. Pathway genes and events pages are only built after the pathway card was read (Reactome card, WikiPathways catalog, or the card passed in by `get pathway`), so an empty page is now `complete`. `partial` for refused rows is unchanged; no schema change.
- WHO `drug/who.rs:36` is not this case: it is the WHO search, whose status `status::search` overwrites at line 71 (an empty search is already `complete`), and `get` at line 86 is the identity lookup itself (`card.is_some()`). Not changed.
- Gate j40928 passed on df8be86cd (branch merged with origin/main). Cases: `tests/cases/disease/empty-sections.yaml` (MONDO:0005812 genes `complete` and empty; MONDO:9999999 genes and phenotypes `absent`, exit 1), recorded live in j40906. `tests/cases/pathway/empty-relations.yaml` (WP5034 genes `complete` and empty through `pathway genes` and `get pathway`; WP9999999 `absent`), catalogs recorded live in j40885 and the WP5034 GPML in j40896. The disease-monarch cache-admission "empty" control now expects `complete`, with Monarch's real `/entity/MONDO:0000001` reply recorded in j40906 (job code temporarily forced the entity read, since that page has rows).
- Further drift, deferred: `pathway genes` and `pathway events` with an unknown ID (WP9999999, R-HSA-9999999) answer `failed` with source reason `not-found`, not `absent`, because `relation::read` treats every error as an identity failure; `get pathway` with the same IDs answers `absent`. A Reactome pathway with no contained events would get a 404 from `containedEvents` and read as failed too, though none exists today. Also seen: `sync wikipathways` on the M5 failed with no reason in j40896, j40906, j40913 and j40919 (even with a fresh cache dir) after succeeding in j40885, while the same catalogs downloaded fine by curl; not investigated.
