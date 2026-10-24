# 0070 — Handle skipped predecessors in live follow commands

Status: COMPLETE.
Landed: 092c27944a1f10ea69458a62aea92ed9152531dd

Milestone: 1

Depends on: 0055

## Outcome

The live diagnostic handles a follow command whose predecessor is ineligible without raising a runner error or borrowing unrelated output.

- Why: the first public run skipped the exit-3 predecessor in `article/search/checked-request.yaml`, then raised `follow lacks predecessor` on its dependent command.
- Starts from: landed live diagnostic and the first-run report and analysis in Yellow the evidence store under ticket 0068.
- Keeps: explicit synthetic, keyed, local and external-source exclusions; isolated settings; source pacing; bounded runs; ordinary replay semantics.
- Changes: the live runner's step eligibility and predecessor handling, plus focused command cases. Either run a known local precondition that makes no network request, or skip its dependent command with a specific reason. Never execute an ineligible source call to satisfy the chain. A follow command may consume only its actual predecessor's output.
- Proof: an offline command run exercises the witnessed checked-request chain, an eligible follow chain and a skipped source-dependent chain. Verify inventory and report counts and skip reasons, and verify ordinary replay still works. Run checks through pm jobs. No public source call or fixture refresh is needed.
- Defers: comparator and request-witness changes owned by 0069, scheduling and another whole public run.
