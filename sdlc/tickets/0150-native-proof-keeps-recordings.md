# 0150 — Keep native proof recordings and raw outputs before cleanup

Status: COMPLETE.
Landed: 5adfb43ecd056d9944bdcdab992a659446b7d0a1

Milestone: 3

## Outcome

The two native product-proof entry points copy their recordings and raw outputs to the proof's external directory before the temporary directory is removed, as 0142's proof already does, so the archive holds them.

- Why: in 0141 and 0130 the temporary directory deleted recordings and outputs after successful comparisons. The provider captures and proof logs survived, so no product error followed, but the proofs cannot be rechecked.
- Starts from: the two native proof entry points and 0142's external proof directory.
- Proof: an offline run of each entry point leaves its recordings and raw outputs in the external directory after cleanup, and a failed comparison keeps them too. Historical proofs are not rerun or rebuilt. No provider call. Then make landing-gate runs through pm job.
