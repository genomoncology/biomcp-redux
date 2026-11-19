"""Measure section row sizes for the 1071 proposal: whole reply, rows, and rows cut to a few columns.

Usage: python3 1071-measure.py BIOMCP. Live sources. Bytes are the compact JSON reply.
"Kept" is computed from the reply: each row keeps only the named keys; a dotted path keeps one inner value.
"""
import json
import subprocess
import sys

BIN = sys.argv[1]
SKIP = {"next", "sources", "problems", "query"}

# (name, args, kept columns); "{id}" takes the first row id of the previous case.
CASES = [
    ("trial search", "search trial --condition melanoma --limit 100", "nct_id,title,status"),
    ("trial search fields", "--fields nct_id,title,status search trial --condition melanoma --limit 100", ""),
    ("trial locations", "get trial NCT04613596 locations --limit 100", "facility,city,state,status"),
    ("trial locations near", "get trial NCT04613596 locations --limit 100 --lat 39.96 --lon -83.00",
     "facility,city,state,status,distance_mi"),
    ("trial arms", "get trial NCT04613596 arms --limit 20", "record,data.label,data.kind"),
    ("drug approvals", "get drug NDA018936 approvals --source drugsfda --limit 50",
     "submission_number,event.record,event.attribution,event.status,event.dates"),
    ("drug label search", "search drug --indication cancer --source openfda-label --limit 50",
     "id,source,identity_level,name,effective_date"),
    ("drug label search fields",
     "--fields name,effective_date search drug --indication cancer --source openfda-label --limit 50", ""),
    ("drug targets", "get drug imatinib targets --source chembl --limit 50", ""),
    ("drug indications", "get drug imatinib indications --source opentargets --limit 50", ""),
    ("pathway genes", "get pathway R-HSA-162582 genes --limit 100", "reference,physical_context_count,gene,name"),
    ("pathway events", "get pathway R-HSA-162582 events --limit 100", "kind,db_id,id,name"),
    ("protein features", "protein features P15056 --limit 50",
     "feature.protein,feature.span,feature.kind,attribution,source_coordinates"),
    ("protein get features", "get protein P15056 features --limit 25", ""),
    ("adverse event search", "search adverse-event --drug pembrolizumab --limit 50", "id,source,serious_label"),
    ("adverse event drugs", "get adverse-event {id} drugs --source faers --limit 50", ""),
    ("diagnostic search", "search diagnostic --gene BRAF --source gtr --limit 50",
     "identity_status,header.id,header.name,header.laboratory.name"),
    ("diagnostic genes", "get diagnostic GTR000273063.3 genes --source gtr --limit 50", ""),
    ("diagnostic conditions", "get diagnostic GTR000273063.3 conditions --source gtr --limit 50", ""),
    ("gene clingen", "get gene BRAF clingen", "mondo_id,mode_of_inheritance,disease,classification"),
    ("gene gencc", "get gene BRAF gencc", "id,version,disease,classification,submitter"),
]


def size(value):
    return len(json.dumps(value, separators=(",", ":")))


def lists(node, path, found):
    """Every list of objects under the result, with its path."""
    if isinstance(node, list):
        if node and all(isinstance(item, dict) for item in node):
            found.append(("/".join(path), node))
        for index, item in enumerate(node[:1]):
            lists(item, path + [str(index)], found)
    elif isinstance(node, dict):
        for key, value in node.items():
            if key not in SKIP:
                lists(value, path + [key], found)


def cut(row, paths):
    """The row with only the named keys; a dotted path keeps one value inside an object."""
    kept = {}
    for path in paths:
        head, _, rest = path.partition(".")
        if head not in row:
            continue
        if rest and isinstance(row[head], dict):
            inner = cut(row[head], [rest])
            if inner:
                kept.setdefault(head, {}).update(inner)
        else:
            kept[head] = row[head]
    return kept


def first_id(reply):
    found = []
    lists(reply.get("result", {}), ["result"], found)
    for _, rows in found:
        row = rows[0].get("header", rows[0])
        if isinstance(row, dict) and "id" in row:
            return str(row["id"])
    return ""


def measure(name, args, kept, last):
    args = args.replace("{id}", first_id(last)).split()
    out = subprocess.run([BIN] + args, capture_output=True, text=True, timeout=900).stdout
    try:
        reply = json.loads(out)
    except ValueError:
        print(f"## {name}: not JSON: {out[:300]}")
        return {}
    print(f"## {name}: `{' '.join(args)}` -> {reply.get('schema')} {reply.get('status')} {len(out)} bytes")
    if reply.get("error"):
        print("   error:", json.dumps(reply["error"])[:400])
    found = []
    lists(reply.get("result", {}), ["result"], found)
    found = [(path, rows) for path, rows in found if len(rows) > 1 or not found[1:]]
    for path, rows in sorted(found, key=lambda pair: -size(pair[1]))[:3]:
        print(f"   rows {path}: {len(rows)} rows, {size(rows)} bytes, {size(rows) // len(rows)} per row")
    if not found:
        return reply
    path, rows = max(found, key=lambda pair: size(pair[1]))
    keys = {}
    for row in rows:
        for key, value in row.items():
            keys[key] = keys.get(key, 0) + size({key: value})
    print("   bytes per row by key:", {key: total // len(rows) for key, total in keys.items()})
    print("   sample row:", json.dumps(rows[0], separators=(",", ":"))[:700])
    if kept:
        narrow = [cut(row, kept.split(",")) for row in rows]
        saved = size(rows) - size(narrow)
        print(f"   kept {kept}: {size(narrow) // len(rows)} per row; reply {len(out)} -> {len(out) - saved}"
              f" ({100 * (len(out) - saved) // len(out)}%); outside rows {len(out) - size(rows)}")
    return reply


last = {}
for case in CASES:
    try:
        last = measure(*case, last) or last
    except Exception as error:  # a slow or failing source must not stop the other cases
        print(f"## {case[0]}: {error}")
    sys.stdout.flush()
