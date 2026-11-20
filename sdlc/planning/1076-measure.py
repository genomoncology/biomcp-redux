"""Measure recorded BioMCP replies. Input: BIOMCP_TRANSCRIPT_DIR from make cases, or CLI JSON files.

python3 sdlc/planning/1076-measure.py INPUT... > measurements.tsv
python3 sdlc/planning/1076-measure.py --rank < measurements.tsv > ranked.tsv
No source calls. The output contains locations, field names and UTF-8 byte counts only.
"""
import csv
import json
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path


def compact(value):
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"), sort_keys=True)


def size(value):
    return len(compact(value).encode())


def documents(raw):
    try:
        yield json.loads(raw)
    except (ValueError, UnicodeDecodeError):
        for line in raw.splitlines():
            if line.startswith((b"{", b"[")):
                try:
                    yield json.loads(line)
                except (ValueError, UnicodeDecodeError):
                    pass


def payload(frame):
    if not isinstance(frame, dict):
        return None
    if frame.get("jsonrpc") != "2.0":
        return frame
    body = frame.get("result", frame.get("error"))
    if isinstance(body, dict) and "structuredContent" in body:
        return body["structuredContent"]
    if isinstance(body, dict) and isinstance(body.get("content"), list):
        for item in body["content"]:
            try:
                return json.loads(item["text"])
            except (ValueError, KeyError, TypeError):
                continue
    return body


def executions(record, counts):
    for run in record.get("executions", []):
        if not isinstance(run, dict):
            continue
        if "stdout" in run:
            counts["process_outputs"] += 1
            frames = list(documents(bytes(run["stdout"])))
            if not frames:
                counts["non_json_outputs"] += 1
            yield from frames
        elif "reply" in run:
            yield run["reply"]
        elif "jsonrpc" in run:
            yield run
        # http-concurrent records the same two HTTP replies a second time.
        # Request logs and test instrumentation have no caller-facing payload.


def replies(inputs, counts):
    for source in map(Path, inputs):
        for file in sorted(source.glob("*.json")) if source.is_dir() else [source]:
            record = json.loads(file.read_bytes())
            if record.get("kind") == "trial":
                counts["trials"] += 1
                counts["marked_uncaptured"] += int(record["uncaptured"])
            elif "executions" in record:
                counts["execution_records"] += 1
                for index, frame in enumerate(executions(record, counts)):
                    reply = payload(frame)
                    if isinstance(reply, dict):
                        yield f"{record['key']}#{index}", reply
            elif "schema" in record:
                yield file.stem, record


def lists(node, path=()):
    if isinstance(node, list):
        if node and all(isinstance(row, dict) for row in node):
            yield path, node
        for index, item in enumerate(node):
            yield from lists(item, (*path, index))
    elif isinstance(node, dict):
        for key, value in node.items():
            yield from lists(value, (*path, key))


def constants(rows, prefix=()):
    """Select the largest equal subtree. An equal parent already includes its children."""
    for key in sorted(set.intersection(*(set(row) for row in rows))):
        values = [row[key] for row in rows]
        first = compact(values[0])
        if all(compact(value) == first for value in values[1:]):
            yield (*prefix, key), values[0]
        elif all(isinstance(value, dict) for value in values):
            yield from constants(values, (*prefix, key))


def move(rows, fields):
    """Simulate one adjacent row_defaults object. Verify exact reconstruction."""
    defaults = {}
    stripped = [json.loads(compact(row)) for row in rows]
    for path, value in fields:
        parent = defaults
        for key in path[:-1]:
            parent = parent.setdefault(key, {})
        parent[path[-1]] = value
        for row in stripped:
            parent = row
            for key in path[:-1]:
                parent = parent[key]
            del parent[path[-1]]
    saved = size(rows) - size(stripped) - len(b',"row_defaults":') - size(defaults)
    for row in stripped:
        for path, _ in fields:
            parent, default = row, defaults
            for key in path[:-1]:
                parent, default = parent[key], default[key]
            parent[path[-1]] = default[path[-1]]
    assert compact(stripped) == compact(rows)
    return saved


def display(path):
    return ".".join(str(key) for key in path) or "(root)"


def primary(reply, arrays):
    """Outermost row arrays in the requested sections, otherwise in result."""
    root = ("results",) if "results" in reply and "metadata" in reply else ("result",)
    paths = [p for p, _ in arrays if p[:1] == root and "regulatory_context" not in p]
    if "tools" in reply:
        paths = [p for p, _ in arrays if p[:1] == ("tools",)]
    query = reply.get("query")
    section = query.get("section", []) if isinstance(query, dict) else []
    words = {str(word).replace("-", "_") for word in (section if isinstance(section, list) else [section])}
    named = [p for p in paths if any(key in words for key in p[1:])]
    chosen = []
    for p in named or paths:
        if not any(p[:len(outer)] == outer for outer in chosen):
            chosen.append(p)
    return set(chosen)


