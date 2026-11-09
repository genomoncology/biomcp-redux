# Expose per-architecture selection through the existing cold-release helper instead of reconstructing its full-clone setup in split jobs.

Status: open.

Kind: idea
When: The next release that needs cold qualification split across architectures or machines.

## Problem

Expose per-architecture selection through the existing cold-release helper instead of reconstructing its full-clone setup in split jobs. Prevent the worktree mount preparation failure seen in j36290 while reusing existing cold bootstrap logic; no new framework or release-blocking gate.
