# One PM drives the work, and 1.0 is not published yet

Date: 2026-11-11. Decided by Ian.

Ian: "I'm going to shut down the build coordinator, so we just work with you directly." "I don't want to publish 1.0. I want to get everything done and fixed, but use this as an opportunity to push things up to AWS at some point." "I don't think that there's any big rush to even get this up to the AWS server. I'd rather focus on doing our internal testing and iterating here locally."

On temporary AWS buckets for tests: "which you'll have the rights to do. Obviously, I want to manage costs, but my assumption is it'll cost less than a penny to do most of your tests."

## What this changes

- The separate build coordinator is stood down. The PM drives every lane with its own developers and reviewers.
- The 1.0 publication lane (the history append, the tag, public installs, the docs site) is parked. Its preparation is kept as it stands and is refreshed when Ian asks for publication, because main keeps moving until then.
- The AWS rollout of 1.0 is not scheduled. Work is tested locally and on the bench first.
- Temporary test buckets in the AWS account are approved, at trivial cost. Anything that changes the running service or costs more still goes to Ian.
