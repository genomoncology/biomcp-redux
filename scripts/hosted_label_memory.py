"""Measure a hosted HTTP server's peak resident memory while its call slots read large drug labels.

Usage: python3 scripts/hosted_label_memory.py BIOMCP_BINARY

Each scenario starts a fresh `serve --hosted --http` process over replayed openFDA label pages, so
nothing goes live, sends its calls at once, and reports the server's resident memory before the
calls, the highest sampled value during them and the operating system's peak for the process.
The metformin label lookup reads eight 50-label pages; its skip=50 page is the largest recorded
label reply. The page is served from a sidecar file, as a network read hands over one buffer, and
the scaled scenario repeats each label's section text until the page nears the 32 MiB page bound.
"""
import json
import os
import shutil
import subprocess
import sys
import tempfile
import threading
import time
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RECORDED = ROOT / "tests/replies/drug/label-many-names/openfda-label"
ARGS = ["get", "drug", "metformin", "label", "--label-section", "indications_and_usage"]
SLOTS = 8  # DEFAULT_HTTP_CAPACITY in crates/biomcp/src/config.rs
HOSTED_SLOTS = 32  # the call limit in the HTTP capacity example of docs/configuration.md
PER_USER = 4  # calls that share one caller key in the 32-call scenarios
PAGE_BYTES = 32 * 1024 * 1024  # openfda_label PAGE_BYTES
NEAR_BOUND = 30 * 1024 * 1024
KEY_HEADER = "x-biomcp-key-openfda-api-key"
IDENTITY = {"effective_time", "id", "openfda", "set_id", "version", "spl_product_data_elements"}


