# Site

The documentation site, built with Astro and Starlight. It builds to `site/dist/`.

## Build

1. Build the program from this commit: `cargo build --release` in the repository root.
2. Install the site's packages: `npm ci` in `site/`.
3. Run every example and save its output: `npm run examples`.
4. Build and check the site: `npm run build`.

`npm run dev` serves the site while you write.

## How the site is made

- `src/content/docs/` holds every page. Each page's front matter carries its brief: `title`, `description`, `goal`, `reader`, `kind` and `verified`.
- `examples/<id>/run.sh` is an example the build runs against `../target/release/biomcp`. A `show.<ext>` file, when present, is what the page shows. `output.txt` and `exit` hold the last run. Pages include an example with `<Example id="..." />`.
- `src/data/site.json` holds each fact that appears on more than one page, such as the version and the install lines. Pages read it with `<Fact k="..." />` and `<InstallLine />`.
- `questions/` holds the question list. `npm run answers` writes one answers page per entry marked `answers_page: true`.
- `scripts/gen-reference.mjs` writes the reference pages from the program's help. Fix a wrong reference sentence in the help text, never in the page.
- `WRITING.md` holds the word list's reasons and the rules a check can't see. `src/data/words.json` holds the words the build rejects.

## Checks

`npm run build` fails when an example's run changes, the reference no longer matches the build, Astro warns, a link breaks, a page has no incoming link, front matter is incomplete, a rejected word appears in prose, or the agent files miss a page.

BioMCP examples call live public services. The build checks each example's exit code and saves the fresh output. An example whose source can't be reached from the build machine carries an `unverified` file that says why, and the build skips it.
