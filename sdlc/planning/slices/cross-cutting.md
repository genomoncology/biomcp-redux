# Cross-cutting command accounting

This table accounts for released leaves that have no entity owner. [The released inventory](../parity-0.9.3.json) and [the generator](../../../scripts/parity.py) own the denominator. The retained released executable, source marker and help captures were checked against that inventory. Only help was captured; this accounting makes no source call or command timing claim.

The 0.9.3 `batch` command has positional entity choices, not separate entity leaves. Its four canonical option slots are `--mode`, `--sections`, `--source` and `--pdf`. `serve` is a parent with a `serve http` alias for `serve-http`; parent flags and aliases add no leaves or slots. `list` has positional topic values. Global flags and positional sections stay outside the denominator.

A deferred row records an accepted future capability, not shipped implementation. This ticket changes documentation only. The named product tickets own implementation and executable migration guidance.

<!-- parity-accounting -->
| 0.9 leaf command | Command outcome | Ported or renamed options | Dropped options | Deferred options | Undecided options | Ruling |
| --- | --- | --- | --- | --- | --- | --- |
| `cache clean` | Ported or renamed | — | `--dry-run`, `--max-age`, `--max-size` | — | — | Architecture section 7.5 replaces maintenance with store clean, which removes expired replies, pools, snapshots and incompatible cache files. It has no age or size policy, dry run or size cap. |
| `chart bar` | Dropped by ruling | — | — | — | — | The 2026-10-23 search-and-get scope ruling excludes charts and study analysis from 1.0. Request source-native records through explicit entity commands instead. |
| `chart box` | Dropped by ruling | — | — | — | — | The 2026-10-23 search-and-get scope ruling excludes charts and study analysis from 1.0. Request source-native records through explicit entity commands instead. |
| `chart density` | Dropped by ruling | — | — | — | — | The 2026-10-23 search-and-get scope ruling excludes charts and study analysis from 1.0. Request source-native records through explicit entity commands instead. |
| `chart heatmap` | Dropped by ruling | — | — | — | — | The 2026-10-23 search-and-get scope ruling excludes charts and study analysis from 1.0. Request source-native records through explicit entity commands instead. |
| `chart histogram` | Dropped by ruling | — | — | — | — | The 2026-10-23 search-and-get scope ruling excludes charts and study analysis from 1.0. Request source-native records through explicit entity commands instead. |
| `chart pie` | Dropped by ruling | — | — | — | — | The 2026-10-23 search-and-get scope ruling excludes charts and study analysis from 1.0. Request source-native records through explicit entity commands instead. |
| `chart ridgeline` | Dropped by ruling | — | — | — | — | The 2026-10-23 search-and-get scope ruling excludes charts and study analysis from 1.0. Request source-native records through explicit entity commands instead. |
| `chart scatter` | Dropped by ruling | — | — | — | — | The 2026-10-23 search-and-get scope ruling excludes charts and study analysis from 1.0. Request source-native records through explicit entity commands instead. |
| `chart stacked-bar` | Dropped by ruling | — | — | — | — | The 2026-10-23 search-and-get scope ruling excludes charts and study analysis from 1.0. Request source-native records through explicit entity commands instead. |
| `chart survival` | Dropped by ruling | — | — | — | — | The 2026-10-23 search-and-get scope ruling excludes charts and study analysis from 1.0. Request source-native records through explicit entity commands instead. |
| `chart violin` | Dropped by ruling | — | — | — | — | The 2026-10-23 search-and-get scope ruling excludes charts and study analysis from 1.0. Request source-native records through explicit entity commands instead. |
| `chart waterfall` | Dropped by ruling | — | — | — | — | The 2026-10-23 search-and-get scope ruling excludes charts and study analysis from 1.0. Request source-native records through explicit entity commands instead. |
| `discover` | Dropped by ruling | — | `--full`, `--limit`, `--offset`, `--search` | — | — | The 2026-10-24 no-text-heuristics ruling removes intent routing, local ranking and rewritten fallback searches. Choose an explicit entity search and send literal source text. |
| `gencc sync` | Deferred by ruling | — | — | — | — | The gene slice section 1 defers the GenCC bundle and section; ClinGen supplies the accepted validity source. No GenCC synchronization is approved for the initial slice. |
| `serve-http` | Ported or renamed | `--allowed-hosts`, `--host`, `--port` | `--unsafe-allow-any-host` | — | — | The 2026-10-24 HTTP ruling and architecture section 8.9 use serve --http ADDR; host and port become ADDR and allowed-hosts becomes repeatable --allow-host HOST. A non-loopback listener requires an explicit allowlist. |
| `skill install` | Deferred by ruling | — | — | `--force` | — | Ticket 0187 owns the rewritten 1.0 guide and this released leaf, including status under its 2026-10-26 addition. It must prove commands from passing case steps before implementation earns ported credit. |
| `skill list` | Deferred by ruling | — | — | — | — | Ticket 0187 owns the rewritten 1.0 guide and this released leaf, including status under its 2026-10-26 addition. It must prove commands from passing case steps before implementation earns ported credit. |
| `skill render` | Deferred by ruling | — | — | — | — | Ticket 0187 owns the rewritten 1.0 guide and this released leaf, including status under its 2026-10-26 addition. It must prove commands from passing case steps before implementation earns ported credit. |
| `skill status` | Deferred by ruling | — | — | — | — | Ticket 0187 owns the rewritten 1.0 guide and this released leaf, including status under its 2026-10-26 addition. It must prove commands from passing case steps before implementation earns ported credit. |
<!-- /parity-accounting -->
