# 0963 — record-folder-refuses-symlinks

Status: COMPLETE.
Landed: 6ad5efc7ba5b6b9bb9764b501835b88ff7104886

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

## Progress

Build, 2026-11-15.

- Test first (job j41198 on the M5): `symlink_inside_record_folder_is_refused_for_recording_and_replay` in `crates/biomcp-fetch/tests/record.rs` (body in `record/bytes.rs`) puts a symlink inside the record folder pointing outside it. Binary replies were refused, but a JSON reply was recorded straight through the symlinked source folder into the outside folder (`Ok(())` where a refusal was expected). Only binary reply files checked for symlinks; the recording file itself did not.
- Fix: one shared check in `crates/biomcp-fetch/src/record_sidecar.rs` refuses, with `recording-malformed`, a symlink at the recording or binary reply file or at any folder from the record folder down, including the record folder itself swapped for a symlink mid-capture. JSON recordings, binary reply files and RPC captures all use it, for recording and replay. The record folder's own symlinked parents still resolve once (0954), and folders above it are not checked.
- The test now shows: recording a JSON and a binary reply through a symlinked source folder fails and writes nothing outside; replaying through a symlinked source folder, or through symlinked recording and reply files, fails for both reply kinds without reaching the network; the same recording replays from its real folder. `binary_capture_follows_a_symlinked_parent_folder` and case `pathway/sync-local-failure` still pass (j41220).
