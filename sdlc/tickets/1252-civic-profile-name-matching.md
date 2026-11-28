# 1252 — civic-profile-name-matching

Status: OPEN.

Milestone: 10

## Outcome

CIViC profile names match after trimming whitespace on both sides and ignoring letter case. When an exact name finds nothing, the reply offers near matches as next steps: profile names that start with the input, such as "KRAS R164Q (exon 5b)" for "KRAS R164Q" and "VHL L128R (c.383T>G)" for "VHL L128R".

## Why

Found in a run of all 100 items of the public CIViC MCP benchmark (Schimmelpfennig et al., Bioinformatics Advances 2026, vbag209) through BioMCP MCP tools only, GPT 6 Luna, 2026-11-28: 88 of 126 positive labels right. A reading of the 31 missed items found BioMCP usability faults behind 27 of them; reach is complete. Some benchmark profile names carry a trailing space ("BCOR ITD ") or a qualifier in parentheses, and an exact match failed or found nothing with no way forward.

## Evidence

- Item k006: "BCOR ITD " (trailing space), Central Nervous System Tumor With BCOR Internal Tandem Duplication, no therapy.
- Item k093: "VHL L128R (c.383T>G)", Von Hippel-Lindau Disease, no therapy.

## Proof

- Proof: outside-in cases on recorded CIViC replies: "BCOR ITD " and "bcor itd" reach the BCOR ITD profile; a name that matches nothing exactly, such as "VHL L128R", offers the profiles whose names start with it as next steps, and each offered call works.

## Starts from

- Starts from: CIViC profile name lookup in `crates/biomcp-adapters/src/civic/`.

## Keeps

- Keeps: exact matches unchanged; a near match is offered, never silently substituted.
