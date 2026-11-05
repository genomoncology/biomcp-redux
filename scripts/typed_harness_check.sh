#!/bin/sh
# Synthetic recorder preflight only. No compiler, product, client or provider.
set -eu
export PYTHONDONTWRITEBYTECODE=1
exec python3 - <<'PYTHON'
import json
import os

command = json.load(open("sdlc/pm.json"))["ticketsGate"]
marker = "make landing-gate"
if command.count(marker) != 1 or not command.endswith(marker):
    raise SystemExit("unsupported ticketsGate: expected trailing make landing-gate")
os.execlp("sh", "sh", "-c", command[:-len(marker)] + "python3 scripts/typed_harness_preflight.py check")
PYTHON
