# 0069 — Compare live requests across cache hits and retries

Status: COMPLETE.
Landed: f7fa47ba32b032db4705cc8e5327043575e09d00

Milestone: 1

Depends on: 0055

## Outcome

The live diagnostic distinguishes cache reuse, retry attempts, request-plan changes and provider contract changes. Its report retains enough bounded evidence to explain each flagged result.

- Why: the first public run reported thirteen cache asymmetries and four recorded retry sequences as drift. A recorded 429 followed by success was compared with the successful live response as if the error were the provider contract. Changed PubMed result pools also changed downstream request identities without establishing a contract failure.
- Starts from: landed live diagnostic and the first-run report and analysis in Yellow the evidence store under ticket 0068. Read these witnesses before designing the comparison.
- Keeps: keyless isolation, source pacing, existing child and whole-run budgets, explicit eligibility skips, complete affected-case counts and affected-first report samples. Ordinary replay remains unchanged.
- Changes: `crates/biomcp/tests/runner/live*.rs`, the shared request observer where needed, and focused command cases. Compare successful exchanges in their command and endpoint roles while retaining retries and source failures as evidence. Explain cache reuse without forcing new HTTP requests. Report changed downstream request plans truthfully rather than treating every missing request hash as a removed contract. Keep genuine response-schema and status changes visible.
- Evidence: retain bounded command ordinal, request/source identity, endpoint role, retry status and before/after status or schema differences needed to interpret a flag. The retired-gene case currently cannot be classified from hashes alone. Exclude credentials and raw response bodies; expose truncation when a bound drops evidence. Keep one report pipeline rather than a second diagnostic framework.
- Proof: offline command runs reproduce cache reuse, recorded 429 then success, changed result-pool requests, a real removed field and a real status change. Assert the full report and affected totals. Run the ordinary suite and gate through pm jobs. This ticket needs no public source calls or recording refresh.
- Defers: automatic fixture updates, source contract policy changes, scheduling and another whole public run.
