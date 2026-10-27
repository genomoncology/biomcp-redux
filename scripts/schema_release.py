"""Compare generated schemas with the recorded release schema set.

Print a baseline summary and the selected Rust contract test result.
No network access, publication, or baseline update occurs.
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


def load_baseline(path):
    metadata = json.loads(path.read_text())
    if not isinstance(metadata, dict):
        raise ValueError("baseline manifest must be an object")
    required = {"version", "status", "source_commit", "archive", "sha256", "schemas"}
    if set(metadata) != required:
        raise ValueError("baseline manifest fields differ")
    if not re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+", str(metadata.get("version"))):
        raise ValueError("invalid baseline version")
    if metadata.get("status") not in ("release-candidate", "released"):
        raise ValueError("baseline status must identify a candidate or released set")
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
        # Run both the release comparison and the default generation/drift check.
        environment.pop("UPDATE_SCHEMAS", None)
        subprocess.run([
            "cargo", "test", "--locked", "-p", "biomcp-types", "--features", "schema",
            "--test", "schemas", "release::generated_schemas_preserve_release_contracts",
            "--", "--exact", "--nocapture",
        ], cwd=ROOT, env=environment, check=True)
        subprocess.run([
            "cargo", "test", "--locked", "-p", "biomcp-types", "--features", "schema",
            "--test", "schemas", "checked_in_schemas_match_types", "--", "--exact",
        ], cwd=ROOT, env=environment, check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", type=Path, default=DEFAULT_BASELINE)
    args = parser.parse_args()
    try:
        check(args.baseline)
    except (OSError, ValueError, tarfile.TarError, subprocess.CalledProcessError) as error:
        parser.exit(1, f"schema release gate: {error}\n")


if __name__ == "__main__":
    main()
