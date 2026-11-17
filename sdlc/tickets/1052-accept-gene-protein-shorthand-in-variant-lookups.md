# 1052 — accept-gene-protein-shorthand-in-variant-lookups

Status: COMPLETE.
Landed: 8b2921a4e912d1d8c196d22c176774ec5b26398d

Milestone: 7

## Outcome

A caller can search for a protein substitution in the usual gene-and-change notation, such as `BRAF V600E`, without knowing an rsID or transcript. The CLI accepts quoted and ordinary unquoted search input; the MCP search accepts the same literal. An exact get given this notation should take the caller to the supported lookup and its actual candidates instead of ending at a generic HGVS error.

## Why

Ian reported on November 17 that looking up `BRAF V600E` asks for valid HGVS or an rsID. Explicit gene and protein flags already perform the underlying candidate search.

## Proof

- Proof: Replay `search variant "BRAF V600E"`, `search variant BRAF V600E`, the matching MCP search and the reported get form. Compare candidates against `search variant --gene BRAF --protein V600E`. Retain all candidates where one protein substitution can have multiple alleles; preserve no-match and failure semantics and protect existing identifier and explicit raw-query lookups.

## Starts from

- Starts from: The checked protein grammar and MyVariant candidate search delivered in ticket 0088; verify the actual UI or client route used for a one-string query.

## Keeps

- Keeps: Existing exact identifiers, build and source rules, ancillary sections and explicit MyVariant query behavior.

## Defers

- Defers: Other shorthand classes follow evidence from a real lookup.

## Changes

- Changes: Variant query and CLI/MCP entry points, their help and outside-in cases.
