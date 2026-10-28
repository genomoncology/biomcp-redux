# Artifact license sources

The published 0.9.3 wheel supplies the metadata contract. PyPI's release JSON at https://pypi.org/pypi/biomcp-cli/0.9.3/json identifies `biomcp_cli-0.9.3-py3-none-macosx_10_12_x86_64.whl`, SHA-256 `77a152f7cf1b7e6e81f8f0ef85c4141668cf99159659065447ec75daf9403eeb`. Its METADATA declares `License: MIT`, Homepage and Documentation URLs at https://biomcp.org, Repository at https://github.com/genomoncology/biomcp, and the description marker `mcp-name: io.github.genomoncology/biomcp`.

The root LICENSE copies the bytes at https://raw.githubusercontent.com/genomoncology/biomcp/v0.9.3/LICENSE, including the 2025 Ian Maurer copyright.

The pinned cargo-about source archive at https://crates.io/api/v1/crates/cargo-about/0.9.3/download documents `generate --locked --workspace --fail --format json`. Its `docs/src/cli/generate/config.md` describes accepted-license order and crate exceptions. Its `src/generate.rs` retains distinct license texts, copyright holders and the packages using each text. The producer derives the accepted policy from deny.toml and ships the generated texts without maintaining a dependency inventory. The tool pin lives in release/toolchain.json.
