"""Reproduce literal-name gaps using an existing current-main executable."""
import hashlib
import json
import platform
import subprocess
import sys
import tempfile
from pathlib import Path

names = ["Canonical therapy", "EX-66", "Example brand"]
with tempfile.TemporaryDirectory(prefix="drug-search-baseline-") as temporary:
    root = Path(temporary)
    binary = root / "biomcp"
    member = {("Linux", "x86_64"): "biomcp-linux-x86_64",
              ("Darwin", "arm64"): "biomcp-macos-arm64"}[(platform.system(), platform.machine())]
    subprocess.run(["scp", sys.argv[1].rstrip("/") + "/" + member, str(binary)], check=True)
    binary.chmod(0o755)
    results = {}
    for entity in ["trial", "article"]:
        counts = []
        for index, name in enumerate(names):
            result = subprocess.run([
                str(binary), "--replay", str(Path("tests/replies/search/drug-names").resolve()),
                "--case-cache", str(root / f"{entity}-{index}"),
                "search", entity, name,
            ], capture_output=True, text=True, check=False)
            reply = json.loads(result.stdout)
            if result.returncode != 0:
                raise RuntimeError(f"{entity} {name}: {result.returncode} {reply}")
            source = next(s for s in reply["sources"] if s["source"] == ("ctgov" if entity == "trial" else "pubmed"))
            counts.append(source["count"])
        if len(set(counts)) == 1:
            raise RuntimeError(f"{entity}: baseline no longer reproduces the gap: {counts}")
        results[entity] = dict(names=names, counts=counts)
    report = dict(binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(), literal_name_gap=results)
    output = Path("target/drug-search-baseline.json")
    output.parent.mkdir(exist_ok=True)
    output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))
