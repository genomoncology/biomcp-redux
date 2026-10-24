# 0098 — Compare 1.0 with 0.9.4 on outside question sets (PM-owned, no developer lane)

Status: OPEN.

Milestone: 5

## Outcome

The PM runs the same outside question sets through BioMCP 0.9.4 and the current 1.0 main, slice by slice, and reports, for each command family, how many judged-relevant answers each version finds in its first page and first 50 results, how many calls, bytes and seconds each took, and which questions one version answers and the other misses. Each gap becomes a ticket or a ruling.

- Why: Ian asked on 2026-10-24 for proof that 1.0 gives agents what they need to find answers, measured against outside judgments, and for a long-term comparison with 0.9.4. The PM owns this work and runs it through its own delegates.
- Starts from: the outside question sets and judgments kept outside the repo, never named in it; 0.9.4 built from the 0.9 repository's main; 1.0 built from this repo's main.
- Keeps: no outside question text, judgment or licensed content in the repo; question sets run through `--cases DIR` or experiment scripts outside it; keyless runs; source pacing as each version applies it.
- Proof: each run records both versions' exact revisions, the commands, the date cap that keeps results inside each set's judging period, and per-question results, with raw evidence on the evidence store. Findings land here as short counts and the tickets they produced.
- Order: articles first; trials and variants after their 1.0 search lands.
