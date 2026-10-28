"""Read the reviewed public contract targets from the feature ownership list."""
import argparse
from pathlib import Path
import re
import json
import subprocess

CONTRACT = re.compile(r"([a-z][a-z0-9-]*): (.+[.!?]) Contracts: `([a-z][a-z0-9-]*)/([a-z][a-z0-9_]*)`(?: \(features: `([a-z][a-z0-9_-]*(?:,[a-z][a-z0-9_-]*)*)`\))?\.")


def targets(root):
    result = []
    for line in (root / "sdlc/features.md").read_text().splitlines():
        if "Contracts:" not in line:
            continue
        match = CONTRACT.fullmatch(line)
        if not match:
            raise ValueError(f"invalid public contract ownership: {line}")
        feature, _, package, target, features = match.groups()
        source = root / "crates" / package / "tests" / f"{target}.rs"
        manifest = root / "crates" / package / "Cargo.toml"
        if not source.is_file() or not manifest.is_file():
            raise ValueError(f"public contract target does not exist: {package}/{target}")
        text = manifest.read_text()
        if not re.search(r'^name = "' + re.escape(package) + r'"$', text, re.M):
            raise ValueError(f"public contract package differs: {package}")
        for flag in features.split(",") if features else []:
            if not re.search(r"^" + re.escape(flag) + r"\s*=", text, re.M):
                raise ValueError(f"public contract feature does not exist: {package}/{flag}")
        key = (package, target)
        if any(row[:2] == key for row in result):
            raise ValueError(f"duplicate public contract target: {package}/{target}")
        result.append((package, target, features or "-", feature))
    return result


def library(root):
    """Run excluded suites once, without rerunning a counted contract target."""
    counted = {(row[0], row[1]) for row in targets(root)}
    metadata = json.loads(subprocess.check_output([
        "cargo", "metadata", "--locked", "--no-deps", "--format-version", "1",
    ], cwd=root))
    subprocess.run([
        "cargo", "test", "--workspace", "--exclude", "biomcp", "--lib",
        "--features", "biomcp-types/schema", "--locked",
    ], cwd=root, check=True)
    subprocess.run([
        "cargo", "test", "--locked", "-p", "biomcp", "--bin", "biomcp",
    ], cwd=root, check=True)
    for package in metadata["packages"]:
        if package["name"] == "biomcp":
            continue
        command = ["cargo", "test", "--locked", "-p", package["name"]]
        for target in package["targets"]:
            if "test" in target["kind"] and (package["name"], target["name"]) not in counted:
                command.extend(["--test", target["name"]])
        if "--test" in command:
            if package["name"] == "biomcp-types":
                command.extend(["--features", "schema"])
            subprocess.run(command, cwd=root, check=True)
    subprocess.run([
        "cargo", "test", "--workspace", "--exclude", "biomcp", "--doc",
        "--features", "biomcp-types/schema", "--locked",
    ], cwd=root, check=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--library", action="store_true")
    args = parser.parse_args()
    if args.library:
        library(args.root)
        raise SystemExit(0)
    for row in targets(args.root):
        print("\t".join(row))
