# 0963 — record-folder-refuses-symlinks

Status: OPEN.

Milestone: 7

## Outcome

A test proves that a symlink inside a `--record` folder is still refused after 0954 made the folder's own path resolve through symlinked parents.

## Why

0954 (2026-11-14) resolves the record folder's path once so `/var` on macOS works, and says a symlink inside the folder stays refused, but only a renamed `../outside.bin` path is tested, not a symlink, so a later change could open the folder to writes outside it unnoticed.

## Proof

- Proof: a test in `crates/biomcp-fetch/tests/record*.rs` that places a symlink inside the record folder pointing outside it and shows both recording and replay refuse it.

## Starts from

- Starts from: 0954's change and its test `binary_capture_follows_a_symlinked_parent_folder`; case `pathway/sync-local-failure`.

## Keeps

- Keeps: 0954's behavior for symlinked parents.

## Defers

- Defers: nothing.

## Changes

- Changes: one test, and the record code only if the test fails.
