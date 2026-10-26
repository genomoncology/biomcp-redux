# 0276 — Check the parity ledger against the real 0.9.3 inventory and add a deferred outcome

Status: OPEN.

Milestone: 4

Depends on: 0270

## Outcome

The parity ledger compares every accounting row with a checked-in inventory of the released 0.9.3 leaf commands and their options, so a row that names no real 0.9.3 leaf, or an option the leaf never had, fails `make docs-check`. Not yet decided is the set difference between the inventory and the rows, not a constant minus a count. Accounting rows gain a fourth outcome, Deferred by ruling, which counts as decided and which upgrade.md shows as deferred rather than dropped.

- Why: the milestone 4 review found that `scripts/parity.py` trusts a hard-coded 112 and 375, that slices list per-entity pseudo-leaves such as `list pathway` and `batch trial` that 0.9.3 never had, and that deferrals such as `variant structure` are counted as drops.
- Starts from: `scripts/parity.py`, `make docs-check`, every slice accounting table, upgrade.md's generated table, and the v0.9.3 tag of the public BioMCP repository, whose `--help` tree is the inventory source.
- Rules: the inventory is one generated file checked into `sdlc/planning/`, produced by a script that walks the 0.9.3 help tree and records each leaf with its own options; global flags, positional values and aliases stay excluded as `parity.py` says. Per-entity `list X` and `batch X` rows merge into one `list` row and one `batch` row. Rows whose ruling text says deferred, including `variant structure`, `cell-line drug-response`, `gene cell-lines` and the protein `--disease`, `--existence` and `--include-unreviewed` options, move to Deferred by ruling. Hidden 0.9.3 options that help does not show are added to the inventory by hand with a source line, and the baseline totals follow the inventory.
- Proof: docs-check fails on a planted phantom row, a planted unknown option and a planted missing leaf, and passes on the real tables. upgrade.md shows the deferred rows as deferred. The printed totals equal the inventory's counts.
- Changes: `scripts/parity.py`, the inventory file and its generator, the affected slice tables, ROADMAP.md's ledger and upgrade.md.
- Defers: deciding the remaining undecided rows, which 0275 does on top of this ticket.
- PM ruling 2026-10-26 (m8126 ask 8): a leaf with no row legitimately raises Not yet decided by set difference, so the proof plants a row naming an unknown leaf, a phantom row and a phantom option, each of which fails, plus generated-ledger drift, which fails. Merged per-entity `list X` rows become one canonical `list` row left Not yet decided for 0275 unless an existing ruling settles it.
