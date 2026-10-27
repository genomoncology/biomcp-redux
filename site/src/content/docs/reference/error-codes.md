---
title: Error and exit codes
description: What each BioMCP 1.0.0 process exit code means, from 0 for a result
  to 5 for a local failure.
goal: Find what a BioMCP exit code means and what to do next.
reader: A developer or agent scripting BioMCP
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 sets the process exit code from the reply's status. Scripts can branch on it without reading the output.

| Exit code | Status | Meaning |
| --- | --- | --- |
| 0 | Success | The command finished with status `complete` or `partial`. Read the source outcomes: a partial result names the sources that failed or were skipped. |
| 1 | Absent | The command finished and the sources returned no matching record. Status `absent`. This is the answer for that source, query and filters, not proof that the thing does not exist. |
| 2 | Usage | The arguments were wrong, a selector is retired, or a pool cursor does not match its search. The error names a code such as `usage.arguments` and often the replacement command. Nothing was sent to a source. |
| 3 | Expired | A retained pool or snapshot expired. The reply carries the command that restarts the search. |
| 4 | Failed | Every source the command needed failed. Status `failed`. Each source outcome names its reason, such as `deadline_exceeded` or `rejected`. |
| 5 | Local | A local step failed: the configuration file, the cache, a saved file or writing the output. The error code is `local.failure`. |

Read [results](/results/) for the reply envelope and [troubleshooting](/troubleshooting/) for the common failures.

This page is generated from the help of BioMCP 1.0.0. Fix a wrong sentence in the help text, then regenerate.
