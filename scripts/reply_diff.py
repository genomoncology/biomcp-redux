#!/usr/bin/env python3
"""Compare optional case-runner transcripts. Print at most 20 differences."""
import json
from pathlib import Path
import re
import sys


def load(directory):
    records = {}
    uncaptured = []
    for path in sorted(Path(directory).rglob("*.json")):
        record = json.loads(path.read_text())
        if record.get("kind") == "trial":
            if record["uncaptured"]:
                uncaptured.append(record["name"])
            continue
        key = record["key"]
        if key in records:
            raise ValueError(f"duplicate transcript key: {key}")
        records[key] = record
    if not records:
        raise ValueError(f"no transcripts: {directory}")
    return records, uncaptured


def normalize(record):
    text = bytes(record["stdout"]).decode("utf-8", errors="surrogateescape")
    # These values identify the build or checkout rather than the reply behavior.
    for field in ("executable", "version"):
        value = record.get(field)
        if value:
            text = text.replace(value, f"<{field}>")
            text = text.replace(json.dumps(value)[1:-1], f"<{field}>")
    if record["args"][:1] == ["version"]:
        text = re.sub(r"(?m)^Commit: [^\n]*", "Commit: <commit>", text)
    # Calibration: isolated homes/cache paths and checkout fixture paths vary per run.
    replacements = [(record.get(field), f"<{field}>") for field in ("root", "checkout")]
    args = record["args"]
    for value, replacement in replacements:
        if value:
            text = text.replace(value, replacement)
            text = text.replace(json.dumps(value)[1:-1], replacement)
            args = [arg.replace(value, replacement) for arg in args]
    return {"args": args, "surface": record["surface"], "exit": record["exit"], "stdout": text}



def pointer(left, right, path=""):
    if type(left) is not type(right):
        return path or "/"
    if isinstance(left, dict):
        for key in sorted(left.keys() | right.keys()):
            child = path + "/" + key.replace("~", "~0").replace("/", "~1")
            if key not in left or key not in right:
                return child
            difference = pointer(left[key], right[key], child)
            if difference:
                return difference
    elif isinstance(left, list):
        for index, (before, after) in enumerate(zip(left, right)):
            difference = pointer(before, after, path + f"/{index}")
            if difference:
                return difference
        if len(left) != len(right):
            return path + f"/{min(len(left), len(right))}"
    elif left != right:
        if path.endswith("/stdout"):
            try:
                difference = pointer(json.loads(left), json.loads(right), path)
                if difference:
                    return difference
            except (ValueError, TypeError):
                pass
            index = next((i for i, pair in enumerate(zip(left, right))
                          if pair[0] != pair[1]), min(len(left), len(right)))
            return path + f"/@character:{index}"
        return path or "/"
    return None


def main():
    if len(sys.argv) != 3:
        print("usage: reply_diff.py BASE HEAD", file=sys.stderr)
        return 2
    try:
        (base, base_missing), (head, head_missing) = (load(d) for d in sys.argv[1:])
        for label, missing in (("BASE", base_missing), ("HEAD", head_missing)):
            if missing:
                print(f"{label} uncaptured trials ({len(missing)}):", file=sys.stderr)
                for name in sorted(missing):
                    print(f"  {name}", file=sys.stderr)
        differences = 0
        for key in sorted(base.keys() | head.keys()):
            location = ("/ missing in " + ("HEAD" if key in base else "BASE")
                        if key not in base or key not in head
                        else pointer([normalize(e) for e in base[key]["executions"]],
                                     [normalize(e) for e in head[key]["executions"]]))
            if location:
                differences += 1
                if differences <= 20:
                    print(f"{key}: {location}")
        if differences > 20:
            print(f"{differences - 20} more differing keys")
        return int(differences > 0)
    except (OSError, ValueError, KeyError, TypeError) as error:
        print(f"reply_diff: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
