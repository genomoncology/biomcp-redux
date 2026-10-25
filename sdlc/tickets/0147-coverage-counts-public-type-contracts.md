# 0147 — Count reviewed public type contracts as outside-in callers in coverage

Status: COMPLETE.
Landed: b1831c57cc8e81f6e8a00e03f273cea49e44708f

Milestone: 3

## Outcome

The coverage gate counts two kinds of outside-in caller toward the unchanged 90.41 percent floor and 0.20 tolerance: command cases, as today, and tests of reviewed public type contracts. A type contract test deserializes literal inputs through an exported type and asserts its published validation and wire facts, with no hooks. A checked list names each counted contract target and the feature that owns it. The gate also prints the command-only figure for comparison with history.

- Why: type tickets such as 0124 and 0138 land types before their commands, so their required validators have only contract callers, and three coverage-only failures followed. The repo counted only command cases; the shared outside-in practice also accepts a public API boundary as a caller. Moving every type into its first source ticket would serialize parallel work.
- Changes 0063's measurement policy. Library unit tests never count; only targets on the checked list do, and adding one needs review like any other contract change.
- Proof: the gate passes on current main with both figures printed; a library test outside the list does not raise the counted figure; a contract target missing from the list does not count. Then make landing-gate runs through pm job.
