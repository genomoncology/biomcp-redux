# 1172 — typed-tools-strict-clients

Status: COMPLETE.
Landed: 39b2b4b6d75ea31260f5797412e3047b8ec4c948

Milestone: 7

## Outcome

Typed MCP tools work for clients that must send every field (OpenAI strict function calling, and any client that fills a schema completely). In the active action's object, an empty string or null means the option was not given. Every optional field's schema lets such a client say "not given" (null, or an explicit any choice) instead of forcing a real value, so a filled-in enum cannot silently narrow a search. The tool definitions stay within their existing size limits.

## Why

Round 2 rerun on 1.0.0-edb9f81b1eb9 (eval repository, batch r2-fx2-o; Sol through pi with the openai provider): after ticket 1171, 49 typed calls were still refused with "option given without a value" for empty strings, e.g. trial search with `acronym: ""`, drug get with `civic_cursor: ""`, diagnostic search with `bundle_revision: ""`. The same calls carried first-choice enum values the agent did not ask for (trial `phase: "NA"`, `sex: "ALL"`, `sponsor_type: "other"`; drug `evidence_level: "A"`), which would filter results if accepted. The chatbot still passed 21 of 24 with 227 failed calls.

## Proof

- Proof: replays of the logged calls: the trial, drug and diagnostic calls run with their real arguments, and a filled-in enum that the agent can mark as not given does not filter; a schema check shows every optional field accepts null or an any choice; a strict-mode schema validation (all properties required) of each typed tool passes; existing limits stay as they are.

## Changes

- Changes: typed argument parsing, the typed tool schemas, and replay cases.
