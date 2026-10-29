"""Make outside-in cases from retained public normalization replies without source calls.

Usage: python3 scripts/variant_native_values_replay.py CAPTURE_DIR OUTPUT_DIR
The capture stays outside Git. OUTPUT_DIR holds original controls and one-value
mutations for the ordinary cases runner, plus unchanged source recording identities.
"""
import copy
import hashlib
import json
from pathlib import Path
import sys

INPUT = "NM_004333.4:c.1799T>A"
BODIES = {
    "mutalyzer": ("0000.body", "ec2ac3668671f24b464e9d787bca95a31a1face1b4ef34941ea60045cf55e80a"),
    "variantvalidator": ("0001.body", "9584fb07e52ccec752ea0337a745ec2c9df6dd62bf7f0dbda53d424a703722d7"),
}


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2) + "\n")


def source_cases(capture, output, source):
    filename, expected_hash = BODIES[source]
    raw = (capture / filename).read_bytes()
    if hashlib.sha256(raw).hexdigest() != expected_hash:
        raise ValueError("retained body hash differs: " + source)
    original = json.loads(raw)
    target = "corrected_description" if source == "mutalyzer" else "hgvs_lrg_variant"
    record = original if source == "mutalyzer" else original[INPUT]
    if not isinstance(record.get(target), str) or not record[target]:
        raise ValueError("present native mutation target missing: " + source)
    if source == "mutalyzer" and original.get("normalized_description") != INPUT:
        raise ValueError("Mutalyzer identity changed")
    if source == "variantvalidator" and record.get("hgvs_transcript_variant") != INPUT:
        raise ValueError("VariantValidator identity changed")
    mutation = copy.deepcopy(original)
    changed = mutation if source == "mutalyzer" else mutation[INPUT]
    changed[target] = 42
    restored = copy.deepcopy(mutation)
    (restored if source == "mutalyzer" else restored[INPUT])[target] = record[target]
    if restored != original:
        raise ValueError("mutation changed another native value")
    fixtures = Path(__file__).resolve().parents[1] / "tests/replies/variant/normalization"
    seeds = list((fixtures / source / source).glob("*.json"))
    if len(seeds) != 1:
        raise ValueError("source recording identity must be unique")
    seed = json.loads(seeds[0].read_text())
    receipt = json.loads((capture / filename.replace(".body", ".receipt.json")).read_text())
    if seed["method"] != "GET" or seed["source"] != source or seed["url"] != receipt["url"] or receipt["status"] != 200:
        raise ValueError("capture and replay request identities differ: " + source)
    args = ["variant", "normalize", "--source", source, "--hgvs", INPUT]
    if source == "variantvalidator":
        args += ["--build", "GRCh38"]
    for name, body in [("positive", raw.decode()), ("bad", json.dumps(mutation))]:
        recording = copy.deepcopy(seed)
        recording["exchanges"] = [{
            "status": 200, "content_type": "application/json", "delay_ms": 0,
            "retry_after_ms": None, "body": {"kind": "text", "value": body},
        }]
        replay = output / "replies" / source / name
        write_json(replay / source / seeds[0].name, recording)
        expect = {"/result/results/0/outcome": "resolved"}
        if source == "mutalyzer":
            expect["/result/results/0/normalized"] = INPUT
            expect["/result/results/0"] = {"omits": ["corrected"]} if name == "bad" else {"includes_keys": ["corrected"]}
            if name == "positive":
                expect.pop("/result/results/0")
                expect["/result/results/0/corrected"] = record[target]
        else:
            field = INPUT + ".hgvs_lrg_variant"
            expect["/result/results/0/mappings/*/field"] = {
                "includes": INPUT + ".hgvs_transcript_variant",
                "excludes" if name == "bad" else "includes_all_of": field if name == "bad" else [field],
            }
        if name == "bad":
            field = target if source == "mutalyzer" else INPUT + "." + target
            expect["/problems/*/field"] = [field]
        write_json(output / "cases" / (source + "-" + name + ".yaml"), {
            "feature": "variant-normalization", "live_check": "retained public capture",
            "story": "Original identity and one independent native normalization value",
            "replay": str(replay), "steps": [{"run": args, "expect": expect}],
        })


