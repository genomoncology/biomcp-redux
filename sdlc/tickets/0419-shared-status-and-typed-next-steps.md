# 0419 — Choose every status in one place

Status: OPEN.

Milestone: 5

Depends on: 0454, 0415, 0416, 0495, 0501

## Outcome

`biomcp-query/src/status.rs` chooses every status with the item, get and search rules in architecture 6.11 and ruling 1. `Outcome` moves into `status.rs` with a private `status` field, so entities pass parts and the compiler refuses any other place that sets a status. The dropped-record rule reads the refusal fact 0495 puts on `Mapped<T>`. The query layer decides to hide a failed body unless it holds a condition resolution; until 0421 supplies the generic `Reply`, the typed `QueryBody` stays internal and `ResponseMeta::encode` applies that decision for every command, and 0421 turns it into an absent body (m15847). One `empty`, `finish` and `page_info` replace the copies.

- Why: disease and SEER mark any problem partial, phenotype ignores problems, pathway marks any problem partial, and disease and phenotype ignore failed secondary sources. Status choices sit in 101 query files. The binary hides bodies in `command.rs` and `disease.rs`.
- Starts from: `crates/biomcp-query/src/status.rs`, the entity `overall`, `empty` (24), `finish` (8) and `page_info` (3) copies, `crates/biomcp-adapters/src/mapped.rs` `SourcePage`, the body-hiding lines in `crates/biomcp/src/command.rs` and `command/disease.rs`.
- Rules: ruling 1. A problem in an optional field changes no status, and a dropped record makes the reply partial. A failed secondary source makes an item partial. The store restores stored statuses through `status::restored`. No text ratchet; the private field is the check.
- Transition: 0417 lands with today's trial mapping, where any trial detail or page problem marks the reply partial, including an omitted optional document label. This ticket applies ruling 1 to those trial paths (`biomcp-query` trial `get.rs`, `page.rs`, `trial.rs` and `document.rs`) and changes the blank-label case to expect complete.
- Proof: one case per rule row, each failing on the base build: a disease reply whose only problem omits an optional field is complete; a phenotype reply with a dropped record is partial; an item with a successful identity and a failed secondary source is partial, shown outside-in on an existing multi-source get over recorded replies if one exists and otherwise by the shared item-owner test, because disease reads one source (m15847); a pathway reply whose only problem omits an optional field is complete. A planted `outcome.status = Status::Partial` in an entity module does not compile. The reply diff lists only these cases and the cases this ticket names, 0418's cases included.
- Changes: the files above, the entity modules and `tests/cases/`.
- Defers: next steps (0458).
