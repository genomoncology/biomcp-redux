# 0856 — trial-search-widens-variant-spellings

Status: OPEN.

Milestone: 7

## Outcome

A trial search for a protein change finds the trials that write the same change another way (V600E, Val600Glu, p.V600E, and where the design review says so the broader codon form V600), says in the reply which spellings it searched, and searches the literal text only with `--exact-term`, as drug names do since 0830.

## Why

Bench 0037's blind persona rerun (2026-11-12) found this the largest silent undercount left after 0830: patient-navigator's `--variant V600E` found 1 of 3 matched BRAF/MEK plus immunotherapy trials where `V600` found all 3; "exon 19 deletion" also lost trials; life-sciences-ci's `--variant G12C` missed CodeBreaK 202. 0830 deferred condition and protein-change spellings to a separate ticket that was never written.

## Proof

- Proof: first measure, on live ClinicalTrials.gov, how many trials each spelling finds for BRAF V600E, KRAS G12C and EGFR exon 19 deletion, and which spellings trial records actually use; put the table here before choosing.
- Proof: a routine case with recorded replies shows `--variant V600E` finding the trials written as V600E, Val600Glu and (if chosen) V600, with the searched spellings listed, and `--exact-term` returning the literal count; continuation pages keep page 1's spellings.

## Starts from

- Starts from: trial search's `--variant`/`--criteria` paths in `crates/biomcp-query/src/trial/`, the variant normalizers BioMCP already has (`variant_normalize`, HGVS protein forms), and 0830's widening pattern (one disclosure notice, frozen names, never fewer than literal, totals not summed).
- PM decision: 2026-11-12. Design review first: whether widening a specific change (V600E) to its codon (V600) is right is a clinical-meaning question, since V600K patients match V600 trials too. Default to exact change in its spellings; codon widening only if the review shows trials mostly write the codon and the reply says so. Consult expert-assist if this stays unsettled.

## Keeps

- Keeps: literal search with `--exact-term`; no result presented as widened when it was not; native totals never summed per spelling; existing trial row fields and paging.

## Defers

- Defers: condition-name spellings (a separate ticket if the measurement shows a comparable gap); NCI trial search stays literal with its notice.

## Changes

- Changes: `crates/biomcp-query/src/trial/`, `tests/cases/trial/`

## Progress

### Measurement, 2026-11-12

Live ClinicalTrials.gov, BioMCP built from 5e8fa6030 (job j38556), overlap sets fetched in full from the registry API with the same parameters BioMCP sends (jobs j38556, j38563). "Variant" is `--variant`, which BioMCP sends as unquoted general text (`query.term`); "Criteria" is `--criteria`, sent as `AREA[EligibilityCriteria]"..."`. "Only" means trials no other exact spelling in the group finds.

| Change | Spelling | Variant | Only | Criteria | Only |
|---|---|---|---|---|---|
| BRAF V600E | V600E | 260 | 213 | 499 | 459 |
| | p.V600E | 5 | 5 | 4 | 3 |
| | Val600Glu | 0 | 0 | 1 | 1 |
| | p.Val600Glu | 0 | 0 | 0 | 0 |
| | c.1799T>A | 0 | 0 | 1 | 0 |
| | V600E/K | 47 | 0 | 40 | 0 |
| | exact spellings, one OR query | 265 | | 504 | |
| | codon V600 | 201 | adds 158 | 321 | adds 288 |
| | V600X | 0 | | 7 | adds 7 |
| KRAS G12C | G12C | 202 | 188 | 218 | 211 |
| | p.G12C | 37 | 23 | 29 | 22 |
| | Gly12Cys | 2 | 2 | 0 | 0 |
| | p.Gly12Cys | 0 | 0 | 0 | 0 |
| | c.34G>T | 1 | 1 | 0 | 0 |
| | exact spellings, one OR query | 227 | | 240 | |
| | codon G12 | 73 | adds 64 | 47 | adds 46 |
| | G12X | 5 | adds 3 | 4 | adds 3 |
| EGFR exon 19 deletion | "exon 19 deletion" (phrase) | 72 (80 unquoted) | 63 | 353 | 332 |
| | exon 19 deletions | 18 | 13 | 53 | 50 |
| | exon 19 del | 5 | 4 | 19 | 17 |
| | ex19del | 22 | 20 | 87 | 85 |
| | 19del | 24 | 19 | 90 | 76 |
| | del19 | 13 | 11 | 26 | 22 |
| | E746_A750del | 0 | 0 | 1 | 1 |
| | all spellings, union | 142 | | 606 | |

Findings:

