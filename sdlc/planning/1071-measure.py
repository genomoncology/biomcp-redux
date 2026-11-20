"""Measure 1071: each list's whole reply against the same call with --fields, both real replies.

Usage: python3 1071-measure.py BIOMCP [NAME...]. Live sources. Bytes are the compact JSON reply
as printed. It also checks that the narrowed rows are the whole rows, in the same order,
each with keys left out.
"""
import json
import subprocess
import sys

BIN = sys.argv[1]
ONLY = sys.argv[2:]
SKIP = {"next", "sources", "problems", "query", "fields"}

# (name, args, fields)
CASES = [
    ("trial locations", "get trial NCT04613596 locations --limit 100", "facility,city,state,status"),
    ("trial locations near", "get trial NCT04613596 locations --limit 100 --lat 39.96 --lon -83.00",
     "facility,city,state,status,distance_mi"),
    ("trial arms", "get trial NCT04613596 arms --limit 20", "data.label,data.kind"),
    ("drug approvals", "get drug NDA018936 approvals --source drugsfda --limit 50", "event.status,event.dates"),
    ("drug label search", "search drug --indication cancer --source openfda-label --limit 50", "name,effective_date"),
    ("pathway genes", "get pathway R-HSA-162582 genes --limit 100", "name"),
    ("pathway genes subcommand", "pathway genes R-HSA-162582 --limit 100", "name"),
    ("pathway events", "get pathway R-HSA-162582 events --limit 100", "id,name"),
    ("protein features", "protein features P15056 --limit 50", "feature.kind,feature.span"),
    ("adverse event search", "search adverse-event --drug pembrolizumab --limit 50", "serious_label"),
    ("adverse event drugs", "get adverse-event {id} drugs --source faers --limit 50",
     "medicinalproduct,role_label,indication"),
    ("diagnostic search", "search diagnostic --gene BRAF --source gtr --limit 50",
     "header.name,header.laboratory.name"),
    ("diagnostic genes", "get diagnostic GTR000509442.12 genes --source gtr --limit 50", "gene"),
    ("gene clingen", "get gene BRAF clingen", "disease,classification"),
    ("gene gencc", "get gene BRAF gencc", "disease,classification,submitter"),
]


def run(args):
    out = subprocess.run([BIN] + args, capture_output=True, text=True, timeout=900).stdout
    try:
        return json.loads(out), len(out.encode())
    except ValueError:
        return {"_raw": out[:300]}, len(out.encode())


def lists(node, path, found):
    """Every list of objects in the result, with its path."""
    if isinstance(node, list):
        if node and all(isinstance(item, dict) for item in node):
            found.append(("/".join(path), node))
        for index, item in enumerate(node):
            lists(item, path + [str(index)], found)
    elif isinstance(node, dict):
        for key, value in node.items():
            if key not in SKIP:
                lists(value, path + [key], found)


def within(part, full):
    """Every object keeps only keys of the same whole object; every list keeps its length and order."""
    if isinstance(part, dict) and isinstance(full, dict):
        return all(key in full and (key == "fetched" or within(value, full[key])) for key, value in part.items())
    if isinstance(part, list) and isinstance(full, list):
        return len(part) == len(full) and all(within(p, f) for p, f in zip(part, full))
    return part == full


def size(value):
    return len(json.dumps(value, separators=(",", ":")))


def first_id(reply):
    found = []
    lists(reply.get("result", {}), ["result"], found)
    for _, rows in found:
        if "id" in rows[0]:
            return str(rows[0]["id"])
    return ""


last = {}
for name, args, fields in CASES:
    if ONLY and name not in ONLY:
        continue
    args = args.replace("{id}", first_id(last)).split()
    whole, whole_bytes = run(args)
    narrow, narrow_bytes = run(args + ["--fields", fields])
    last = whole
    print(f"## {name}: `{' '.join(args)}` --fields {fields}")
    if whole.get("status") not in ("complete", "partial") or narrow.get("status") not in ("complete", "partial"):
        print(f"   not measured: whole {whole.get('status')} {json.dumps(whole.get('error', whole.get('_raw')))[:200]};"
              f" narrow {narrow.get('status')} {json.dumps(narrow.get('error', narrow.get('_raw')))[:300]}")
        continue
    found = []
    lists(whole.get("result", {}), ["result"], found)
    path, rows = max(found, key=lambda pair: (len(pair[1]), size(pair[1])))
    narrowed = narrow.get("result", {})
    for step in path.split("/")[1:]:
        narrowed = narrowed[int(step)] if isinstance(narrowed, list) else narrowed.get(step, {})
    same = within(narrow.get("result"), whole.get("result"))
    print(f"   rows {path}: {len(rows)}; per row {size(rows) // len(rows)} -> {size(narrowed) // max(len(narrowed), 1)};"
          f" reply {whole_bytes} -> {narrow_bytes} ({100 * narrow_bytes // whole_bytes}%);"
          f" fields echo {narrow.get('fields')}; same rows in same order: {same}")
    print("   first narrowed row:", json.dumps(narrowed[0] if narrowed else None, separators=(",", ":"))[:400])
    sys.stdout.flush()
