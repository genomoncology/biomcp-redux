"""Exercise the installed GWAS study and association commands against their source."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import time
import urllib.request

ROOT = Path(__file__).resolve().parents[1]


def main():
    revision = subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip()
    destination = ROOT / "target" / "gwas-studies-evidence" / revision
    destination.mkdir(parents=True, exist_ok=False)
    executable = Path(os.environ["BIOMCP_TEST_EXECUTABLE"]).resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix="installed-gwas-") as folder:
        isolated = Path(folder)
        binary = isolated / "biomcp"
        shutil.copy2(executable, binary)
        env = {"PATH": os.environ["PATH"], "HOME": str(isolated),
               "XDG_CONFIG_HOME": str(isolated / "config"),
               "XDG_CACHE_HOME": str(isolated / "cache")}
        version = subprocess.check_output([str(binary), "version", "--verbose"], env=env, text=True)
        (destination / "version.txt").write_text(version)
        assert "Commit: " + revision + "\n" in version, version
        results = {}
        for name, options in [("studies", ["--studies", "--limit", "50"]),
                              ("associations", ["--limit", "50"]),
                              ("default", [])]:
            output = destination / name
            output.mkdir()
            record = output / "source"
            argv = [str(binary), "search", "gwas", "--pubmed", "17554300", *options,
                    "--record", str(record), "--fresh"]
            started = time.monotonic()
            result = subprocess.run(argv, env=env, cwd=isolated, capture_output=True, text=True)
            elapsed = time.monotonic() - started
            (output / "stdout.json").write_text(result.stdout)
            (output / "stderr.txt").write_text(result.stderr)
            (output / "command.json").write_text(json.dumps({
                "source_revision": revision, "arguments": argv[1:],
                "elapsed_seconds": elapsed, "exit_code": result.returncode,
                "budget": "command default; no override"}, indent=2) + "\n")
            assert result.returncode == 0, (name, result.returncode, result.stderr)
            reply = json.loads(result.stdout)
            assert reply["status"] == "complete", (name, reply)
            recordings = [json.loads(path.read_text()) for path in (record / "gwas").glob("*.json")]
            bodies = [(item["url"], json.loads(item["exchanges"][0]["body"]["value"]))
                      for item in recordings]
            rows = reply["result"]["items"]
            if name == "studies":
                native = next(body["_embedded"]["studies"] for url, body in bodies
                              if "/studies?" in url)
                assert len(rows) == len(native) == 7
                assert {row["id"] for row in rows} == {row["accession_id"] for row in native}
                for row in rows:
                    source = next(item for item in native if item["accession_id"] == row["id"])
                    assert row["publication"] == str(source["pubmed_id"])
                    assert row["initial_sample_description"] == source["initial_sample_size"]
                    count = next(body["page"]["totalElements"] for url, body in bodies
                                 if "accession_id=" + row["id"] in url)
                    assert row["association_count"] == count
            else:
                source_ids = {"association:" + str(row["association_id"])
                              for _, body in bodies for row in body["_embedded"]["associations"]}
                assert {row["id"] for row in rows} == source_ids
                assert len(rows) == (50 if name == "associations" else 3)
            results[name] = {"elapsed_seconds": elapsed, "returned": len(rows), "exit_code": 0}
        (destination / "results.json").write_text(json.dumps(results, indent=2) + "\n")
    for name, url in [("rest-api-doc.yaml", "https://www.ebi.ac.uk/gwas/rest/api/v2/rest-api-doc.yaml"),
                      ("rest-api.html", "https://www.ebi.ac.uk/gwas/docs/programmatic-access/rest-api/")]:
        with urllib.request.urlopen(url, timeout=30) as response:
            (destination / name).write_bytes(response.read())
    print(json.dumps({"evidence": str(destination), "results": results}, indent=2))


if __name__ == "__main__":
    main()
