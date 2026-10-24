# 0121 — Record the article token table's missing questions and tick the budget

Status: OPEN.

Milestone: 1

Depends on: 0119

## Outcome

Each article token table question that 0119 lists as having no recording gets one bounded keyless capture from landed main, becomes a replay case with a size ceiling, and has its measured tokens entered in `planning/slices/article.md`. When every row is measured or marked failed, the PM ticks the article token budget box.

- Why: 0119 found exact recordings for only six of the table's questions. The comparison with 0.9 needs the same questions, so the missing ones are recorded rather than replaced.
- Proof: one pm job from landed main runs the listed commands with existing retries only and no refresh loops. Each success becomes a replay case, from outside the repo where terms bar storage. Each failure is recorded through pm progress and its row says so. Then make landing-gate runs through pm job.
- PM ruling 2026-10-24 on two new ceilings: the amivantamab default case takes its measured size plus 15 percent (1,576 tokens against 0.9's 1,354). The stated reason is that the capture returned a different, current set of ten papers whose titles alone take 1,697 bytes, and no field breakdown of 0.9's output survives. The F1761I supplement first page takes measured plus 15 percent for now, labeled as not yet showing the queried match; 0122 fixes the page and lowers that ceiling. Every existing ceiling stays.
