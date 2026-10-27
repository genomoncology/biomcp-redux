# 0312 — Point the variant card to ClinVar and show more ClinVar rows by default

Status: OPEN.

Milestone: 5

## Outcome

The variant card names the ClinVar section as a next step, and the ClinVar section's default page shows enough rows that condition-level classifications are visible, with the total and a next page step when more exist.

- Why: the 0098 variant run on 2026-10-27 (experiment 496, keyless; 19 ClinVar reclassification questions and 88 ClinGen eRepo dev variants) found that 1.0 answered every ClinVar question and more ClinGen questions than 0.9.4 once the ClinVar section was read, and 1.0 read ClinVar for 7 indels 0.9.4 could not. Two gaps remain. The card shows no classification and does not point to the ClinVar section, so an agent stopping at the card finds nothing. The ClinVar section's default page shows 3 of 24 to 32 assertions, so 0.9.4, which prints every row, answered 3 condition-level questions 1.0 missed.
- Starts from: the variant card's next steps, the ClinVar section and its paging, the measured caps decision, and the experiment 496 raw outputs.
- Rules: PM ruling 2026-10-27. The card makes no extra ClinVar call and shows no classification; it only names the section. The ClinVar default page rises to 10 rows. The section states its total row count and a next page step when rows remain. Caps follow the measured plus 15 percent rule.
- Proof: a card case shows the ClinVar next step; a ClinVar case with more than 10 assertions shows 10 rows, the total and the next page step; the help and size caps pass.
- Changes: the variant card steps, the ClinVar section default and their cases and caps.
- Defers: nothing.
