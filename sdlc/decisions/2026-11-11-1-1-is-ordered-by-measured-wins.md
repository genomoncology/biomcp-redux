# 1.0 ships as it is; 1.1 is ordered by what the bench measures

Date: 2026-11-11.

## Ruling

Ian, 2026-11-11, in chat with the new PM, which now owns BioMCP, hosting and the bench:

- "Okay, I'll ship 1.0 as is. I'll work with the developer on that."
- The goal for what he has called 1.1: "for BioMCP to have as many of the sources and capabilities needed to make the personas as happy as possible", "making sure that the command-line tool is as efficient as it can be", "making sure the MCP is as efficient as it can be", and "thinking through how we're going to add state to the MCP both locally and on the cloud in an effective manner, which could include using S3 buckets or SQLite for managing data."
- On describing it to agents: "some users will be inside of a coding agent like Claude Code and some will be in more of a traditional chatbot like Claude, we'll need to figure out how to maximize the value in both of those scenarios."
- The bar: "BioMCP has got to be more efficient and effective, or else it's not worth the time."

## What it means here

PM's ordering, not Ian's words. The 2026-11-07 full evaluation (`biomed-agent-bench/results/full-evaluation.md`) had BioMCP tied with plain curl on one model and behind on the other, with far more failed calls in the MCP modes. So 1.1 runs in this order, each step measured on the bench against a 1.0 baseline: efficiency, then the gaps where curl beat BioMCP in the persona runs, then saved work. The milestones are in `sdlc/planning/milestones.md`.

Saved-work record formats are `biomcp.*` schemas in `biomcp-types`, as 2026-11-01-biodata-holds-pure-content rules: BioData holds content, the consumer holds requests, snapshots and storage.

## Options considered

Add 1.1 capabilities before publishing 1.0 (delays the tool-name change on the live service, which is cheapest before beta users arrive); or ship 1.0 and build 1.1 on it. Ian chose to ship.
