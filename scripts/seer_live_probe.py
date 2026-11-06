"""Bound public SEER calls; report status/timing without caller credentials."""
import json
import time
from pathlib import Path
import urllib.error
import urllib.request

BASE = "https://seer.cancer.gov/statistics-network/explorer/source/content_writers/"
PATHS = [
    BASE + "get_var_formats.php",
    BASE + "render_region_5.php?site=40&data_type=1&graph_type=10&compareBy=sex&sex=1&race=1&age_range=1&year_range=5&data_model=4",
    BASE + "render_region_5.php?site=40&data_type=4&graph_type=1&compareBy=sex&sex=1&race=1&age_range=1&relative_survival_interval=5&year_range=31&data_model=4",
]
output = Path("dist/seer-live")
output.mkdir(parents=True, exist_ok=True)
for index, url in enumerate(PATHS):
    if index:
        # Leave at least the conservative SEER quota's one-second gap between probes.
        time.sleep(1)
    start = time.monotonic()
    try:
        with urllib.request.urlopen(url, timeout=15) as response:
            body = response.read(4_000_000)
            (output / f"{index}.json").write_bytes(body)
            result = {"status": response.status, "bytes": len(body)}
            parsed = json.loads(body)
            if isinstance(parsed, str):
                parsed = json.loads(parsed)
            result["document_keys"] = list(parsed)
            if "info" in parsed:
                result["info"] = parsed["info"]
            if url.endswith("get_var_formats.php"):
                catalog = json.loads(body)
                formats = catalog.get("VariableFormats", {})
                result["format_keys"] = list(formats)
                result["labels"] = {key: formats.get(key) for key in ("year_range", "stage", "relative_survival_interval")}
    except Exception as error:
        result = {"error": type(error).__name__, "status": getattr(error, "code", None)}
    report = {"url": url, "seconds": round(time.monotonic()-start, 2), **result}
    (output / f"{index}-summary.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report), flush=True)
