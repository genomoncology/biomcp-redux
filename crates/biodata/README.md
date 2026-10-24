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

`HgncId` checks positive namespace-local numbers and parses canonical HGNC text. `PartialDate` preserves year, month or day precision and checks the Gregorian calendar. Both serialize as strings through `Display` and deserialize through `FromStr`. `Position1` serializes as a positive integer. `Span0` serializes as a closed object with `start` and `end`; its constructor checks half-open bounds and permits an empty span.

The optional `schema` feature describes canonical serialized values. Constructors and JSON deserialization also check calendar validity, identifier numeric overflow and the relationship between span bounds. Those checks exceed the generated schemas. Schema drift and compatibility tests keep the checked-in contracts aligned with the public values.
