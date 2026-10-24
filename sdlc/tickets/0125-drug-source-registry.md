# 0125 — Register the drug sources and components with use labels

Status: OPEN.

Milestone: 3

Depends on: 0124

## Outcome

The one source registry declares each kept drug source and each exposed MyChem component once, with the use label, disable group and terms name and URL from `planning/slices/drug.md` section 2's registry table: `mychem` and its DrugBank Open identity, ChEMBL, DrugCentral, GtoPdb, UNII, NDC and openFDA cross-reference components; `chembl`; `opentargets`; `civic`; `openfda-label`; `drugsfda`; `ema`; and `ddinter`, labeled `noncommercial` in the `noncommercial` group. OncoKB keeps its existing `restricted` declaration. Every source starts enabled. `--disable-sources`, the configured disable list and the `noncommercial` and `restricted` groups skip a source before any request, cache read or replay, and source outcomes carry the use label and terms on empty, skipped and failed outcomes too.

- Why: the open-by-default decision gives every source a use label and group in one registry, and 0116's PM review asked for each drug source's label before build.
- Starts from: `crates/biomcp-adapters/src/registry.rs`; decision `2026-10-24-sources-are-open-by-default.md`; drug.md's registry table and component policy.
- Rules: WHO, FDA orphan, DGIdb, PharmacoDB and CVX get no runtime declaration. Disabling one MyChem component keeps unrelated permitted fields, and disabling `mychem` skips the whole service. Unknown component rights never read as open. No second terms table.
- Proof (P01): cases first. A case per declaration asserts its label, group and terms; a `--disable-sources ddinter` case, a `noncommercial` group case and a configured-list case assert the skip with zero requests and no cache or replay read; a MyChem component case asserts the disabled component's fields are gone and the others stay; an OncoKB `restricted` case asserts the existing skip. Stub-backed, with no provider call. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. `identity_level` comes from the endpoint or record type, never from a field's shape. Then make landing-gate runs through pm job.
