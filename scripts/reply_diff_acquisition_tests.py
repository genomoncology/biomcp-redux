"""Cache acquisitions bind clocks only within their recorded ownership."""
import hashlib
import json

from reply_diff_test_support import ReplyDiffCase


class ReplyAcquisitions(ReplyDiffCase):
    def test_fetched_clocks_bind_only_inside_the_recorded_cache_root(self):
        key = "case/monarch.yaml/step/"
        for directory, side in [(self.base, "base"), (self.head, "head")]:
            for step in range(6):
                clock = [10, 11, 11][step // 2] if side == "base" else 20
                self.write(directory, key + str(step), json.dumps({
                    "result": {"fetched": clock, "name": "same"},
                }).encode(), root=f"/tmp/{side}/{step // 2}")
        result = self.compare()
        self.assertEqual(result.returncode, 0, result.stderr + result.stdout)
        path = self.head / (hashlib.sha256((key + "1").encode()).hexdigest() + ".json")
        original = path.read_text()
        for mutation in ["clock", "public", "root", "missing", "unpaired"]:
            with self.subTest(mutation=mutation):
                record = json.loads(original)
                execution = record["executions"][0]
                if mutation in ("clock", "public"):
                    reply = json.loads(bytes(execution["stdout"]))
                    reply["result"]["fetched" if mutation == "clock" else "name"] = 21
                    execution["stdout"] = list(json.dumps(reply).encode())
                elif mutation == "root":
                    execution.pop("root")
                elif mutation == "missing":
                    record["executions"] = []
                else:
                    record["executions"].append(dict(execution))
                path.write_text(json.dumps(record))
                self.assertNotEqual(self.compare().returncode, 0)
        path.write_text(original)

    def test_pool_clocks_stay_consistent_per_recorded_root_and_pool(self):
        for directory, side in [(self.base, "base"), (self.head, "head")]:
            for step in range(4):
                created = 10 + step // 2 if side == "base" else 20
                self.write(directory, "case/pool.yaml/step/" + str(step), json.dumps({
                    "page": {"pool": {"id": side, "created": created,
                                      "expires": created + 90, "size": 1}},
                }).encode(), root=f"/tmp/{side}/{step // 2}")
        result = self.compare()
        self.assertEqual(result.returncode, 0, result.stderr + result.stdout)
        self.write(self.head, "case/pool.yaml/step/1", json.dumps({
            "page": {"pool": {"id": "head", "created": 21, "expires": 111, "size": 1}},
        }).encode(), root="/tmp/head/0")
        self.assertNotEqual(self.compare().returncode, 0)

    def test_native_request_metadata_compares_exactly_in_its_own_schema(self):
        key = "cache/mychem-search/optional/requests"
        for directory in (self.base, self.head):
            self.write(directory, "case/help.yaml/step/0")
            (directory / "native.json").write_text(json.dumps({
                "key": key, "executions": [[["CONNECT host", "GET /native"]]],
            }))
        result = self.compare()
        self.assertEqual(result.returncode, 0, result.stderr + result.stdout)
        for mutation in ["changed", "missing", "malformed"]:
            path = self.head / "native.json"
            if mutation == "missing":
                path.unlink()
            else:
                requests = [[["CONNECT host", "GET /changed"]]] if mutation == "changed" else [{}]
                path.write_text(json.dumps({"key": key, "executions": requests}))
            result = self.compare()
            self.assertNotEqual(result.returncode, 0)

    def catalog_sync_capture(self, directory, side, mutation=None):
        root = "/tmp/" + side
        for step in range(4):
            clock = 10 if side == "base" else (20 if step == 0 else 21)
            sync = step < 2 or step == 3
            stale = step == 3
            value = {"schema": "biomcp.sync/7" if sync else "biomcp.study-page/5",
                     "status": "partial" if stale else "complete",
                     "query": {"source": "cbioportal"},
                     "sources": [{"source": "cbioportal", "state": "stale" if stale else "ok"}],
                     "result": {"snapshot": {"revision": "same", "acquired": clock}}}
            args = ["sync", "cbioportal-studies"] if sync else ["search", "study", "ATLAS"]
            surface = "cli"
            if step == 1:
                if mutation == "delimiter":
                    args = ["--"] + args
                if mutation == "flag-value":
                    args = ["--case-cache"] + args
                frame = {"jsonrpc": "2.0", "method": "tools/call",
                         "params": {"name": "biomcp", "arguments": {"args": args}}}
                args = ["serve", "<requests>", json.dumps(frame)]
                surface = "mcp"
                value = {"jsonrpc": "2.0", "result": {"structuredContent": value}}
            extra = dict(sequence_start=step * 10, sequence_end=step * 10 + 8)
            if mutation == "superseded" and step == 2:
                value["result"]["snapshot"]["acquired"] = 20
            if mutation == "overlap" and step == 2:
                extra["sequence_start"] = 15
            if mutation == "missing" and step == 2:
                extra = {}
            if mutation == "invalid" and step == 2:
                extra["sequence_start"] = "4"
            if mutation == "duplicate" and step == 2:
                extra["sequence_end"] = 38
            self.write(directory, "case/sync.yaml/step/" + str(step),
                       json.dumps(value).encode(), args=args, root=root, surface=surface, **extra)

    def test_catalog_reads_follow_latest_accepted_sync_and_refusals_keep_it(self):
        self.catalog_sync_capture(self.base, "base")
        self.catalog_sync_capture(self.head, "head")
        result = self.compare()
        self.assertEqual(result.returncode, 0, result.stderr + result.stdout)
        for mutation in ["superseded", "overlap", "missing", "invalid", "duplicate",
                         "delimiter", "flag-value"]:
            with self.subTest(mutation=mutation):
                self.catalog_sync_capture(self.head, "head", mutation)
                self.assertNotEqual(self.compare().returncode, 0)

    def test_case_cache_after_delimiter_does_not_declare_catalog_ownership(self):
        for directory, side in [(self.base, "base"), (self.head, "head")]:
            root = "/tmp/" + side
            for step in range(2):
                self.write(directory, "case/delimiter.yaml/step/" + str(step), json.dumps({
                    "result": {"snapshot": {"revision": "same",
                        "acquired": 10 if side == "base" else 20 + step}},
                }).encode(), root=root,
                    args=["help", "--", "--case-cache", root + "/cache" + str(step)])
        self.assertNotEqual(self.compare().returncode, 0)

    def test_catalog_acquisition_binds_per_root_and_revision(self):
        key = "case/catalog.yaml/step/"
        for directory, side in [(self.base, "base"), (self.head, "head")]:
            for step, (root, revision, clock) in enumerate([
                ("first", "old", 10), ("second", "old", 11),
                ("first", "new", 11), ("first", "old", 10),
            ]):
                self.write(directory, key + str(step), json.dumps({
                    "result": {"snapshot": {"revision": revision,
                                              "acquired": clock if side == "base" else 20}},
                }).encode(), root="/tmp/" + side + "/" + root)
        result = self.compare()
        self.assertEqual(result.returncode, 0, result.stderr + result.stdout)
        # A coincidence between independent roots does not permit one epoch to change.
        self.write(self.head, key + "3", json.dumps({
            "result": {"snapshot": {"revision": "old", "acquired": 21}},
        }).encode(), root="/tmp/head/first")
        self.assertNotEqual(self.compare().returncode, 0)
        self.write(self.head, key + "3", json.dumps({
            "result": {"snapshot": {"revision": "old", "acquired": 20}},
        }).encode(), root="/tmp/head/second")
        self.assertNotEqual(self.compare().returncode, 0)
