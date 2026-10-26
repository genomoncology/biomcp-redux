# 0250 — Give every research result a canonical URL and title from its source identity

Status: OPEN.

Milestone: 4

Depends on: none

## Outcome

Research search and fetch can cite pathway, study, GWAS, adverse-event, NCI trial and WHO rows even though their headers carry no URL field. Architecture's research section requires each result to carry a canonical publisher URL and an exact get argument list, and omits uncitable rows with a count. The 0249 audit found that PathwayHeader, StudyHeader, StudyCard/1, AdverseEventHeader, WHO DrugWhoRow, NciTrial and the GWAS association lack a URL, and AdverseEventHeader and the GWAS association lack a scalar title. Without a fix, those rows would all be omitted. PM ruling 2026-10-26:

- Each source declaration in the source registry gains at most one canonical record URL template per native ID kind, such as a Reactome stable ID, a GCST accession, a GWAS association ID, an NCI trial ID or a cBioPortal study ID. The template is verified against the publisher's own record page before it is added. The registry is its only owner.
- The research projector uses a shape's native URL field first, and the registry template second. When a shape has no native title, the title is the source's display name followed by the native ID, such as "GWAS Catalog association 6110". No title is composed from free text.
- A source with no stable public record page, which may include WHO prequalification rows, gets no template, and its rows stay omitted with the count, as architecture says.
- CPIC pairs have no get target, and every research hit requires exact get arguments as its ID, so CPIC pair rows stay omitted and counted. No citation-only hit contract is added.

- Starts from: the source registry in `crates/biomcp-adapters/src/registry.rs`, the research projector in `crates/biomcp/src/mcp/research.rs`, and the 0249 audit.
- Changes: the registry and its structural check, the projector, and research cases. No type or schema edits.
- Proof: one bounded keyless pm job fetches one known record page per template and records status and final URL, with no retries. Research search and fetch cases show a URL and title for a Reactome pathway, a cBioPortal study, a GWAS association and study, a FAERS report and an NCI trial. A case shows a source without a template omitted and counted. The 0249 registry check then passes with omissions only for shapes whose source has no template. Then make landing-gate runs through pm job.
