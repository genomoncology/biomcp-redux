# 0273 — Read every source key only when its source is called

Status: OPEN.

Milestone: 4

Depends on: 0266

## Outcome

Every source key, such as `NCBI_API_KEY`, `NCI_API_KEY`, `ONCOKB_TOKEN`, `UMLS_API_KEY`, `OPENFDA_API_KEY` and `ORCID_ACCESS_TOKEN`, is read from the config file or its 0.9 environment variable only when its source is called, through the deferred loader that 0266 adds for `S2_API_KEY`. A command that never calls a keyed source reads no key.

- Why: Ian asked that a key be accessed only when its API is called, and `secrets_from` in the config code reads every key for every command today.
- Starts from: the 0266 per-Fetch deferred loader, `secrets_from` and the settings code in `config.rs`, architecture.md's settings section, and `2026-10-26-new-sources-follow-open-by-default.md`.
- Keeps: config-first precedence, the 0.9 variable names, keyed accounting, cached-reply secret-reflection containment, and the 0266 definition of called, which includes raw-cache and replay validation for that source. Disabled sources and commands served from query pools never load a key. No global state and no new dependency.
- Changes: the config and settings code, the transport credential lookup, and the affected credential containment cases.
- Proof: one case per keyed source shows a command for an unrelated source reads none of the keys, observed through an environment that fails the run if a key variable is read, and a command for that source reads only its own key. The existing credential containment cases pass unchanged. No source calls.
- Defers: nothing.
