# The user is an agent

Decided 2026-10-23 by Ian.

- BioMCP's user is an agent that supports cancer care decisions for clinicians.
- Ideal state is speed and progressive disclosure: return just enough, and tell the agent which subcommand or option drills deeper.
- Trials set the pattern. A search returns trial headers, and subcommands drill into locations, outcomes and other sections. Every entity follows it.
- The CLI tree must be easy for an agent to understand: one spelling per idea, `search` for headers, `get` for a card, `<entity> <relation> <id>` for drill-downs, and typed next steps in every response.
