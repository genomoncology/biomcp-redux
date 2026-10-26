# 0267 — Get the ORCID author card and its public works

Status: COMPLETE.
Landed: 8f7434c8b0dfbf64620d3cafadf412b7a5e0f3b0

Milestone: 4

Depends on: 0266

## Outcome

`get author orcid:0000-0002-4299-3018` validates the checksum before any request and returns the `AuthorCard` from the exact public ORCID person record, with the public name and the provider link. `author papers orcid:0000-0002-4299-3018` returns the public work groups from one bounded whole works reply, each group with its ordered assertion summaries and each external identifier's type, value and relationship, paged locally with `--limit`, `--offset` and `--pool`, and the page states that ORCID sends the whole works list in one reply. A paper row offers an exact article pivot only for a native PMID or DOI. ORCID registers as a `noncommercial` source with its public API terms and record URL template.

- Why: Questions A17 to A20, A22 and A24 ask what an exact ORCID record claims, and public issues 289 and 290 require exact detail, accepted public visibility and the shared noncommercial group.
- Starts from: author.md's ORCID source boundaries, command shape and case list, with the author rulings and the new-sources decision; 0264's ORCID witnesses; 0265's types; 0266's registration; the source registry and record URL templates from 0250; `SecretName` in `crates/biomcp-fetch/src/declaration.rs`.
- Rules: an ORCID iD with a bad checksum fails before I/O. The returned iD must equal the requested one, or the reply is dropped with a problem and never enters the cache. The public lowercase visibility value is accepted, and every other visibility value stays excluded. A work group is never counted as several publications, and no assertion moves between groups. No Semantic Scholar request is sent for an ORCID input, and no name search or merge links the two sources. The optional token comes from the config file, then `ORCID_ACCESS_TOKEN`, and is read only when ORCID is called; this adds an ORCID secret to `SecretName`. No OAuth exchange, token issuance or account creation. The works reply has a body bound, and a truncated or oversized reply is a failed outcome, never a partial list. `noncommercial` in `disable_sources` switches ORCID off before cache and network.
- Proof: outside-in cases first, watched to fail. Cases reuse 0264's captures where the request matches exactly, and one bounded pm job captures once, with zero retries and the token inherited by name only, any exact request a case needs that those archives lack. A17 and A19 assert exact work groups and native PMID and DOI assertions with their types and relationships; A18 asserts the public name; A20 asserts one group with several assertions counted once; A22 asserts Atul Butte's works with no link to Semantic Scholar 1716151; A24 asserts the lowercase public visibility accepted and another visibility excluded; the absent iD asserts absent distinct from failed; the local page walk asserts no gap or duplicate. A synthetic wrong returned iD asserts the drop and no cache admission. Recorded 429, 500, timeout, broken body and oversized replies assert a failed outcome with source and terms. A `noncommercial` disable case asserts the skip before cache and network. The job measures each command in CLI and MCP output, and each case gets a ceiling at measured size plus 15 percent, recorded in author.md. No licensed content enters the repo. Then make landing-gate runs through pm job.
- Defers: retirements and remaining failure cases to 0269.
- Changes: a new ORCID adapter under `crates/biomcp-adapters/src/`, the ORCID secret in `crates/biomcp-fetch/src/declaration.rs` and config key handling, the author query code, the source registry, cases and author.md.
