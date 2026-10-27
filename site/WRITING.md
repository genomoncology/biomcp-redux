# Writing biomcp.org

The rules a check can't see, and the word list behind `src/data/words.json`.

## One name per thing

| Use | Not | Why |
| --- | --- | --- |
| source | backend, provider, API, datasource | The help and every reply use `source` (`--source`, `sources[]`). |
| entity | resource type, object | `search ENTITY`, `get ENTITY` in the help. |
| section | view, part, expansion | Section words in `biomcp help get gene`. |
| relation command | helper, pivot | `protein domains`, `article citations`. 0.9 called some of these helpers. |
| next command | suggestion, hint | The reply field is `next`. |
| use group | license class, tier | `sources[].use`: open, attribution, noncommercial, restricted. |
| bundle | dataset, dump, mirror | `biomcp sync` downloads a bundle. |
| key | token, credential, secret | `biomcp key set`. The environment variable may say TOKEN; prose says key. |
| store | cache database | `biomcp store clean`. The cache is part of the store. |
| reply | response, payload | One JSON object per command. |
| command | subcommand, tool call | The help lists commands. |
| MCP server | MCP endpoint, plugin | `biomcp serve`. The plugin is the Claude Code package that starts it. |

## Identifiers we reuse

BRAF (HGNC:1097), BRAF V600E (rs113488022, clinvar:13961, CA123643, NM_004333.4:c.1799T>A), KRAS, melanoma (MONDO:0005105), NCT06887088, PMID 33123537 (PMC7573235), vemurafenib (CHEMBL1229517, NDA202429, Zelboraf label 38eea320-7e0c-485a-bc30-98c3c45e2763), DPYD, P15056, R-HSA-5674135, A-375 (CVCL_0132), seizure (HP:0001250), rs7903146. Add a new identifier only when none of these shows the behavior.

## Clinical wording

- Pages say how to get data, never what a patient should do.
- A listing, label, guideline or report count is the source's statement. Say so; never turn it into advice.
- Empty is not absence: an empty result describes one source, query and filters.
- Never use a real patient's case. Individual FAERS reports are shown by ID and flags only.
- Every new clinical sentence goes to the clinical reviewer.

## Pages

- One kind per page, its brief in front matter, the outline for its kind.
- Name BioMCP and the version (`<Fact k="version" />`) in the first paragraph.
- Every command shown ran in the build (`examples/`). A fragment that did not run says so.
- End each page with a link to the next useful page.
- Generated reference pages are never edited. Fix the help text, then run `npm run reference`.
- Generated pages quote the program's help and schemas, which still say "provider" in places. A word marked `generated` in `src/data/words.json` passes on generated pages and fails everywhere else. Hand-written pages render help text through components such as `<CommandTable entity="gene" />`, so they never copy it.
- An example the build does not rerun has an `unverified` file. Write its reason for readers: what the command needs (a key, a reachable host, a bundle) and what the reader sees when that is missing. Never write about the machine that builds the site.
