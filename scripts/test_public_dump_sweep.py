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
    def run_sweep(self, kind, content, *, digest=None, limit=None, observed=None, explanations=None):
        with tempfile.TemporaryDirectory() as temporary:
            folder = Path(temporary)
            data = gzip.compress(content.encode(), mtime=0)
            (folder / "source.gz").write_bytes(data)
            manifest = {"product_revision": "synthetic-product-reply", "sources": [{
                "kind": kind, "path": "source.gz", "release": "synthetic-edge-sample",
                "url": "https://example.invalid/public-format-fixture",
                "rights": "CC0-1.0", "sha256": digest or hashlib.sha256(data).hexdigest(),
                "refusal_explanations": explanations or [],
            }]}
            (folder / "manifest.json").write_text(json.dumps(manifest))
            command = [sys.executable, str(TOOL), str(folder / "manifest.json"),
                       "--values", BRIDGE]
            if observed is None:
                command += ["--values-only"]
            else:
                for row in observed:
                    row.setdefault("source_sha256", [manifest["sources"][0]["sha256"]])
                (folder / "observed.jsonl").write_text("\n".join(json.dumps(row) for row in observed))
                command += ["--observed", str(folder / "observed.jsonl")]
            if limit is not None:
                command += ["--max-records", str(limit)]
            result = subprocess.run(command, text=True, capture_output=True, check=False)
            return result, json.loads(result.stdout) if result.stdout else None

    def test_counts_each_row_and_preserves_compound_profile(self):
        text = ("molecular_profile_id\tname\tvariant_ids\n"
                "9\tBRAF V600E AND NOT KRAS G12D\t12,13\n")
        result, report = self.run_sweep("civic_profiles", text)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(report["sources"][0]["records"], 1)
        self.assertEqual(report["types"]["profile"]["parsed"], 1)
        self.assertEqual(report["types"]["identifier"]["parsed"], 2)

    def test_source_values_cannot_disappear_behind_successful_records(self):
        bundle = {"condition": {"MONDO:0004992": {"id": "MONDO:0004992", "label": "cancer"}},
                  "therapy": {"therapy:1": {"id": "therapy:1", "label": "drug"}}}
        result, report = self.run_sweep("clinvar_gks", json.dumps(bundle))
        self.assertEqual(result.returncode, 0, result.stderr)
        for kind in ("disease", "drug"):
            self.assertEqual(report["types"][kind]["parsed"], 1)
        text = "variant_id\tvariant\tgene\thgvs_descriptions\t5_prime_partner\t3_prime_partner\n1\tFusion\t\t\tBCR\tABL1\n"
        result, report = self.run_sweep("civic_variants", text)
        self.assertEqual(report["types"]["gene"]["parsed"], 2)

    def test_native_profile_members_do_not_become_empty_parsed_profiles(self):
        text = "molecular_profile_id\tname\tvariant_ids\n1\tBCR::ABL1 Fusion\t1\n"
        result, report = self.run_sweep("civic_profiles", text)
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertEqual(report["types"]["profile"]["parsed"], 0)
        self.assertEqual(report["types"]["profile"]["unexplained"], 1)
        self.assertIn("native_member_references", report["refusals"][0]["reason"])

    def test_observation_revision_and_source_hash_must_match_manifest(self):
        text = "variant_id\tvariant\tgene\thgvs_descriptions\n1\tV600E\tBRAF\t\n"
        reply = {"items": [{"answer": {"gene": {"symbol": "BRAF"}},
                             "sections": {"civic": {"data": {"variant_count": 1}}}}]}
        observation = self.observation("civic", reply)
        observation["revision"] = "another-product-revision"
        result, report = self.run_sweep("civic_variants", text, observed=[observation])
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertEqual(report["comparisons"]["failures"]["product_revision_mismatch"], 1)

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

    def test_clingen_evidence_keeps_disease_and_only_explicit_criterion_citations(self):
        record = {"uuid": "synthetic", "condition": {"id": "MONDO:0004992", "label": "cancer"},
                  "outside": "PMID: 999", "evidenceLine": [
                      {"type": "CriterionAssessment", "text": "PMIDs: 123, 456"}]}
        result, report = self.run_sweep("clingen_evidence", json.dumps(record))
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(report["types"]["disease"]["parsed"], 1)
        self.assertEqual(report["types"]["citation"]["parsed"], 2)

    def test_civic_evidence_checks_disease_therapy_citation_and_free_text(self):
        text = ("evidence_id\tmolecular_profile_id\tsource_type\tcitation_id\tdisease\tdoid\ttherapies\tevidence_statement\n"
                "1\t9\tPubMed\t123\tcancer\t162\tdrug one,drug two\t  original source text  \n")
        result, report = self.run_sweep("civic_evidence", text)
        self.assertEqual(result.returncode, 0, result.stderr)
        for kind, count in (("disease", 1), ("drug", 2), ("citation", 1), ("source_text", 1)):
            self.assertEqual(report["types"][kind]["parsed"], count, kind)

    def test_clingen_summary_checks_reference_qualified_notations_and_ids(self):
        text = ("HGNC Gene Symbol\tDisease\tMondo Id\tHGVS Expressions\tAllele Registry Id\tClinVar Variation Id\n"
                "BRAF\tcancer\tMONDO:0004992\tNM_004333.6:c.1799T>A\tCA123456\t13961\n")
        result, report = self.run_sweep("clingen_summary", text)
        self.assertEqual(result.returncode, 0, result.stderr)
        for kind in ("gene", "disease", "hgvs", "ca", "clinvar_variation"):
            self.assertEqual(report["types"][kind]["parsed"], 1, kind)

    def observation(self, source, result, record=None, release="synthetic-edge-sample"):
        return {"source": source, "release": release, "revision": "synthetic-product-reply",
                "retrieved": "2026-10-29", "record": record, "reply": {"result": result}}

    def test_compares_saved_gene_reply_and_reports_missing_and_wrong_counts(self):
        text = "variant_id\tvariant\tgene\thgvs_descriptions\n1\tV600E\tBRAF\t\n2\tG12D\tKRAS\t\n"
        reply = {"items": [{"answer": {"gene": {"symbol": "BRAF"}},
                             "sections": {"civic": {"data": {"variant_count": 9}}}}]}
        result, report = self.run_sweep("civic_variants", text, observed=[self.observation("civic", reply)])
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertEqual(report["comparisons"]["failures"],
                         {"civic_gene_count_mismatch": 1, "civic_gene_unobserved": 1})

    def test_compares_clinvar_ids_through_source_graph_and_rejects_changed_spelling(self):
        allele = "ga4gh:VA." + "a" * 32
        bundle = {"variation": {"clinvar:12": {"members": ["#/allele/" + allele]}},
                  "allele": {allele: {"name": "source allele"}}}
        reply = {"candidates": [{"identity": {"ids": ["clinvar:12", allele]},
                                  "classification": "CAUTION is prose, not an allele ID"}]}
        observation = self.observation("clinvar", reply, "clinvar:12")
        result, report = self.run_sweep("clinvar_gks", json.dumps(bundle), observed=[observation])
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(report["comparisons"]["checked"]["clinvar_ids"], 2)
        reply["candidates"][0]["identity"]["ids"][0] = "clinvar:012"
        result, report = self.run_sweep("clinvar_gks", json.dumps(bundle), observed=[observation])
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertEqual(report["comparisons"]["failures"], {"clinvar_identifier_spelling_mismatch": 1})

    def test_source_date_drift_never_counts_as_equality(self):
        text = "variant_id\tvariant\tgene\thgvs_descriptions\n1\tV600E\tBRAF\t\n"
        observed = [self.observation("civic", {"items": []}, release="different-source-date")]
        result, report = self.run_sweep("civic_variants", text, observed=observed)
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertEqual(report["comparisons"]["failures"]["source_release_mismatch"], 1)

    def test_only_exact_publisher_explanations_allow_refused_values(self):
        text = (ROOT / "tests/fixtures/public-dump-sweep/clinvar-edge.tsv").read_text()
        _, initial = self.run_sweep("clinvar_summary", text)
        refusal = initial["refusals"][0]
        explanation = {key: refusal[key] for key in ("record", "type", "value", "reason")}
        explanation["source_rule"] = "HGVS descriptions require a body: https://hgvs-nomenclature.org/stable/"
        result, report = self.run_sweep("clinvar_summary", text, explanations=[explanation])
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(report["types"]["hgvs"]["explained"], 1)
        explanation["value"] = "another input"
        result, report = self.run_sweep("clinvar_summary", text, explanations=[explanation])
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertEqual(report["types"]["hgvs"]["unexplained"], 1)

    def test_gkm_composite_proposition_keys_and_typed_codings(self):
        # Synthetic records follow the two documented GKS/GKM section inventories.
        bundle = {"varcond-proposition": {"VCV000000001.1-G-PATH": {
            "id": "VCV000000001.1-G-PATH", "subjectVariant": "#/variation/clinvar:12"}},
            "variation": {"clinvar:12": {"mappings": [{"coding": {
                "system": "https://www.ncbi.nlm.nih.gov/snp/", "code": "1799945"}}]}},
            "sequenceReference": {"SQ.example": {"type": "SequenceReference"}}}
        result, report = self.run_sweep("clinvar_gks", json.dumps(bundle))
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(report["types"]["native_identifier"]["parsed"], 2)
        self.assertEqual(report["types"]["rs"]["parsed"], 1)

    def test_official_cc0_example_checks_all_published_hgvs_expressions(self):
        folder = ROOT / "tests/fixtures/public-dump-sweep"
        source = json.loads((folder / "manifest.json").read_text())["sources"][0]
        data = (folder / source["path"]).read_bytes()
        self.assertEqual(hashlib.sha256(data).hexdigest(), source["sha256"])
        result, report = self.run_sweep("clinvar_gks_record", data.decode())
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(report["types"]["hgvs"]["parsed"], 18)
        self.assertEqual(report["types"]["expression"]["parsed"], 4)
        self.assertEqual(report["types"]["gene"]["parsed"], 1)

    def test_official_monthly_civic_headers_and_rows_parse_without_repair(self):
        folder = ROOT / "tests/fixtures/public-dump-sweep"
        sources = json.loads((folder / "manifest.json").read_text())["sources"]
        for source in sources:
            if not source["kind"].startswith("civic_"):
                continue
            with self.subTest(kind=source["kind"]):
                data = (folder / source["path"]).read_bytes()
                self.assertEqual(hashlib.sha256(data).hexdigest(), source["sha256"])
                result, report = self.run_sweep(source["kind"], data.decode())
                self.assertEqual(result.returncode, 0, result.stderr + str(report))
                self.assertEqual(report["sources"][0]["records"], 1)


if __name__ == "__main__":
    unittest.main()
