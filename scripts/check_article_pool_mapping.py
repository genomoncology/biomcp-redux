"""Exercise conditional pool demotion with an existing native binary; never build."""
import hashlib
import json
import os
from pathlib import Path
import shlex
import shutil
import sqlite3
import subprocess
import sys
import tempfile


def main():
    host = os.environ.get("ARTICLE_POOL_REVIEW_HOST")
    binary = os.environ["ARTICLE_POOL_BINARY"] if len(sys.argv) == 1 else sys.argv[1]
    source = os.environ["ARTICLE_POOL_SOURCE"] if len(sys.argv) == 1 else sys.argv[2]
    checkout = os.environ["ARTICLE_POOL_CHECKOUT"] if len(sys.argv) == 1 else sys.argv[3]
    if host:
        command = shlex.join(["python3", "-", binary, source, checkout])
        subprocess.run(["ssh", host, command], input=Path(__file__).read_bytes(), check=True)
        return
    checkout = Path(checkout)
    actual = subprocess.check_output(["git", "-C", str(checkout), "rev-parse", "HEAD"], text=True).strip()
    if actual != source:
        raise ValueError(f"artifact checkout is {actual}, expected {source}")
    digest = hashlib.sha256(Path(binary).read_bytes()).hexdigest()
    with tempfile.TemporaryDirectory(prefix="article-pool-mapping-") as temporary:
        root = Path(temporary)
        executable = root / "biomcp"
        shutil.copy2(binary, executable)
        replay = root / "replay"
        shutil.copytree(checkout / "tests/replies/article/fold/failed-pubmed", replay)
        for file in replay.rglob("*.json"):
            recording = json.loads(file.read_text())
            for exchange in recording["exchanges"]:
                exchange["status"] = 200
            file.write_text(json.dumps(recording))
        # Match the shared Rust launcher: child-only home/config/cache and no credentials.
        environment = {}
        for name, directory in [("HOME", "home"), ("USERPROFILE", "home"),
                                ("APPDATA", "config"), ("LOCALAPPDATA", "cache"),
                                ("XDG_CONFIG_HOME", "config"), ("XDG_CACHE_HOME", "cache")]:
            (root / directory).mkdir(exist_ok=True)
            environment[name] = str(root / directory)
        command = [str(executable), "search", "article", "AKT1", "--author", "Carpten",
                   "--source", "pubmed,europepmc", "--replay", str(replay),
                   "--case-cache", str(root / "case")]

        def invoke(*extra):
            reply = subprocess.run(command + list(extra), cwd=root, env=environment,
                                   capture_output=True, text=True, check=True)
            if reply.stderr:
                raise ValueError(reply.stderr)
            return json.loads(reply.stdout)

        first = invoke()
        if first["status"] != "complete":
            raise ValueError("initial answer incomplete")
        older = first["page"]["pool"]["id"]
        newer_reply = invoke("--fresh")
        newer = newer_reply["page"]["pool"]["id"]
        if newer_reply["status"] != "complete" or older == newer:
            raise ValueError("fresh answer did not publish another complete pool")
        databases = list((root / "case").rglob("*.sqlite"))
        if len(databases) != 1:
            raise ValueError(f"expected one case database, found {databases}")
        with sqlite3.connect(databases[0]) as database:
            def current():
                return database.execute("SELECT pool FROM pool_current").fetchall()

            def fail_pool(pool):
                encoded, = database.execute("SELECT outcomes FROM pool WHERE id=?", (pool,)).fetchone()
                outcomes = json.loads(encoded)
                sources = outcomes if isinstance(outcomes, list) else outcomes["sources"]
                source_outcome = next(item for item in sources if item["source"] == "europepmc")
                source_outcome.update(state="failed", reason="unavailable")
                database.execute("UPDATE pool SET outcomes=? WHERE id=?", (json.dumps(outcomes), pool))
                database.commit()

            fail_pool(older)
            retained = invoke("--pool", older)
            if retained["status"] != "partial" or retained["result"] != first["result"]:
                raise ValueError("older partial pool lost its stable explicit answer")
            if current() != [(newer,)]:
                raise ValueError("older pool demotion erased the newer writer's mapping")
            fail_pool(newer)
            if invoke("--pool", newer)["status"] != "partial" or current():
                raise ValueError("current partial pool retained its reusable mapping")
            recovered = invoke()
            recovered_id = recovered["page"]["pool"]["id"]
            if recovered["status"] != "complete" or recovered_id in (older, newer):
                raise ValueError("new search did not recover independently")
            if current() != [(recovered_id,)]:
                raise ValueError("recovered whole answer was not reusable")
    if hashlib.sha256(Path(binary).read_bytes()).hexdigest() != digest:
        raise ValueError("original artifact changed during review")
    print(json.dumps({"source": source, "binary_sha256": digest, "mapping_check": "PASS"}))


if __name__ == "__main__":
    main()
