# Agent tooling goes in 1.0; the website and saved data are 1.1

Date: 2026-11-11. Changes the same day's "1.0 ships as it is" and the order in 2026-11-11-1-1-is-ordered-by-measured-wins.

## Ruling

Ian, 2026-11-11, after the 1.0 baseline: "go ahead and start planning to get tickets done and into the queue for 1.0. I have no interest in really publishing 1.0. I just have an interest in keeping things segmented between the datasets, the website stuff, and the tooling needed on the web and on the MCP. Getting the tooling right so the agents are efficient and effective is a top priority and should be pushed into 1.0. The website and the dataset stuff should be 1.1."

Also Ian, same day: Amazon Quick goes live in late November, is "the first real push", and does not load every MCP tool up front but finds tools by search.

## What it means here

- 1.0 is not published until its agent tooling is efficient and effective. Milestone 7 (efficiency) is 1.0 work and lands on main.
- 1.1 is saved work, datasets, documents and the website. It stays on branch `1.1`.
- PM's reading, not Ian's words: the small field and filter fixes already being built (tickets 0823 to 0830) are tooling, so they join 1.0 when reviewed. New sources and the new composite commands are tooling too and follow in 1.0 after the efficiency work, unless Ian says they wait.
- The typed MCP tools stay hosting's mode for Amazon Quick. The baseline's loss in that mode is treated as an instructions and argument-shape problem to fix, not a reason to drop it.

## Evidence

`biomed-agent-bench/results/baseline-1.0.md`: right answers tie across modes; only the command line on one model beats plain curl on tokens; agents leave BioMCP for curl; about half of searches return partial; the typed mode on Sonnet has 128 calls refused before reaching BioMCP.
