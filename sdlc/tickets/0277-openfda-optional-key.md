# 0277 — Send the openFDA optional key

Status: OPEN.

Milestone: 4

Depends on: 0273

## Outcome

Every openFDA adapter, including drug labels, Drugs@FDA, FAERS, MAUDE, PMA, 510(k) and companion diagnostics, sends `OPENFDA_API_KEY` or the config file's `openfda_api_key` when it is set and runs at the public rate when it is not, through the deferred key loader. ROADMAP.md's openFDA key box ticks in the landing commit.

- Why: ROADMAP.md, the legacy key names decision and diagnostic.md promise the openFDA optional key, while `crates/biomcp-fetch/src/secret.rs` has no openFDA secret and every FDA adapter declares `KeyUse::None`, so a 0.9 user's key is silently ignored.
- Starts from: the PubMed and ClinVar optional key pattern, `secret.rs`, the FDA adapters' `SourceSpec` declarations, `2026-10-25-legacy-key-names.md` and the 0273 deferred loader.
- Rules: the key is sent only to openFDA hosts in the place openFDA documents, never printed, recorded or cached in a key, and the quota uses openFDA's keyed and keyless limits.
- Proof: per adapter family, a keyless replay case and a keyed replay case with a synthetic key assert the request carries the key only when set and that recordings and output never contain it. Existing FDA cases pass unchanged. No source calls.
- Changes: `secret.rs`, the FDA adapter declarations, config key names, help for `key set`, the affected cases and ROADMAP.md.
- Defers: nothing.
