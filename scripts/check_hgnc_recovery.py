"""Build once and exercise HGNC recovery through copied, installed executables."""
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))
from release.artifacts import cargo


def main():
    os.chdir(ROOT)
    product = Path(cargo("dev")["executable"])
    runner = Path(cargo("dev", tests="tools")["executable"])
    with tempfile.TemporaryDirectory(prefix="hgnc-recovery-") as directory:
        installed = Path(directory)
        binary = installed / product.name
        tool = installed / runner.name
        shutil.copy2(product, binary)
        shutil.copy2(runner, tool)
        shutil.copytree(ROOT / "tests", installed / "fixtures")
        environment = {**os.environ, "BIOMCP_TEST_EXECUTABLE": str(binary),
                       "BIOMCP_TEST_FIXTURES": str(installed / "fixtures")}
        names = ["hgnc-first-snapshot-recovers"] if len(sys.argv) == 1 else sys.argv[1:]
        for name in names:
            subprocess.run([str(tool), "--proofs", "cache-resilience/" + name,
                            "--exact", "--test-threads", "1"], cwd=installed,
                           env=environment, check=True)


if __name__ == "__main__":
    main()
