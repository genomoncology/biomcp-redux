"""Prepare one candidate's public release payloads without publishing."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import zipfile


PLATFORMS = [
    {"os": "ubuntu-24.04", "platform": "manylinux_2_28_x86_64", "arch": "amd64", "image": "quay.io/pypa/manylinux_2_28_x86_64"},
    {"os": "ubuntu-24.04-arm", "platform": "manylinux_2_28_aarch64", "arch": "arm64", "image": "quay.io/pypa/manylinux_2_28_aarch64"},
    {"os": "macos-15-intel", "platform": "macosx_11_0_x86_64", "arch": "", "image": ""},
    {"os": "macos-14", "platform": "macosx_11_0_arm64", "arch": "", "image": ""},
    {"os": "windows-2022", "platform": "win_amd64", "arch": "", "image": ""},
]


def identity(args):
    metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--locked", "--no-deps", "--format-version", "1"], text=True))
    version = next(package["version"] for package in metadata["packages"] if package["name"] == "biomcp")
    if not re.fullmatch(r"[0-9a-f]{40}", args.revision):
        raise ValueError("candidate revision must be a full Git commit")
    if args.event == "push" and args.tag != "v" + version:
        raise ValueError("release tag must match the committed Cargo version")
    print("version=" + version)
    print("matrix=" + json.dumps({"include": PLATFORMS}, separators=(",", ":")))


def protection(args):
    settings = json.loads(args.settings.read_text())
    rules = settings.get("protection_rules", [])
    if not any(rule.get("type") == "required_reviewers" and rule.get("reviewers")
               and rule.get("prevent_self_review") is True for rule in rules):
        raise ValueError("release-publication must require a separate release operator's review after Ian approves")
    if settings.get("can_admins_bypass") is not False:
        raise ValueError("release-publication must disable administrator approval bypass")


def candidates(args):
    records = {}
    for path in args.artifacts.glob("*/release.json"):
        record = json.loads(path.read_text())
        tag = record["platform"]
        if tag in records:
            raise ValueError("duplicate native platform")
        if record["version"] != args.version or record["revision"] != args.revision or record["distribution"] != "biomcp-cli":
            raise ValueError("staged artifacts must share the candidate identity")
        records[tag] = path.parent
    if set(records) != {row["platform"] for row in PLATFORMS}:
        raise ValueError("all five native release platforms are required")
    return records


def copy_native(directory, output, tag, registry):
    for pattern in ["*.whl", "*.tar.gz", "*.zip"]:
        for path in directory.glob(pattern):
            shutil.copy2(path, output / path.name)
    wheel, = directory.glob("*.whl")
    if not wheel.name.endswith("-py3-none-" + tag + ".whl"):
        raise ValueError("wheel platform differs from candidate record")
    archive, = [path for path in directory.iterdir() if path.name.endswith((".tar.gz", ".zip"))]
    checksum = archive.with_name(archive.name + ".sha256")
    expected = hashlib.sha256(archive.read_bytes()).hexdigest() + "  " + archive.name
    if checksum.read_text().strip() != expected:
        raise ValueError("archive checksum differs from candidate bytes")
    shutil.copy2(checksum, output / checksum.name)
    if json.loads((directory / "server.json").read_text()) != registry:
        raise ValueError("registry payload differs between native producers")
    # Desktop support is promised on macOS and Windows only.
    if tag.startswith(("macosx_", "win_")):
        bundle, = directory.glob("*.mcpb")
        shutil.copy2(bundle, output / bundle.name)
    if tag.startswith("manylinux_"):
        arch = "amd64" if tag.endswith("x86_64") else "arm64"
        image = directory / ("biomcp-container-" + arch + ".tar")
        shutil.copy2(image, output / image.name)


def formula(output, version):
    lines = ['class Biomcp < Formula', '  desc "BioMCP CLI and MCP server"',
             '  homepage "https://biomcp.org"', '  version "' + version + '"',
             '  on_macos do']
    for arch, selector in [("arm64", "on_arm"), ("x86_64", "on_intel")]:
        name = "biomcp-darwin-" + arch + ".tar.gz"
        checksum = (output / (name + ".sha256")).read_text().split()[0]
        lines += ['    ' + selector + ' do',
                  '      url "https://github.com/genomoncology/biomcp/releases/download/v' + version + '/' + name + '"',
                  '      sha256 "' + checksum + '"', '    end']
    lines += ['  end', '  def install', '    bin.install "biomcp"', '  end', 'end', '']
    (output / "biomcp.rb").write_text("\n".join(lines))


def plugin(source, output, version):
    metadata = json.loads((source / ".claude-plugin/plugin.json").read_text())
    marketplace = json.loads((source / ".claude-plugin/marketplace.json").read_text())
    entry, = marketplace["plugins"]
    if metadata["name"] != "biomcp" or marketplace["name"] != "biomcp" or entry["name"] != "biomcp" or entry["source"] != "./":
        raise ValueError("plugin and marketplace must preserve the candidate identity")
    if entry["mcpServers"] != {"biomcp": {"command": "biomcp", "args": ["serve"]}}:
        raise ValueError("plugin must launch installed biomcp serve")
    guide = source / "skills/SKILL.md"
    if not guide.is_file():
        raise ValueError("plugin must contain the release skill guide")
    metadata["version"] = entry["version"] = version
    paths = [path for path in (source / "skills").rglob("*") if path.is_file()]
    with zipfile.ZipFile(output / "biomcp-plugin.zip", "w", zipfile.ZIP_DEFLATED) as archive:
        for name, value in [("plugin", metadata), ("marketplace", marketplace)]:
            archive.writestr(".claude-plugin/" + name + ".json", json.dumps(value, indent=2) + "\n")
        for path in sorted(paths):
            archive.write(path, path.relative_to(source).as_posix())


def stage(args):
    records = candidates(args)
    args.output.mkdir(parents=True, exist_ok=False)
    registry = json.loads((next(iter(records.values())) / "server.json").read_text())
    if registry.get("name") != "io.github.genomoncology/biomcp" or registry.get("version") != args.version:
        raise ValueError("registry identity differs from candidate")
    for tag, directory in records.items():
        copy_native(directory, args.output, tag, registry)
    (args.output / "server.json").write_text(json.dumps(registry, indent=2) + "\n")
    shutil.copy2(Path(__file__).parent / "install.sh", args.output / "install.sh")
    formula(args.output, args.version)
    plugin(args.plugin, args.output, args.version)
    index = {"version": args.version, "revision": args.revision,
             "image": "ghcr.io/genomoncology/biomcp", "publication": "requires Ian's approval and the release operator"}
    (args.output / "candidate.json").write_text(json.dumps(index, indent=2) + "\n")
    print(json.dumps(index))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    candidate = commands.add_parser("identity")
    candidate.add_argument("--event", choices=["push", "workflow_dispatch"], required=True)
    candidate.add_argument("--tag", required=True)
    candidate.add_argument("--revision", required=True)
    environment = commands.add_parser("protection")
    environment.add_argument("settings", type=Path)
    payloads = commands.add_parser("stage")
    for name in ["artifacts", "output", "plugin"]:
        payloads.add_argument("--" + name, type=Path, required=True)
    for name in ["version", "revision"]:
        payloads.add_argument("--" + name, required=True)
    args = parser.parse_args()
    {"identity": identity, "protection": protection, "stage": stage}[args.command](args)


if __name__ == "__main__":
    try:
        main()
    except (KeyError, ValueError, OSError) as error:
        sys.exit(str(error))
