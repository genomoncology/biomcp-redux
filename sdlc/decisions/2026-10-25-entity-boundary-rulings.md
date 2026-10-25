# Entity boundary rulings

Decided 2026-10-25 by Ian, under the entity-driven framing in `2026-10-23-the-user-is-an-agent.md`.

- Studies: 1.0 gets each cBioPortal study and the counts cBioPortal itself reports. 0.9's local study calculations (top mutated genes, filters, co-occurrence, cohort splits, survival and comparisons over downloaded study files) are dropped, with retirement messages.
- Gene-set enrichment: 0.9's `enrich` command and g:Profiler are dropped. An agent can call g:Profiler itself.
- AlphaGenome: kept as a variant section that returns the API's numeric predictions for one variant, only with the user's own key and under the publisher's terms. No plots or images.
