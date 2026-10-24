# Sources are open by default

Date: 2026-10-24

Decided by Ian.

Most BioMCP users are academic researchers, so every source and every field is on by default, including fields whose terms allow only noncommercial use. A commercial user is responsible for knowing each source's terms and switches off what their use does not allow.

- Each source and each component field that carries its own terms is labeled with its `Use` (`open`, `attribution`, `noncommercial`, `restricted`), its terms name and its publisher's page, in the output and in `biomcp health`.
- `disable_sources` in the config file and `--disable-sources` accept source names and the groups `noncommercial` and `restricted`, as 0057 built. A group also switches off component fields inside an aggregator, such as dbNSFP components carried by MyVariant, while the aggregator's other fields stay on. Switched-off data reports `skipped` with reason `disabled` and sends no request for it.
- Help and the docs tell commercial users to set `"disable_sources": ["noncommercial", "restricted"]`.
- Keys keep their own rules: a source whose terms require the user's key, such as OncoKB, runs only with it (`2026-10-24-keyed-sources-run-only-with-the-user-s-key.md`); a source whose key only raises the rate runs at its public rate without one and at the higher rate with one.

This replaces the PM's interim exclusion of dbNSFP's academic-branch components in 0092 (m4589).
