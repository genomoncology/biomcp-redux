# The user is an agent

Decided 2026-10-23 by Ian. Scope rewritten 2026-11-01 to match the workspace scope decision.

## Scope

BioMCP's user is an agent. It serves cancer care, rare disease and other medical questions (Ian, 2026-10-25: "They can be restricted to cancer care, rare disease, or really any medical question."), and life sciences and biopharma research from public data, such as biomarker discovery and market analysis of drug targets (Ian, 2026-11-01: "widen it. too restrictive."). BioMCP gives entity-driven access to data sources and APIs: it can download a source's data as a local bundle, search for entities, and get or fetch a specific entity. It does not download, install or run other bioinformatics tools automatically (Ian, 2026-10-25: "I just don't want to be downloading other bioinformatics tools and things like that automatically, but the agent using BioMCP could be using it in a portfolio of tools.").

BioMCP work does not design molecules, antibodies, peptides or sequences, engineer organisms or vectors, or work on toxins. Ian keeps this limit so the team can keep using Claude models (Ian, 2026-11-01: "the exclusion list is a good idea so that I keep working with Claude models").

## Shape

- Ideal state is speed and progressive disclosure: return just enough, and tell the agent which subcommand or option drills deeper.
- Trials set the pattern. A search returns trial headers, and subcommands drill into locations, outcomes and other sections. Every entity follows it.
- The CLI tree must be easy for an agent to understand: one spelling per idea, `search` for headers, `get` for a card, `<entity> <relation> <id>` for drill-downs, and typed next steps in every response.
