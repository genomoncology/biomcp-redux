# 0098 — Compare 1.0 with 0.9.4 on outside question sets (PM-owned, no developer lane)

Status: OPEN.

Milestone: 5

## Outcome

The PM runs the same outside question sets through BioMCP 0.9.4 and the current 1.0 main, slice by slice, and reports, for each command family, how many judged-relevant answers each version finds in its first page and first 50 results, how many calls, bytes and seconds each took, and which questions one version answers and the other misses. Each gap becomes a ticket or a ruling. 0.9.4 is the floor, not the target: where 1.0 already wins, the run still names what would make it better.

- Why: Ian asked on 2026-10-24 for proof that 1.0 gives agents what they need to find answers, measured against outside judgments, and for a long-term comparison with 0.9.4. The PM owns this work and runs it through its own delegates.
- Starts from: the outside question sets and judgments kept outside the repo, never named in it; 0.9.4 built from the 0.9 repository's main; 1.0 built from this repo's main.
- Keeps: no outside question text, judgment or licensed content in the repo; question sets run through `--cases DIR` or experiment scripts outside it; keyless runs; source pacing as each version applies it.
- Proof: each run records both versions' exact revisions, the commands, the date cap that keeps results inside each set's judging period, and per-question results, with raw evidence on the evidence store. Findings land here as short counts and the tickets they produced.
- Article run 1, 2026-10-24, 1.0 at 78ac5876 against 0.9.4 at 4050fdc7d, keyless, 120 judged topics, raw evidence on the evidence store under this ticket: judged-relevant papers in the first 50 rows averaged 8.5 for 1.0 and 5.0 (typed) or 4.4 (plain) for 0.9.4; in the first 10, 2.1 against 1.9 or 1.4; 1.0 took 1.4 s per topic against 8 to 9 s. Half of 1.0's rows were judged against a fifth of 0.9.4's, so the old judgments favor 1.0 by an unknown amount. 0.9.4 led on 7 topics, from short or empty strict pools and relevant papers just past the first page; this produced 0102. Keyless Semantic Scholar was rate limited for 0.9.4 in most topics.
- Trial run 1, 2026-10-24, 1.0 at 58679817 against 0.9.4 at 4050fdc7d, keyless, ClinicalTrials.gov only, 118 judged topics, raw evidence on the evidence store under this ticket: judged-eligible trials in the first 50 rows averaged 3.10 (typed) and 3.00 (plain) for 1.0 against 2.06 and 2.37 for 0.9.4; in the first 10, 1.18 and 1.08 against 0.54 and 0.82. 1.0 typed won 41 topics and lost 7, at 0.22 s and one request per search against 2.81 s and 13.5 requests. Most eligible trials have closed since judging. This produced 0123. PM rulings: plain text stays literal registry search text, as trial ruling 10 says, and help steers agents to the typed options; registry order stays the default, with `--status` for recruiting trials.
- Remaining sets, Jarvis m8766 2026-10-27: dated cancer questions (19 ClinVar reclassifications for variants and 11 FDA oncology approvals for drugs) and possibly civic or ClinGen eRepo benchmark material for variants. No outside judged set exists for disease, diagnostic, GWAS or phenotype, which is recorded as a gap. The variant and drug runs go through a new experiment folder.
- Order: articles first; trials and variants after their 1.0 search lands; drugs and diseases after their adapters land, starting from 0136's contained 0.9 outputs.
