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
