"""Build the CLI and print the reply of every 0860 error case, live, in full."""
import json
import os
import subprocess
import sys
import tempfile

RECORD = {"normalize-car-reason", "normalize-mutalyzer-reason", "normalize-vv-reason", "study-cna-profile"}
PMIDS = [str(30000000 + n) for n in range(21)]
CASES = [
    ("normalize-all-no-build", ["variant", "normalize", "--source", "all", "--hgvs", "NM_004333.4:c.1799T>A"]),
    ("normalize-vv-no-build", ["variant", "normalize", "--source", "variantvalidator", "--hgvs", "NM_004333.4:c.1799T>A"]),
    ("normalize-car-reason", ["variant", "normalize", "--source", "car", "--hgvs", "NM_001165963.4:c.4729T>C"]),
    ("normalize-mutalyzer-reason", ["variant", "normalize", "--source", "mutalyzer", "--hgvs", "NM_001165963.4:c.4729T>C"]),
    ("normalize-vv-reason", ["variant", "normalize", "--source", "variantvalidator", "--hgvs", "NM_001165963.4:c.4729T>C", "--build", "GRCh38"]),
    ("normalize-car-control", ["variant", "normalize", "--source", "car", "--hgvs", "NM_001165963.4:c.4762T>C"]),
    ("incidence-no-source", ["get", "disease", "MONDO:0016512", "incidence"]),
    ("incidence-ols", ["get", "disease", "MONDO:0016512", "incidence", "--source", "ols"]),
    ("incidence-seer-mondo", ["get", "disease", "MONDO:0016512", "incidence", "--source", "seer"]),
    ("incidence-seer-site", ["get", "disease", "Brain and other nervous system", "--source", "seer", "incidence", "--section-limit", "3"]),
    ("snapshot-invented", ["get", "phenotype", "HP:0001252", "synonyms", "--snapshot", "kabuki_case_q4"]),
    ("snapshot-memo", ["get", "phenotype", "HP:0001252", "synonyms", "--snapshot", "memo-001"]),
    ("snapshot-hash-absent", ["get", "phenotype", "HP:0001252", "synonyms", "--snapshot", "0" * 64]),
    ("article-21-ids", ["get", "article"] + PMIDS),
    ("variant-query-gene-shorthand", ["search", "variant", "--query", "SCN1A c.4729T>C"]),
    ("study-cna-profile", ["get", "study", "brca_tcga", "mutations", "--gene", "POLQ", "--molecular-profile", "brca_tcga_gistic", "--section-limit", "5"]),
    ("protein-offset-no-pool", ["protein", "structures", "O75417", "--limit", "25", "--offset", "25"]),
    ("trial-get-fields", ["get", "trial", "NCT04613596", "--fields", "nct_id,title,phases"]),
]


def main():
    os.environ.setdefault("RUSTUP_TOOLCHAIN", "1.95.0")
    only = set(sys.argv[1:])
    subprocess.run(["cargo", "build", "--locked", "-q", "-p", "biomcp", "--bin", "biomcp"], check=True)
    binary = os.path.join(os.environ.get("CARGO_TARGET_DIR", "target"), "debug", "biomcp")
    cache = tempfile.mkdtemp(prefix="biomcp-0860-")
    env = dict(os.environ, XDG_CACHE_HOME=cache)
    for label, args in CASES:
        if only and label not in only:
            continue
        record = os.path.join(cache, "record", label)
        extra = ["--record", record] if label in RECORD else []
        if extra:
            os.makedirs(record, exist_ok=True)
        if extra:
            # Also show the reply without recording, so a record-mode failure cannot hide it.
            plain = subprocess.run([binary] + args, capture_output=True, env=env, timeout=300)
            print(f"\n### {label} (no record) exit={plain.returncode}\n" + plain.stdout.decode("utf-8", "replace")[:6000])
        result = subprocess.run([binary] + args + extra, capture_output=True, env=env, timeout=300)
        out = result.stdout.decode("utf-8", "replace")
        print(f"\n### {label} exit={result.returncode}\nCMD: biomcp {' '.join(args)}")
        try:
            print(json.dumps(json.loads(out), indent=1)[:6000])
        except ValueError:
            print("RAW: " + out[:3000])
        if result.stderr.strip():
            print("STDERR: " + result.stderr.decode("utf-8", "replace")[:1000])
        for root, _, files in os.walk(record):
            for name in sorted(files):
                path = os.path.join(root, name)
                print(f"--- RECORDED {os.path.relpath(path, record)}")
                with open(path, encoding="utf-8") as handle:
                    print(handle.read())


if __name__ == "__main__":
    main()