def context_parts(reply, selected, row_sizes):
    # Show envelope members and immediate result siblings after subtracting selected arrays.
    def residue(path, value):
        return size(value) - sum(row_sizes[p] for p in selected if p[:len(path)] == path)

    for key, value in reply.items():
        if key == "result" and isinstance(value, dict):
            for field, part in value.items():
                yield f"result.{field}", residue((key, field), part)
        else:
            yield key, residue((key,), value)


def family(reply):
    schema = reply.get("schema")
    if isinstance(schema, str):
        return schema
    if "results" in reply and "metadata" in reply:
        return "research-search"
    if "text" in reply and "metadata" in reply:
        return "research-fetch"
    return "protocol/" + next(iter(sorted(reply)), "empty")


# Measure named, stable fields separately from incidental equal values on the same page.
CANDIDATES = {
    ("variant-card", "result.candidates.*.sections.conservation.data.scores"): {"subject", "attribution"},
    ("cell-line-xrefs", "result.items.items"): {"category", "namespace"},
    ("drug-approvals", "result.approvals.data.records"): {"event.attribution", "event.record"},
    ("article-page", "result.items"): {"open_access_source", "license_not_checked"},
    ("protein-features", "result.items"): {"attribution", "source_coordinates", "feature.protein"},
    ("diagnostic-genes", "result.items"): {"test_id", "test_type"},
    ("diagnostic-conditions", "result.items"): {"test_id", "test_type"},
    ("drug-indications", "result.indications.data.records"): {"attribution", "drug"},
}


def measure(name, reply, write, counts):
    total = size(reply)
    schema = family(reply)
    arrays = list(lists(reply))
    row_sizes = {path: size(rows) for path, rows in arrays}
    selected = primary(reply, arrays)
    outside = total - sum(row_sizes[p] for p in selected)
    counts["replies"] += 1
    if selected:
        counts["with_rows"] += 1

    def emit(kind, path, field, n, amount):
        write((kind, schema, name, total, path, field, n, amount))

    emit("reply", "", "", sum(len(rows) for p, rows in arrays if p in selected), total)
    if 4 * outside > total:
        parts = sorted(context_parts(reply, selected, row_sizes), key=lambda pair: -pair[1])
        emit("outside" if selected else "rowless", "", parts[0][0] if parts else "", 0, outside)
        for part, amount in parts:
            if 4 * amount > total:
                emit("component", part, "", 0, amount)
    for path, rows in arrays:
        if len(rows) < 10:
            continue
        counts["lists_10_plus"] += 1
        fields = list(constants(rows))
        if fields:
            stem = schema.split("/")[0].removeprefix("biomcp.")
            pattern = display("*" if isinstance(key, int) else key for key in path)
            wanted = CANDIDATES.get((stem, pattern), set())
            subset = [(p, v) for p, v in fields if display(p) in wanted]
            if subset:
                emit("candidate", display(path), ",".join(display(p) for p, _ in subset), len(rows), move(rows, subset))
            for field in fields:
                emit("field", display(path), display(field[0]), len(rows), move(rows, [field]))
            emit("together", display(path), ",".join(display(p) for p, _ in fields), len(rows), move(rows, fields))


def ranked(records, write):
    """Order alternative changes by total bytes. Combine several lists in one reply first."""
    groups = defaultdict(Counter)
    for row in records:
        if row["kind"] in {"field", "together", "candidate", "component", "outside", "rowless"}:
            path = re.sub(r"(?<=\.)[0-9]+(?=\.|$)", "*", row["path"])
            field = row["field"] if row["kind"] == "field" else ""
            key = row["kind"], row["schema"], path, field
            groups[key][row["reply_id"]] += int(row["bytes"])
    write(("kind", "schema", "path", "field", "replies", "cases", "total_bytes", "min_per_reply", "max_per_reply"))
    for key, counts in sorted(groups.items(), key=lambda group: (-sum(group[1].values()), group[0])):
        values = counts.values()
        cases = {re.sub(r"/(?:step|[0-9a-f]{64})/\d+$", "", name.rsplit("#", 1)[0]) for name in counts}
        write((*key, len(counts), len(cases), sum(values), min(values), max(values)))


if __name__ == "__main__":
    writer = csv.writer(sys.stdout, delimiter="\t", lineterminator="\n")
    if sys.argv[1:] == ["--rank"]:
        ranked(csv.DictReader(sys.stdin, delimiter="\t"), writer.writerow)
        sys.exit(0)
    writer.writerow(("kind", "schema", "reply_id", "reply_bytes", "path", "field", "rows", "bytes"))
    counts = Counter()
    for name, reply in replies(sys.argv[1:], counts):
        measure(name, reply, writer.writerow, counts)
    if not counts["replies"]:
        raise SystemExit("No structured replies found")
    print(dict(counts), file=sys.stderr)
