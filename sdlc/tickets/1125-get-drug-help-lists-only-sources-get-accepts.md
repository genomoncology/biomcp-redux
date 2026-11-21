# 1125 — get-drug-help-lists-only-sources-get-accepts

Status: COMPLETE.
Landed: 008348933e34f1f34eee4a25b849cba7d1db1d17

Milestone: 7

## Outcome

`biomcp help get drug`, the generated command reference and the MCP drug tool list only sources that `get drug` accepts. Today the `get drug` line in `docs/reference/commands.md` names `purple-book`, which only `search drug` serves, and `get drug NAME --source purple-book` is refused. Every source listed for a command works with it, or the listing says which sections or forms it serves.

## Why

Agents read the source list to choose a call; a listed source that is refused costs a turn and a recovery. Found by the developer of ticket 1121 on main 8a69b280f (2026-11-21).

## Proof

- Proof: a check that, for every command, each source in its help line is accepted by that command (at least past validation, offline), and that fails on the base commit for `get drug` and `purple-book`.

## Starts from

- Starts from: where `help get drug` and the reference build their "Sources:" line (shared per-entity source lists or the coverage map used by the front map), and the `get drug` source enum. Check whether any other entity has the same gap.

## Keeps

- Keeps: `search drug --source purple-book` and its help unchanged.
- Keeps: the first-read ceilings; the fix should shrink, not grow, what is read first.

## Defers

- Defers: nothing.

## Changes

- Changes: the help and reference source lists, and one new check.
