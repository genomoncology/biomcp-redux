# 1073 — unknown-words-after-a-command-are-refused

Status: OPEN.

Milestone: 7

## Outcome

A word BioMCP does not understand is refused, never dropped. `biomcp get gene BRAF bogussection` and `biomcp get gene BRAF extra words here` say which word was not understood and list the sections the record has, and exit as a usage error.

## Why

Found by the review of ticket 1064 on 2026-11-19, on main before that change: both commands exit 0 and return the plain gene card. A caller that misspells a section gets a reply that looks like success and does not hold what it asked for. An agent then either reports the card as if it were the section or spends turns working out what happened.

## Proof

- Proof: a case over every entity's `get` (gene, variant, disease, drug, trial, article, protein, pathway, phenotype, diagnostic, cell line, study) with one unknown trailing word: each is refused as usage, names the word, and its `next` step runs. The developer first lists which entities drop the word today.
- Proof: several IDs given on purpose, as in `get gene BRCA1 BRCA2 --section clingen`, still work.

## Starts from

- Starts from: the review job j43131 for ticket 1064; the usage refusal builder as ticket 1064 leaves it.
- PM decision, 2026-11-19: where a trailing word could be either a second ID or a section, say in the refusal which it was taken for. A near match to a real section may be named in the refusal, but is never applied silently.

## Keeps

- Keeps: every call that is valid today returns the same reply. The refusal stays short.

## Defers

- Defers: the same check for typed MCP calls if ticket 1064 does not already cover unknown fields there.

## Changes

- Changes: the `get` argument handling for each entity, cases, help where it lists sections.

## Decisions

- PM decision, 2026-11-19 (after the first build): letter case never decides ID or section; `get gene BRAF egfr` and typed `id: ["braf","egfr"]` keep working. A trailing word after the first ID is, in this order and without a request: (a) a section when it matches one of this entity's sections in any case; (b) refused as an unknown section when it is near one of them or matches another entity's section name, with the near section as the next call when that call checks; (c) otherwise an ID in any case. A real symbol caught by (b) is read as an ID when given first or with `--section`.
- Near match as built: the same spelling up to letter case, dashes, underscores and one plural s; or the single nearest section within one edit of a name of five to seven letters, or two edits of a name of eight or more. Names under five letters (such as `all`, `cna`, `pdf`) match only by spelling, so ALK is not read as `all`.
- Collision check, 2026-11-19, over the HGNC complete set (symbols, previous symbols and aliases, 103,579 words) and human UniProt entry names and gene names (378,219 words): for gene, ARMS (alias) and PDF (approved symbol) name other records' sections and prostein (alias) is near `protein`; for protein, also CNA, isoform and protein from UniProt gene-name fields. No UniProt entry name collides. Each still reads as an ID given first or with `--section`.
- PM decision, 2026-11-19: a word near nothing is an ID; when a get of several IDs finds some and not others, the reply names the IDs not found in a top-level `absent` list after `status`. It does not make the reply `partial`: an absent ID is an answer, not a failed source. It sits beside 1059's `incomplete` rather than in it, because `incomplete` entries name a source that failed and mean `partial`.
- PM decision, 2026-11-19: a refusal names the word only when it is plain (letters, digits, dash and underscore, up to 64 characters), as unknown option names already are; other text is never repeated, as the error envelope requires.
- Typed `id: "BRAF bogussection"` is one ID with a space, sent as one lookup; its reply is `absent` at the top. It is not split, because some IDs, such as drug names, hold spaces.
- PM decision, 2026-11-19: pgx keeps its retirement guidance for unknown section words.