def clinvar_cases(capture, output):
    raw = capture.read_bytes()
    if hashlib.sha256(raw).hexdigest() != "f806ec35a3d4e12667835fa80b205fe9ad7f1e45a7c9cc1e56a0d0d162df0054":
        raise ValueError("retained ClinVar recording hash differs")
    recording = json.loads(raw)
    xml = recording["exchanges"][0]["body"]["value"]
    needle = '<Classification DateLastEvaluated="2019-05-21">'
    if xml.count(needle) != 1:
        raise ValueError("ClinVar native date target missing or ambiguous")
    args = ["variant", "assertions", "clinvar:666267", "--source", "clinvar",
            "--assertion", "SCV000965609", "--version", "1"]
    for name in ["positive", "bad"]:
        value = copy.deepcopy(recording)
        if name == "bad":
            value["exchanges"][0]["body"]["value"] = xml.replace(needle,
                '<Classification DateLastEvaluated="not-a-date">')
        replay = output / "replies" / "clinvar" / name
        write_json(replay / "clinvar" / capture.name, value)
        path = "/result/assertions/0/statement/attribution"
        expect = {path + "/record": "SCV000965609"}
        if name == "positive":
            expect[path + "/evaluated"] = "2019-05-21"
        else:
            expect[path] = {"omits": ["evaluated"]}
            expect["/problems/*/field"] = ["evaluated"]
        write_json(output / "cases" / ("clinvar-" + name + ".yaml"), {
            "feature": "variant-assertions", "live_check": "retained public capture",
            "story": "A native SCV retains its identity when its optional date is invalid",
            "replay": str(replay), "steps": [{"run": args, "expect": expect}],
        })


    identity = 'Accession="SCV000965609"'
    if xml.count(identity) != 1 or 'Accession="SCV005094146"' not in xml:
        raise ValueError("ClinVar assertion identity or valid sibling missing")
    for name in ["identity-positive", "identity-bad"]:
        value = copy.deepcopy(recording)
        if name == "identity-bad":
            value["exchanges"][0]["body"]["value"] = xml.replace(identity, 'Accession="BAD"')
        replay = output / "replies" / "clinvar" / name
        write_json(replay / "clinvar" / capture.name, value)
        expect = {"/result/assertions/*/statement/attribution/record": {
            "includes_all_of": ["SCV005094146", "RCV000824811", "VCV000666267"],
        }}
        if name == "identity-bad":
            expect["/result/assertions/*/statement/attribution/record"]["excludes"] = "SCV000965609"
            expect["/problems/*/field"] = ["Accession"]
        else:
            expect["/result/assertions/*/statement/attribution/record"]["includes"] = "SCV000965609"
        write_json(output / "cases" / ("clinvar-" + name + ".yaml"), {
            "feature": "variant-assertions", "live_check": "retained public capture",
            "story": "One invalid native SCV identity leaves valid sibling assertions available",
            "replay": str(replay), "steps": [{"run": args[:5], "expect": expect}],
        })


def main():
    capture, output = map(lambda value: Path(value).resolve(), sys.argv[1:3])
    if output.exists():
        raise ValueError("output directory must be new")
    for parent in (output, *output.parents):
        if (parent / ".git").exists():
            raise ValueError("captured native replies must stay outside Git")
    for source in BODIES:
        source_cases(capture, output, source)
    if len(sys.argv) == 4:
        clinvar_cases(Path(sys.argv[3]).resolve(), output)
    print("Prepared source-bound original controls and one-value mutations")


if __name__ == "__main__":
    main()
