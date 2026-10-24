# 0074 — Give installed command proofs canonical temporary space

Status: OPEN.

Milestone: 1

## Outcome

Installed-artifact smoke commands and their nested case runners use owned canonical temporary space rather than falling back to a symlinked operating-system temporary path after environment isolation.

- Why: the same installed binary, case runner and read-only KRAS recordings fail when the smoke environment clears TMPDIR and pass when TMPDIR names an existing canonical job directory. The failure is in proof setup, independent of installer archives.
- Starts from: the exact original/canonical TMPDIR comparison in job j10231. Preserve its witness and change only the setup needed to make isolated installed commands use their owned temporary directory.
- Changes: `release/smoke.py` environment construction and focused installed-proof checks. Preserve configured pm job temporary space where valid, or create and remove an owned canonical child directory. Pass it to nested commands and keep configuration, credentials and cache isolated.
- Keeps: selected installed executable, shared fixture selector, read-only recordings, source confinement, native archive proof and stdio/HTTP behavior. Do not relax symlink confinement or change the product's cache policy.
- Proof: run the installed command and nested runner with the witnessed KRAS recordings outside the checkout, with the outer TMPDIR absent and with an explicit canonical job TMPDIR. Both isolated paths must pass and leave no temporary files. Verify cleanup on failure. Use pm jobs and the routine gate; no source calls or recording refresh.
- Defers: new package channels, global machine configuration and final native-platform execution owned by 0066. Coordinate the small smoke helper change with 0059's installed-HTTP bridge rather than replacing it.
