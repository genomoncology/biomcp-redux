# BioMCP

BioMCP searches and fetches biomedical records from public sources: genes, variants, clinical trials, articles, drugs, diseases and <!--fact:counts.entities-->16<!--/fact--> entities in all, across <!--fact:counts.sources-->57<!--/fact--> sources. It is one program with two faces. In the shell it is the `biomcp` command. In an AI agent it is an MCP server with one `biomcp` tool that takes the same commands. It is for researchers, curators and developers who want source records with their identifiers and terms, from the shell or through an agent. This README covers BioMCP <!--fact:version-->1.0.0<!--/fact-->.

Each command sends its words and identifiers over the network to the public sources it names, such as HGNC, PubMed or ClinicalTrials.gov. Do not put names, record numbers or other identifying patient data in a query.

Documentation: **https://biomcp.org/**

## One command

```sh
biomcp get gene BRAF
```

The reply is one JSON object. Its status, the gene and the source outcomes, filtered with `jq`, from the run in the docs build (`site/examples/qs-get-gene`):

<!--example:qs-get-gene-->
```json
{
  "status": "complete",
  "gene": {
    "symbol": "BRAF",
    "hgnc_id": "HGNC:1097",
    "name": "B-Raf proto-oncogene, serine/threonine kinase",
    "location": "7q34"
  },
  "sources": [
    {
      "source": "hgnc",
      "state": "ok",
      "reason": null
    },
    {
      "source": "mygene",
      "state": "ok",
      "reason": null
    },
    {
      "source": "oncokb",
      "state": "skipped",
      "reason": "no_key"
    }
  ]
}
```
<!--/example-->

Every reply names the sources it asked and whether each answered. OncoKB was skipped because it needs your own key.

## Install

<!--fact:install.script-->
```sh
curl -fsSL https://biomcp.org/install.sh | bash
```
<!--/fact-->

Or install the PyPI package `biomcp-cli`, which provides the `biomcp` command:

<!--fact:install.uv-->
```sh
uv tool install biomcp-cli
```
<!--/fact-->

Homebrew, the container image `ghcr.io/genomoncology/biomcp`, the `.mcpb` desktop extension and the Claude Code plugin are on the [install page](https://biomcp.org/getting-started/installation/). Check the install:

```sh
biomcp version
```

## The smallest useful example

Which registered trials match melanoma?

```sh
biomcp search trial --condition melanoma
```

Which PubMed articles match AKT1 E17K?

```sh
biomcp search article 'AKT1 E17K'
```

Add `--format md` to any command for Markdown instead of JSON. Each reply's `next` list holds the commands that page the results or fetch more of a record.

To use BioMCP from Claude, Codex, Cursor, VS Code or another MCP client, print ready-to-paste configuration with:

```sh
biomcp mcp-config
```

## Documentation and next steps

- [Quickstart](https://biomcp.org/getting-started/first-query/): a search, a record and a section in five minutes.
- [Connect an agent](https://biomcp.org/getting-started/mcp-clients/): MCP clients, the skill and the plugin.
- [How BioMCP works](https://biomcp.org/concepts/progressive-disclosure/): the command grammar.
- [Reference](https://biomcp.org/reference/): every command, entity, source and exit code, generated from the build.
- [Upgrading from 0.9](https://biomcp.org/upgrade-from-0-9/).

## What BioMCP supports and what it does not

- Platforms: <!--fact:platforms_detail-->Linux and macOS on x86_64 and arm64, and Windows on x86_64<!--/fact-->.
- Every source is on by default, including noncommercial and restricted ones. Commercial users should set `{"disable_sources": ["noncommercial", "restricted"]}` in `config.json` and check each remaining publisher's terms. Public API access grants no extra reuse rights.
- OncoKB and NCI need your own key: `biomcp key set oncokb`. Other keys are optional and raise rate limits. `biomcp key list` shows their state.
- BioMCP retrieves source records. It does not rank treatments, judge trial eligibility, compute risk from report counts, or make clinical decisions. An empty result shows what one source returned for one query, not that something does not exist.
- BioMCP has no self-update command. Upgrade with the channel that installed it.

## Help, bugs and contributing

- Questions and bug reports: [GitHub issues](https://github.com/genomoncology/biomcp/issues). Include the command, `biomcp version`, and the reply's `error` or `sources` entries.
- Development: the Rust code is in `crates/`. Run `make docs-generate` after command changes, `make docs-check` to check generated documentation and documented examples, and `make landing-gate` for formatting, lint, coverage and dependency checks. [ROADMAP.md](ROADMAP.md) states scope and source availability.
- The documentation site is in `site/`. Its [README](site/README.md) explains how it builds.

## License

MIT. Each data source keeps its own terms; the [sources reference](https://biomcp.org/reference/data-sources/) links each one.
