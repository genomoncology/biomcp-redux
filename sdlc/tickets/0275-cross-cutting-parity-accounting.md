# 0275 — Account for the 0.9 commands that no entity slice owns

Status: OPEN.

Milestone: 4

Depends on: 0270, 0276

## Outcome

Every 0.9 leaf command and option is ported, renamed or dropped by a named ruling in an accounting table, so the parity ledger in ROADMAP.md shows zero not yet decided. A new `sdlc/planning/slices/cross-cutting.md` accounting table covers the 0.9 leaf commands that no entity slice lists, such as health, serve, skill, cache, chart, update or batch commands, each with its 1.0 spelling, its deferral or its retirement and the ruling that decides it. The two gene rows left undecided are closed in place: `gene drugs` is dropped under the pathway ruling that retired reverse gene-to-drug search, and `gene cspec document` moves to the variant slice's table.

- Why: milestone 4 is done only when every 0.9 search and get command and option is ported, renamed or dropped by a named ruling, and the ledger still shows 33 unlisted commands plus 2 gene rows.
- Starts from: `scripts/parity.py` and its 112-command, 375-option baseline; the v0.9.3 tag's clap command definitions as the inventory; every slice accounting table; ROADMAP.md's parity table; architecture.md's command grammar; and the 2026-10-26 rulings.
- Keeps: the accounting table format, the generated ledger and the docs check. A command whose disposition needs a product ruling the PM has not made goes to the PM with options before this ticket lands, and no disposition is guessed.
- Changes: the new cross-cutting accounting file, the gene, variant, drug and pharmacogenomics slice tables, and the regenerated ROADMAP.md ledger.
- PM ruling 2026-10-26 after the milestone 4 review: on top of 0276's inventory, the unlisted real leaves are `search all`, `discover`, `batch`, `list`, `health`, `serve`, `serve-http`, `mcp tools`, `mcp-config`, four `cache` commands, `gencc sync`, four `skill` commands, twelve `chart` commands, `update`, `uninstall` and `version`. The `skill` rows cite 0187, including `skill status`. Variant filters held undecided by variant ruling 7, `gene cspec document` with `variant cspec`, and the undecided drug and pharmacogenomics options move to Deferred by ruling with their rulings, so zero undecided means each row is ported, renamed, dropped or deferred by a named ruling.
- Proof: `python3 scripts/parity.py` prints zero not yet decided commands and options with totals of 112 and 375, and `make docs-check` passes. A reviewer checks each new row against v0.9.3 help output and an existing 1.0 command or ruling.
- Defers: implementing any newly ported command, which gets its own ticket.
