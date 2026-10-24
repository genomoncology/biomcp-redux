# biodata

Typed values for BioMCP. This crate has no I/O and no commands.

`Source`, `Reason` and `ProblemCode` use canonical names for parsing, display and JSON. `Text` trims surrounding whitespace and rejects empty text. JSON deserialization uses the same checks as construction. `Error` returns a stable code and retains no rejected input.

`Problem` describes a field that could not be accepted from a source record. Its optional `record` uses checked text and is omitted from JSON when absent. Unknown JSON fields are rejected.

```rust
use biodata::{Problem, ProblemCode, Source, Text};

let problem = Problem {
    source: "pubmed".parse::<Source>()?,
    record: Some(Text::new(" record-123 ")?),
    field: "publication_type".into(),
    code: ProblemCode::ValueUnknown,
};
assert_eq!(problem.record.as_ref().map(Text::as_str), Some("record-123"));
# Ok::<(), biodata::Error>(())
```
