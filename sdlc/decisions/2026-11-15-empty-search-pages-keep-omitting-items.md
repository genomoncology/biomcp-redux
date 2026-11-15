# Empty search pages keep omitting items

Decided 2026-11-15 by Ian, in chat with the PM.

## Decision

- A search page with no matches leaves out its `items` list, like every other empty list, under the 2026-10-23 rule (`2026-10-23-json-is-the-default-output.md`). Its `total` and `returned` say 0.
- The docs and the skill say so, so an agent reads `total` before reading `items`.
- No exception for page lists. Ian: "Yeah, that's fine." in answer to the PM's recommendation to keep the rule.

## Why

- dx-lab-director round 2 (call 27): an agent's script read `.items` on an empty page and failed. 0948 first wrote empty `items` on every page; its gate (j41088) failed 110 tool proofs that check replies never carry an empty list, so the exception would spread past the page types and leave a special case for later work to remember.
- `total` already answers whether anything matched.

## What it changes

- 0948 documents the omission instead of writing empty lists.
