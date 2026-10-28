# 0371 — Apply the bad-value rule to PubMed and ORCID

Status: COMPLETE.
Landed: 145b1870b1e487bccbc29be62215d179501e79e9

Milestone: 5

Depends on: 0360

## Outcome

PubMed and ORCID optional text and ID fields that drop an invalid value with `.ok()`, `transpose` or a raw text parse instead record a problem through 0360's shared helper, and the reply still returns. Required identity fields still fail the item. The ticket records the family's audited sites, each classed as migrated, identity refusal or legitimate absence.

- Why: the 0360 developer's audit found about 170 `.ok()` and `transpose` sites plus raw text deserialization; 0360 keeps the shared helper and the named fixes, and this ticket finishes the rule (coordinator m10687, PM ruling 2026-10-27). Ian's ruling that 1.0 ships every known fix.
- Starts from: 0360's shared helper and rule, and the audit search list.
- Rules: no text heuristics; required identity fields still fail the item. The other families moved to 0372 (ComplexPortal, InterPro), 0373 (MyChem, OLS, MyDisease), 0374 (EMA) and 0375 (Reactome) by PM ruling m10687 ask 2.
- Proof: the inventory lists the family's audited sites with their class; each migrated field has a case with a bad value that returns the reply with a problem; a ratchet fails on a new unclassified `.ok()` in adapter map code.
- Scope (PM ruling 2026-10-28, m12076): identity is the PMID, the ORCID iD and an external ID's kind and value, never a title, display name, work type or relationship. An invalid display field is left out with a problem and the record stays. `ArticleHeader.title`, `Article.title`, `AuthorHeader.name`, `OrcidWorkAssertion.work_type` and `OrcidExternalId.relationship` become optional, with the Europe PMC and Semantic Scholar constructors and renderers that share them. An invalid external ID member is left out with an `external_ids` problem and its work and group stay.
- Schema versions (PM ruling 2026-10-28, m12148): a field that was required and becomes optional changes what response consumers can rely on, so each schema that contains one of the five fields takes a major bump, as 0372 and 0373 do. The compatibility check stays as it is.
- Changes: adapter map and raw modules, the five shared fields above, their constructors, renderers and schemas, their cases, the baseline and the ratchet.
- Defers: nothing.
