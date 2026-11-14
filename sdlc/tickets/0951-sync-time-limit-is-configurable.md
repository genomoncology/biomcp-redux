# 0951 — sync-time-limit-is-configurable

Status: OPEN.

Milestone: 7

## Outcome

A user can set the bulk sync time limit (today a fixed 120 seconds) through BioMCP's configuration, with 120 s kept as the default, and GTR's after-sync read uses the same setting instead of its own fixed 120 s. A sync that runs out of time says which limit it hit and how to raise it.

## Why

0931's review (2026-11-14): `FetchConfig::sync_budget` (`crates/biomcp-fetch/src/lib.rs:172`, from 87282f9ad) is a config field with a default but nothing in `crates/biomcp/src` lets a user set it, and `crates/biomcp-adapters/src/gtr/bundle.rs:54` has its own fixed `Duration::from_secs(120)` (6d0dec4fa). 0931 grows the GTR sync from about 57 MB to 245 MB; it took 43.7 s on the M5, but below about 2.5 MB/s it will time out where it used to succeed. Workspace rule: make configuration values configurable and choose a default.

## Proof

- Proof: a case or test sets a short limit and gets the named timeout message; the default stays 120 s; GTR's after-sync read follows the setting.

## Starts from

- Starts from: the two lines above; how other FetchConfig values reach users (environment or config file).

## Keeps

- Keeps: the 120 s default; other limits.

## Defers

- Defers: nothing.

## Changes

- Changes: config plumbing, the GTR read, help/docs, a test.
