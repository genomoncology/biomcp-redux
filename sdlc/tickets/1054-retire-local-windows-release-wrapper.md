# 1054 — retire-local-windows-release-wrapper

Status: COMPLETE.
Landed: a4be8590105cbafdfe0cb7f61753e9621362f803

Milestone: 7

## Outcome

BioMCP's release instructions and focused checks use the existing GitHub Windows jobs as their Windows path. The retired local Windows wrapper and its dedicated helpers and tests are removed.

## Why

The shared Windows VM is retired under the [November 18 Windows ruling](../decisions/2026-11-18-windows-checks-run-in-github-release-workflows.md).

## Proof

- Proof: Run the existing focused release-channel and hosted-section checks through `pm job`. Verify that active instructions and scripts have zero references to the local wrapper, VM lock or SSH path, and that the GitHub Windows jobs retain their native build and qualification steps.

## Starts from

- Starts from: `release/qualification.md`, `scripts/check_release_channels.sh`, `scripts/check_windows_release.sh`, `scripts/check_windows_release.cmd`, `scripts/windows_vm_lock.sh`, `scripts/test_windows_vm_lock.py`, `scripts/wait_windows_ssh.py` and `scripts/windows_remote.py`.

## Keeps

- Keeps: Existing GitHub Windows release and recovery jobs, tracked historical evidence and Unix release checks.

## Changes

- Changes: `release/qualification.md`, `scripts/check_release_channels.sh`, `scripts/check_windows_release.*`, `scripts/windows_vm_lock.sh`, `scripts/test_windows_vm_lock.py`, `scripts/wait_windows_ssh.py`, `scripts/windows_remote.py`
