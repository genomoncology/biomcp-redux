"""Exercise Make's worker defaults through a recording Cargo executable."""
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
CARGO = r'''#!/usr/bin/env python3
import json, os, sys
print(json.dumps({"args": sys.argv[1:], "jobs": os.environ.get("CARGO_BUILD_JOBS"),
                  "threads": os.environ.get("RUST_TEST_THREADS")}))
'''


def main():
    with tempfile.TemporaryDirectory() as directory:
        tools = Path(directory)
        cargo = tools / "cargo"
        cargo.write_text(CARGO)
        cargo.chmod(0o755)
        base = {key: value for key, value in os.environ.items() if key not in {
            "PM_JOB_CORES", "CARGO_BUILD_JOBS", "RUST_TEST_THREADS", "MAKEFLAGS",
            "MFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
        }}
        base["PATH"] = str(tools) + os.pathsep + base["PATH"]
        cases = [
            ({}, None, None),
            ({"PM_JOB_CORES": "2"}, "2", "2"),
            ({"PM_JOB_CORES": "2.25"}, "2", "2"),
            ({"PM_JOB_CORES": "0.5"}, "1", "1"),
            ({"PM_JOB_CORES": "2.25", "CARGO_BUILD_JOBS": "4"}, "4", "2"),
            ({"PM_JOB_CORES": "2", "RUST_TEST_THREADS": "5"}, "2", "5"),
            ({"CARGO_BUILD_JOBS": "3", "RUST_TEST_THREADS": "4"}, "3", "4"),
        ]
        for settings, jobs, threads in cases:
            result = subprocess.run(["make", "--no-print-directory", "-s", "test"],
                                    cwd=ROOT, env=dict(base, **settings), check=True,
                                    capture_output=True, text=True)
            actual = json.loads(result.stdout)
            expected = ["test", "--workspace", "--locked"]
            if threads:
                expected += ["--", "--test-threads", threads]
            assert actual == {"args": expected, "jobs": jobs, "threads": threads}, (settings, actual)
    print("Make honors core bookings and caller compiler/test worker overrides")


if __name__ == "__main__":
    main()
