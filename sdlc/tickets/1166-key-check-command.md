# 1166 — key-check-command

Status: COMPLETE.
Landed: d7c5bf41003d1de110e14fad10de03cf3cf633f5

Milestone: 7

## Outcome

BioMCP can check whether a source key works: `biomcp key check SOURCE` on the CLI, and the same check for the hosting wrapper over its internal route with the per-request credential, returns working, rejected (with the publisher's reason), or could not check (with why), for each keyed source: NCBI, NCI, openFDA, AlphaGenome and Semantic Scholar. ClinicalTrials.gov reports that it needs no key.

## Why

Ian, 2026-11-23: storing API tokens on the site "and checking they work". Hosting ticket 0118 built the account screen's "Check key" button, but the pinned core offers no provider check, so the button can only say the check is unavailable.

## Proof

- Proof: replay cases for each keyed source with a working key, a rejected key and an unreachable publisher return the three outcomes; each check makes one cheap authenticated request and never prints the key; the hosted route accepts only the per-request credential the wrapper passes, never a key from the MCP client.

## Changes

- Changes: a key-check command over the existing key registry and each adapter's cheapest authenticated endpoint, the hosted internal route, and replay cases.
