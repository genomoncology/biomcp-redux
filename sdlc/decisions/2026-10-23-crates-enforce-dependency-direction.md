# Crates enforce dependency direction

Decided 2026-10-23 by the PM, applying Ian's layered design and his ratchet ruling.

- The crates and their allowed dependencies are the table in `planning/architecture.md`.
- `biodata` does no I/O. Its own `clippy.toml` disallows file, environment, network, process and clock items one by one.
- Adapters never depend on the store, the services, rendering or the binary. Rendering depends only on `biodata`.
- Only the binary reads the config file or the environment.
- Cargo and clippy check boundaries. The ratchet check adds only what they cannot, such as which crates name `reqwest`.
