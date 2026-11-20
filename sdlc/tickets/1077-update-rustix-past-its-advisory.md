# 1077 — update-rustix-past-its-advisory

Status: COMPLETE.
Landed: 2c8f0b46a10882c5244853d39513ff26c2626182

Milestone: 7

## Outcome

The lockfile holds `rustix` 1.1.6 or later, and the landing gate's dependency audit passes again.

## Why

On 2026-11-20 the gate's audit began failing on every branch: advisory RUSTSEC-2026-0486 marks `rustix` 1.1.5 unsound (its extended-attribute functions mark unwritten bytes as initialized when given a zero-length buffer). Main's lockfile has 1.1.5 through `tempfile`. First seen in job j43358. Until this lands, nothing can land.

## Proof

- Proof: `make landing-gate` passes on the branch, where its audit step fails on the base commit with RUSTSEC-2026-0486.

## Starts from

- Starts from: the advisory's own fix, `cargo update -p rustix`. Change the lockfile and nothing else.
- PM decision, 2026-11-20: BioMCP does not call the affected functions, so this is not an exposure; it is fixed by updating, not by adding an ignore entry.

## Keeps

- Keeps: no other dependency moves unless the update forces it; say which if so.

## Defers

- Defers: nothing.

## Changes

- Changes: `Cargo.lock`.
