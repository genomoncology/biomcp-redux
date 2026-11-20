# 1075 — pdf-reads-run-in-a-capped-child-process

Status: COMPLETE.
Landed: d00df60558d20708a638acf4c3dca392ed475f56

Milestone: 7

## Outcome

Every PDF is read in a child process of BioMCP's own binary, with a memory cap the operating system enforces where it can and a time limit after which the parent kills the child. A killed or crashed child becomes a refusal that says the document could not be read safely. A hole in PDF reading that nobody has found then costs at most the cap and the time limit.

## Why

Ticket 1068 found that hostile PDFs could make the PDF library use gigabytes of memory, and closed each known route with a scan of the raw bytes before the library loads. Three review rounds each found a new way to make that scan and the library read the same file differently. Two parsers of the same hostile bytes will keep differing somewhere, so matching them rule by rule cannot be shown complete. A cap on the process does not depend on finding every hole.

## Proof

- Proof: a test PDF that passes the scan but makes the reader allocate past the cap (built in the test, using a test-only switch if no real file does it) is refused as "could not be read safely" within the time limit on Linux, and the parent's memory stays flat. It fails on the base commit.
- Proof: a child that never answers is killed at the time limit and the caller gets the same refusal.
- Proof: every recorded PDF in the repo reads byte-for-byte the same through the child, and the developer reports the added time per read measured on Linux and macOS.

## Starts from

- Starts from: the reader as ticket 1068 leaves it, and the developer's design of 2026-11-19: a hidden `pdf-read` subcommand of the same binary; the binary crate owns the spawning because the adapters crate's lint rules forbid starting processes; the child sets its own memory limit first thing through `rustix`'s safe call, since the workspace denies `unsafe`.
- PM decisions, 2026-11-19: approved as designed. The cap is 1 GiB of address space by default and configurable; measure the binary's own baseline before fixing the default. On macOS, where the system does not enforce a process memory limit, the time limit alone is accepted for a user's own machine, and the docs say so. The hosted server runs on Linux and must have the cap. The scan from ticket 1068 stays as the cheap first line.

## Keeps

- Keeps: reply shapes do not change apart from the new refusal. Running BioMCP's own binary is not running another tool. No new dependency beyond making `rustix`, already in the lockfile, a direct one.

## Defers

- Defers: the cap on how many PDFs the hosted server reads at once (ticket 1074), which bounds total memory.

## Changes

- Changes: the binary crate's PDF path, the adapters' reader interface, config, docs/cautions.md, tests.
