# New sources follow open by default

Date: 2026-10-26

Decided by Ian.

BioMCP is an open-source tool for academic and commercial users, and its default experience is academic. Semantic Scholar, ORCID and AlphaGenome follow `2026-10-24-sources-are-open-by-default.md` like every other source, so no separate license ruling is needed before building or capturing them.

- Each source carries its reviewed `Use` label, terms name and publisher page. Semantic Scholar and ORCID are `noncommercial`. AlphaGenome is `restricted`, because its terms exclude commercial entities.
- Commercial users switch them off with `"disable_sources": ["noncommercial", "restricted"]`, and help and docs say so.
- Keys come from the XDG config file, then the 0.9 environment variable, as `2026-10-25-legacy-key-names.md` says. A key is read only when its source is called. AlphaGenome runs only with the user's `ALPHAGENOME_API_KEY`, and Semantic Scholar and ORCID run at their public rate without one.
- Development captures may inherit the machine's 0.9 keys by name. The agent never prints, copies or records a key value.

This replaces the PM's 2026-10-26 pause on new Semantic Scholar, ORCID and AlphaGenome captures and the blocked line on 0198.
