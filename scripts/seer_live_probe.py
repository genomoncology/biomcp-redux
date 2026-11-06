"""Bound public SEER calls; report status/timing without caller credentials."""
import json
import time
import urllib.error
import urllib.request

BASE = "https://seer.cancer.gov/statistics-network/explorer/source/content_writers/"
PATHS = [
    BASE + "get_var_formats.php",
    BASE + "render_region_5.php?site=40&data_type=1&graph_type=10&compareBy=sex&sex=1&race=1&age_range=1&year_range=5&data_model=4",
    BASE + "render_region_5.php?site=40&data_type=4&graph_type=1&compareBy=sex&sex=1&race=1&age_range=1&relative_survival_interval=5&year_range=31&data_model=4",
    "https://biomcp.genomoncology.com/mcp",
]
for url in PATHS:
    start = time.monotonic()
    try:
        with urllib.request.urlopen(url, timeout=15) as response:
            body = response.read(4_000_000)
            result = {"status": response.status, "bytes": len(body)}
            if url.endswith("get_var_formats.php"):
                catalog = json.loads(body)
                formats = catalog.get("VariableFormats", {})
                result["format_keys"] = list(formats)
                result["labels"] = {key: formats.get(key) for key in ("year_range", "stage", "relative_survival_interval")}
    except Exception as error:
        result = {"error": type(error).__name__, "status": getattr(error, "code", None)}
    print(json.dumps({"url": url, "seconds": round(time.monotonic()-start, 2), **result}), flush=True)
