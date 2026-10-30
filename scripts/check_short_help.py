"""Supplemental output budgets for the compiled command-help surface."""
import json
import os
import subprocess
import tempfile
from pathlib import Path


def check(binary):
    with tempfile.TemporaryDirectory(prefix="biomcp-help-") as folder:
        env = {key: value for key, value in os.environ.items() if key in ("PATH", "SystemRoot")}
        env.update(HOME=folder, XDG_CONFIG_HOME=folder, XDG_CACHE_HOME=folder, APPDATA=folder)

        def run(words, form="json"):
            result = subprocess.run([binary, "--format", form, *words], env=env, capture_output=True, check=True)
            assert not result.stderr, result.stderr
            return result.stdout

        root = json.loads(run(["help"]))
        for form in ("json", "md"):
            for words in (["help"], ["--help"]):
                output = run(words, form)
                assert len(output) <= 4096, (words, form, len(output))
            for command in root["result"]["commands"]:
                words = ["help", *command["command"]]
                output = run(words, form)
                assert len(output) <= 8192, (words, form, len(output))
                assert b"&#58;" not in output, (words, form)
        drug = json.loads(run(["help", "get", "drug"]))
        assert any(arg["name"] == "--label-section" for arg in drug["result"]["commands"][0]["arguments"])
        markdown = run(["help", "exit"], "md")
        assert b"0:" in markdown and b"&#58;" not in markdown
        print("Root and every command scope meet JSON/Markdown budgets; drug label and colon checks pass.")


if __name__ == "__main__":
    import sys
    check(str(Path(sys.argv[1]).resolve()))
