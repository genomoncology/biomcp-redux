"""Installed commands own canonical temporary space, including on failure."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


class InstalledTemporarySpace(unittest.TestCase):
    @unittest.skipUnless(sys.platform == 'darwin' and shutil.which('brew'), 'staged formula installation needs macOS Homebrew')
    def test_staged_formula_installs_the_selected_candidate(self):
        import hashlib
        from release.artifacts import binary_archive, platform_tag
        from release.workflow import formula
        binary = Path(os.environ['BIOMCP_INSTALLED_BINARY']).resolve()
        verbose = subprocess.check_output([str(binary), 'version', '--verbose'], text=True)
        revision = next(line.removeprefix('Commit: ') for line in verbose.splitlines() if line.startswith('Commit: '))
        version = verbose.splitlines()[0].removeprefix('biomcp ')
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            staged = root / 'staged'
            staged.mkdir()
            for arch in ('arm64', 'x86_64'):
                binary_archive(staged, binary, 'macosx_11_0_' + arch)
            (staged / 'candidate.json').write_text(json.dumps({'version': version, 'revision': revision}))
            formula(staged, version)
            record = {'version': version, 'revision': revision, 'platform': platform_tag(False),
                      'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest()}
            (root / 'release.json').write_text(json.dumps(record))
            command = [sys.executable, 'release/gate.py', '--artifacts', str(root),
                       '--scratch', str(root / 'check'), '--staged-formula', str(staged / 'biomcp.rb')]
            result = subprocess.run(command, capture_output=True, text=True, timeout=300)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            rows = json.loads((root / 'check/report.json').read_text())
            self.assertEqual([row['command'] for row in rows], [['version'], ['version', '--verbose'], ['health', '--offline']])
            self.assertTrue(all(row['artifact'] == 'staged-homebrew-formula' for row in rows))
            self.assertTrue(all(row['revision'] == revision for row in rows))

    def test_installed_alphagenome_uses_selected_binary_without_checkout(self):
        from release.smoke import environment
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            binary = root / ("biomcp.exe" if os.name == "nt" else "biomcp")
            runner = root / ("case-runner.exe" if os.name == "nt" else "case-runner")
            shutil.copy2(os.environ["BIOMCP_INSTALLED_BINARY"], binary)
            shutil.copy2(os.environ["BIOMCP_CASE_RUNNER"], runner)
            env = environment(root)
            env["BIOMCP_TEST_EXECUTABLE"] = str(binary)
            command = [str(runner), "--installed-alphagenome-proof"]
            result = subprocess.run(command, env=env, cwd=root, capture_output=True, text=True, timeout=120)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn("variant/alphagenome-native-groups-local-tls", result.stdout)
            self.assertIn("1 passed", result.stdout)
            invalid = subprocess.run(command + ["unexpected"], env=env, cwd=root,
                                     capture_output=True, text=True, timeout=120)
            self.assertNotEqual(invalid.returncode, 0)
            if os.name != "nt":
                # Keep the actual CLI but break only its transport through a closed local proxy.
                config = root / "broken-network.json"
                config.write_text(json.dumps({"proxy": "http://127.0.0.1:0",
                                               "keys": {"alphagenome_api_key": "synthetic"}}))
                broken = root / "broken-transport"
                broken.write_text(f"#!{sys.executable}\nimport os, sys\nos.execv({str(binary)!r}, [{str(binary)!r}] + sys.argv[1:] + ['--config', {str(config)!r}])\n")
                broken.chmod(0o755)
                env["BIOMCP_TEST_EXECUTABLE"] = str(broken)
                result = subprocess.run(command, env=env, cwd=root, capture_output=True, text=True, timeout=120)
                self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
                self.assertIn("variant/alphagenome-native-groups-local-tls", result.stdout)
                self.assertIn("1 failed", result.stdout)

    def test_isolated_installed_commands_and_cleanup(self):
        binary = Path(os.environ["BIOMCP_INSTALLED_BINARY"]).resolve()
        runner = Path(os.environ["BIOMCP_CASE_RUNNER"]).resolve()
        version = os.environ["BIOMCP_INSTALLED_VERSION"]
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            installed = root / "installed"
            installed.mkdir()
            shutil.copy2(binary, installed / binary.name)
            shutil.copy2(runner, installed / "case-runner")
            fixtures = root / "fixtures"
            (fixtures / "cases").mkdir(parents=True)
            shutil.copy("tests/cases/gene/kras.yaml", fixtures / "cases/kras.yaml")
            for name in ["gene/main", "article/search/synthetic-small"]:
                shutil.copytree(Path("tests/replies") / name, fixtures / "replies" / name)
            for path in fixtures.rglob("*"):
                if path.is_file():
                    path.chmod(0o444)
            parent = root / "job-tmp"
            parent.mkdir()
            sentinel = parent / "keep"
            sentinel.write_text("job-owned")
            script = """
import json, pathlib, sys
from release.smoke import smoke
created = []
sys.addaudithook(lambda event, args: created.append(str(pathlib.Path(args[0]).resolve())) if event == 'tempfile.mkdtemp' else None)
try:
    smoke(*sys.argv[1:])
finally:
    print(json.dumps({'temporary_roots': created}))
"""
            for configured, fail in [(False, False), (True, False), (False, True), (True, True)]:
                with self.subTest(configured=configured, fail=fail):
                    env = {key: value for key, value in os.environ.items() if key not in {"TMPDIR", "TMP", "TEMP"}}
                    if configured:
                        env["TMPDIR"] = str(parent)
                    selected = installed / ("absent" if fail else binary.name)
                    result = subprocess.run([sys.executable, "-c", script, str(selected), str(installed / "case-runner"), str(fixtures), version], env=env, capture_output=True, text=True, timeout=300)
                    self.assertEqual(result.returncode == 0, not fail, result.stdout + result.stderr)
                    roots = json.loads(result.stdout.splitlines()[-1])["temporary_roots"]
                    self.assertEqual(len(roots), 1)
                    self.assertFalse(Path(roots[0]).exists(), "smoke leaked its temporary directory")
                    if configured:
                        self.assertEqual(Path(roots[0]).parent, parent)
                    self.assertEqual(list(parent.iterdir()), [sentinel])
                    self.assertEqual(sentinel.read_text(), "job-owned")


if __name__ == "__main__":
    unittest.main()
