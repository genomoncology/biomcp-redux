"""Shared live census transport and sampling."""
from dataclasses import dataclass
import json
import os
from pathlib import Path
import random
import subprocess
import time
from urllib.error import HTTPError, URLError
from urllib.parse import urlencode
from urllib.request import Request, urlopen

from nightly_live_check import PASSED_ENVIRONMENT, RETRY_WAIT

MAX_SOURCE_BYTES = 64 * 1024 * 1024
SAMPLE_SIZE = 8


class CensusError(ValueError):
    """A diagnostic containing only fixed text or an enumerated reason."""


class TransientSourceError(CensusError):
    """A source could not answer before its deadline or was unavailable."""


@dataclass(frozen=True)
class Measurement:
    total: int
    reached: int
    sampled: int

    def __post_init__(self):
        if not 0 <= self.reached <= self.sampled <= self.total:
            raise ValueError("inconsistent measurement")


class Census:
    def __init__(self, executable, home):
        self.executable = executable
        self.home = Path(home)
        self.environment = {name: os.environ[name] for name in PASSED_ENVIRONMENT
                            if name in os.environ}
        self.environment.update(HOME=home, XDG_CACHE_HOME=home + "/cache",
                                XDG_CONFIG_HOME=home + "/config")
        self.last_call = None
        self.recording = 0

    def pace(self):
        if self.last_call is not None:
            remaining = 1 - (time.monotonic() - self.last_call)
            if remaining > 0:
                time.sleep(remaining)
        self.last_call = time.monotonic()

    def native(self, url, params, headers=None, payload=None, *, raw=False):
        request_headers = {"User-Agent": "BioMCP weekly reach census"}
        if not raw:
            request_headers["Accept"] = "application/json"
        request_headers.update(headers or {})
        request = Request(url + ("?" + urlencode(params) if params else ""),
                          data=payload, headers=request_headers)
        try:
            return self.read_native(request, raw)
        except (TimeoutError, URLError) as error:
            if isinstance(error, HTTPError) and (
                error.code not in {500, 502, 503, 504} or error.headers.get("Retry-After")
            ):
                raise
            time.sleep(RETRY_WAIT)
        return self.read_native(request, raw)

    def read_native(self, request, raw):
        self.pace()
        with urlopen(request, timeout=30) as response:
            content = response.read(MAX_SOURCE_BYTES + 1)
            if len(content) > MAX_SOURCE_BYTES:
                raise CensusError("native reply exceeds the source capture bound")
            if raw:
                return content, response.headers
            return (json.loads(content) if content.strip() else None), response.headers

    def reply(self, args, record=False, allow_usage=False):
        try:
            return self.read_reply(args, record, allow_usage)
        except (TransientSourceError, subprocess.TimeoutExpired):
            if record:
                raise
            time.sleep(RETRY_WAIT)
        return self.read_reply(args, record, allow_usage)

    def read_reply(self, args, record, allow_usage):
        self.pace()
        command = [self.executable, "--format", "json"]
        if record:
            self.recording += 1
            folder = f"recording-{self.recording}" if record is True else record
            command += ["--record", str(self.home / folder)]
        process = subprocess.run(command + list(args), env=self.environment,
                                 capture_output=True, timeout=120, check=False)
        reply = json.loads(process.stdout)
        if not isinstance(reply, dict) or "status" not in reply:
            raise CensusError("malformed BioMCP envelope")
        if process.returncode == 2 and not allow_usage:
            raise CensusError("BioMCP rejected census arguments")
        for source in reply.get("sources", []):
            reason = source.get("reason")
            if reason in {"deadline-exceeded", "unavailable"}:
                raise TransientSourceError(f"BioMCP could not observe the source: {reason}")
            if reason in {"rate-limited", "disabled", "offline", "no-key", "tls", "proxy",
                          "local-quota"}:
                raise CensusError(f"BioMCP could not observe the source: {reason}")
        return reply


def offsets(label, total, count=SAMPLE_SIZE):
    return sorted(random.Random(label).sample(range(total), min(total, count)))


def read(document, *path):
    for key in path:
        if isinstance(document, list) and isinstance(key, int) and 0 <= key < len(document):
            document = document[key]
        elif isinstance(document, dict):
            document = document.get(key)
        else:
            return None
    return document


def source_page(census, label, fetch, probe):
    """Sample exact native offsets while checking that the reported total is stable."""
    total, first = fetch(0)
    reached = 0
    sample = offsets(label, total)
    for offset in sample:
        observed, row = (total, first) if offset == 0 else fetch(offset)
        if observed != total or row is None:
            raise CensusError("native population changed during its sample")
        reached += bool(probe(row))
    return Measurement(total, reached, len(sample))


def sample_rows(label, rows, probe, total=None):
    sample = offsets(label, len(rows))
    reached = sum(bool(probe(rows[index])) for index in sample)
    return Measurement(len(rows) if total is None else total, reached, len(sample))
