# 1175 — disable-sources-component-error

Status: COMPLETE.
Landed: 8acf294a51c9a82524c8b32a4b061cb924ff5770

Milestone: 7

## Outcome

`--disable-sources` given a component name such as `cosmic` (listed by `biomcp help sources` as `myvariant:cosmic`) returns a usage error that names the bad value and the accepted forms, or accepts the component, whichever the source registry supports.

## Why

On 1.0.0-87f56ceabae5, `biomcp get gene BRAF --disable-sources cosmic` returned only "expected a supported command with checked arguments", with no option, value or valid values. Found by the docs writers on 2026-11-24.

## Proof

- Proof: CLI and typed MCP cases for `--disable-sources cosmic` and `myvariant:cosmic` show either acceptance or an error naming the option, the value and the accepted names.

## Changes

- Changes: source-exclusion argument parsing and its cases.
