"""Feature install-script: exercise the actual installer with isolated downloads."""
from release.artifacts import binary_archive
from release.local_server import server
import hashlib
import io
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile
import unittest
import zipfile

SCRIPT = Path(__file__).with_name("install.sh").resolve()


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
                payload = self.root / "candidate"
                payload.write_bytes(b"#!/bin/sh\necho candidate\n")
                payload.chmod(0o755)
                if platform.startswith("windows"):
                    tag = "win_amd64"
                elif platform.startswith("darwin"):
                    tag = "macosx_11_0_" + platform.removeprefix("darwin-")
                else:
                    tag = "manylinux_2_28_" + ("aarch64" if platform.endswith("arm64") else "x86_64")
                archive = binary_archive(release, payload, tag)
                self.assertEqual(archive.name, "biomcp-" + platform + suffix)
                with server(self.root) as base:
                    self.environment["BIOMCP_RELEASE_BASE_URL"] = base + "/releases"
                    result = self.install("--version", "1.0.0")
                self.assertEqual(result.returncode, 0, result.stderr)
                binary = self.destination / ("biomcp.exe" if suffix == ".zip" else "biomcp")
                self.assertEqual(binary.read_bytes(), b"#!/bin/sh\necho candidate\n")
                self.assertTrue(os.access(binary, os.X_OK))
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
        unnamed = self.root / "local-archive"
        unnamed.write_bytes(archive.read_bytes())
        self.assertEqual(self.install(str(unnamed), checksum, str(self.root / "unnamed")).returncode, 0)
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

    def test_zip_rejects_extra_paths_and_preserves_existing_install(self):
        self.platform("MSYS_NT-10.0")
        release = self.root / "releases/download/v1.0.0"
        release.mkdir(parents=True)
        self.destination.mkdir()
        installed = self.destination / "biomcp.exe"
        installed.write_bytes(b"previous executable")
        with server(self.root) as base:
            self.environment.update(BIOMCP_RELEASE_BASE_URL=base + "/releases", BIOMCP_VERSION="v1.0.0")
            archive, _ = candidate(release, "biomcp-windows-x86_64.zip", member="../escape.exe")
            result = self.install()
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("archive contents", result.stderr)
            self.assertEqual(installed.read_bytes(), b"previous executable")
            self.assertFalse(self.root.joinpath("escape.exe").exists())
            archive.write_bytes(b"truncated archive")
            archive.with_name(archive.name + ".sha256").write_text(hashlib.sha256(archive.read_bytes()).hexdigest())
            self.assertNotEqual(self.install().returncode, 0)
            self.assertEqual(installed.read_bytes(), b"previous executable")

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
