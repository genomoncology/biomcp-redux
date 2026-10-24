# 0100 — Supply the Windows download proof's required shell tools

Status: COMPLETE.
Landed: 08ee616a70f843dc3424ab21a8ce20dd64cb5d38

Milestone: 1

## Outcome

The isolated loopback-download installer proof can execute its required shell tools on Windows without inheriting credentials or unrelated user configuration.

- Why: platform job j10592 passed desktop, registry and local ZIP installation with the retained candidate and fixed runner. The loopback-download row set PATH to Python's Windows os.defpath (`.;C:\\bin`), then the same owned Git Bash could not find mktemp. Its local ZIP row had already succeeded with the same shell.
- Starts from: exact gate head ed472ae and retained 9bd46f799 candidate, logs, per-row report and remaining-evidence archive on Yellow `data/biomcp-redux/0066-windows-final/`. No product or installer defect is established by this setup failure.
- Changes: the narrow download-proof environment in `release/gate.py` and its enforcing check. Supply an explicit checked tool path or named required tool directories for the selected shell on Windows, preserving isolated home, config, cache, required OS variables and omitted credentials. Keep native POSIX behavior. Do not add arbitrary user PATH inheritance or a second environment framework.
- Proof: reproduce the loopback-download failure with retained candidate bytes, then prove the Windows download row's checksum, ZIP extraction, byte identity and existing installed checks pass. Assert required tools are available while unrelated configuration, proxies and synthetic secret markers remain excluded. Run routine checks through pm jobs. Reuse the built artifact and runner; repeat neither product builds nor successful platform rows.
- Keeps: all passed wheel, desktop, registry, local ZIP, replay, concurrency and HTTP proofs. Each booked Windows run stays within an hour and restores the VM to stopped; only owned folders change and the owner-approved BuildTools remain read-only.
- Defers: source-free key-reload and aggregate platform completion, which 0066 runs after this fix, and publication.
