# 0130 — Search Drugs@FDA and read approvals by application and product

Status: COMPLETE.
Landed: 4206426c178429cc3604a8201c50866174268572

Milestone: 3

Depends on: 0124

Depends on: 0125

Depends on: 0129

## Outcome

`search drug TEXT --source drugsfda` returns native application and product candidates. `get drug NDA021335 approvals --source drugsfda` returns that application's native product and history rows as `AuthorizationEvent` records with their source status and date roles, paged with `--section-limit` and `--section-offset`, and `--product-number` selects one explicit native product. Where a Drugs@FDA row carries an SPL set ID, `next` names the exact `get drug SETID label --source openfda-label` step.

- Why: the bank's US approval questions (D02, D04, D08, D23) need native application rows, and drug.md keeps Drugs@FDA source-scoped with the SPL set as its only proven link to labels.
- Starts from: drug.md's Drugs@FDA rows, the proven joins table and case A01; 0129's label commands for the next step; 0124 and 0125.
- Rules: a phrase match is a candidate, so trastuzumab returning BLA761139 for Enhertu never makes Enhertu trastuzumab. A missing product is unavailable, never the first product. An old approval row never becomes a current approval flag, and no event reaches a card. The SPL step does not collapse an application's products. `identity_level` comes from the endpoint or record type, never from a field's shape.
- Proof: outside-in cases first. One bounded keyless pm job captures a positive exact application-number get and a `--product-number` known positive before adapter code, plus the venetoclax row carrying set b118a40d-6b56-cee3-10f6-ded821a97018. Cases S04 for `drugsfda` (D04's trastuzumab and Enhertu rows stay separate) and A01 (D02, D04, D08, D23 native rows, explicit product, the SPL set step) replay the recorded replies with size ceilings. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
- PM ruling 2026-10-25: DrugApprovals holds source-owned typed FDA product and submission records, each submission using the shared AuthorizationEvent for its event and keeping its own submission number, review priority, class code and description, property types, documents and date roles as sent. Products keep brand, ingredients and strength, dosage form, nullable route, marketing status, reference flags and TE code as sent. EMA records gain no FDA fields. No submission is tied to a product the source does not tie it to. Because one application can carry many submissions, the section pages with `--section-offset` and `--section-limit` and has a measured ceiling. Schema versions follow the compatibility check.
- PM rulings 2026-10-25: (3) Drugs@FDA search returns a source-owned page of native application and product candidates built from the approved FDA product records, and that page joins shared candidate navigation; the common DrugPage gains no FDA-only fields. (4) The next step from an approval is the exact SPL identity card, whose own native heading actions then lead to label sections; no heading is guessed.
