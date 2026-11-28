# 1244 — restore-typed-catalog-headroom

Status: OPEN.

Milestone: 10

## Outcome

The typed eager catalog (all 18 tools, `TYPED_EAGER_CATALOG definition_bytes`) is at least 8 KB (8,192 bytes) smaller than on main when this ticket starts, with no tool, action, argument or allowed value lost and no meaning lost from any description. The ceiling, `DEFAULT_TYPED_OUTPUT_DEFINITION_BYTES` (192 * 1024 = 196,608 bytes) in `crates/biomcp/src/config/mcp.rs`, is unchanged.

## Why

PM ruling 2026-11-28: the catalog is within about 100 bytes of its ceiling, and several source reach fixes each add about 2.8 KB to it (1233 measured 196,546 and 1236 measured 196,507 on their own branches, both from about 193,700), so fixes keep colliding on size. This runs ahead of the remaining reach tickets and lands before 1233, 1236 and 1218, which then rebase onto it.

## Evidence

- `crates/biomcp/tests/runner/typed_catalog.rs` prints `TYPED_EAGER_CATALOG tools=18 definition_bytes=N`; branch measurements in this milestone ran 193,688 to 196,546.

## Proof

- Proof: the before and after `definition_bytes` on the same base, both reported; the existing schema, help and typed catalog cases pass with their expectations unchanged except where they pin the removed wording.

## Starts from

- Starts from: the tool and argument descriptions and examples that build the typed catalog.

## Keeps

- Keeps: the ceiling value; every tool, action, argument, enum value and required field; what `help` serves.

## Changes

- Changes: remove phrasing repeated across tool and argument descriptions, shorten examples, and drop catalog text the help tool already serves, pointing to help where needed.
