# Finish BioMCP 1.0 with AWS validation before publication

Date: 2026-11-09. Source: Ian's answers in the BioMCP 1.0 finish session.

The question shown to Ian was: “Your earlier decision requires AWS validation and a source review before publishing 1.0 (sdlc/decisions/2026-11-05-hosting-validates-before-1-0.md:7–9). Should that remain part of finishing 1.0?”

Ian selected “Keep AWS first (Recommended)”. The selected option's description was: “Finish core qualification and prepare hosting in parallel; publish after AWS validation and the source review.”

Ian then said: “Implement the plan.” He added: “you can keep your builds on M5 or yellow.”

## Implementation outcome

- Finish the core fixes and qualification, and prepare the hosting handoff in parallel.
- AWS validation and the source review stay before publication. This confirms that part of `2026-11-05-hosting-validates-before-1-0.md`; the completed core is the input to hosting's validation.
- Builds for this work may run on M5 or Yellow. Use the existing tracked jobs and release rehearsal; this permission does not change the free public rehearsal arrangement.
- Prepare the public-history merge, account changes and refreshed docs site for review. This implementation instruction does not itself push a public release tag, deploy AWS, change accounts or publish the site. Those actions keep their existing approval points.

This records the approved implementation outcome and order. Ticket and job status remain in pm.
