# 1135 — bump-url-for-rustsec-2026-0592

Status: OPEN.

Milestone: 7

## Outcome

The landing gate's dependency audit passes on main again: the `url` crate moves from 2.5.8 to 2.5.9, the release the audit names as the fix for RUSTSEC-2026-0592, and nothing else in `Cargo.lock` changes beyond what that update needs.

## Why

Ticket 1130's landing gate (pm job j45366, 2026-11-22) stopped at the audit, which flagged `url` 2.5.8 on main. Every landing is blocked until main passes.

## Proof

- `make landing-gate` passes on the branch through `pm job`.
- The diff to `Cargo.lock` is listed in the report, package by package.

## Starts from

- Starts from: main.

## Keeps

- Keeps: no advisory ignore or audit exception is added.

## Defers

- Defers: nothing.

## Changes

- Changes: `Cargo.lock`, and `Cargo.toml` only if a version floor needs raising.
