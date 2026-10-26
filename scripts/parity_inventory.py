"""Walk released BioMCP help and print its leaf and own-option inventory.

Run against the public v0.9.3 executable: python3 scripts/parity_inventory.py /path/to/biomcp
The executable is invoked only with --help. Aliases, globals, positionals and
parent options are excluded. Hidden options below come from the released source.
"""

import argparse
import json
import os
import re
import subprocess
import tempfile
from pathlib import Path

REVISION = "5641de7d7b79b53872f8599dbb4dba22e611dcaf"
GLOBALS = {"--help", "--version", "--json", "--no-cache", "--disable-sources"}
# Clap omits aliases from Commands; serve is a separately declared MCP alias.
ALIASES = {("serve",)}


def generate(binary):
    leaves = {}
    with tempfile.TemporaryDirectory() as directory:
        env = {
            "PATH": os.environ.get("PATH", ""),
            "HOME": directory,
            "XDG_CONFIG_HOME": directory,
            "XDG_CACHE_HOME": directory,
            "NO_COLOR": "1",
            "TERM": "dumb",
            "COLUMNS": "160",
        }

        def walk(command):
            text = subprocess.run(
                [str(binary), *command, "--help"],
                env=env,
                capture_output=True,
                text=True,
                check=True,
            ).stdout
            section = None
            children = []
            options = set()
            for line in text.splitlines():
                if line and not line[0].isspace() and line.endswith(":"):
                    section = line[:-1]
                elif section == "Commands":
                    match = re.match(r"  ([a-z][a-z0-9-]*)\s", line)
                    if match and match[1] != "help":
                        children.append(match[1])
                elif line.startswith("  "):
                    # Only option declarations, never prose examples or defaults.
                    declaration = re.match(
                        r"  (?:-[A-Za-z], )?(--[a-z][a-z0-9-]*)\b", line
                    )
                    if declaration and declaration[1] not in GLOBALS:
                        options.add(declaration[1])
            if children:
                for child in children:
                    if (*command, child) not in ALIASES:
                        walk((*command, child))
            elif command:
                leaves[" ".join(command)] = {"options": sorted(options)}

        walk(())
    # ChartArgs is flattened only in these released leaf payloads. Its short-help
    # omissions and hidden MCP switch must retain their original option slots.
    return {
        "release": "0.9.3",
        "revision": REVISION,
        "leaves": dict(sorted(leaves.items())),
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("binary", type=Path)
    args = parser.parse_args()
    print(json.dumps(generate(args.binary.resolve()), indent=2))
