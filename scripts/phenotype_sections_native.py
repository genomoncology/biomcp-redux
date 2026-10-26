"""Prepare external section fixtures from two unchanged verified evidence archives."""
import hashlib
import json
from pathlib import Path
import sys
import tarfile


def read_archive(path, digest, count):
    data = path.read_bytes()
    if hashlib.sha256(data).hexdigest() != digest:
        raise ValueError("archive hash changed")
    with tarfile.open(path) as archive:
        members = archive.getmembers()
        names = [item.name for item in members]
        if len(names) != len(set(names)) or len(names) != count:
            raise ValueError("archive inventory changed")
        content = {item.name: archive.extractfile(item).read() for item in members}
    manifest = json.loads(content["manifest.json"])
    if set(content) != set(manifest) | {"manifest.json"}:
        raise ValueError("manifest inventory changed")
    for name, expected in manifest.items():
        if hashlib.sha256(content[name]).hexdigest() != expected:
            raise ValueError(f"member hash changed: {name}")
    return content


def prepare(first, second, output):
    cards = read_archive(
        first, "74b34e6090b6f3a3f636a933a5b4977ecc44b112052736f0e42b7e98b942c31e", 143
    )
    admission = read_archive(
        second, "4340c3f959015ae77b0328156a2f635d58107ccda7a61e45248f623ae8ff3428", 71
    )
    output.mkdir(parents=True)
    plan = []
    for row in json.loads(cards["initial__plan.json"]):
        if row["words"][0] != "get" or row.get("status", 200) != 200:
            continue
        body = cards["initial__" + row["body"]]
        source = row["source"]
        sections = ["synonyms", "xrefs"] if source == "hpo" else ["synonyms", "xrefs", "hierarchy"]
        for section in sections:
            append(plan, output, row, body, section)
        if source == "monarch":
            for offset in [5, 10]:
                append(plan, output, row, body, "hierarchy", offset=offset)
            append(plan, output, row, body, "hierarchy", limit=2)
    body = admission["xref-first-0000.body"]
    receipt = json.loads(admission["xref-first-0000.receipt.json"])
    row = {
        "source": "monarch", "target": "/v3/api/entity/HP%3A0000256",
        "words": ["get", "phenotype", "HP:0000256"],
        "fetched": receipt.get("fetched", receipt.get("finished", 1792976779)),
    }
    append(plan, output, row, body, "xrefs")
    (output / "plan.json").write_text(json.dumps(plan, indent=2) + "\n")
    print(f"Prepared {len(plan)} exact retained section witnesses with zero provider requests")


def append(plan, output, row, body, section, offset=0, limit=5):
    name = f"{len(plan):02}.body"
    (output / name).write_bytes(body)
    words = row["words"][:3] + [section]
    if row["source"] == "monarch":
        words += ["--source", "monarch"]
    if offset:
        words += ["--section-offset", str(offset)]
    if limit != 5:
        words += ["--section-limit", str(limit)]
    plan.append({
        "body": name, "sha256": hashlib.sha256(body).hexdigest(),
        "source": row["source"], "target": row["target"], "words": words,
        "fetched": row["fetched"], "section": section, "offset": offset, "limit": limit,
    })


if __name__ == "__main__":
    prepare(*(Path(value) for value in sys.argv[1:]))
