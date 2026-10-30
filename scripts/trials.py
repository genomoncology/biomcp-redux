"""Select exact names from an existing test target before preparing its build."""
from pathlib import Path
import subprocess
from typing import Callable, Sequence


def run_selected(target: Path, cwd: Path, names: Sequence[str],
                 prepare: Callable[[], None]) -> None:
    """List and run the same target in its required directory using exact names."""
    target = target.resolve(strict=True)
    cwd = cwd.resolve(strict=True)
    listed = subprocess.run([str(target), "--list"], cwd=cwd, check=True,
                            capture_output=True, text=True)
    available = {line.removesuffix(": test") for line in listed.stdout.splitlines()
                 if line.endswith(": test")}
    selected = list(dict.fromkeys(names))
    unknown = [name for name in selected if name not in available]
    if not selected or unknown:
        raise ValueError("trial selection requires exact names from this target's --list: "
                         + ", ".join(unknown))
    prepare()
    for name in selected:
        subprocess.run([str(target), name, "--exact"], cwd=cwd, check=True)
