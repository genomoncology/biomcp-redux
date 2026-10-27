#!/usr/bin/env python3
"""Run the pinned secret scanner and disclose only finding paths and rule names."""
import argparse
import gzip
import hashlib
import io
import json
import os
from pathlib import Path, PurePosixPath
import platform
import stat
import subprocess
import tarfile
import tempfile
import urllib.request
import zipfile

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


# These bounds fail closed. They include uncompressed container layers and all
# historical blobs; they never tell the scanner to silently skip large input.
MAX_FILE = 512 * 1024 * 1024
MAX_TOTAL = 8 * 1024 * 1024 * 1024
MAX_DEPTH = 16
MAX_FILES = 200000


def inputs(mode, target, env):
    if mode != "history":
        def unreadable(error):
            raise error
        for directory, directories, files in os.walk(target, followlinks=False, onerror=unreadable):
            for name in sorted(directories + files):
                path = Path(directory) / name
                if path.is_symlink() or not (path.is_file() or path.is_dir()):
                    raise ValueError("nonregular artifact")
                if path.is_file():
                    with path.open("rb") as source:
                        yield path.relative_to(target).as_posix(), source.read(MAX_FILE + 1)
        return
    def git(*args):
        return subprocess.check_output(["git", "--no-replace-objects", "-C", str(target), *args],
                                       env=env, stderr=subprocess.DEVNULL)
    if git("rev-parse", "--is-shallow-repository").strip() != b"false":
        raise ValueError("secret history scan requires a complete checkout")
    # Enumerate actual reachable blobs, not patches: merge-only additions, binary
    # attributes and default diff suppression must not hide historical content.
    objects = git("rev-list", "--objects", "--all", "HEAD").splitlines()
    names = {}
    for row in objects:
        oid, _, name = row.partition(b" ")
        if len(oid) == 40:
            names.setdefault(oid, name.decode("utf-8", errors="replace"))
    ids = git("rev-list", "--objects", "--all", "HEAD", "--no-object-names")
    types = subprocess.check_output(["git", "--no-replace-objects", "-C", str(target), "cat-file", "--batch-check"],
                                    input=ids, env=env, stderr=subprocess.DEVNULL)
    with subprocess.Popen(["git", "--no-replace-objects", "-C", str(target), "cat-file", "--batch"],
                          stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, env=env) as process:
        try:
            for row in types.splitlines():
                oid, kind, size = row.split()
                if kind != b"blob":
                    continue
                if int(size) > MAX_FILE:
                    raise ValueError("historical blob exceeds screening limit")
                process.stdin.write(oid + b"\n")
                process.stdin.flush()
                header = process.stdout.readline().split()
                if header != [oid, kind, size]:
                    raise ValueError("historical blob unavailable")
                payload = process.stdout.read(int(size))
                if len(payload) != int(size) or process.stdout.read(1) != b"\n":
                    raise ValueError("historical blob incomplete")
                yield names.get(oid) or "git-blob/" + oid.decode(), payload
        finally:
            process.stdin.close()
            process.stdout.close()
            if process.wait():
                raise ValueError("historical blob reader failed")


