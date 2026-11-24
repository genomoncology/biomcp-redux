# 1171 — typed-tools-ignore-other-actions

Status: COMPLETE.
Landed: 6a2e09de084560405e863e83bdb1f7999af0b42b

Milestone: 7

## Outcome

A typed MCP call is read from the object its `action` names, and every other action's object is ignored, whatever it holds. The reply may note which objects it ignored. The CLI-style mistakes agents still make get a suggestion that runs: `get drug X --section label` suggests the positional section (`get drug X label`), `trial document NCT... --file F` suggests the positional file, and `help sources ema` suggests the call that lists or describes that source.

## Why

Round 2 rerun on 1.0.0-996652e45219 (eval repository, batch r2-fix-o, Sol through the chatbot over MCP): ticket 1168 accepted other actions' objects only when empty or default, but this client fills every action's object with schema minimums, e.g. article `{"action":"search","search":{...real...},"text":{"part":1},"citations":{"limit":1,"source":"icite"},"grant_search":{"fiscal_year":2026,"limit":1}}`. BioMCP refused it ("Put arguments in the action's object named in valid_values"): 64 refusals across article, drug and diagnostic in 24 runs, more than before. The chatbot's failed calls rose from 204 to 251 and it passed 20 of 24. Also seen: `--section` on get drug (20), `help sources ema` (11), `trial document --file` (6).

## Proof

- Proof: replays of the logged calls: the article search above returns results; the drug and diagnostic calls run their named action; the three CLI-style mistakes return a suggestion that, followed, succeeds. Existing limits stay as they are.

## Changes

- Changes: typed argument parsing, the usage suggestions, and replay cases.
