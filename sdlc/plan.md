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
- AlphaGenome: Ian approved the streaming gRPC transport. Build it in the shared fetch layer with zero provider calls: codec (0320), worker (0321), persistence (0322), then cost and target proof (0323), source and key registration (0333), the ScoreVariant encoder (0337); then the variant section with one clinically chosen capture (0198). It ships in 1.0, as `decisions/2026-10-27-1-0-ships-every-known-fix.md` says.

### 5. Close every known gap

- Docs findings, first pass: help entry points (0338), argument errors and grammar (0339), configuration and proxy (0340), schema IDs (0341), health (0342), output pipe (0343), help reference (0357), MCP output schemas (0358), schema formats (0359), adapter bad values (0360, then 0371), enums and typed IDs (0361), output consistency (0362), live failures (0363), use case how-tos (0369).
- Docs findings, second pass: MCP config path (0352), HTTP MCP revisions (0353), openFDA generic names (0354), cBioPortal sync (0355), MCP refusals (0356), variant population (0364), GRCh37 normalize (0365), store size (0366), drugsfda items (0367).
- Security: screening (0347), inert source text (0348), outbound host registry (0349), signed release provenance (0370).
- Split-off and hygiene: PubMed warnings (0334), semantic argument names (0345), HGNC reachability (0368), build cost and check order (0344), export comment (0346), output budgets (0350), local fixture abort (0351).
- Order: schema-changing tickets (0341, then 0358, 0359, 0361, 0362) early, because each regenerates the baseline.

### 6. Rehearse and release

- Cut the release candidate from main once every ticket above lands (0336).

- Rehearse the final candidate on every platform (0293).
- Ian approves the release, and the release operators publish.
- Append this repository's history to the public repository with one merge commit, keeping old tags and releases (0294), after Ian's explicit approval of that step.
