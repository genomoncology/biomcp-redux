"""Make outside-in cases from retained public normalization replies without source calls.

Usage: python3 scripts/variant_native_values_replay.py CAPTURE_DIR OUTPUT_DIR
The capture stays outside Git. OUTPUT_DIR holds original controls and one-value
mutations for the ordinary cases runner, plus unchanged source recording identities.
"""
import copy
import hashlib
import json
import math
import struct
import tarfile
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
        write_json(replay / "clinvar" / "b0dc08f8b2e327a3cc14eea0aff96c83994e24add7db09654e86e8c3f801b4ae.json", value)
        path = "/result/assertions/0/unmapped/attribution"
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
        write_json(replay / "clinvar" / "b0dc08f8b2e327a3cc14eea0aff96c83994e24add7db09654e86e8c3f801b4ae.json", value)
        expect = {
            "/result/assertions/0/unmapped/attribution/record": "VCV000666267",
            "/result/assertions/1/statement/attribution/record": "VCV000666267",
            "/result/assertions/2/unmapped/attribution/record": "RCV000824811",
            "/result/assertions/3/statement/attribution/record": "RCV004669148",
        }
        if name == "identity-bad":
            expect["/result/assertions"] = {"count": 5}
            expect["/result/assertions/4/statement/attribution/record"] = "SCV005094146"
            expect["/problems/*/field"] = ["Accession"]
        else:
            expect["/result/assertions"] = {"count": 6}
            expect["/result/assertions/4/unmapped/attribution/record"] = "SCV000965609"
            expect["/result/assertions/5/statement/attribution/record"] = "SCV005094146"
        write_json(output / "cases" / ("clinvar-" + name + ".yaml"), {
            "feature": "variant-assertions", "live_check": "retained public capture",
            "story": "One invalid native SCV identity leaves valid sibling assertions available",
            "replay": str(replay), "steps": [{"run": args[:5], "expect": expect}],
        })


