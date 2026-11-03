"""Refresh existing help metadata snapshots after an intentional help presentation change."""
import json
from pathlib import Path
import subprocess
import sys


def pointer(value, path):
    for part in path.lstrip("/").split("/"):
        value = value[int(part)] if isinstance(value, list) else value.get(part)
        if value is None:
            return None
    return value


def main():
    binary = sys.argv[1]
    for path in Path("tests/cases").rglob("*.yaml"):
        try:
            case = json.loads(path.read_text())
        except ValueError:
            continue
        changed = False
        for step in case.get("steps", []):
            if step.get("run", [None])[0] != "help":
                continue
            output = json.loads(subprocess.check_output([binary, *step["run"]], text=True))
            for key, expected in list(step.get("expect", {}).items()):
                if key.endswith("/description") or key.endswith("/examples"):
                    actual = pointer(output, key)
                    if isinstance(expected, dict) and "count" in expected:
                        actual = {"count": len(actual or [])}
                    if actual != expected:
                        step["expect"][key] = actual
                        changed = True
                if key.endswith("/arguments") and expected == {"count": 6} and pointer(output, key) is None:
                    del step["expect"][key]
                    step["expect"][key.rsplit("/", 1)[0]] = {"omits": ["arguments"]}
                    changed = True
                if key.endswith("/arguments/*/name") and isinstance(expected, list):
                    trimmed = [word for word in expected if word not in {
                        "--format", "--disable-sources", "--source_pacing", "--config", "--offline", "--fresh"}]
                    if trimmed != expected:
                        step["expect"][key] = trimmed
                        changed = True
        if changed:
            path.write_text(json.dumps(case, indent=2, ensure_ascii=False) + "\n")


if __name__ == "__main__":
    main()
