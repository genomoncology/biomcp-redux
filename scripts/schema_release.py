"""Generate checked-in schemas or compare them with the release schema set.

Print a baseline summary and the selected Rust contract test result.
Generation checks compatibility before writing. No baseline update occurs.

With --changed, as the landing gate runs it (0997), compare only when the branch changed
the comparison's inputs since it left main: the reply types and their checked-in schemas
(crates/biomcp-types/), the baseline and reviews (release/schema-*), this script, or the
workspace manifest, lock file, toolchain or Makefile. When no base can be found it compares.
"""
import argparse
import hashlib
import io
import json
import os
from pathlib import Path
import re
import subprocess
import tarfile
import tempfile

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_BASELINE = ROOT / "release/schema-baseline.json"
GENERATION_TEST = "checked_in_schemas_match_types"
BASE_REF = os.environ.get("SCHEMA_RELEASE_BASE", "origin/main")
INPUTS = ("crates/biomcp-types/", "release/schema-", "scripts/schema_release.py", ".cargo/")
WORKSPACE_FILES = ("Cargo.toml", "Cargo.lock", "rust-toolchain", "rust-toolchain.toml", "Makefile")


def changed():
    """Whether the branch changed an input of the comparison since it left main."""
    try:
        base = subprocess.run(["git", "merge-base", "HEAD", BASE_REF], check=True,
                              capture_output=True, text=True).stdout.strip()
        # Names only, without rename detection, so a partial clone fetches no file contents.
        listed = subprocess.run(["git", "diff", "-z", "--name-only", "--no-renames", base, "HEAD"],
                                check=True, capture_output=True).stdout
    except subprocess.CalledProcessError as error:
        detail = os.fsdecode(error.stderr or b"").strip()
        print(f"schema-release: no base from {BASE_REF} ({detail}); comparing", flush=True)
        return True
    paths = [os.fsdecode(path) for path in listed.split(b"\0") if path]
    inputs = [path for path in paths if path in WORKSPACE_FILES or path.startswith(INPUTS)]
    print(f"schema-release base={base} inputs={','.join(inputs[:5]) or 'none'}"
          f"{',...' if len(inputs) > 5 else ''}", flush=True)
    return bool(inputs)


def run_schema_test(name, environment, nocapture=False):
    command = [
        "cargo", "test", "--locked", "-p", "biomcp-types", "--features", "schema",
        "--test", "schemas", name, "--", "--exact",
    ]
    if nocapture:
        command.append("--nocapture")
    result = subprocess.run(command, cwd=ROOT, env=environment,
                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    print(result.stdout, end="", flush=True)
    result.check_returncode()
    passed = re.findall(rf"^test {re.escape(name)} \.\.\. ok$", result.stdout, re.MULTILINE)
    summaries = re.findall(
        r"^test result: ok\. 1 passed; 0 failed; 0 ignored; 0 measured; [0-9]+ filtered out",
        result.stdout, re.MULTILINE,
    )
    if len(passed) != 1 or len(summaries) != 1:
        raise ValueError(f"{name} must run and pass exactly once; check the schema feature and test target")


def generate():
    run_schema_test(GENERATION_TEST, dict(os.environ, UPDATE_SCHEMAS="1"))


def load_baseline(path):
    metadata = json.loads(path.read_text())
    if not isinstance(metadata, dict):
        raise ValueError("baseline manifest must be an object")
    required = {"version", "status", "source_commit", "archive", "sha256", "schemas"}
    if set(metadata) != required:
        raise ValueError("baseline manifest fields differ")
    if not re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+", str(metadata.get("version"))):
        raise ValueError("invalid baseline version")
    if metadata.get("status") not in ("provisional-candidate", "release-candidate", "released"):
        raise ValueError("baseline status must identify a provisional candidate, release candidate or released set")
    if not re.fullmatch(r"[0-9a-f]{40}", str(metadata.get("source_commit"))):
        raise ValueError("invalid baseline source commit")
    archive = metadata.get("archive")
    if not isinstance(archive, str) or Path(archive).name != archive:
        raise ValueError("baseline archive must be a sibling filename")
    names = metadata.get("schemas")
    if (not isinstance(names, list) or not names or
            any(not isinstance(name, str) or not re.fullmatch(r"[a-z0-9-]+\.json", name)
                for name in names) or len(set(names)) != len(names)):
        raise ValueError("invalid baseline schema inventory")
    data = (path.parent / archive).read_bytes()
    if hashlib.sha256(data).hexdigest() != metadata.get("sha256"):
        raise ValueError("baseline archive checksum differs")
    return metadata, data


def extract_baseline(metadata, data, destination):
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as archive:
        members = archive.getmembers()
        names = [member.name for member in members]
        if sorted(names) != sorted(metadata.get("schemas")):
            raise ValueError("baseline archive inventory differs")
        for member in members:
            if not member.isfile():
                raise ValueError("baseline archive must contain only schema files")
            stream = archive.extractfile(member)
            if stream is None:
                raise ValueError("baseline schema cannot be read")
            schema = stream.read()
            if not isinstance(json.loads(schema), dict):
                raise ValueError("baseline schema must be an object")
            (destination / member.name).write_bytes(schema)


def check(path):
    metadata, data = load_baseline(path)
    with tempfile.TemporaryDirectory(prefix="biomcp-schema-baseline-") as folder:
        extract_baseline(metadata, data, Path(folder))
        print(f"Schema baseline: {metadata.get('version')} "
              f"({metadata.get('status')}, {metadata.get('source_commit')})", flush=True)
        environment = dict(os.environ, BIOMCP_SCHEMA_BASELINE=folder)
        reviews = path.with_name("schema-reviews.json")
        environment.pop("BIOMCP_SCHEMA_REVIEWS", None)
        if reviews.exists():
            environment["BIOMCP_SCHEMA_REVIEWS"] = str(reviews.resolve())
        # Run both the release comparison and the default generation/drift check.
        environment.pop("UPDATE_SCHEMAS", None)
        run_schema_test("generated_schemas_preserve_release_contracts", environment, nocapture=True)
        run_schema_test(GENERATION_TEST, environment)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", type=Path, default=DEFAULT_BASELINE)
    parser.add_argument("--generate", action="store_true", help="regenerate checked-in schemas")
    parser.add_argument("--changed", action="store_true",
                        help="compare only when the branch changed the comparison's inputs")
    args = parser.parse_args()
    try:
        if args.generate:
            generate()
        elif not args.changed or changed():
            check(args.baseline)
    except (OSError, ValueError, tarfile.TarError, subprocess.CalledProcessError) as error:
        parser.exit(1, f"schema release gate: {error}\n")


if __name__ == "__main__":
    main()
