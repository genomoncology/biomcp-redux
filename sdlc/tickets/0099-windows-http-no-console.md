# 0099 — Start HTTP on Windows without a console

Status: OPEN.

Milestone: 1

## Outcome

The installed Windows binary starts `serve --http` under a process without a console, while keeping normal console interrupt shutdown when a console exists.

- Why: platform proof built the exact candidate at 9bd46f799 and passed wheel installation, stdio, CLI/MCP replay and cache concurrency. All HTTP cases exited before readiness. Focused job j10523 showed the same binary exits 5 silently without a console, but serves health/MCP and exits cleanly on CTRL_BREAK after its parent allocates a console.
- Starts from: retained candidate SHA-256 `b46fb8dd12be46380ea4e1ced75efdc877e0cc8fe31f10224868b894385baddc`, runtime witness, console report and build provenance in Yellow `data/biomcp-redux/0066-windows-final/`. Investigate the actual startup error before changing the likely interrupt-handler boundary.
- Changes: fix the narrow HTTP startup dependency on a Windows console. Preserve bounded connection/worker ownership and ordinary console interrupts; do not weaken the protocol, add a shutdown endpoint or require users to allocate a console. Keep errors bounded and secret-free.
- Proof: use real installed Windows processes with and without a console. Both must reach readiness and pass health/MCP checks. The console case must handle CTRL_BREAK and exit cleanly with workers closed. Reuse built candidate and runner during diagnosis; rebuild product once the fix requires it. Run the existing installed HTTP cases against the fixed candidate, then let 0066 continue only its missing channel rows. No source calls or full platform matrix repeat.
- Keeps: already proved wheel, replay, stdio and cache behavior. All builds and VM probes run through booked pm jobs. Windows uses only owned work folders and the explicitly owner-approved BuildTools path read-only; each run stays within an hour and restores the VM to stopped.
- Defers: publishing, new Windows tool installation, and aggregate platform completion owned by 0066.