- The biggest gap is where `--variant` looks, not how the change is spelled. `--variant V600E` searches general text and finds 260; the same word in eligibility criteria finds 499; together 623. G12C: 202 and 218, together 280. One query `(spellings) OR AREA[EligibilityCriteria](spellings)` returns exactly the union of the two (V600E 631, G12C 303, exon 19 661).
- Protein spellings add little for BRAF (260 to 265) and more for KRAS (202 to 227, mostly `p.G12C`). Three-letter and `c.` forms are nearly unused by trial records.
- Exon 19 spellings matter most: the phrase finds 72 in general text and 353 in criteria, while all spellings find 142 and 606. ClinicalTrials.gov does not merge "deletion" with "deletions" in a quoted phrase, nor ex19del, 19del or del19.
- ClinicalTrials.gov's own search expands a little: its default expansion versus `EXPANSION[None]` changes V600E 258 to 260, ex19del 2 to 22 (letter case), and unquoted "exon 19 deletion" 55 to 80. It never maps one-letter to three-letter codes, `p.` forms or exon abbreviations. V600E is one token, so `V600` matches only text that writes the bare codon. "V600E/K" matches as V600E plus K and adds nothing.
- Codon forms: with the gene anchored (spellings AND BRAF), V600/V600X add 154 trials to 262; 142 write BRAF, mostly "BRAF V600 mutation" or "V600 E/K"; 9 name another allele; a rough text check flags about 10 that may exclude V600E (unchecked by hand). With KRAS anchored, G12/G12X add 12, all naming KRAS. Without a gene, G12 adds 50 in general text, including 6 psychiatry trials matching the "PANSS G12" questionnaire item, and most never mention KRAS.
- Searching the typed text unquoted, as today, finds 3 exon 19 trials that the quoted spellings miss, so the typed text must stay in the widened query as typed.

### Proposal

1. Spellings. A protein substitution that `protein_query_forms` parses (V600E, p.Val600Glu, G12C ...) searches its four forms: one-letter and three-letter, each with and without `p.`. An "exon N deletion" or "exon N insertion" phrase searches a fixed pattern of spellings: "exon N deletion(s)" / "insertion(s)", "exon N del" / "ins", exNdel / exNins, Ndel / Nins, delN / insN. Coding-DNA forms (c.1799T>A) are not generated, since trials almost never use them and the conversion needs a transcript. Anything else is searched as typed, and the notice says it was not widened. The typed text is always one of the alternatives, unquoted as today, so the widened search never finds fewer trials than the literal one.
2. Where. `--variant` searches general text OR eligibility criteria in one query, because that closes the largest measured gap (V600E 260 to 631). `--criteria` searches its spellings inside `AREA[EligibilityCriteria](...)`. This changes `--variant` from general text only, so the PM should confirm it is in scope; otherwise it becomes its own ticket.
3. Codon. See the open question below. Recommendation: widen to the codon (V600, V600X; G12, G12X) only when `--gene` is given. The gene is already ANDed into the query, so codon matches stay tied to the gene. The notice lists the codon forms separately as codon level, "trials written for any change at this position; check each trial's allele", and names the trials on the page that matched only through the codon. Without `--gene`, there is no codon widening, and the reply suggests the gene-anchored search as a next step. BioMCP already fetches each row's criteria text when `--gene` is set (for gene notices), so it can check locally which rows matched only the codon without a second request. Rows stay in the registry's order and keep their fields.
4. Query syntax and limits. Alternatives are quoted, sorted (stable cache key) and joined with OR in parentheses, then ANDed with the gene and keywords exactly as today. At most 7 alternatives per input, each well under the registry's 12-word-per-name limit; the measured 7-spelling exon 19 query with both fields was accepted. If the registry rejects the widened query (400), BioMCP searches the typed text and says so, as 0830 does for drug names.
5. Totals and paging. One OR query, so the registry's own total is reported and never summed. Page 1's widened query is sent as an exact query and carried in the next-page request, the same mechanism trial search uses for drug names (`sent_query` / `exact()`), so later pages keep page 1's spellings and never recompute them.
6. Disclosure. One compact notice per searched input: the typed text, the spellings searched, the codon forms if any, and the fields searched. Adding a notice kind changes the trial-page reply schema, so it takes a new major (trial-page/16). The alternative is to reuse the drug-name notice shape under a general "term spellings" name; that is the same schema cost, so a new specific kind is clearer.
7. `--exact-term` sends today's request unchanged: the literal text, general text only for `--variant`. NCI search stays literal with its notice.

### Open question for the PM

Whether V600E widens to V600. The PM default is the exact change only, with the codon only if trials mostly write the codon. The measurement shows the codon is not the majority form (154 codon-only trials against 262 exact, with BRAF anchored), but those trials are mostly "BRAF V600 mutation" trials that a V600E patient is the main intended population for. In the bench run, `--variant V600E` found 1 of 3 relevant trials where V600 found all 3.

Expert assist (Fable, 2026-11-12, advice only) recommended codon widening only when the gene is known, with codon-only matches marked per trial, not only in the notice. It also recommended dropping the "mostly writes the codon" test: that test needs extra counting queries, its threshold is arbitrary, and it makes behaviour differ unpredictably by variant. Its stated risks are trials specific to another allele (V600K-only, G12D/G12V inhibitors), explicit exclusions ("non-V600", "G12C excluded"), and the codon appearing in prior-therapy text. Not verified: the roughly 10 possible exclusions, what share of added trials truly admit the exact allele, and codons outside BRAF and KRAS (EGFR T790, IDH1 R132, PIK3CA H1047).

Options:

- (a) Never widen to the codon; the reply only suggests the codon search. This is precise, but misses about a third of BRAF candidates unless the agent follows the suggestion.
- (b) Recommended: widen with `--gene` only, and mark codon-only trials.
- (c) Widen always. Rejected: without a gene, G12 brings in psychiatry and other unrelated trials.
