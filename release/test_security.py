"""Security boundaries exercised through the real scanner and audit commands."""
import io
import json
import os
from pathlib import Path
from release.smoke import environment
import secrets
import subprocess
import sys
import tarfile
import tempfile
import unittest
import zipfile

ROOT = Path(__file__).resolve().parents[1]


class Screening(unittest.TestCase):
    def test_scanner_rejects_historical_and_nested_artifact_secrets_without_disclosure(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            # Construct only in owned scratch; no token or secret fixture enters Git.
            token = "ghp_" + "".join(secrets.choice("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789") for _ in range(36))
            repo = root / "repo"
            repo.mkdir()
            env = {"PATH": os.defpath, "HOME": str(root), "GIT_CONFIG_NOSYSTEM": "1"}
            def git(*args):
                subprocess.run(["git", "-C", str(repo), *args], env=env, check=True, capture_output=True)
            git("init", "--quiet")
            git("config", "user.name", "Fixture")
            git("config", "user.email", "fixture@example.invalid")
            (repo / "credential.txt").write_text("access_token = " + token + "\n")
            git("add", ".")
            git("commit", "--quiet", "-m", "Add fixture")
            git("rm", "credential.txt")
            git("commit", "--quiet", "-m", "Remove fixture")
            artifacts = root / "artifacts"
            artifacts.mkdir()
            inner = root / "payload.whl"
            with zipfile.ZipFile(inner, "w") as archive:
                archive.writestr("credential.txt", "access_token = " + token + "\n")
            with tarfile.open(artifacts / "container.tar", "w") as archive:
                archive.add(inner, arcname="layer/payload.whl")
            for mode, target in [("history", repo), ("artifacts", artifacts)]:
                result = subprocess.run([sys.executable, str(ROOT / "release/secrets.py"), mode, str(target)],
                                        cwd=ROOT, env=env, capture_output=True, text=True, timeout=180)
                self.assertEqual(result.returncode, 1, "planted fixture must fail secret screening")
                self.assertTrue(token not in result.stdout + result.stderr, "scanner disclosed matched text")
                self.assertTrue("access_token =" not in result.stdout + result.stderr, "scanner disclosed a source line")
                findings = [json.loads(line) for line in result.stdout.splitlines()]
                self.assertTrue(findings)
                self.assertTrue(all(set(row) == {"file", "rule"} for row in findings))
                self.assertIn("github-pat", {row["rule"] for row in findings})

    def test_scanner_covers_binary_members_and_merge_only_history(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            token = "ghp_" + secrets.token_hex(18)
            payload = b"MZ" + bytes(128) + ("access_token = " + token + "\n").encode()
            artifacts = root / "artifacts"
            artifacts.mkdir()
            env = {**environment(root), "PATH": os.defpath, "GIT_CONFIG_NOSYSTEM": "1"}
            def screen(mode, target):
                result = subprocess.run([sys.executable, str(ROOT / "release/secrets.py"), mode, str(target)],
                                        cwd=ROOT, env=env, capture_output=True, text=True)
                self.assertEqual(result.returncode, 1, "planted binary or merge fixture must fail screening")
                self.assertNotIn(token, result.stdout + result.stderr)
                self.assertTrue(all(set(json.loads(line)) == {"file", "rule"} for line in result.stdout.splitlines()))
                return result.stdout
            for name in ["biomcp.exe", "package.dist-info/release.json"]:
                with zipfile.ZipFile(artifacts / "candidate.whl", "w") as archive:
                    archive.writestr(name, payload)
                self.assertIn(name, screen("artifacts", artifacts))
            repo = root / "repo"
            repo.mkdir()
            def git(*args, data=None):
                return subprocess.run(["git", "-C", str(repo), *args], input=data, env=env,
                                      check=True, capture_output=True).stdout.decode().strip()
            git("init", "--quiet")
            git("config", "user.name", "Fixture")
            git("config", "user.email", "fixture@example.invalid")
            git("commit", "--allow-empty", "--quiet", "-m", "Initial fixture")
            first = git("rev-parse", "HEAD")
            git("commit", "--allow-empty", "--quiet", "-m", "Second fixture")
            second = git("rev-parse", "HEAD")
            (repo / "merge-only.txt").write_text("access_token = " + token + "\n")
            (repo / "historical-opaque.bin").write_bytes(b"PK\x03\x04synthetic prefix\n" + ("access_token = " + token + "\n").encode())
            with zipfile.ZipFile(repo / "historical-valid.bin", "w", zipfile.ZIP_DEFLATED) as archive:
                archive.writestr("credential.exe", payload)
            buffer = io.BytesIO()
            with zipfile.ZipFile(buffer, "w") as archive:
                archive.writestr("credential.exe", payload)
            malformed = bytearray(buffer.getvalue())
            malformed[30] ^= 1  # Local and central filenames intentionally disagree.
            (repo / "historical-structure.bin").write_bytes(malformed)
            git("add", ".")
            tree = git("write-tree")
            merged = git("commit-tree", tree, "-p", first, "-p", second, data=b"Merge fixture\n")
            git("update-ref", "HEAD", merged)
            findings = screen("history", repo)
            self.assertIn("historical-opaque.bin", findings)
            self.assertIn("historical-structure.bin", findings)
            self.assertIn("historical-valid.bin!credential.exe", findings)

    def test_scanner_refuses_unreadable_artifacts_and_never_follows_links(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            artifacts = root / "artifacts"
            artifacts.mkdir()
            env = {**environment(root), "PATH": os.defpath}
            def screen():
                return subprocess.run([sys.executable, str(ROOT / "release/secrets.py"), "artifacts", str(artifacts)],
                                      cwd=ROOT, env=env, capture_output=True, text=True)
            candidate = artifacts / "candidate.zip"
            candidate.write_bytes(b"PK\x03\x04broken archive")
            self.assertNotEqual(screen().returncode, 0, "corrupt archive must fail closed")
            with zipfile.ZipFile(candidate, "w") as archive:
                archive.writestr("../escape.txt", "fixture")
            self.assertNotEqual(screen().returncode, 0, "escaping member must fail closed")
            with zipfile.ZipFile(candidate, "w", zipfile.ZIP_DEFLATED) as archive:
                archive.writestr("hidden/", "directory content")
            self.assertNotEqual(screen().returncode, 0, "directory payload must not be skipped")
            candidate.unlink()
            candidate.symlink_to(root / "absent")
            self.assertNotEqual(screen().returncode, 0, "filesystem links must not be skipped")
            candidate.unlink()
            payload = b"fixture"
            for _ in range(18):
                buffer = io.BytesIO()
                with zipfile.ZipFile(buffer, "w", zipfile.ZIP_DEFLATED) as archive:
                    archive.writestr("nested.zip", payload)
                payload = buffer.getvalue()
            candidate.write_bytes(payload)
            self.assertNotEqual(screen().returncode, 0, "depth limits must fail closed")
            candidate.unlink()
            with tarfile.open(artifacts / "layer.tar", "w") as archive:
                link = tarfile.TarInfo("usr/share/man/fixture.1.gz")
                link.type, link.linkname = tarfile.SYMTYPE, "/outside/fixture"
                archive.addfile(link)
            self.assertEqual(screen().returncode, 0, "container links must be screened without dereferencing")

    def test_audit_denies_an_unknown_git_source_after_successful_resolution(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            dependency = root / "dependency"
            dependency.mkdir()
            (dependency / "src").mkdir()
            (dependency / "src/lib.rs").write_text("pub fn fixture() {}\n")
            (dependency / "Cargo.toml").write_text('[package]\nname="source-fixture"\nversion="0.1.0"\nedition="2021"\nlicense="MIT"\n')
            env = {**environment(root), **{name: os.environ[name] for name in ["PATH", "CARGO_HOME", "RUSTUP_HOME"] if name in os.environ}, "GIT_CONFIG_COUNT": "1", "GIT_CONFIG_KEY_0": "url." + dependency.as_uri() + ".insteadOf",
                   "GIT_CONFIG_VALUE_0": "https://unknown.example.invalid/dependency",
                   "CARGO_HOME": os.environ.get("CARGO_HOME", str(Path.home() / ".cargo")),
                   "RUSTUP_HOME": os.environ.get("RUSTUP_HOME", str(Path.home() / ".rustup")), "CARGO_NET_GIT_FETCH_WITH_CLI": "true", "GIT_TERMINAL_PROMPT": "0"}
            for args in [["init", "--quiet"], ["add", "."], ["-c", "user.name=Fixture", "-c", "user.email=fixture@example.invalid", "commit", "--quiet", "-m", "Add fixture"]]:
                subprocess.run(["git", "-C", str(dependency), *args], env=env, check=True, capture_output=True)
            project = root / "project"
            project.mkdir()
            (project / "src").mkdir()
            (project / "src/lib.rs").touch()
            (project / "Cargo.toml").write_text('[package]\nname="audit-fixture"\nversion="0.1.0"\nedition="2021"\nlicense="MIT"\n[dependencies]\nsource-fixture={git="https://unknown.example.invalid/dependency"}\n')
            for name in ["Makefile", "deny.toml"]:
                (project / name).write_bytes((ROOT / name).read_bytes())
            resolved = subprocess.run(["cargo", "generate-lockfile"], cwd=project, env=env, capture_output=True, text=True, timeout=120)
            self.assertEqual(resolved.returncode, 0, resolved.stderr)
            result = subprocess.run(["make", "audit"], cwd=project, env=env, capture_output=True, text=True, timeout=120)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("unknown.example.invalid/dependency", result.stdout + result.stderr)
            self.assertIn("source", result.stdout + result.stderr)
            self.assertRegex(result.stdout + result.stderr, "(?i)(unapproved|not explicitly allowed|not allowed|unknown-git)")


if __name__ == "__main__":
    unittest.main()
