# 1061 proposal: one way to page a list

2026-11-19. Measured on main be96d8d40 (Linux build), live sources. Bytes are the whole JSON reply; this JSON runs about 3.5 bytes per token.

## What main does today

- **Three paging pairs.** Searches and the subcommands (`pathway genes`, `protein features`, `article citations`) take `--limit/--offset`. Most `get ... SECTION` take `--section-limit/--section-offset` (a ruling: "Study: get study sections page with --section-limit and --section-offset, as other entities do"). `get trial` takes `--limit/--offset` and refuses `--section-limit`. `get variant` has both: `--limit/--offset` page identity candidates, `--section-*` page its sections. `--page` exists only on `variant recurrence` (a native page number).
- **Caps.** 25 (protein, cell line, author), 50 (most), 100 (`search trial`, `get trial`), 200 (Drugs@FDA approvals only). Defaults are 3 to 10 rows, set by the token budget decision (2026-10-24); this proposal leaves defaults alone.
- **Tokens.** A pool or snapshot is "a stored snapshot, so paging is stable and a warm page makes no request" (decision 2026-10-23, the cache). Trial locations keep the whole ClinicalTrials.gov record, keyed by content hash, so pages 2 to 16 read one copy. `search trial` needs `--next-page` because the ClinicalTrials.gov API pages only by cursor. Pathway, protein and trial sections refuse any nonzero offset without a token (`saved.rs`, `OffsetWithoutSaved`, `TrialSectionPool`); `search article`, `article citations` and `search study` instead start a new pool at the asked offset and work.
- **Past the end.** Every list measured returns an empty page with `total` once it is allowed to run (approvals, disease, gene, pathway with its pool). The ticket's "offset 50 on 45 rows refused" is the token rule, not an end check. Only character offsets into text (trial eligibility) refuse a position past the end, which is right: text is not a list.

## The one way

1. `--limit N --offset M` (typed tools: `limit`, `offset`) page every list: search rows, a named section's rows, a subcommand's rows, trial locations. Both section names stay accepted as other names, and `get trial` also takes `--section-limit/--section-offset`.
2. An offset without a token starts a fresh snapshot and returns that page, with the new pool in the reply and its `next` step, as `search article` already does. A pool, when given, still pins the snapshot, so a caller who follows `next` keeps the stable-paging guarantee. An unknown or expired pool keeps today's refusal. `search trial` without `--next-page` walks native pages (up to 1,000 rows each) inside the command deadline.
3. An offset past the end returns an empty page with `page.total`, never usage.
4. `--page` stays refused except on `variant recurrence`; the message names `--offset`. A limit above the cap is refused naming the cap.
5. `get variant` is the one collision. With a named section, `--limit/--offset` page that section; without one they page candidates, as today. The alternative, refusing `--limit` with a section and naming `--section-limit`, keeps two ways; I recommend the first.
6. Trial section pages add `page {offset, returned, total}` beside today's `offset`/`total`/`continuation`, so every list has the same reply shape the ticket keeps.

## Caps, from measured sizes

| List (record) | bytes/row | 10 | 50 | 100 | 200 |
|---|---|---|---|---|---|
| Approvals (NDA018936, 89 rows) | 495 | 10,551 | 35,399 | 47,279 (all 89) | ~102 K proj. |
| Pathway genes (R-HSA-162582) | 277 | 3,759 | 14,832 | ~28.7 K proj. | ~56 K proj. |
| Trial locations (NCT04613596, 776) | 273 | 5,797 | 16,970 | 30,387 | ~58 K proj. |
| Drug label search (indication cancer) | 422 | 4,748 | 21,643 | ~43 K proj. | ~85 K proj. |
| Trial search (melanoma) | 602 | 7,109 | 30,589 | 61,265 | ~121 K proj. |

"proj." is the measured page plus measured bytes per row, where today's cap stopped the call. Recommendation: **200** for lists cut from a stored snapshot or a whole record (trial sections, pathway genes and events, approvals, protein relations, cited-by lists); the source is not asked again, and 200 narrow rows stay under about 60 KB (17 K tokens). 776 sites become 4 calls instead of 16. **100** for searches the source pages itself, where the source's own page allows it; a source with a smaller native page keeps its cap and its help says so. No whole-list option: all 776 sites would be about 215 KB (60 K tokens).

## Fewer columns would do more than a higher cap

`--fields` exists today but only for search rows; on a section it refuses ("--fields chooses columns of search rows only"). Measured: trial search, 100 rows, 61,265 bytes; with `--fields nct_id,title,status` 20,093 (-67%); with `status` 9,125 (-85%). Label search, 50 rows, 21,643 to 13,000 (-40%). Location rows, recomputed from the reply, go from 274 to 140 bytes with facility, city, state, country and status, so 200 narrowed sites cost what 100 full ones do. Recommend a follow-up ticket extending `--fields` to section rows; it is outside 1061. Two other per-row costs `--fields` cannot cut: a `next` pivot step per row (pathway genes 47 bytes, label search about 120) and URLs rebuilt from IDs.

## Schema and help impact

- CLI: one declaration in `grammar/query_aliases.rs` maps `limit`/`offset` onto the section fields for any command that has only the section pair, and back for `get trial`. No new query fields; the query echo shows the canonical field.
- MCP: `mcp/typed/forms.rs` advertises one name per action (`limit`, `offset`) and accepts the other unadvertised, as it does `input` for `id`. No action grows; section actions shrink by a few characters. Tool descriptions are 339 to 906 characters, far under 3,500; the largest action budget in play is `get_drug` (2,900).
- Help words that state caps change ("one to fifty", "At most 50 rows, or 200 for Drugs@FDA approvals"); schema maximums change.

## Cases to add (replay fixtures)

- Page to the end with the same `--limit/--offset`: `search trial --condition melanoma`, `get drug NDA022363 approvals --source drugsfda`, `get pathway R-HSA-5693579 genes` without a pool, `get trial NCT04613596 locations --limit 200`.
- Each old form still works: `--section-limit/--section-offset`, `pathway genes --pool`, and typed `section_limit`.
- Offset past the end gives an empty page with total: 50 on the 45 pathway genes; 800 on 776 sites.
- A given pool pins the snapshot; an unknown pool keeps its refusal.
- `--page 2` and `--limit 201` are refused naming `--offset` and the cap.
- `get variant ID clinvar --limit 2 --offset 2` pages the section.
