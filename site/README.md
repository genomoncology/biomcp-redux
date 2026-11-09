# Site

The documentation site, built with Astro and Starlight. It builds to `site/dist/`.

## What a clean machine needs

- The Node version and site packages pinned in `release/toolchain.json`, and npm.
- The Rust toolchain from `rust-toolchain.toml`, to build the program.
- `bash`, `jq` and `curl`. Most examples pipe through `jq`; the server examples call `curl`.
- The explicit live-example refresh needs network access to its permitted public sources and bundle hosts. Routine builds rerun metadata-only examples in an empty cache and render saved permitted outputs.
- On a network that needs a proxy or a private certificate authority, a BioMCP config file for the examples (see `SITE_EXAMPLE_CONFIG` below).

## Build

1. Build the program from this commit: `cargo build --locked -p biomcp --bin biomcp` in the repository root.
2. Install the site's packages: `npm ci` in `site/`.
3. Run every example and save its output: `npm run examples`. Only this command changes files in `examples/`.
4. Build and check the site: `npm run build`. The build changes no tracked file.

`npm run dev` serves the site while you write. `npm run readme` renders the facts and example output in the repository README; the build only checks them.

## Environment variables

- `SITE_EXAMPLE_CONFIG`: the path of a BioMCP `config.json` that the examples and the reference generator use. Each copies only its `proxy` and `ca_bundle` settings to `biomcp/config.json` in a temporary config folder. Keys and unrelated settings never reach an example. Live examples disable the `noncommercial` and `licensed` groups; metadata-only examples use a separate empty configuration. Use it for machine settings such as `proxy` and `ca_bundle`, never for keys. Keep the file outside the repository; never commit a machine's config.

## How the site is made

- `src/content/docs/` holds every page. Each page's front matter carries its brief: `title`, `description`, `goal`, `reader`, `kind` and `verified`.
- `examples/<id>/run.sh` is an example the build runs against `../target/debug/biomcp` (or `BIOMCP_SITE_BIN_DIR`). Pages include an example with `<Example id="..." />`. The files in an example's folder:
  - `show.<ext>`, when present, is what the page shows; `run.sh` runs it.
  - `output.txt`, `exit` and `ran` (the day of the run) hold the last saved run. `npm run examples` writes them; never edit them by hand. A run that fails a rule below is not saved.
  - `expect-status` names a top-level reply status the example shows on purpose, such as `failed` for a missing key or bundle. The run must produce it.
  - `unverified` holds the reason, for readers, that the build does not run this example. The page shows the command and the reason. `npm run examples -- ID` still runs it when named.
  - `needs` lists example ids that must run before this one, one per line.
  - `exact` marks an example whose output reads no live source: help, keys, usage errors, offline health, skill and MCP configuration output. The build compares its output and exit byte for byte with the saved run, so a program change that alters it fails the build until the example is rerun and its page reviewed. Mark every such example.
- `src/data/site.json` holds each fact that appears on more than one page, such as the version and the install lines. Pages read it with `<Fact k="..." />` and `<InstallLine />`. Prose never types the version.
- `questions/` holds the question list. `npm run answers` writes one answers page per entry marked `answers_page: true`. An answers page is verified on the day its example last ran.
- `scripts/gen-reference.mjs` writes the reference pages and the source pages from detailed command help, schemas and the registry. `src/data/command-notes.yaml` gives each command a plain one-sentence description, and `src/data/schema-notes.yaml` gives plain glosses for jargon in schema descriptions; the help text stays quoted on each command page, and the jargon is a product fix. Fix a wrong reference sentence in the help text, never in the page. A page's `verified` date changes when its text changes.
- The existing `make docs-generate` command writes complete contracts to `src/data/help.json` alongside the Markdown reference. The site reads canonical reply schemas from those contracts; compact interactive help is not a complete catalog. It makes no biomedical source calls. Historical live schema captures are omitted from the release tree.
- `scripts/pages.mjs` writes each page's Markdown twin and `llms-full.txt` text. It renders every component outside code; the shared ones are there, and this site's own (`TerminalHero`, `InstallLine`, `EnvelopeTable`, `SourceTable`, `CommandTable`, `KeyTable`) are in `src/lib/markdown-components.mjs`. A component without a Markdown form fails the build, and the check fails on a raw component tag in a twin or `llms-full.txt`.
- `WRITING.md` holds the word list's reasons and the rules a check can't see. `src/data/words.json` holds the words the build rejects.
- `public/social-card.png` is the checked-in link-preview card of every page. Regenerate it only through an approved isolated browser runner after a brand change.

