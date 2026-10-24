# 0067 — Use one supported wire-enum recipe

Status: OPEN.

Milestone: 1

Depends on: 0057

## Outcome

New checked string enums use the existing supported schema helper, and the architecture teaches that helper. Schema generation stops rediscovering unsupported enum forms and keywords.

- Why: a read-only root cause analysis found four failed jobs from three mistakes: two independent plain enum derivations emitted unsupported oneOf branches; one shared uniqueItems addition failed in two lanes. No product behavior defect was found.
- Starts from: architecture 3.2 and 3.9; existing wire_enum helper; 0041 j9667, 0057 j9733, 0042 j9742 and 0041 j9747. Legitimate schema version increases are outside this class.
- Keeps: public wire values, checked deserialization, runtime source uniqueness, strict compatibility checks and current behavior.
- Changes: `crates/biodata/src/macros.rs`, `crates/biodata/src/response/status.rs`, `crates/biodata/src/response/mod.rs`, `crates/biodata/src/response/source.rs`, `crates/biodata/src/response/step.rs`, `crates/biodata/src/source.rs`, `sdlc/planning/architecture.md`. The stacked article-filter ticket consumes the shared helper for its new type.
- Proof: move the existing helper to the existing crate-wide macro module without changing its implementation; use it for ArticleFilter rather than repeating schema values; replace the misleading enum example and link the helper. State the compatibility subset beside the generation recipe, retaining runtime uniqueness. Existing schema generation, round trips and outside-in cases pass with unchanged wire values. Add no framework or gate.
- Count: baseline four failed jobs from three mistakes. The PM checks one week after landing for zero new failures from unsupported unit-enum schema forms or unsupported schema keywords, and reopens the analysis if either recurs.
- Defers: broader JSON Schema support and any product interface change.
