"""Exercise reply comparison through its command line; run outside the routine gate."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

SCRIPT = Path(__file__).with_name("reply_diff.py")


class ReplyDiff(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.base = Path(self.directory.name, "base")
        self.head = Path(self.directory.name, "head")
        self.base.mkdir()
        self.head.mkdir()

    def write(self, directory, key, stdout=b'{"status":"complete"}\n', code=0, **extra):
        execution = dict(args=["help"], surface="cli", exit=code, stdout=list(stdout))
        execution.update(extra)
        path = directory / (hashlib.sha256(key.encode()).hexdigest() + ".json")
        path.write_text(json.dumps(dict(key=key, executions=[execution])))

    def compare(self):
        return subprocess.run([sys.executable, str(SCRIPT), str(self.base), str(self.head)],
                              capture_output=True, text=True, check=False)

    def test_reply_and_exit_changes_name_the_case_pointer(self):
        key = "case/schema/help.yaml/step/0"
        self.write(self.base, key)
        for stdout, code, pointer in [
            (b'{"status":"completE"}\n', 0, "/0/stdout/status"),
            (b'{"status":"complete"}\n', 7, "/0/exit"),
            (b'{ "status":"complete"}\n', 0, "/0/stdout/@character:"),
        ]:
            with self.subTest(pointer=pointer):
                self.write(self.head, key, stdout, code)
                result = self.compare()
                self.assertEqual(result.returncode, 1, result.stderr)
                self.assertIn(key, result.stdout)
                self.assertIn(pointer, result.stdout)

    def test_trial_output_and_arguments_remain_significant(self):
        key = "trial/housekeeping/version/argument-hash/0"
        self.write(self.base, key, b"original\n")
        for output, args in [(b"changed\n", ["help"]), (b"original\n", ["version"])]:
            self.write(self.head, key, output, args=args)
            result = self.compare()
            self.assertEqual(result.returncode, 1, result.stderr)
            self.assertIn(key, result.stdout)

    def test_build_identity_and_owned_paths_are_masked(self):
        key = "case/version.yaml/step/0"
        for directory, suffix in [(self.base, "base"), (self.head, "head")]:
            root = "/tmp/" + suffix
            output = f"biomcp 1.{suffix}\nCommit: {suffix}\nConfig: {root}/config\n"
            self.write(directory, key, output.encode(), args=["version", "--verbose"],
                       version="1." + suffix, root=root)
        result = self.compare()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout, "")

    def test_data_colliding_with_build_metadata_is_never_masked(self):
        key = "case/source.yaml/step/0"
        for field in ["version", "executable", "root", "checkout"]:
            with self.subTest(field=field):
                for directory, value in [(self.base, "1.0.0"), (self.head, "1.0.1")]:
                    self.write(directory, key, json.dumps({"source": value}).encode(),
                               **{field: value})
                result = self.compare()
                self.assertEqual(result.returncode, 1, result.stderr)
                self.assertIn("/0/stdout/source", result.stdout)

    def runtime_capture(self, directory, suffix, *, bad_reference=False,
                        lifetime=90, status="complete"):
        root = "/tmp/" + suffix
        pool = suffix + "-pool"
        common = dict(root=root, args=["search", "trial"])
        self.write(directory, "case/runtime.yaml/step/0", json.dumps({
            "page": {"pool": {"id": pool, "created": 10 if suffix == "base" else 20,
                                "expires": (10 if suffix == "base" else 20) + lifetime, "size": 1}},
            "fetched": 30 if suffix == "base" else 40,
            "next": [{"kind": "page", "args": ["search", "trial", "--pool", pool]}],
            "status": status,
        }).encode(), **common)
        self.write(directory, "case/runtime.yaml/step/1", json.dumps({
            "query": {"pool": pool},
            "result": {"snapshot": {"acquired": 50 if suffix == "base" else 60}},
        }).encode(), root=root,
            args=["search", "trial", "--pool", "broken" if bad_reference else pool])
        self.write(directory, "case/runtime.yaml/step/2",
                   ("# Trial\n**fetched:** " + ("30" if suffix == "base" else "40") + "\n").encode(),
                   **common)
        self.write(directory, "case/runtime.yaml/step/3", json.dumps({
            "result": {"file": {"data": {"path": root + "/reply.json"}}},
        }).encode(), root=root, args=["get", "trial", "--save", "reply.json"])

    def test_runtime_values_bind_across_the_whole_capture(self):
        self.runtime_capture(self.base, "base")
        self.runtime_capture(self.head, "head")
        result = self.compare()
        self.assertEqual(result.returncode, 0, result.stderr + result.stdout)
        self.assertEqual(result.stdout, "")

    def test_runtime_binding_rejects_changed_reference_lifetime_and_data(self):
        self.runtime_capture(self.base, "base")
        for mutation in [dict(bad_reference=True), dict(lifetime=91), dict(status="partial")]:
            with self.subTest(mutation=mutation):
                self.runtime_capture(self.head, "head", **mutation)
                result = self.compare()
                self.assertNotEqual(result.returncode, 0, result.stdout)

    def test_record_path_hash_keys_pair_only_through_declared_names(self):
        def key(args, root=None):
            if root:
                args = [a.replace(root, '<root>') for a in args]
            digest = hashlib.sha256(json.dumps(args, separators=(",", ":")).encode()).hexdigest()
            return "trial/cache/replay/" + digest + "/0"
        for directory, suffix in [(self.base, "base"), (self.head, "head")]:
            root = "/tmp/" + suffix
            args = ["get", "gene", "--record", root + "/record"]
            self.write(directory, key(args, root), args=args, root=root)
            other = "/tmp/other-" + suffix
            args = ["get", "gene", "--config", other + "/config", "--replay", root + "/record"]
            self.write(directory, key(args, other), args=args, root=other)
        result = self.compare()
        self.assertEqual(result.returncode, 0, result.stderr + result.stdout)
        args = ["get", "gene", "--config", "/tmp/other-head/config", "--replay", "/tmp/head/record"]
        changed = args[:-1] + ["/tmp/head/another"]
        self.write(self.head, key(args, '/tmp/other-head'), args=changed, root="/tmp/other-head")
        self.assertEqual(self.compare().returncode, 1)

    def test_json_source_text_and_literal_arguments_remain_significant(self):
        key = 'case/literal.yaml/step/0'
        for place in ['source', 'args', 'mcp-source', 'source-args']:
            with self.subTest(place=place):
                for directory, clock in [(self.base, 10), (self.head, 20)]:
                    literal = json.dumps({'fetched': clock})
                    output = {'result': {'description': literal}}
                    extra = {}
                    if place == 'args':
                        output = {'status': 'complete'}
                        extra['args'] = ['search', 'article', literal]
                    elif place == 'mcp-source':
                        output = {'jsonrpc': '2.0', 'id': 1, 'result': {
                            'structuredContent': output,
                            'content': [{'type': 'text', 'text': json.dumps(output)}]}}
                        extra['surface'] = 'mcp'
                    elif place == 'source-args':
                        output = {'result': {'args': ['search', 'trial', '--pool', literal]}}
                    self.write(directory, key, json.dumps(output).encode(), **extra)
                result = self.compare()
                self.assertEqual(result.returncode, 1, result.stderr + result.stdout)

    def test_mcp_runtime_fields_bind_only_inside_transport_frames(self):
        for directory, suffix in [(self.base, 'base'), (self.head, 'head')]:
            pool = suffix + '-pool'
            reply = {'page': {'pool': {'id': pool, 'created': 10 if suffix == 'base' else 20,
                                      'expires': 100 if suffix == 'base' else 110, 'size': 1}},
                     'query': {'pool': pool}}
            frame = {'jsonrpc': '2.0', 'id': 1, 'result': {
                'structuredContent': reply,
                'content': [{'type': 'text', 'text': json.dumps(reply)}]}}
            request = {'jsonrpc': '2.0', 'id': 1, 'method': 'tools/call', 'params': {
                'name': 'biomcp', 'arguments': {'args': ['search', 'trial', '--pool', pool]}}}
            self.write(directory, 'case/mcp.yaml/step/0', json.dumps(frame).encode(),
                       surface='mcp', args=['serve', '<requests>', json.dumps(request)])
        result = self.compare()
        self.assertEqual(result.returncode, 0, result.stderr + result.stdout)

    def test_mcp_runtime_binding_preserves_unrelated_json_escape_bytes(self):
        for directory, clock in [(self.base, 10), (self.head, 20)]:
            reply = {'fetched': clock, 'description': 'path/a'}
            frame = {'jsonrpc': '2.0', 'id': 1, 'result': {
                'structuredContent': reply,
                'content': [{'type': 'text', 'text': json.dumps(reply)}]}}
            output = json.dumps(frame)
            if directory == self.head:
                # Change only the text string's outer slash spelling.
                output = 'path\\/a'.join(output.rsplit('path/a', 1))
            self.write(directory, 'case/mcp-escape.yaml/step/0', output.encode(), surface='mcp')
        result = self.compare()
        self.assertEqual(result.returncode, 1, result.stderr + result.stdout)

    def test_declared_record_directory_does_not_bind_neighbor_paths(self):
        for directory, suffix in [(self.base, 'base'), (self.head, 'head')]:
            root = '/tmp/' + suffix
            self.write(directory, 'case/record.yaml/step/0',
                       args=['get', 'gene', '--record', root + '/record'], root=root)
            args = ['get', 'gene', '--replay', root + '/unrelated']
            digest = hashlib.sha256(json.dumps(args, separators=(',', ':')).encode()).hexdigest()
            self.write(directory, 'trial/cache/replay/' + digest + '/0', args=args,
                       root='/tmp/other-' + suffix)
            # Declare the record directory in the same trial owner too.
            self.write(directory, 'trial/cache/replay/declaration/0',
                       args=['get', 'gene', '--record', root + '/record'], root=root)
        result = self.compare()
        self.assertEqual(result.returncode, 1, result.stderr + result.stdout)

    def test_record_hash_alias_preserves_trial_occurrence(self):
        for directory, suffix in [(self.base, 'base'), (self.head, 'head')]:
            root = '/tmp/' + suffix
            args = ['get', 'gene', '--record', root + '/record']
            stable = [a.replace(root, '<root>') for a in args]
            digest = hashlib.sha256(json.dumps(stable, separators=(',', ':')).encode()).hexdigest()
            self.write(directory, 'trial/cache/replay/' + digest + '/0', args=args, root=root)
            args = ['get', 'gene', '--replay', root + '/record']
            digest = hashlib.sha256(json.dumps(args, separators=(',', ':')).encode()).hexdigest()
            occurrence = '0' if suffix == 'base' else '1'
            self.write(directory, 'trial/cache/replay/' + digest + '/' + occurrence,
                       args=args, root='/tmp/other-' + suffix)
        result = self.compare()
        self.assertEqual(result.returncode, 1, result.stderr + result.stdout)

    def test_uncaptured_trials_are_disclosed(self):
        for directory in [self.base, self.head]:
            self.write(directory, "case/help.yaml/step/0")
            (directory / "lifecycle.json").write_text(json.dumps(
                dict(kind="trial", name="mcp/broken-output", uncaptured=True)))
        result = self.compare()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("mcp/broken-output", result.stderr)


if __name__ == "__main__":
    unittest.main()
