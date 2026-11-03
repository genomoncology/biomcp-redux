"""Check installed discovery and measure every public command against an installed baseline."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile


def run(binary, args, env):
    result = subprocess.run([str(binary), *args], env=env, capture_output=True, text=True, timeout=30)
    assert result.returncode == 0, (args, result.stdout, result.stderr)
    return result.stdout


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("binary", type=Path)
    parser.add_argument("--baseline", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    with tempfile.TemporaryDirectory() as scratch:
        root = Path(scratch)
        args.output = args.output or root / "proof"
        args.output.mkdir(parents=True, exist_ok=True)
        env = {key: value for key, value in os.environ.items()
               if key in {"PATH", "SystemRoot", "WINDIR", "TMPDIR", "TEMP", "TMP"}}
        env.update(HOME=str(root), XDG_CONFIG_HOME=str(root / "config"),
                   XDG_CACHE_HOME=str(root / "cache"), APPDATA=str(root / "config"))
        binary = root / "bin" / "biomcp"
        binary.parent.mkdir()
        shutil.copy2(args.binary, binary)
        front = json.loads(run(binary, ["help"], env))["result"]["commands"][0]["description"]
        assert run(binary, ["--format", "md", "help"], env).startswith(front + "\n\n")
        assert len(front) <= int(os.environ.get("BIOMCP_FRONT_MAP_CHARACTER_LIMIT", 2500))
        assert "--" not in front
        rendered = run(binary, ["skill", "render"], env)
        assert rendered.startswith(front + "\n\n")
        install = root / "installed"
        run(binary, ["skill", "install", str(install)], env)
        skill = (install / "skills/biomcp/SKILL.md").read_text()
        assert skill.split("\n---\n", 1)[1].lstrip().startswith(front + "\n\n")
        request = {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
            "protocolVersion": "2025-11-25", "capabilities": {},
            "clientInfo": {"name": "front-map-proof", "version": "1"}}}
        process = subprocess.run([str(binary), "serve"], input=json.dumps(request) + "\n",
                                 env=env, capture_output=True, text=True, timeout=30)
        reply = json.loads(process.stdout.splitlines()[0])
        assert reply["result"]["instructions"] == front
        (args.output / "front-map.txt").write_text(front)
        (args.output / "initialize.json").write_text(process.stdout)
        (args.output / "version.json").write_text(run(binary, ["version", "--verbose"], env))
        raw = args.output / "help"
        raw.mkdir(exist_ok=True)
        measurements = []
        pending = [tuple(row.get("command", [])) for row in
                   json.loads(run(binary, ["help"], env))["result"]["commands"]]
        if not args.baseline:
            pending = []
        visited = set()
        while pending:
            path = pending.pop()
            if not path or path in visited:
                continue
            visited.add(path)
            output = run(binary, ["help", *path], env)
            children = [tuple(row.get("command", [])) for row in json.loads(output)["result"].get("commands", [])]
            deeper = [child for child in children if child[:len(path)] == path and len(child) > len(path)]
            if deeper:
                pending.extend(deeper)
                continue
            for fmt in ("json", "md"):
                command = ["--format", fmt, "help", *path]
                current = run(binary, command, env)
                name = "-".join(path) + "." + fmt
                (raw / ("candidate-" + name)).write_text(current)
                record = {"command": list(path), "format": fmt, "characters": len(current)}
                if args.baseline:
                    baseline = run(args.baseline, command, env)
                    record["baseline_characters"] = len(baseline)
                    (raw / ("baseline-" + name)).write_text(baseline)
                measurements.append(record)
        report = {"binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
                  "map_characters": len(front), "map_bytes": len(front.encode()),
                  "measurements": measurements}
        if args.baseline:
            report["baseline_sha256"] = hashlib.sha256(args.baseline.read_bytes()).hexdigest()
            for fmt in ("json", "md"):
                rows = [row for row in measurements if row["format"] == fmt]
                before = max(row["baseline_characters"] for row in rows)
                after = max(row["characters"] for row in rows)
                report[fmt] = {"longest_before": before, "longest_after": after, "reduction": 1 - after / before}
        (args.output / "proof.json").write_text(json.dumps(report, indent=2) + "\n")
        print(json.dumps({key: value for key, value in report.items() if key != "measurements"}))
        if args.baseline:
            assert report["json"]["reduction"] >= 0.40, report["json"]
            assert report["md"]["reduction"] >= 0.40, report["md"]


if __name__ == "__main__":
    main()
