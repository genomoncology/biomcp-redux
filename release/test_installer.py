"""Feature install-script: exercise the actual installer with isolated downloads."""
from contextlib import contextmanager
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
import hashlib
import io
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile
from threading import Thread
import unittest
import zipfile

SCRIPT = Path(__file__).with_name("install.sh").resolve()


class QuietHandler(SimpleHTTPRequestHandler):
    def log_message(self, *_args):
        pass


@contextmanager
def server(directory):
    http = ThreadingHTTPServer(("127.0.0.1", 0), partial(QuietHandler, directory=str(directory)))
    thread = Thread(target=http.serve_forever, daemon=True)
    thread.start()
    try:
        yield "http://127.0.0.1:" + str(http.server_port)
    finally:
        http.shutdown()
        thread.join()
        http.server_close()


def candidate(directory, name, content=b"#!/bin/sh\necho candidate\n", member=None):
    path = directory / name
    if name.endswith(".zip"):
        with zipfile.ZipFile(path, "w") as archive:
            archive.writestr(member or "biomcp.exe", content)
    else:
        with tarfile.open(path, "w:gz") as archive:
            entry = tarfile.TarInfo(member or "biomcp")
            entry.size, entry.mode = len(content), 0o755
            archive.addfile(entry, io.BytesIO(content))
    checksum = hashlib.sha256(path.read_bytes()).hexdigest()
    path.with_name(name + ".sha256").write_text(checksum + "  " + name + "\n")
    return path, checksum


class Installer(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.bin = self.root / "bin"
        self.bin.mkdir()
        self.destination = self.root / "installed"
        self.home = self.root / "home"
        self.home.mkdir()
        self.environment = {"PATH": str(self.bin) + os.pathsep + os.defpath, "HOME": str(self.home), "BIOMCP_INSTALL_DIR": str(self.destination)}

    def platform(self, system="Linux", machine="x86_64"):
        uname = self.bin / "uname"
        uname.write_text("#!/bin/sh\ncase \"$1\" in -s) echo '" + system + "';; -m) echo '" + machine + "';; esac\n")
        uname.chmod(0o755)

    def install(self, *args):
        return subprocess.run(["/bin/bash", str(SCRIPT), *args], env=self.environment, cwd=self.home, capture_output=True, text=True, timeout=20)

    def test_public_platform_archives_and_version_interface(self):
        platforms = [("Linux", "x86_64", "linux-x86_64", ".tar.gz"), ("Linux", "aarch64", "linux-arm64", ".tar.gz"), ("Darwin", "arm64", "darwin-arm64", ".tar.gz"), ("Darwin", "x86_64", "darwin-x86_64", ".tar.gz"), ("MSYS_NT-10.0", "x86_64", "windows-x86_64", ".zip"), ("MINGW64_NT-10.0", "amd64", "windows-x86_64", ".zip"), ("CYGWIN_NT-10.0", "x86_64", "windows-x86_64", ".zip")]
        for system, machine, platform, suffix in platforms:
            with self.subTest(system=system, machine=machine):
                self.platform(system, machine)
                release = self.root / "releases/download/v1.0.0"
                release.mkdir(parents=True, exist_ok=True)
                candidate(release, "biomcp-" + platform + suffix)
                with server(self.root) as base:
                    self.environment["BIOMCP_RELEASE_BASE_URL"] = base + "/releases"
                    result = self.install("--version", "1.0.0")
                self.assertEqual(result.returncode, 0, result.stderr)
                binary = self.destination / ("biomcp.exe" if suffix == ".zip" else "biomcp")
                self.assertEqual(binary.read_bytes(), b"#!/bin/sh\necho candidate\n")
                binary.unlink()

    def test_latest_environment_and_local_tar_share_checksum_boundary(self):
        self.platform()
        release = self.root / "releases/latest/download"
        release.mkdir(parents=True)
        archive, checksum = candidate(release, "biomcp-linux-x86_64.tar.gz")
        with server(self.root) as base:
            self.environment["BIOMCP_RELEASE_BASE_URL"] = base + "/releases"
            self.assertEqual(self.install().returncode, 0)
        self.assertEqual(self.install(str(archive), checksum, str(self.root / "offline")).returncode, 0)
        self.environment["BIOMCP_VERSION"] = "v1.0.0"
        archive.with_name(archive.name + ".sha256").write_text("0" * 64 + "\n")
        self.destination.joinpath("biomcp").unlink()
        self.assertNotEqual(self.install(str(archive), "0" * 64, str(self.destination)).returncode, 0)
        self.assertFalse(self.destination.joinpath("biomcp").exists())

    def test_unverified_or_unsafe_download_never_installs(self):
        self.platform()
        release = self.root / "releases/download/v1.0.0"
        release.mkdir(parents=True)
        archive, checksum = candidate(release, "biomcp-linux-x86_64.tar.gz")
        with server(self.root) as base:
            self.environment.update(BIOMCP_RELEASE_BASE_URL=base + "/releases", BIOMCP_VERSION="v1.0.0")
            for bad in ["0" * 64, checksum + "  other.tar.gz", checksum + "\n" + checksum]:
                archive.with_name(archive.name + ".sha256").write_text(bad + "\n")
                result = self.install()
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("checksum", result.stderr.lower())
                self.assertFalse(self.destination.exists())
            archive.with_name(archive.name + ".sha256").unlink()
            self.assertNotEqual(self.install().returncode, 0)
            for member in ["../escape", "bin/biomcp"]:
                candidate(release, archive.name, member=member)
                self.assertNotEqual(self.install().returncode, 0)
                self.assertFalse(self.destination.exists())
                self.assertFalse(self.root.joinpath("escape").exists())

    def test_unknown_platform_names_failure_and_help_needs_no_download(self):
        for system, machine, message in [("Plan9", "x86_64", "Plan9"), ("Linux", "mips", "mips"), ("MSYS_NT", "arm64", "windows/arm64")]:
            self.platform(system, machine)
            result = self.install()
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(message, result.stderr)
            self.assertFalse(self.destination.exists())
        self.assertEqual(self.install("--help").returncode, 0)


if __name__ == "__main__":
    unittest.main()
