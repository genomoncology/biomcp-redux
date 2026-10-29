"""Exercise the offline sweep through its command and the real value bridge."""
import gzip
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
TOOL = ROOT / "scripts/public_dump_sweep.py"
BRIDGE = os.environ.get("PUBLIC_DUMP_VALUES")


@unittest.skipUnless(BRIDGE, "build public_dump_values and set PUBLIC_DUMP_VALUES")
class Sweep(unittest.TestCase):
    def run_sweep(self, kind, content, *, digest=None, limit=None):
        with tempfile.TemporaryDirectory() as temporary:
            folder = Path(temporary)
            data = gzip.compress(content.encode(), mtime=0)
            (folder / "source.gz").write_bytes(data)
            manifest = {"sources": [{
                "kind": kind, "path": "source.gz", "release": "synthetic-edge-sample",
                "url": "https://example.invalid/public-format-fixture",
                "rights": "CC0-1.0", "sha256": digest or hashlib.sha256(data).hexdigest(),
            }]}
            (folder / "manifest.json").write_text(json.dumps(manifest))
            command = [sys.executable, str(TOOL), str(folder / "manifest.json"),
                       "--values", BRIDGE]
            if limit is not None:
                command += ["--max-records", str(limit)]
            result = subprocess.run(command, text=True, capture_output=True, check=False)
            return result, json.loads(result.stdout) if result.stdout else None

    def test_counts_each_row_and_preserves_compound_profile(self):
        text = ("molecular_profile_id\tmolecular_profile\tvariant_ids\n"
                "9\tBRAF V600E AND NOT KRAS G12D\t12,13\n")
        result, report = self.run_sweep("civic_profiles", text)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(report["sources"][0]["records"], 1)
        self.assertEqual(report["types"]["profile"]["parsed"], 1)
        self.assertEqual(report["types"]["identifier"]["parsed"], 2)

    def test_refusals_do_not_disappear_and_exit_nonzero(self):
        fixture = ROOT / "tests/fixtures/public-dump-sweep/clinvar-edge.tsv"
        result, report = self.run_sweep("clinvar_summary", fixture.read_text())
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertEqual(report["sources"][0]["records"], 2)
        self.assertEqual(report["types"]["clinvar_variation"]["parsed"], 2)
        self.assertEqual(report["types"]["hgvs"]["refused"], 1)
        self.assertEqual(report["refusals"][0]["record"], 2)
        self.assertTrue(report["refusals"][0]["reason"])
        self.assertEqual(report["spelling_mismatches"], 0)

    def test_hash_mismatch_stops_before_parsing(self):
        result, report = self.run_sweep("clinvar_summary", "irrelevant", digest="0" * 64)
        self.assertEqual(result.returncode, 2)
        self.assertIsNone(report)
        self.assertIn("checksum_mismatch", result.stderr)

    def test_bundle_streams_records_and_reports_unknown_sections(self):
        bundle = {"variation": {"clinvar:12": {"id": "clinvar:12", "name": "BRAF"}},
                  "allele": {"ga4gh:VA." + "a" * 32: {
                      "expressions": [{"syntax": "hgvs.c", "value": "NM_004333.6:c.1799T>A"}]}},
                  "futureSection": {"unknown:1": {"value": "do not silently skip"}}}
        result, report = self.run_sweep("clinvar_gks", json.dumps(bundle))
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertEqual(report["sources"][0]["records"], 3)
        self.assertEqual(report["types"]["variant"]["parsed"], 1)
        self.assertEqual(report["types"]["clinvar_variation"]["parsed"], 1)
        self.assertEqual(report["types"]["hgvs"]["parsed"], 1)
        self.assertEqual(report["unmapped_sections"], {"futureSection": 1})

    def test_sample_limit_marks_run_incomplete(self):
        text = "variant_id\tvariant\tgene\thgvs_descriptions\n1\tV600E\tBRAF\t\n2\tG12D\tKRAS\t\n"
        result, report = self.run_sweep("civic_variants", text, limit=1)
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertFalse(report["complete"])
        self.assertEqual(report["gene_counts"], {"BRAF": 1})


if __name__ == "__main__":
    unittest.main()
