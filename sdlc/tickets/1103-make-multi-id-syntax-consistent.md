# 1103 — make-multi-id-syntax-consistent

Status: COMPLETE.
Landed: 28fe2a547a36d333dd8a6358f8764071a82b3940

Milestone: 7

## Outcome

Batch-capable get commands accept space-separated IDs and comma-separated lists of validated IDs. A malformed list gets a copyable correction wherever the IDs can be recovered.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get trial NCT04657016,NCT04660643 flow --fields=record,data.title,data.milestones` returned `value not accepted: give error.option a value valid_values accepts`. The same two trial IDs work as separate arguments. This is a syntax gap as well as a recovery-message gap.

## Proof

- Proof: Use the two reported NCT IDs with both forms and a second batch-capable entity. Check order and projection, plus a malformed list that teaches the working form.

## Starts from

- Starts from: The trial batch input checks and existing multiple-ID parsers. Keep quoted single identifiers that can contain a comma unambiguous.

## Keeps

- Keeps: Space-separated get IDs remain valid.

## Defers

- Defers: Adding batching to commands that only accept one record.

## Changes

- Changes: Shared get input parsing, help and cases.
