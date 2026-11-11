# Stop the token race with curl; focus on the personas

Date: 2026-11-11. Decided by Ian.

## What Ian said

After the bench round on fixed tooling and the root-cause record (`sdlc/records/2026-11-11-token-gap-root-cause.md`) showed a strong model with curl matches BioMCP on lookups it knows by heart:

> "Okay, we should stop fighting that battle. That's fine. Where do we stand on the personas, then? We should move off of this problem. Let's focus now on the personas."

## What changes

- Milestone 7's done-when, "each BioMCP mode uses at least 20% fewer total tokens than plain curl … on both models, across repeats", no longer gates 1.0. Milestone 7's tickets stay where they cut cost or failed calls for chat clients such as Amazon Quick, which have no curl.
- The measure Ian asked for is the persona suites rerun on current main and graded by a different model (bench ticket 0037), which is milestone 8's done-when.
- Persona-facing fixes go first in the queue.
