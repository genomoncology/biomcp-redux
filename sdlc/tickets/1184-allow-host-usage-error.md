# 1184 — allow-host-usage-error

Status: COMPLETE.
Landed: 8983e311d40e553e5217922975030693a203f822

Milestone: 7

## Outcome

A malformed `--allow-host` value such as `'bad host'` prints a usage error naming the option, the value and the expected form, and exits with the usage exit code.

## Why

Found by the 1179 developer on 2026-11-25: `--allow-host 'bad host'` exits 2 with no output on main.

## Proof

- Proof: A CLI case for a malformed `--allow-host` checks stderr and the exit code; it fails on main.

## Starts from

- Starts from: `--allow-host` parsing in the `serve` command.

## Keeps

- Keeps: Valid `--allow-host` values behave as today.

## Defers

- Defers: Nothing.

## Changes

- Changes: serve option parsing and its case.
