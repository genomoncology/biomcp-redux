# 0099 — Keep Windows SystemRoot in isolated installed and case-runner environments

Status: COMPLETE.
Landed: f4a53d8e63b91b9e6b1debfa130e4431fec28afc

Milestone: 1

## Outcome

The installed-artifact smoke and the case runner keep the Windows variables the OS needs, such as `SystemRoot`, when they isolate a child's environment, so the installed Windows binary starts `serve --http` with or without a console. HOME, config, cache and keys stay isolated. Product HTTP code is unchanged.

- Why: platform proof at 9bd46f799 saw every Windows HTTP case exit before readiness. j10523 first suggested a console dependency. j10543 then showed handler registration succeeds without a console, and j10545 proved the unchanged wheel binary starts under both a console and CREATE_NO_WINDOW when `SystemRoot` is present, and fails in both when it is absent. The cause is the test setup stripping a required OS variable.
- Starts from: fix e7afca6d in the smoke environment and shared test launcher; retained evidence in Yellow `data/biomcp-redux/0066-windows-final/`.
- Changes: preserve a fixed, named list of required Windows variables in isolated child environments; force a CREATE_NO_WINDOW proof row. No product code change.
- Proof: a 2×2 run with and without a console and with and without `SystemRoot` records the expected starts and failures; the installed HTTP cases pass against the existing candidate bytes with CREATE_NO_WINDOW; the console CTRL_BREAK proof stays valid; a case shows HOME, config, cache and key variables still isolated. 0066 then continues only its missing channel rows.
- Keeps: proved wheel, replay, stdio and cache behavior; booked pm jobs for all VM work; owner-approved read-only tool paths; VM restored to stopped.
- Defers: publishing and aggregate platform completion owned by 0066.