def alphagenome_cases(archive, output):
    expected = "f6b2b3d579d868ec6da0d0e7b71831ccf4f7388707f961ece8ff42d769b2df51"
    if hashlib.sha256(archive.read_bytes()).hexdigest() != expected:
        raise ValueError("retained AlphaGenome capture archive differs")
    prefix = "recording/alphagenome/"
    stem = "83049d02ed85aa57e6b5de3e7e81c326b7b9f34a93563e159dc40bc4776407e3"
    with tarfile.open(archive) as captured:
        manifest_bytes = captured.extractfile("./manifest.json").read()
        if hashlib.sha256(manifest_bytes).hexdigest() != "f1c353a6fd6b3d600bba85d2ae139c45a849bbdc2055c512b33df9925945724b":
            raise ValueError("AlphaGenome original manifest differs")
        manifest = json.loads(manifest_bytes)
        files = {name: captured.extractfile("./" + name).read() for name in manifest
                 if name.startswith(prefix)}
    for name, data in files.items():
        if hashlib.sha256(data).hexdigest() != manifest[name]:
            raise ValueError("AlphaGenome native sidecar differs: " + name)
    recording = json.loads(files[prefix + stem + ".json"])
    if (recording["method"], recording["source"], recording["post"]["request_body_sha256"]) != (
            "POST", "alphagenome", "dae4da38865fa72f90cf6a5c90ba337315d92f2a686c61b174df2d9380faa8aa"):
        raise ValueError("AlphaGenome captured request identity differs")
    if len(recording["exchanges"][0]["rpc"]["messages"]) != 6 or recording["exchanges"][0]["rpc"]["terminal"] != 0:
        raise ValueError("AlphaGenome complete native stream missing")
    name = prefix + stem + "-grpc-0-0.bin"
    data = files[name]

    def number(position, stop):
        value = 0
        for shift in range(0, 70, 7):
            if position >= stop:
                raise ValueError("truncated original protobuf varint")
            byte = data[position]
            position += 1
            value |= (byte & 127) << shift
            if byte < 128:
                return value, position
        raise ValueError("oversized original protobuf varint")

    def fields(start, stop):
        result = []
        while start < stop:
            key, start = number(start, stop)
            tag, wire = key >> 3, key & 7
            if wire == 2:
                size, start = number(start, stop)
                end = start + size
            elif wire in (1, 5):
                end = start + (8 if wire == 1 else 4)
            elif wire == 0:
                _, end = number(start, stop)
            else:
                raise ValueError("unsupported original protobuf wire type")
            if not tag or end > stop:
                raise ValueError("invalid original protobuf field")
            result.append((tag, wire, start, end))
            start = end
        return result

    def child(values, tag, wire=2):
        found = [(start, end) for key, kind, start, end in values if (key, kind) == (tag, wire)]
        if len(found) != 1:
            raise ValueError("native protobuf target missing or ambiguous")
        return found[0]

    descriptor = child(fields(0, len(data)), 1)
    scored = child(fields(*descriptor), 1)
    metadata = child(fields(*scored), 2)
    tracks = [(start, end) for tag, wire, start, end in fields(*metadata) if (tag, wire) == (2, 2)]
    target = None
    for index, bounds in enumerate(tracks):
        native = fields(*bounds)
        means = [(start, end) for tag, wire, start, end in native if (tag, wire) == (14, 5)]
        if len(means) == 1:
            start, end = means[0]
            mean = struct.unpack("<f", data[start:end])[0]
            if math.isfinite(mean):
                label = data[slice(*child(native, 1))].decode("utf-8")
                if label:
                    target = index, start, end, mean, label
                    break
    if target is None:
        raise ValueError("present finite AlphaGenome nonzero_mean target missing")
    index, start, end, mean, label = target
    genes = [(start, end) for tag, wire, start, end in fields(*metadata) if (tag, wire) == (3, 2)]
    if not genes:
        raise ValueError("native gene identity control missing")
    gene_id = data[slice(*child(fields(*genes[0]), 1))].decode("utf-8")
    mutation = data[:start] + struct.pack("<f", float("nan")) + data[end:]
    if mutation[:start] + data[start:end] + mutation[end:] != data or len(mutation) != len(data):
        raise ValueError("AlphaGenome mutation changed another native value")
    root = Path(__file__).resolve().parents[1]
    seed = root / "tests/replies/variant-population/main/car/0166319161158cc3ea119c855cc84e61a90691e42236715f623817a2eab0e883.json"
    resolver = json.loads(seed.read_text())
    row = resolver["exchanges"][0]["body"]["value"]["genomicAlleles"][0]
    if resolver["method"] != "GET" or row["hgvs"] != ["NC_000007.14:g.140753336A>T"]:
        raise ValueError("controlled resolver identity differs from actual scorer request")
    path = "/result/candidates/0/sections/alphagenome/data/scorers/0/rows/0"
    for kind in ["positive", "bad"]:
        replay = output / "replies/alphagenome" / kind
        for filename, raw in files.items():
            destination = replay / filename.removeprefix("recording/")
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(mutation if kind == "bad" and filename == name else raw)
        write_json(replay / "car" / seed.name, resolver)
        expect = {"/result/candidates/0/record/id": "CA123643", path + "/tensor_index": [0, 0, index],
                  path + "/track/name": label, path + "/gene/gene_id": gene_id}
        if kind == "positive":
            expect[path + "/track/nonzero_mean"] = mean
        else:
            expect[path + "/track"] = {"omits": ["nonzero_mean"]}
            expect["/problems/*/field"] = [f"scorers.0.tracks.{index}.nonzero_mean"]
        write_json(output / "cases" / ("alphagenome-" + kind + ".yaml"), {
            "feature": "variant-alphagenome", "live_check": "retained0198 actual scorer with controlled resolver",
            "story": "One present native optional metadata mean preserves its tensor scalar and track",
            "config": {"keys": {"alphagenome_api_key": "fixture-alpha-native-value-key"}},
            "replay": str(replay), "steps": [{"run": ["get", "variant", "CA123643", "alphagenome",
            "--section-offset", str(index)], "expect": expect}],
        })
    write_json(output / "alphagenome-provenance.json", {"archive_sha256": expected,
        "original_manifest_sha256": hashlib.sha256(manifest_bytes).hexdigest(),
        "original_member_sha256": {key: manifest[key] for key in files},
        "target": f"scorers.0.tracks.{index}.nonzero_mean", "target_byte_range": [start, end],
        "original_mean": mean, "mutated_member_sha256": hashlib.sha256(mutation).hexdigest()})


def main():
    capture, output = map(lambda value: Path(value).resolve(), sys.argv[1:3])
    if output.exists():
        raise ValueError("output directory must be new")
    for parent in (output, *output.parents):
        if (parent / ".git").exists():
            raise ValueError("captured native replies must stay outside Git")
    for source in BODIES:
        source_cases(capture, output, source)
    if len(sys.argv) >= 4:
        clinvar_cases(Path(sys.argv[3]).resolve(), output)
    if len(sys.argv) == 5:
        alphagenome_cases(Path(sys.argv[4]).resolve(), output)
    print("Prepared source-bound original controls and one-value mutations")


if __name__ == "__main__":
    main()
