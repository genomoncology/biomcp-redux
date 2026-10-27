#!/usr/bin/env python3
"""Install local candidates and exercise them outside the checkout. No publication."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import zipfile

from artifacts import cargo, inspect
from smoke import environment, smoke
from local_server import server


def installer_environment(root, bash):
    env = environment(root)
    path = os.defpath
    if os.name == "nt":
        shell = Path(bash).resolve(strict=True).parent
        prefix = shell.parent.parent if shell.parent.name.lower() == "usr" else shell.parent
        # Git Bash keeps POSIX utilities and the downloader in these directories.
        directories = [shell, prefix / "usr/bin", prefix / "mingw64/bin"]
        if "SystemRoot" in env:
            directories.append(Path(env["SystemRoot"]) / "System32")
        path = os.pathsep.join(dict.fromkeys(str(item) for item in directories if item.is_dir()))
    return {**env, "PATH": path}


def install_wheel(wheel, venv, root, host_python, name="biomcp"):
    subprocess.run([host_python, "-m", "venv", "--without-pip", str(venv)], check=True)
    python = venv / ("Scripts/python.exe" if os.name == "nt" else "bin/python")
    install_env = {**environment(root), "PATH": os.defpath, "PIP_CONFIG_FILE": os.devnull, "PIP_NO_CACHE_DIR": "1", "UV_CACHE_DIR": str(root / "uv-cache")}
    uv = shutil.which("uv")
    if uv:
        installer = [uv, "--offline", "--no-config", "pip", "install", "--python", str(python)]
    else:
        subprocess.run([str(python), "-m", "ensurepip"], check=True, env=install_env)
        installer = [str(python), "-m", "pip", "install"]
    subprocess.run(installer + ["--no-index", "--no-deps", str(wheel)], check=True, env=install_env)
    return venv / (("Scripts/" + name + ".exe") if os.name == "nt" else ("bin/" + name))


def installed(args):
    output = args.artifacts.resolve()
    wheel = next(output.glob("*.whl"))
    record = inspect(wheel)  # All distributions must match these Cargo candidate bytes.
    root = args.scratch.resolve()
    root.mkdir(parents=True, exist_ok=True)
    fixtures = root / "fixtures"
    source = Path(__file__).resolve().parent.parent
    executable = root / ("case-runner.exe" if os.name == "nt" else "case-runner")
    if not args.prepared:
        (fixtures / "cases").mkdir(parents=True, exist_ok=True)
        shutil.copy(source / "tests/cases/gene/kras.yaml", fixtures / "cases/kras.yaml")
        for name in ["gene/main", "article/search/synthetic-small"]:
            shutil.copytree(source / "tests/replies" / name, fixtures / "replies" / name, dirs_exist_ok=True)
        for path in fixtures.rglob("*"):
            if path.is_file():
                path.chmod(0o444)
        shutil.copy2(Path(cargo("release", tests=True)["executable"]), executable)
        shutil.copytree(source / "release", root / "tools/release", dirs_exist_ok=True, ignore=shutil.ignore_patterns("__pycache__"))
    if args.prepare_only:
        return
    results = []
    def check(kind, binary, server_args=("serve",), declared_tools=None):
        if hashlib.sha256(binary.read_bytes()).hexdigest() != record["binary_sha256"]:
            raise ValueError(kind + " installed bytes differ from the candidate")
        completed = smoke(binary, executable, fixtures, record["version"], server_args)
        if declared_tools is not None and sorted(completed["tools"]) != sorted(declared_tools):
            raise ValueError("desktop manifest tools differ from installed server")
        results.append({"artifact": kind, "platform": record["platform"], "result": "pass", "checks": ["candidate-bytes"] + completed["checks"]})
        print(json.dumps(results[-1]), flush=True)
    check("wheel", install_wheel(wheel, root / "venv", root, args.python))
    bundle = next(output.glob("*.mcpb"))
    extension = root / "desktop-extension"
    with zipfile.ZipFile(bundle) as package:
        manifest = json.loads(package.read("manifest.json"))
        entry = manifest["server"]["entry_point"]
        if set(package.namelist()) != {"manifest.json", entry} or entry not in {"server/biomcp", "server/biomcp.exe"}:
            raise ValueError("unexpected desktop bundle members")
        extension.mkdir()
        binary = extension / entry
        binary.parent.mkdir()
        binary.write_bytes(package.read(entry))
        binary.chmod(0o755)
        (extension / "manifest.json").write_bytes(package.read("manifest.json"))
    config = manifest["server"]["mcp_config"]
    if manifest["name"] != "biomcp" or manifest["version"] != record["version"] or config["command"] != "${__dirname}/" + entry or config["args"] != ["serve"]:
        raise ValueError("desktop manifest launch contract differs")
    check("desktop-extension", binary, config["args"], [tool["name"] for tool in manifest["tools"]])
    registry = json.loads((output / "server.json").read_text())
    package, = registry["packages"]
    if registry["name"] != "io.github.genomoncology/biomcp" or registry["version"] != record["version"] or package["registryType"] != "pypi" or package["identifier"] != record["distribution"] or package["version"] != record["version"] or package["transport"] != {"type": "stdio"}:
        raise ValueError("registry identity, package or transport differs")
    arguments = [argument["value"] for argument in package["packageArguments"] if argument["type"] == "positional"]
    if arguments != ["serve"]:
        raise ValueError("registry package launch contract differs")
    registry_binary = install_wheel(wheel, root / "registry-venv", root, args.python, package["identifier"])
    check("mcp-registry", registry_binary, arguments)
    archive = next(path for path in output.iterdir() if path.name.endswith((".tar.gz", ".zip")))
    suffix = ".exe" if archive.name.endswith(".zip") else ""
    bash = shutil.which("bash") or "/bin/bash"
    destination = root / "installer-bin"
    subprocess.run([bash, (source / "release/install.sh").as_posix(), archive.as_posix(), hashlib.sha256(archive.read_bytes()).hexdigest(), destination.as_posix()], check=True)
    check("install-script", destination / ("biomcp" + suffix))
    downloads = root / "downloads"
    release = downloads / "download" / ("v" + record["version"])
    release.mkdir(parents=True)
    for item in [archive, archive.with_name(archive.name + ".sha256")]:
        shutil.copy2(item, release / item.name)
    public_destination = root / "download-installer-bin"
    with server(downloads) as base:
        download_env = {**installer_environment(root, bash), "BIOMCP_RELEASE_BASE_URL": base, "BIOMCP_INSTALL_DIR": public_destination.as_posix()}
        subprocess.run([bash, (source / "release/install.sh").as_posix(), "--version", record["version"]], env=download_env, check=True)
    check("download-install-script", public_destination / ("biomcp" + suffix))
    if not homebrew(output, root, check):
        results.append({"artifact": "homebrew-formula", "platform": record["platform"], "result": "deferred", "reason": "Homebrew is unavailable"})
    (root / "report.json").write_text(json.dumps(results, indent=2) + "\n")
    print(json.dumps(results))


def homebrew(output, root, check):
    """Install the candidate in an owned local tap and remove the whole prefix."""
    brew = shutil.which("brew")
    if not brew:
        return False
    brew_repo = subprocess.check_output([brew, "--repository"], text=True).strip()
    with tempfile.TemporaryDirectory(prefix="homebrew-", dir=root) as directory:
        owned = Path(directory)
        prefix = owned / "prefix"
        subprocess.run(["git", "clone", "--quiet", "--shared", "--local", brew_repo, str(prefix)], check=True)
        tap = prefix / "Library/Taps/biomcp/homebrew-candidate"
        formula = tap / "Formula/biomcp.rb"
        formula.parent.mkdir(parents=True)
        subprocess.run(["git", "init", "--quiet", str(tap)], check=True)
        shutil.copyfile(output / "biomcp.rb", formula)
        formula_env = {**environment(owned), "PATH": os.defpath + ":" + str(prefix / "bin"), "HOMEBREW_NO_AUTO_UPDATE": "1", "HOMEBREW_NO_INSTALL_FROM_API": "1", "HOMEBREW_NO_ANALYTICS": "1", "HOMEBREW_NO_ENV_HINTS": "1", "HOMEBREW_CACHE": str(owned / "cache"), "HOMEBREW_LOGS": str(owned / "logs"), "HOMEBREW_TEMP": str(owned / "tmp")}
        subprocess.run([str(prefix / "bin/brew"), "install", "--formula", "--ignore-dependencies", "biomcp/candidate/biomcp"], env=formula_env, check=True)
        check("homebrew-formula", prefix / "bin/biomcp")
    return True


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--scratch", type=Path, required=True)
    parser.add_argument("--prepared", action="store_true", help="Use the already copied runner and public fixtures")
    parser.add_argument("--prepare-only", action="store_true")
    parser.add_argument("--python", default="python3")
    installed(parser.parse_args())


if __name__ == "__main__":
    main()
