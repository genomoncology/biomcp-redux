"""Release staging contracts exercised through the operator command."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
import zipfile


class Staging(unittest.TestCase):
    def test_environment_requires_an_independent_operator(self):
        for rules, accepted in [([], False), ([{"type": "required_reviewers", "reviewers": []}], False),
                                ([{"type": "required_reviewers", "reviewers": [{"type": "Team"}], "prevent_self_review": False}], False),
                                ([{"type": "required_reviewers", "reviewers": [{"type": "Team"}], "prevent_self_review": True}], True)]:
            with self.subTest(rules=rules), tempfile.TemporaryDirectory() as directory:
                settings = Path(directory) / "environment.json"
                settings.write_text(json.dumps({"protection_rules": rules}))
                result = subprocess.run([sys.executable, "release/workflow.py", "protection", str(settings)], capture_output=True, text=True)
                self.assertEqual(result.returncode == 0, accepted, result.stderr)

    def prepare(self, root):
        artifacts = root / "artifacts"
        platforms = ["manylinux_2_28_x86_64", "manylinux_2_28_aarch64",
                     "macosx_11_0_x86_64", "macosx_11_0_arm64", "win_amd64"]
        archives = ["linux-x86_64.tar.gz", "linux-arm64.tar.gz",
                    "darwin-x86_64.tar.gz", "darwin-arm64.tar.gz", "windows-x86_64.zip"]
        for platform, archive in zip(platforms, archives):
            directory = artifacts / platform
            directory.mkdir(parents=True)
            record = {"version": "1.0.0", "revision": "a" * 40,
                      "platform": platform, "distribution": "biomcp-cli"}
            (directory / "release.json").write_text(json.dumps(record))
            name = "biomcp-" + archive
            (directory / name).write_bytes(platform.encode())
            digest = hashlib.sha256(platform.encode()).hexdigest()
            (directory / (name + ".sha256")).write_text(digest + "  " + name + "\n")
            (directory / ("biomcp_cli-1.0.0-py3-none-" + platform + ".whl")).touch()
            (directory / ("biomcp-" + platform + ".mcpb")).touch()
            registry = {"name": "io.github.genomoncology/biomcp", "version": "1.0.0"}
            (directory / "server.json").write_text(json.dumps(registry))
            if platform.startswith("manylinux"):
                arch = "amd64" if platform.endswith("x86_64") else "arm64"
                (directory / ("biomcp-container-" + arch + ".tar")).touch()
        plugin = root / "plugin"
        (plugin / ".claude-plugin").mkdir(parents=True)
        (plugin / ".claude-plugin/plugin.json").write_text('{"name":"biomcp","version":"1.0.0","skills":["./skills/"]}')
        (plugin / ".claude-plugin/marketplace.json").write_text('{"name":"biomcp","plugins":[{"name":"biomcp","source":"./","version":"1.0.0","mcpServers":{"biomcp":{"command":"biomcp","args":["serve"]}}}]}')
        (plugin / "skills").mkdir()
        (plugin / "skills/SKILL.md").write_text("BioMCP guide\n")
        return artifacts, plugin

    def stage(self, root, artifacts, plugin):
        return subprocess.run([sys.executable, "release/workflow.py", "stage",
                               "--artifacts", str(artifacts), "--plugin", str(plugin),
                               "--version", "1.0.0", "--revision", "a" * 40,
                               "--output", str(root / "staged")], capture_output=True, text=True)

    def test_stage_preserves_public_channels_and_native_payloads(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifacts, plugin = self.prepare(root)
            result = self.stage(root, artifacts, plugin)
            self.assertEqual(result.returncode, 0, result.stderr)
            staged = root / "staged"
            formula = (staged / "biomcp.rb").read_text()
            for arch in ["arm64", "x86_64"]:
                self.assertIn("https://github.com/genomoncology/biomcp/releases/download/v1.0.0/biomcp-darwin-" + arch + ".tar.gz", formula)
                digest = hashlib.sha256(("macosx_11_0_" + arch).encode()).hexdigest()
                self.assertIn(digest, formula)
            self.assertNotIn("file:", formula)
            self.assertEqual(len(list(staged.glob("*.whl"))), 5)
            self.assertEqual(len(list(staged.glob("*.mcpb"))), 3)
            self.assertEqual(len(list(staged.glob("biomcp-container-*.tar"))), 2)
            self.assertEqual((staged / "install.sh").read_bytes(), Path("release/install.sh").read_bytes())
            with zipfile.ZipFile(staged / "biomcp-plugin.zip") as archive:
                self.assertIn(".claude-plugin/marketplace.json", archive.namelist())
                self.assertEqual(archive.read("skills/SKILL.md"), b"BioMCP guide\n")
                self.assertEqual(json.loads(archive.read(".claude-plugin/plugin.json"))["skills"], ["./skills/"])

    def test_stage_refuses_incomplete_or_mixed_candidates(self):
        for defect in ["missing-platform", "revision", "checksum", "registry", "plugin", "skills"]:
            with self.subTest(defect=defect), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                artifacts, plugin = self.prepare(root)
                linux = artifacts / "manylinux_2_28_x86_64"
                if defect == "missing-platform":
                    (linux / "release.json").unlink()
                elif defect == "revision":
                    path = linux / "release.json"
                    path.write_text(path.read_text().replace("a" * 40, "b" * 40))
                elif defect == "checksum":
                    (linux / "biomcp-linux-x86_64.tar.gz").write_bytes(b"corrupt")
                elif defect == "registry":
                    (linux / "server.json").write_text('{"version":"0.9.3"}')
                elif defect == "plugin":
                    (plugin / ".claude-plugin/plugin.json").write_text('{"name":"other"}')
                else:
                    (plugin / ".claude-plugin/plugin.json").write_text('{"name":"biomcp"}')
                self.assertNotEqual(self.stage(root, artifacts, plugin).returncode, 0)


if __name__ == "__main__":
    unittest.main()
