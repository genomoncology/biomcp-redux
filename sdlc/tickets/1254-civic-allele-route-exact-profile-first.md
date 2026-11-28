# 1254 — civic-allele-route-exact-profile-first

Status: OPEN.

Milestone: 10

## Outcome

On the CIViC allele route, the reply shows the exact single-variant profile first, or lists every profile that holds the allele, so a model asked about one variant reads that variant's own evidence and not a combined profile's.

## Why

Found in a run of all 100 items of the public CIViC MCP benchmark (Schimmelpfennig et al., Bioinformatics Advances 2026, vbag209) through BioMCP MCP tools only, GPT 6 Luna, 2026-11-28: 88 of 126 positive labels right. A reading of the 31 missed items found BioMCP usability faults behind 27 of them; reach is complete. For a point variant the route led with a combined profile, and the model answered from it or found no positive claim.

## Evidence

- Item k013: BRCA1 M1775K, no disease or therapy.
- Item k035: EZH2 Y646C, no disease or therapy.

## Proof

- Proof: outside-in cases on recorded CIViC replies: BRCA1 M1775K and EZH2 Y646C show the single-variant profile first, and every other profile holding the allele is listed.

## Starts from

- Starts from: `crates/biomcp-adapters/src/civic/variant.rs`.
