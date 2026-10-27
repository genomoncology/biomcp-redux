# 0340 — Accept common NO_PROXY entries and name the cause of configuration errors

Status: OPEN.

Milestone: 5

## Outcome

`NO_PROXY` accepts bare IPv6 addresses such as `::1` and CIDR ranges such as `10.0.0.0/8`, and matches hosts against them. An entry BioMCP still cannot parse is skipped with a notice instead of failing the command. Every configuration failure names its cause: the environment variable or the config file field, never a value. `config.json` accepts `auto_sync_max_bytes`, as SKILL.md documents.

- Why: the docs lead's product findings report on main a01667d (2026-10-27), rechecked by the PM on the release candidate build from hosted run 40542150253. `NO_PROXY=::1` or `NO_PROXY=10.0.0.0/8` makes every command fail with `local.failure` and exit 5, including `--offline` commands, and cloud and corporate machines commonly set these entries. An unknown config key such as `formatt` fails the same way with no field named. `{"auto_sync_max_bytes":1000}` is rejected although SKILL.md presents it as a setting.
- Starts from: `config.rs` proxy and config loading, `biomcp_fetch::Network::new`, the architecture's corporate routing section and the Fetch `auto_sync_max_bytes` setting.
- Rules: PM ruling 2026-10-27 replaces the architecture's line that CIDR refuses as a local configuration failure; update that section. Malformed ports and credentials in bypass rules still refuse, now with a named cause. Proxy credentials never enter replies, recordings or errors.
- Proof: cases where `NO_PROXY` holds `::1`, `[::1]:8080`, `10.0.0.0/8` and `fd00::/8` run an offline command with exit 0 and route matching hosts directly; an unparseable entry yields a notice; `{"formatt":"md"}` names `formatt`; `auto_sync_max_bytes` in config limits an automatic sync.
- Changes: `config.rs`, Fetch network routing, the architecture section, cases and SKILL.md if wording changes.
- Defers: nothing.
