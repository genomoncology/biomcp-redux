# Site

The documentation site, built with Astro and Starlight. It builds to `site/dist/`.

## What a clean machine needs

- Node 22.12 or later (`package.json` `engines`) and npm.
- The Rust toolchain from `rust-toolchain.toml`, to build the program.
- `bash`, `jq` and `curl`. Most examples pipe through `jq`; the server examples call `curl`.
- Network to the public sources the examples read, and to the hosts of the bundles they synchronize. Every build starts from an empty cache.
- On a network that needs a proxy or a private certificate authority, a BioMCP config file for the examples (see `SITE_EXAMPLE_CONFIG` below).

## Build

1. Build the program from this commit: `cargo build --release` in the repository root.
2. Install the site's packages: `npm ci` in `site/`.
3. Run every example and save its output: `npm run examples`. Only this command changes files in `examples/`.
4. Build and check the site: `npm run build`. The build changes no tracked file.

`npm run dev` serves the site while you write. `npm run readme` renders the facts and example output in the repository README; the build only checks them.

## Environment variables

- `SITE_EXAMPLE_CONFIG`: the path of a BioMCP `config.json` that the examples and the reference generator use. Each copies it to `biomcp/config.json` in its own temporary config folder. Use it for machine settings such as `proxy` and `ca_bundle`, never for keys. Keep the file outside the repository; never commit a machine's config.

## How the site is made

- `src/content/docs/` holds every page. Each page's front matter carries its brief: `title`, `description`, `goal`, `reader`, `kind` and `verified`.
- `examples/<id>/run.sh` is an example the build runs against `../target/release/biomcp`. Pages include an example with `<Example id="..." />`. The files in an example's folder:
  - `show.<ext>`, when present, is what the page shows; `run.sh` runs it.
  - `output.txt`, `exit` and `ran` (the day of the run) hold the last saved run. `npm run examples` writes them; never edit them by hand. A run that fails a rule below is not saved.
  - `expect-status` names a top-level reply status the example shows on purpose, such as `failed` for a missing key or bundle. The run must produce it.
  - `unverified` holds the reason, for readers, that the build does not run this example. The page shows the command and the reason. `npm run examples -- ID` still runs it when named.
  - `needs` lists example ids that must run before this one, one per line.
- `src/data/site.json` holds each fact that appears on more than one page, such as the version and the install lines. Pages read it with `<Fact k="..." />` and `<InstallLine />`. Prose never types the version.
- `questions/` holds the question list. `npm run answers` writes one answers page per entry marked `answers_page: true`. An answers page is verified on the day its example last ran.
- `scripts/gen-reference.mjs` writes the reference pages and the source pages from the program's help. Fix a wrong reference sentence in the help text, never in the page. A page's `verified` date changes when its text changes.
- `WRITING.md` holds the word list's reasons and the rules a check can't see. `src/data/words.json` holds the words the build rejects.
- `public/social-card.png` is the link-preview card of every page. After a brand change, run `NODE_PATH=$(npm root -g) npm run social-card` (it needs Playwright with Chromium) and commit the PNG.

## How the examples run

BioMCP examples call live public services. Each example runs as `bash -o pipefail run.sh`, with standard output and standard error merged, so a failed command anywhere in a pipe fails it. An example also fails when its output holds `jq: error`, or when a JSON reply's top-level `status` is `failed` and its `expect-status` file does not name `failed`.

The examples of one build share a fresh environment: a temporary `HOME`, `XDG_CACHE_HOME`, `XDG_CONFIG_HOME` and `TMPDIR`, and `PATH` with the built program first. Bundles download once per build into that cache. No variable ending in `_API_KEY`, `_TOKEN`, `_SECRET` or `_PASSWORD` reaches an example, so pages show what a reader without keys sees. From the shell only `HTTPS_PROXY`, `HTTP_PROXY`, `NO_PROXY` and `SSL_CERT_FILE` pass through, plus `SITE_EXAMPLE_CONFIG`.

Examples that run `biomcp sync`, and every example another one `needs`, run one at a time before the rest start, so no read races a bundle download. Keyless requests to some sources are rate limited, so a failed example runs up to twice more after a pause. An example that runs longer than its limit is stopped with every process it started.

`npm run build` reruns every example and writes the fresh output to `.example-runs/`, which git ignores. It fails when an exit code differs from the saved run, or when the shape of the replies changes: the schema and status of each reply, or the keys of each JSON object. It also stops when `src/data/site.json` `version` differs from the version that `biomcp version --verbose` prints.

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
- the agent files miss a page;
- an old address in `src/data/old-addresses.txt`, or the `.md` twin of an old page address, no longer resolves, or a redirect in `src/data/redirects.json` lands on anything but a built page;
- `robots.txt`, `sitemap.xml`, `version.json`, the social card, or a page's `og:image` is missing.

## Publishing

Publish only after the release that `site.json` names has its artifacts, since every install line installs that release.

`dist/` holds every page, its Markdown twin, `llms.txt`, `llms-full.txt`, `robots.txt`, `sitemap-index.xml` and the same index at `sitemap.xml`, and `version.json` (the version, the source commit and branch, the program's build commit and the build time).

Every install line runs `https://biomcp.org/install.sh`, and the build does not make that file. Publishing must copy `release/install.sh` from this repository to `dist/install.sh`. The build lists `/install.sh` as an old address it does not make; `npm run check:publish` fails until the file is in place.
