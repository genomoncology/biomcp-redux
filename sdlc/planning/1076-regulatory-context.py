"""Size a lossless alternative for FDA context in an existing drug-indications JSON reply."""
from collections import Counter
from copy import deepcopy
import json
import sys
from pathlib import Path


def encoded(value):
    return json.dumps(value, sort_keys=True, ensure_ascii=False, separators=(",", ":")).encode()


def defaults_for(rows):
    """Only inherit a property if every input has it. Keep arrays and exceptions on the row."""
    defaults = {}
    stripped = [dict(row) for row in rows]
    for key in sorted(set.intersection(*(set(row) for row in rows))):
        values = [row[key] for row in rows]
        if all(isinstance(value, dict) for value in values):
            common, overrides = defaults_for(values)
            if common:
                defaults[key] = common
                for row, override in zip(stripped, overrides):
                    row[key] = override
        elif all(not isinstance(value, (dict, list)) for value in values):
            frequent, count = Counter(map(encoded, values)).most_common(1)[0]
            if count > 1:
                defaults[key] = json.loads(frequent)
                for row, value in zip(stripped, values):
                    if encoded(value) == frequent:
                        del row[key]
    return defaults, stripped


def factor(group, key):
    rows = group.get(key, [])
    if len(rows) > 1:
        common, stripped = defaults_for(rows)
        if common and len(encoded({key: stripped, "row_defaults": common})) < len(encoded({key: rows})):
            group.update({key: stripped, "row_defaults": common})


def inherit(defaults, row):
    for key, value in defaults.items():
        if isinstance(value, dict):
            inherit(value, row[key])
        elif key not in row:
            row[key] = value


def expand(group, key):
    defaults = group.pop("row_defaults", {})
    for row in group.get(key, []):
        inherit(defaults, row)


def compare(reply):
    original = reply["result"]["regulatory_context"]
    ctx = deepcopy(original)
    products = {}
    for app in ctx["applications"]:
        if "products" in app:
            app["product_indices"] = [products.setdefault(encoded(product), len(products)) for product in app.pop("products")]
        if "data" in app["approvals"]:
            factor(app["approvals"]["data"], "records")
    if products:
        ctx["product_catalog"] = {"products": [json.loads(product) for product in products]}
        factor(ctx["product_catalog"], "products")
    factor(ctx, "applications")
    before, after = len(encoded(original)), len(encoded(ctx))
    total = len(encoded(reply)) - before + after

    expand(ctx, "applications")
    catalog = ctx.pop("product_catalog", {})
    expand(catalog, "products")
    for app in ctx["applications"]:
        if "product_indices" in app:
            app["products"] = [catalog["products"][i] for i in app.pop("product_indices")]
        if "data" in app["approvals"]:
            expand(app["approvals"]["data"], "records")
    assert encoded(ctx) == encoded(original)
    return before, after, total


if __name__ == "__main__":
    for file in sys.argv[1:]:
        before, after, total = compare(json.loads(Path(file).read_bytes()))
        print(f"{Path(file).name}  context {before} -> {after} bytes, whole reply {total} bytes, context {100*after/total:.1f}%")
