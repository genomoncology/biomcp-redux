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
