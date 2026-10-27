#!/usr/bin/env python3
"""Run the pinned secret scanner and disclose only finding paths and rule names."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import tarfile
import tempfile
import urllib.request

# Official release assets and SHA-256 digests:
# https://github.com/gitleaks/gitleaks/releases/tag/v8.30.1
# CLI: https://github.com/gitleaks/gitleaks/blob/v8.30.1/README.md#usage
VERSION = "8.30.1"
ARCHIVES = {
    ("Darwin", "arm64"): ("darwin_arm64", "b40ab0ae55c505963e365f271a8d3846efbc170aa17f2607f13df610a9aeb6a5"),
    ("Darwin", "x86_64"): ("darwin_x64", "dfe101a4db2255fc85120ac7f3d25e4342c3c20cf749f2c20a18081af1952709"),
    ("Linux", "x86_64"): ("linux_x64", "551f6fc83ea457d62a0d98237cbad105af8d557003051f41f3e7ca7b3f2470eb"),
    ("Linux", "aarch64"): ("linux_arm64", "e4a487ee7ccd7d3a7f7ec08657610aa3606637dab924210b3aee62570fb4b080"),
}


def scanner(cache, scratch):
    suffix, digest = ARCHIVES[(platform.system(), platform.machine())]
    name = "gitleaks_" + VERSION + "_" + suffix + ".tar.gz"
    cache.mkdir(parents=True, exist_ok=True)
    archive = cache / name
    if not archive.exists():
        url = "https://github.com/gitleaks/gitleaks/releases/download/v" + VERSION + "/" + name
        with urllib.request.urlopen(url, timeout=120) as response:
            payload = response.read()
        if hashlib.sha256(payload).hexdigest() != digest:
            raise ValueError("scanner download checksum mismatch")
        # Atomic replacement also permits concurrent gates using the same cache.
        with tempfile.NamedTemporaryFile(dir=cache, delete=False) as output:
            output.write(payload)
        Path(output.name).replace(archive)
    if hashlib.sha256(archive.read_bytes()).hexdigest() != digest:
        raise ValueError("scanner cache checksum mismatch")
    binary = scratch / "gitleaks"
    with tarfile.open(archive) as bundle:
        member = bundle.getmember("gitleaks")
        if not member.isfile():
            raise ValueError("scanner executable is not a regular file")
        binary.write_bytes(bundle.extractfile(member).read())
    binary.chmod(0o700)
    return binary


def scan(mode, target, cache):
    target = target.resolve(strict=True)
    if mode == "history":
        shallow = subprocess.check_output(["git", "-C", str(target), "rev-parse", "--is-shallow-repository"], text=True).strip()
        if shallow != "false":
            raise ValueError("secret history scan requires a complete checkout")
    with tempfile.TemporaryDirectory(prefix="biomcp-secret-scan-") as directory:
        root = Path(directory).resolve()
        binary = scanner(cache.resolve(), root)
        config, template, ignore, report = [root / name for name in ["config.toml", "report.tmpl", "ignore", "report.txt"]]
        config.write_text('[extend]\nuseDefault = true\n')
        # Never create a raw JSON/SARIF finding report. Gitleaks renders these two
        # fields directly, so matched lines, secrets and commit messages never persist.
        template.write_text('{{range .}}{{printf "%s\\t%s\\n" .File .RuleID}}{{end}}')
        ignore.touch()
        command = [str(binary), "git" if mode == "history" else "dir", str(target),
                   "--config", str(config), "--gitleaks-ignore-path", str(ignore),
                   "--ignore-gitleaks-allow", "--redact=100", "--no-banner", "--no-color",
                   "--log-level", "error", "--max-archive-depth", "100",
                   "--report-format", "template", "--report-template", str(template),
                   "--report-path", str(report)]
        if mode == "history":
            command += ["--log-opts=--all --full-history"]
        # Share the installed-test environment owner. The scanner gets no host
        # keys, config, scanner overrides or Git credentials.
        if __package__:
            from .smoke import environment
        else:
            from smoke import environment
        env = {**environment(root), "PATH": os.defpath, "GIT_CONFIG_NOSYSTEM": "1", "GIT_TERMINAL_PROMPT": "0"}
        result = subprocess.run(command, cwd=root, env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        if result.returncode == 1 and report.is_file():
            for line in report.read_text().splitlines():
                path, rule = line.split("\t", 1)
                print(json.dumps({"file": path, "rule": rule}))
            return 1
        if result.returncode:
            raise ValueError("secret scanner failed; diagnostic output was suppressed")
        print("Secret scan passed")
        return 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=["history", "artifacts"])
    parser.add_argument("target", type=Path)
    parser.add_argument("--cache", type=Path, default=Path("target/security-tools"))
    args = parser.parse_args()
    try:
        raise SystemExit(scan(args.mode, args.target, args.cache))
    except (OSError, ValueError, KeyError, subprocess.SubprocessError, tarfile.TarError):
        parser.exit(2, "Secret screening failed; diagnostic output was suppressed\n")