def replies(target, scale, calls):
    """Copy the recorded pages, one exchange per call, with the largest body in a sidecar."""
    folder = target / "openfda-label"
    folder.mkdir(parents=True)
    largest = 0
    for path in RECORDED.glob("*.json"):
        recording = json.loads(path.read_text())
        exchange = recording["exchanges"][0]
        exchange["delay_ms"] = 0
        body = json.dumps(exchange["body"]["value"], separators=(",", ":")).encode()
        if len(body) > 1024 * 1024:
            if scale:
                page = exchange["body"]["value"]
                times = max(1, NEAR_BOUND // len(body))
                for label in page["results"]:
                    for name, value in label.items():
                        if name not in IDENTITY and isinstance(value, list):
                            label[name] = value * times
                body = json.dumps(page, separators=(",", ":")).encode()
                if len(body) > PAGE_BYTES:
                    raise SystemExit(f"scaled page {len(body)} bytes passes the page bound")
            sidecar = path.stem[:16] + ".bin"
            (folder / sidecar).write_bytes(body)
            exchange["body"] = {"kind": "sidecar", "value": sidecar}
            largest = max(largest, len(body))
        recording["exchanges"] = [exchange] * calls
        (folder / path.name).write_text(json.dumps(recording))
    return largest


def rss_kib(pid):
    result = subprocess.run(["ps", "-o", "rss=", "-p", str(pid)], capture_output=True, text=True)
    return int(result.stdout.strip() or 0)


def post(address, index, users):
    body = json.dumps({"jsonrpc": "2.0", "id": index, "method": "tools/call",
                       "params": {"name": "biomcp", "arguments": {"args": ARGS}}}).encode()
    headers = {"Content-Type": "application/json", "Accept": "application/json, text/event-stream",
               "MCP-Protocol-Version": "2025-11-25"}
    if users:
        # Each caller key has its own openFDA bucket, as separate callers have.
        headers[KEY_HEADER] = f"synthetic-measurement-key-{index % users}"
    request = urllib.request.Request(f"http://{address}/mcp", data=body, headers=headers)
    with urllib.request.urlopen(request, timeout=120) as response:
        reply = json.loads(response.read())
    result = reply.get("result", {})
    content = result.get("structuredContent") or json.loads(result["content"][0]["text"])
    status = content.get("status")
    if result.get("isError") or status != "complete":
        raise RuntimeError(f"call {index}: {json.dumps(reply)[:2000]}")
    return content["result"]["identity"]["name"]


def scenario(binary, name, calls, users, scale):
    with tempfile.TemporaryDirectory(prefix="biomcp-label-memory-") as temporary:
        root = Path(temporary)
        for folder in ("home", "config/biomcp", "cache", "replies"):
            (root / folder).mkdir(parents=True)
        (root / "config/biomcp/config.json").write_text(json.dumps({"source_data_max_age_days": 1_000_000}))
        page = replies(root / "replies", scale, calls)
        env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"), "HOME": str(root / "home"),
               "XDG_CONFIG_HOME": str(root / "config"), "XDG_CACHE_HOME": str(root / "cache")}
        server = subprocess.Popen(
            [binary, "serve", "--hosted", "--http", "127.0.0.1:0", "--replay", str(root / "replies"),
             "--case-cache", str(root / "case-cache"), "--http_max_calls", str(max(calls, SLOTS)),
             "--http_max_connections", str(2 * max(calls, SLOTS))],
            cwd=root, env=env, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
            text=True)
        try:
            line = server.stderr.readline()
            if not line.startswith("{"):
                server.terminate()
                raise RuntimeError(f"server did not start: {line}{server.stderr.read()[:2000]}")
            ready = json.loads(line)
            address = ready["listening"]
            threading.Thread(target=lambda: shutil.copyfileobj(server.stderr, open(os.devnull, "w")),
                             daemon=True).start()
            urllib.request.urlopen(f"http://{address}/health", timeout=30).read()
            idle = rss_kib(server.pid)
            samples, done = [], threading.Event()

            def sample():
                while not done.is_set():
                    samples.append(rss_kib(server.pid))
                    time.sleep(0.02)
            sampler = threading.Thread(target=sample)
            sampler.start()
            barrier, outcomes = threading.Barrier(calls), [None] * calls

            def call(index):
                barrier.wait()
                try:
                    outcomes[index] = post(address, index, users)
                except Exception as error:  # every failure is reported with the measurement
                    outcomes[index] = error
            started = time.monotonic()
            workers = [threading.Thread(target=call, args=(index,)) for index in range(calls)]
            for worker in workers:
                worker.start()
            for worker in workers:
                worker.join()
            elapsed = time.monotonic() - started
            done.set()
            sampler.join()
            after = rss_kib(server.pid)
        finally:
            server.terminate()
        _, _, usage = os.wait4(server.pid, 0)
        server.returncode = 0
        # ru_maxrss is bytes on macOS and KiB on Linux.
        peak = usage.ru_maxrss // (1024 if sys.platform == "darwin" else 1)
        failures = [str(outcome) for outcome in outcomes if not isinstance(outcome, str)]
        return {"scenario": name, "calls": calls, "caller_keys": users, "largest_page_bytes": page,
                "idle_rss_mib": round(idle / 1024, 1), "sampled_peak_rss_mib": round(max(samples) / 1024, 1),
                "process_peak_rss_mib": round(peak / 1024, 1), "rss_after_mib": round(after / 1024, 1),
                "peak_over_idle_mib": round((peak - idle) / 1024, 1), "seconds": round(elapsed, 2),
                "labels": sorted({o for o in outcomes if isinstance(o, str)}), "failures": failures}


def main():
    binary = os.path.abspath(sys.argv[1])
    runs = [("one call", 1, 0, False), ("8 calls, keyless", SLOTS, 0, False),
            ("8 calls, 8 caller keys", SLOTS, SLOTS, False),
            ("8 calls, 8 caller keys, page scaled near 32 MiB", SLOTS, SLOTS, True),
            ("32 calls, 8 caller keys", HOSTED_SLOTS, HOSTED_SLOTS // PER_USER, False),
            ("32 calls, 8 caller keys, page scaled near 32 MiB", HOSTED_SLOTS, HOSTED_SLOTS // PER_USER, True)]
    results = []
    for run in runs:
        try:
            result = scenario(binary, *run)
        except Exception as error:  # report the scenario and go on to the next
            result = {"scenario": run[0], "failures": [repr(error)]}
        results.append(result)
        print(json.dumps(result), flush=True)
    print(json.dumps({"benchmark": "hosted-label-memory", "host": sys.platform,
                      "machine": os.uname().machine, "binary": binary}))
    return 1 if any(result["failures"] for result in results) else 0


if __name__ == "__main__":
    sys.exit(main())
