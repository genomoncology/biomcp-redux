"""Security boundaries exercised through the real scanner and audit commands."""
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
                self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
                self.assertNotIn(token, result.stdout + result.stderr)
                self.assertNotIn("access_token =", result.stdout + result.stderr)
                findings = [json.loads(line) for line in result.stdout.splitlines()]
                self.assertTrue(findings)
                self.assertTrue(all(set(row) == {"file", "rule"} for row in findings))
                self.assertIn("github-pat", {row["rule"] for row in findings})

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
