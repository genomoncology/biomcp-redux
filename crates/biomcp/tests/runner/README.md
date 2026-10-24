# Case assertions

The case target discovers YAML files as named tests. `--cases DIR` selects an
external case directory. Cargo filters and `--list` retain those relative names.
Each case shares one isolated environment across its steps.

`expect` maps JSON pointers to assertions. Scalars and arrays mean equality.
Objects contain operators combined with AND; use `equals` for a literal object.
The runner validates every step and assertion before launching any command.
Unknown operators, invalid shapes and malformed pointers fail the selected case.

Implemented operators:

| Operator | Expected value | Meaning |
| --- | --- | --- |
| `equals` | Any JSON value | Exact equality, including objects |
| `includes` | Any JSON value | An array contains an equal member |
| `count` | Nonnegative integer | Exact array length |
| `rank_of` | Nonempty object of string members to positive integers | First one-based position of each exact string member |
| `max_bytes` | Nonnegative integer | Ceiling on compact JSON bytes |
| `max_tokens` | Nonnegative integer | Ceiling on compact JSON bytes divided by four, rounded up |

Each step also accepts `max_bytes` for raw stdout and `max_tokens` for compact
JSON. Arrays are ordered. Membership ignores multiplicity. Missing paths fail;
explicit null is a value. Ordinary empty arrays can equal `[]`, have count zero,
and fail membership normally. A missing rank member fails.

A `*` pointer segment expands an array in order. Further segments must exist with
the correct container on every selected branch. Multiple expansions flatten to
one result array. Zero-result wildcard selections fail before assertions run.
Wildcards apply only to arrays. Other segments follow JSON Pointer escaping.

The schema cases demonstrate the implemented syntax against real command output.

## Approved syntax for future cases

The following operators and ranges remain unimplemented until a harvested case
requires them. An assertion using one currently fails as an unknown operator or
invalid exact value.

| Operator | Approved meaning |
| --- | --- |
| `subset` | Recursively match the expected object's keys; compare non-object leaves with exact equality |
| `includes_all_of` | Load a JSON array from the named floor file and require every value as an equal array member; ignore multiplicity |
| `before: [A, B]` | Require distinct present values with the first position of A before the first position of B |
| `unique: true` | Compare whole array member values for uniqueness |
| `regex` | Search with the supplied pattern; authors supply anchors when needed |

Future `count` and `rank_of` bounds accept an exact unsigned integer, an anchored
comparison string using `<`, `<=`, `>` or `>=`, or an inclusive `{min, max}` object
with at least one bound. Unknown bounds and reversed bounds fail validation.
Ranks remain one-based and absent members still fail.

A floor path must be nonempty, relative to the case's parent and nonabsolute.
Normal parent traversal supports external case directories. The loaded JSON must
be an array. An empty floor is allowed but cannot override wildcard-empty
failure. Future code constants must bound floor reads, regex source size and
compiled regex size. Every operator, pattern and floor must validate before any
command launches.

The release artifact gate sets `BIOMCP_TEST_EXECUTABLE` to an absolute installed binary. The shared launcher uses it for CLI, MCP and process proofs, including nested runner checks, and never falls back after a failed installed launch. `BIOMCP_TEST_FIXTURES` selects copied public fixtures outside the checkout. `--installed-process-proof` runs the existing ordinary-cache publication proof without discovering cases from the source checkout.
