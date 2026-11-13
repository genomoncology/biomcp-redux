# 0886 — phenotype-set-ranks-genes

Status: OPEN.

Milestone: 7

## Outcome

A set of HPO terms returns one ranked list of candidate genes (and diseases) in one call, with each row's score and the source's method named, so a counselor's three coded terms become a shortlist without intersecting pages by hand.

## Why

In the rare-disease persona run of 2026-11-13 (biomed-agent-bench `personas/rare-disease/runs/2026-11-13/log.md`, analysis beside it; code-mode run in `runs/2026-11-13-raw/`), `search phenotype "global developmental delay muscular hypotonia seizure"` returned total 0 (log.md:48-50) and the agent intersected three 50-row gene pages by hand, which is meaningless when each term pulls about 2,000 genes. The code-mode agent called Monarch's `/v3/api/semsim/search/{termset}/Human Genes` once and got a ranked list (raw log.md:31-37).

## Proof

- Proof: a recorded case where HP:0001263, HP:0001252 and HP:0001250 return Monarch's ranked genes with scores matching the recorded reply, the metric named, and Monarch's limit stated; a disease group works the same way; an unknown HPO ID is refused with its own message.

## Starts from

- Starts from: `crates/biomcp-adapters/src/monarch/`, the phenotype commands.

## Keeps

- Keeps: per-term `get phenotype <HP> genes`.

## Defers

- Defers: our own ranking; scoring a named gene's rank beyond Monarch's limit.

## Changes

- Changes: `crates/biomcp-adapters/src/monarch/`, `crates/biomcp-query/src/phenotype/`, `crates/biomcp-types/`, `tests/cases/phenotype/`, `docs/`

## Progress

- Measured Monarch live (jobs j39141, j39147, j39157, 2026-11-13). `GET /v3/api/semsim/search/{termset}/{group}` takes comma-separated term IDs; groups Human Genes, Human Diseases and four model-organism gene groups; metrics `ancestor_information_content` (default), `jaccard_similarity`, `phenodigm_score`, anything else 422; `limit` 1 to 50, default 10, over 50 a 422; `offset` is ignored, so there are no further pages. The reply is a bare array; each row has `subject`, `score`, both direction scores, `similarity` (termsets and per-term best matches) and `search_settings` (engine ducksim, metric, directionality bidirectional, search_mode hybrid, information_content ontology). Hybrid mode scores only a Jaccard-prefiltered set, which explains why SCN1A, CDKL5, SCN2A and KMT2D miss the top 50. An unknown ID alone returns `[]`; mixed with known IDs it is listed in `object_termset` with a null label and ranked without, silently. Repeats change the score. A 50-row reply is about 900 KB. Both hosts returned 502 for every path for some minutes mid-run, then recovered.
- Shape: the text of `search phenotype` that is only HP IDs (commas or spaces) now asks Monarch for the ranking; `--group genes|diseases` (default genes) and `--metric` apply only to that form; `--offset` is refused for it. Unquoted space-separated IDs get the existing guidance with the quoted call as the next step. This form was retired with a fixed message; that refusal is removed.
- Reply `biomcp.phenotype-matches/1`: rank, id, label, category, score, both direction scores and each queried term's best match per row; the queried terms with Monarch's labels; Monarch's stated method; `source_limit` 50. Gene rows pivot to `get gene`, disease rows to `get disease`.
- An HP ID Monarch does not know (null label, or an empty reply) refuses the set with its own message, next steps `get phenotype` per unknown ID and the same ranking without them.
- Case `tests/cases/phenotype/set-ranks.yaml` with replies recorded live in job j39191; help example added; cautions section added.