def prepare(target, mode, root, env):
    directory = root / "inputs"
    directory.mkdir()
    mapping, total = {}, 0
    def add(name, payload, depth=0):
        nonlocal total
        total += len(payload)
        if len(payload) > MAX_FILE or total > MAX_TOTAL or depth > MAX_DEPTH or len(mapping) >= MAX_FILES:
            raise ValueError("artifact screening limit exceeded")
        key = str(len(mapping)) + ".txt"
        mapping[key] = name
        # Neutral names remove default path allowlists; a plain-text prefix also
        # prevents MIME detection from silently skipping executable bytes. UTF-8
        # replacement preserves every ASCII credential byte and its boundaries.
        (directory / key).write_text("screened content\n" + payload.decode("utf-8", errors="replace"), encoding="utf-8")
        suffix = name.lower()
        def member_name(member):
            path = PurePosixPath(member.replace("\\", "/"))
            if path.is_absolute() or ".." in path.parts or (path.parts and ":" in path.parts[0]):
                raise ValueError("escaping archive member")
            return name + "!" + member
        if payload.startswith((b"PK\x03\x04", b"PK\x05\x06", b"PK\x07\x08")) or suffix.endswith((".zip", ".whl", ".mcpb")):
            with zipfile.ZipFile(io.BytesIO(payload)) as archive:
                for member in archive.infolist():
                    nested = member_name(member.filename)
                    if member.is_dir():
                        continue
                    if member.file_size > MAX_FILE or total + member.file_size > MAX_TOTAL:
                        raise ValueError("expanded archive exceeds screening limit")
                    kind = stat.S_IFMT(member.external_attr >> 16)
                    if kind not in (0, stat.S_IFREG, stat.S_IFLNK):
                        raise ValueError("unsupported archive member")
                    with archive.open(member) as source:
                        add(nested + "!link-target" if kind == stat.S_IFLNK else nested,
                            source.read(MAX_FILE + 1), depth + 1)
        elif payload.startswith(b"\x1f\x8b") or suffix.endswith((".gz", ".tgz")):
            with gzip.GzipFile(fileobj=io.BytesIO(payload)) as source:
                add(name + "!uncompressed.tar" if suffix.endswith((".tar.gz", ".tgz")) else name + "!uncompressed", source.read(MAX_FILE + 1), depth + 1)
        elif suffix.endswith(".tar") or (len(payload) > 262 and payload[257:262] == b"ustar"):
            with tarfile.open(fileobj=io.BytesIO(payload), mode="r:") as archive:
                for member in archive:
                    nested = member_name(member.name)
                    if member.isdir():
                        continue
                    if member.issym() or member.islnk():
                        # Container layers contain links. Scan their metadata,
                        # never dereference them or extract their target paths.
                        add(nested + "!link-target", member.linkname.encode(), depth + 1)
                    elif member.isfile():
                        if member.size > MAX_FILE or total + member.size > MAX_TOTAL:
                            raise ValueError("expanded archive exceeds screening limit")
                        with archive.extractfile(member) as source:
                            add(nested, source.read(MAX_FILE + 1), depth + 1)
                    else:
                        raise ValueError("unsupported archive member")
        elif suffix.endswith((".7z", ".rar", ".bz2", ".xz", ".zst")) or payload.startswith((b"7z\xbc\xaf\x27\x1c", b"Rar!", b"BZh", b"\xfd7zXZ\x00", b"\x28\xb5\x2f\xfd")):
            raise ValueError("unsupported archive compression")
    for name, payload in inputs(mode, target, env):
        add(name, payload)
    return directory, mapping


def scan(mode, target, cache):
    if target.is_symlink():
        raise ValueError("symbolic-link scan target")
    target = target.resolve(strict=True)
    if not target.is_dir():
        raise ValueError("scan target must be a directory")
    with tempfile.TemporaryDirectory(prefix="biomcp-secret-scan-") as directory:
        root = Path(directory).resolve()
        if __package__:
            from .smoke import environment
        else:
            from smoke import environment
        env = {**environment(root), "PATH": os.defpath, "GIT_CONFIG_NOSYSTEM": "1", "GIT_TERMINAL_PROMPT": "0",
               "GIT_NO_LAZY_FETCH": "1", "GIT_GRAFT_FILE": os.devnull, "GIT_CONFIG_COUNT": "2",
               "GIT_CONFIG_KEY_0": "core.hooksPath", "GIT_CONFIG_VALUE_0": os.devnull,
               "GIT_CONFIG_KEY_1": "core.fsmonitor", "GIT_CONFIG_VALUE_1": "false"}
        binary = scanner(cache.resolve(), root)
        staging, mapping = prepare(target, mode, root, env)
        config, template, ignore, report = [root / name for name in ["config.toml", "report.tmpl", "ignore", "report.txt"]]
        config.write_text('[extend]\nuseDefault = true\n')
        template.write_text('{{range .}}{{printf "%s\t%s\n" .File .RuleID}}{{end}}')
        ignore.touch()
        command = [str(binary), "dir", str(staging), "--config", str(config), "--gitleaks-ignore-path", str(ignore),
                   "--ignore-gitleaks-allow", "--redact=100", "--no-banner", "--no-color", "--log-level", "error",
                   "--report-format", "template", "--report-template", str(template), "--report-path", str(report)]
        # Never retain raw scanner diagnostics or JSON/SARIF finding reports.
        result = subprocess.run(command, cwd=root, env=env, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)
        if result.stderr:
            raise ValueError("secret scanner could not screen every input")
        if result.returncode == 1 and report.is_file():
            for line in report.read_text().splitlines():
                path, rule = line.split("\t", 1)
                print(json.dumps({"file": mapping[Path(path).name], "rule": rule}))
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
    except (OSError, ValueError, KeyError, subprocess.SubprocessError, tarfile.TarError, zipfile.BadZipFile, RuntimeError, EOFError):
        parser.exit(2, "Secret screening failed; diagnostic output was suppressed\n")
