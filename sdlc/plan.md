# Plan: parity and release

This plan covers milestone 5. Every 1.0 entity slice has landed, so this milestone proves that 1.0 keeps what 0.9 users rely on, makes 1.0 safe to run as a long-lived server, documents it, builds every release artifact, and ends with the history replacement after Ian approves the release. It names stages and the tickets that own them.

## Why this order

Robustness and public-text fixes come first, because the parity run, the docs and the rehearsal should all measure the code that will ship. The parity run comes before the docs rewrite, because any fact 1.0 loses against 0.9 becomes a fix that the docs must describe. The pipeline and schema gate come before the rehearsal, because the rehearsal installs what the pipeline builds. The history replacement is last because it is the step that cannot be undone.

## Stages

### 1. Close milestone 4 and harden the server

- Finish the native concurrency diagnosis (0287).
- MCP server robustness (0283), store upgrades that keep bundles (0284, then coexisting generations in 0295), one-snapshot ownership checks (0296), replay miss reasons through one mapping (0285) and check hardening (0286).
- Port `version` and `mcp-config` (0288).
- Remove private infrastructure and vendor names from public text (0279).

### 2. Prove parity

- Run the harvested cases against 0.9.3 and 1.0 with a runner (0291) declared endpoints, a whole-case inventory and trial and pathway plans (0298), gene, article, variant, author and protein plans and the residual cases in parallel (0300 to 0305) on shared projection extensions (0306), multi-step and stateful execution (0299), then a partial recorded comparison, projection and completeness fixes (0307) and common sequence step shapes (0308) and a few comparable facts for each zero-coverage family (0309), then a whole run from main that fills the deferred size and timing comparison (0297). Each fact 1.0 loses becomes a bug ticket.
- Run the outside judged agent comparison against the latest released 0.9 (0098), PM-owned.
- Capture the LitSense gene search when the source allows it (0072).

### 3. Document

- Rewrite the README as a short guide, generate the reference docs and recipes (0292).
- Ship the agent skill guide (0187).

### 4. Build the release

- Carry the 0.9 release pipeline over, with publication gated on Ian's approval (0289).
- Compare schemas with the last released copy (0290).
- AlphaGenome transport design goes to Ian with options (0271); the AlphaGenome section follows only if he approves a transport (0198).

### 5. Rehearse and release

- Rehearse the final candidate on every platform (0293).
- Ian approves the release, and the release operators publish.
- Replace the public repository's history (0294), after Ian's explicit approval of that step.
