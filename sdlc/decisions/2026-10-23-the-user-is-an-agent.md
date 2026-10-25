# The user is an agent

Decided 2026-10-23 by Ian.

- BioMCP's user is an agent that supports cancer care decisions for clinicians.
- Ideal state is speed and progressive disclosure: return just enough, and tell the agent which subcommand or option drills deeper.
- Trials set the pattern. A search returns trial headers, and subcommands drill into locations, outcomes and other sections. Every entity follows it.
- The CLI tree must be easy for an agent to understand: one spelling per idea, `search` for headers, `get` for a card, `<entity> <relation> <id>` for drill-downs, and typed next steps in every response.

Amended 2026-10-25 by Ian: the agent may support any medical question, such as cancer care, rare disease or other clinical care, not only cancer care. The limit is that BioMCP organizes source information to answer questions.

Clarified 2026-10-25 by Ian: the target audience is biomedical research on any human health question. Clinicians are one group within it. 

Framed 2026-10-25 by Ian: BioMCP is entity-driven access to data sources and APIs. It can download a source's data as a local bundle, search for entities, and get or fetch a specific entity with its sections and relations. It never downloads, installs or runs other bioinformatics tools on the user's behalf. An agent may use BioMCP alongside other tools; that portfolio sits outside BioMCP.
