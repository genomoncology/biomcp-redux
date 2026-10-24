# biodata

Typed values for BioMCP. This crate has no I/O and no commands.

`Text` trims surrounding whitespace and rejects empty text. JSON deserialization uses the same checks as construction. `Error` returns a stable code and retains no rejected input. BioMCP owns queries, source identities, problems and response disclosure in `biomcp-types`.

`HgncId` checks positive namespace-local numbers and parses canonical HGNC text. `PartialDate` preserves year, month or day precision and checks the Gregorian calendar. Both serialize as strings through `Display` and deserialize through `FromStr`. `Position1` serializes as a positive integer. `Span0` serializes as a closed object with `start` and `end`; its constructor checks half-open bounds and permits an empty span.

The optional `schema` feature describes canonical serialized values. Constructors and JSON deserialization also check calendar validity, identifier numeric overflow and the relationship between span bounds. Those checks exceed the generated schemas. Schema drift and compatibility tests keep the checked-in contracts aligned with the public values.
