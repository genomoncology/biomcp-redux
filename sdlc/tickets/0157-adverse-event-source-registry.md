# 0157 — Register the adverse event sources and components with use labels

Status: OPEN.

Milestone: 3

Depends on: 0156

## Outcome

The one source registry declares `faers` and `maude` once each, labeled `open` under the openFDA terms and license, and the components `meddra` and `gmdn`, each labeled `restricted` in the `restricted` group. `meddra` covers the source-reported reaction label `patient.reaction.reactionmeddrapt` and its version `patient.reaction.reactionmeddraversionpt`, with the MedDRA data sharing statement as its terms. `gmdn` covers `device.generic_name` and every other device field this ticket names by exact path, with openFDA's license page as its terms. Every source and component starts enabled. `--disable-sources`, the configured disable list and the `restricted` group skip a source or component before any request, cache read or replay, and source outcomes and health carry the use label and terms on empty, skipped and failed outcomes too.

- Why: the open-by-default decision gives every source and component a use label and group in one registry, and the 0152 review asked for openFDA's third-party marks on MedDRA reaction terms and GMDN device names.
- Starts from: `crates/biomcp-adapters/src/registry.rs`; decision `2026-10-24-sources-are-open-by-default.md`; adverse-event.md's source and component terms, with 0152 recheck amendment 4; 0125 and 0139 as the pattern; 0156's types.
- Rules: openFDA sends no field projection, so a disabled component never means fetch and discard. Disabling `meddra` skips the `reactions` section and any `--reaction` search before I/O; the FAERS search, card and `drugs` section stay available because they carry no MedDRA label. Disabling `gmdn` skips MAUDE positional search and the `devices` section before I/O; the MAUDE card and `narrative` section stay available. Disabling a source skips all of its commands. The ticket lists every device field path it places under `gmdn`. VAERS, CDC WONDER and CVX get no runtime declaration. No dictionary is downloaded, no GMDN code is added and no second terms table exists.
- Proof: cases first. A case per declaration asserts its label, group and terms. Cases for `--disable-sources faers`, `maude`, `meddra` and `gmdn`, the `restricted` group and the configured list assert the skip with zero requests and no cache or replay read, and that the allowed commands above stay available. Stub-backed, with no provider call. This ticket adds no filter or exact-ID get; each filter and exact-ID get that later adverse event tickets add has a known-positive capture through one bounded keyless pm job. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
