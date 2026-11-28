# 1248 — gene-get-source-refusal-suggests-a-refused-call

Status: OPEN.

Milestone: 7

## Outcome

When `get gene` refuses `--source`, its `next` is a call that runs: the gene's own section for that source (`clingen`, `gencc` or `constraint`), with no section word the gene card does not have, and the guidance holds when the call carries other options the command does not take.

## Why

Found in a persona evaluation on 2026-11-28 (GLM 5.3, BioMCP command and skill). The skill teaches `get disease "Fanconi anemia" gene-validity --source clingen`. The agent carried that form to genes, `get gene RAD51 gene-validity --source clingen --limit 50`, for nine genes, got the generic "Unknown option --source" refusal with `next` `help get gene`, and went to ClinGen's website with `curl` for the rest of the task. Ticket 1114 made `get gene FANCA validity --source clingen` suggest `get gene FANCA clingen`; two neighbouring forms still miss it.

## Evidence

Built binary 1.0.0 at 0e329b6d1b59:

- `biomcp get gene FANCA gene-validity --source clingen` suggests `get gene FANCA gene-validity clingen`. That suggested call is refused: `unknown section` for `gene-validity`, with `next` `help get gene`.
- `biomcp get gene FANCA gene-validity --source clingen --limit 50` returns the generic `Unknown option --source` with `next` `help get gene`, so the source-to-section guidance of 1114 is lost when another unknown option is present. The same holds for `--limit` alone: `get gene FANCA clingen --limit 50` refuses `--limit` (its `next`, `get gene FANCA clingen`, is right).
- `biomcp get gene FANCA validity --source clingen` works as 1114 intended.

## Proof

- Proof: outside-in cases replay the three refused calls above and `get gene RAD51 gene-validity --source gencc`; each `next` names the right gene section and no disease-only section word, and running each suggested call on saved replies returns the section.

## Starts from

- Starts from: gene get option and section parsing and ticket 1114's refusal.

## Keeps

- Keeps: `get gene` takes no `--source`; disease get keeps `gene-validity --source`.

## Defers

- Defers: accepting `gene-validity` as a gene section alias; file it if agents keep reaching for it after this fix.
