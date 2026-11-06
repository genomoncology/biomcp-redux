# cBioPortal study terms

cBioPortal remains available in all hosted modes with its `attribution` source label. In `restricted`, BioMCP leaves out studies whose reviewed study information explicitly specifies noncommercial terms. This applies to study search, saved search continuations, every study section, and variant recurrence pages. Replies name the omitted studies and the reason. `commercial`, `academic`, local CLI and stdio retain their existing study access. A caller key does not override a study flag in `restricted`.

The flags apply when reading results. Existing catalogs, cached source replies and stored search pools remain readable without synchronizing, changing the store format or changing cache identities. Filtering precedes paging, so study totals and continuation offsets describe the permitted rows. Recurrence for a flagged study returns no observations or counts.

## Reviewed public catalog

Read on 2026-11-06: [public detailed study list](https://www.cbioportal.org/api/studies?projection=DETAILED), 551 studies. The returned JSON had SHA-256 `1345b97d3f636b8ed86339abd55e8b63f5c2c9f38bdf4323b22400f3d1ee91f2`. The restriction wording lives in each study's `description` field, not its access groups or public-read flag.

The [checked-in flag list](../crates/biomcp-adapters/src/cbioportal/commercial-restrictions.tsv) records each flagged study ID, exclusion reason and exact description. Nine descriptions explicitly specify the Creative Commons BY-NC-ND 4.0 license. Eight link to [the license](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.en); the ninth names it in plain text. This list is the shared source for all study filtering.

When the public study list changes, review the descriptions and refresh the flag list in the same change. Copy the exact study ID and description for explicit noncommercial wording. Check new, changed and removed studies. Keep studies without explicit noncommercial wording enabled. Send ambiguous wording to the product owner for a decision; do not infer a commercial restriction from research-only language, access groups, public availability or a citation.

The reviewed list also contains TARGET research-purpose limits and research-only clinical-use cautions. These have no explicit noncommercial wording and remain enabled; their exact wording was referred to the product owner for a decision. This review does not interpret those terms or expand paper and article-text restrictions.