## How the examples run

BioMCP examples call live public services. Each example runs as `bash -o pipefail run.sh`, with standard output and standard error merged, so a failed command anywhere in a pipe fails it. An example also fails when its output holds `jq: error`, or when a JSON reply's top-level `status` is `failed` and its `expect-status` file does not name `failed`.

The examples of one build share a fresh environment: a temporary `HOME`, `XDG_CACHE_HOME`, `XDG_CONFIG_HOME` and `TMPDIR`, and `PATH` with the built program first. Bundles download once per build into that cache. No variable ending in `_API_KEY`, `_TOKEN`, `_SECRET` or `_PASSWORD` reaches an example, so pages show what a reader without keys sees. From the shell only `HTTPS_PROXY`, `HTTP_PROXY`, `NO_PROXY` and `SSL_CERT_FILE` pass through, plus `SITE_EXAMPLE_CONFIG`.

Examples that run `biomcp sync`, and every example another one `needs`, run one at a time before the rest start, so no read races a bundle download. Keyless requests to some sources are rate limited, so a failed example runs up to twice more after a pause. An example that runs longer than its limit is stopped with every process it started.

`npm run build` reruns deterministic examples marked `exact`; refresh live examples with `npm run examples`. The check writes fresh output and writes the fresh output to `.example-runs/`, which git ignores. It fails when an exit code differs from the saved run, when the shape of the replies changes (the schema and status of each reply, or the keys of each JSON object), or when the output of an `exact` example differs at all. It also stops when `src/data/site.json` `version` differs from the version that `biomcp version --verbose` prints.

## Checks

`npm run build` fails when:

- an example fails one of the rules above, or the built program's version differs from `site.json`;
- the README's facts or example output differ from `site.json` and the saved runs;
- the reference no longer matches the build;
- Astro warns;
- a link or an anchor breaks, or a page has no incoming link;
- two source files make the same page;
- front matter is incomplete;
- a rejected word appears in prose (quoted words and blockquotes are exempt), or prose, a title or a description types the version;
- the agent files miss a page, or a Markdown twin or `llms-full.txt` holds a raw component tag;
- a page runs an example that reads a file from its folder, and neither the page nor a page it links shows that file;
- an old address in `src/data/old-addresses.txt`, or the `.md` twin of an old page address, no longer resolves, or a redirect in `src/data/redirects.json` lands on anything but a built page;
- `robots.txt`, `sitemap.xml`, `version.json`, the social card, or a page's `og:image` is missing.

## Visual checks

The normal build checks content, generated reference, links, examples and the exact installer. Browser layout and screenshot checks run separately in an approved isolated browser runner; they are not prerequisites for routine site builds. The checked-in social card is retained.

## Publishing

Publish only after the release that `site.json` names has its artifacts, since every install line installs that release.

`dist/` holds every page, its Markdown twin, `llms.txt`, `llms-full.txt`, `robots.txt`, `sitemap-index.xml` and the same index at `sitemap.xml`, and `version.json` (the version, the source commit and branch, the program's build commit and the build time).

Every install line runs `https://biomcp.org/install.sh`. The build copies `release/install.sh` to `dist/install.sh` and refuses a missing or changed copy. Publishing preserves that file.
