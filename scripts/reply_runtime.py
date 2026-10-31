"""Bind declared runtime values across one complete pair of captures."""
import hashlib
import json
import re
from pathlib import Path


def owner(key):
    if isinstance(key, str):
        if key.startswith('case/'):
            parts = key.rsplit('/step/', 1)
            if len(parts) == 2 and re.fullmatch(r'0|[1-9][0-9]*', parts[1]):
                name = parts[0]
                if all(p not in ('', '.', '..') for p in name.split('/')[1:]):
                    return name
        elif key.startswith('trial/'):
            parts = key.rsplit('/', 2)
            if (len(parts) == 3 and re.fullmatch(r'[0-9a-f]{64}', parts[1])
                    and re.fullmatch(r'0|[1-9][0-9]*', parts[2])
                    and parts[0].startswith('trial/')
                    and all(p not in ('', '.', '..') for p in parts[0].split('/')[1:])):
                return parts[0]
    raise ValueError('invalid transcript key')


class Bindings:
    def __init__(self):
        self.forward = {}
        self.reverse = {}
        self.pools = ({}, {})

    def bind(self, kind, left, right):
        a, b = (kind, left), (kind, right)
        if self.forward.setdefault(a, right) != right or self.reverse.setdefault(b, left) != left:
            raise ValueError(f'inconsistent runtime binding: {kind}')
        return '<runtime>'

    def values(self, left, right, scope, path=(), encoded=(), clocks=True, catalog=None):
        """Return only authorized scalar substitutions, indexed by JSON path."""
        changes = ({}, {})
        def envelope(value):
            if isinstance(value, dict) and value.get('jsonrpc') == '2.0':
                result = value.get('result')
                value = result.get('structuredContent') if isinstance(result, dict) else None
            return value if isinstance(value, dict) else {}
        documents = [envelope(value) for value in (left, right)]
        def schema(prefix):
            return all(str(value.get('schema', '')).startswith(prefix) for value in documents)
        def put(p, a, b, kind):
            if type(a) is not type(b):
                return
            self.bind(kind, a, b)
            changes[0][p] = changes[1][p] = '<runtime>'
        def visit(a, b, p):
            position = p[len(path):]
            if position[:2] == ('result', 'structuredContent'):
                position = position[2:]
            if isinstance(a, dict) and isinstance(b, dict):
                fetched = (position == ('result',)
                           or (position[:2] == ('result', 'items') and len(position) in (3, 4)
                               and isinstance(position[2], int)
                               and (len(position) == 3 or position[3] == 'answer')))
                pool = (position[-2:] == ('page', 'pool')
                        and {'id', 'created', 'expires', 'size'} <= a.keys() & b.keys())
                if pool:
                    if not all(type(x[k]) is int for x in (a, b) for k in ('created', 'expires')):
                        raise ValueError('invalid pool clocks')
                    if a['expires'] - a['created'] != b['expires'] - b['created']:
                        raise ValueError('changed pool lifetime')
                    for index, value in enumerate((a, b)):
                        pool_clocks = (value['created'], value['expires'])
                        if self.pools[index].setdefault((scope, value['id']), pool_clocks) != pool_clocks:
                            raise ValueError('inconsistent pool clocks')
                    put(p + ('id',), a['id'], b['id'], (scope, 'pool'))
                    for k in ('created', 'expires'):
                        changes[0][p + (k,)] = changes[1][p + (k,)] = '<runtime>'
                for k in a.keys() & b.keys():
                    child = p + (k,)
                    if pool and k in ('id', 'created', 'expires'):
                        continue
                    if clocks and ((k == 'fetched' and fetched)
                                   or (k == 'acquired' and position == ('result', 'snapshot'))):
                        if type(a[k]) is int and type(b[k]) is int:
                            kind = (scope, k)
                            if k == 'acquired':
                                if (not catalog or not all(catalog) or not isinstance(a.get('revision'), str)
                                        or a.get('revision') != b.get('revision')):
                                    continue
                                self.bind((scope, 'catalog-root'), *catalog)
                                kind += (catalog[0], a['revision'])
                            put(child, a[k], b[k], kind)
                    elif (k == 'pool' and position[-1:] == ('query',)
                          and isinstance(a[k], str) and isinstance(b[k], str)):
                        if (((scope, 'pool'), a[k]) in self.forward
                                or ((scope, 'pool'), b[k]) in self.reverse):
                            put(child, a[k], b[k], (scope, 'pool'))
                    elif (k == 'pool' and schema('biomcp.trial-cards/') and len(position) == 7
                          and position[:2] == ('result', 'items') and isinstance(position[2], int)
                          and position[3] == 'sections' and position[5:] == ('data', 'continuation')
                          and isinstance(a[k], str) and isinstance(b[k], str)):
                        put(child, a[k], b[k], (scope, 'pool'))
                    elif (k == 'snapshot' and position == ('query',)
                          and isinstance(a[k], str) and isinstance(b[k], str)
                          and ((scope, 'snapshot'), a[k]) in self.forward):
                        put(child, a[k], b[k], (scope, 'snapshot'))
                    else:
                        visit(a[k], b[k], child)
            elif isinstance(a, list) and isinstance(b, list):
                argv = (not p or p == ('params', 'arguments', 'args')
                        or (len(position) == 3 and position[0] == 'next'
                            and isinstance(position[1], int) and position[2] == 'args'))
                for i, (x, y) in enumerate(zip(a, b)):
                    if (argv and i and a[i-1] == b[i-1] == '--pool'
                            and isinstance(x, str) and isinstance(y, str)):
                        if (len(position) == 3 and position[0] == 'next'
                                or ((scope, 'pool'), x) in self.forward
                                or ((scope, 'pool'), y) in self.reverse):
                            put(p + (i,), x, y, (scope, 'pool'))
                    elif (argv and i and a[i-1] == b[i-1] == '--snapshot'
                          and isinstance(x, str) and isinstance(y, str)):
                        # Phenotype section steps own opaque retained source snapshots.
                        declared = (len(position) == 3 and position[0] == 'next'
                                    and schema('biomcp.phenotype-card/'))
                        if declared or ((scope, 'snapshot'), x) in self.forward:
                            put(p + (i,), x, y, (scope, 'snapshot'))
                    else:
                        visit(x, y, p + (i,))
            elif p in encoded and isinstance(a, str) and isinstance(b, str):
                try:
                    aa, bb = json.loads(a), json.loads(b)
                except ValueError:
                    return
                # MCP embeds complete JSON replies inside text content.
                if isinstance(aa, (dict, list)) and isinstance(bb, (dict, list)):
                    nested = self.values(aa, bb, scope, p, clocks=clocks, catalog=catalog)
                    for index in range(2):
                        if nested[index]:
                            changes[index][p] = nested[index]
        visit(left, right, path)
        return changes


def rewrite(text, changes, path=(), spans=False):
    """Replace selected JSON leaves while preserving every other original byte."""
    decoder = json.JSONDecoder()
    def space(i):
        while i < len(text) and text[i].isspace():
            i += 1
        return i
    edits = []
    def parse(i, p):
        i = space(i)
        if text[i] == '{':
            i = space(i + 1)
            while text[i] != '}':
                key, end = decoder.raw_decode(text, i)
                i = parse(space(end) + 1, p + (key,))
                i = space(i)
                if text[i] != ',':
                    break
                i = space(i + 1)
            return i + 1
        if text[i] == '[':
            i, n = space(i + 1), 0
            while text[i] != ']':
                i = space(parse(i, p + (n,)))
                n += 1
                if text[i] != ',':
                    break
                i = space(i + 1)
            return i + 1
        value, end = decoder.raw_decode(text, i)
        if p in changes:
            replacement = changes[p]
            if isinstance(replacement, dict):
                # Map decoded characters back to their original JSON escapes.
                # Only runtime leaf spans may change inside an MCP text string.
                offsets, cursor = [i + 1], i + 1
                while cursor < end - 1:
                    if text[cursor] == '\\':
                        if text[cursor + 1] == 'u':
                            code = int(text[cursor + 2:cursor + 6], 16)
                            cursor += 6
                            if (0xD800 <= code <= 0xDBFF and text[cursor:cursor + 2] == '\\u'
                                    and 0xDC00 <= int(text[cursor + 2:cursor + 6], 16) <= 0xDFFF):
                                cursor += 6
                        else:
                            cursor += 2
                    else:
                        cursor += 1
                    offsets.append(cursor)
                for start, finish, segment in rewrite(value, replacement, p, spans=True):
                    edits.append((offsets[start], offsets[finish],
                                  json.dumps(segment, ensure_ascii=False)[1:-1]))
            else:
                edits.append((i, end, json.dumps(replacement, ensure_ascii=False)))
        return end
    parse(0, path)
    if spans:
        return edits
    for start, end, value in reversed(edits):
        text = text[:start] + value + text[end:]
    return text


def replay_owners(records, directory, entries, evidence):
    """Validate supplemental reviewed evidence without altering original captures.

    Source/helper hashes bind the files reviewed by the map's author. They do
    not assert that a hash proves the helper's ownership or existed in a capture.
    """
    roots = {}
    def checked_file(parent, binding):
        path = Path(binding['file'])
        if path.is_absolute() or '..' in path.parts or not path.parts:
            raise ValueError('invalid ownership evidence path')
        data = (Path(parent) / path).read_bytes()
        if hashlib.sha256(data).hexdigest() != binding['sha256']:
            raise ValueError('ownership evidence hash mismatch')
        return data
    if not isinstance(entries, list):
        raise ValueError('ownership declarations must be a list')
    for entry in entries:
        key, index = entry['key'], entry['execution']
        if type(index) is not int or index < 0:
            raise ValueError('invalid owned execution')
        context = key + '/execution/' + str(index)
        if context in roots:
            raise ValueError('duplicate replay ownership context')
        capture = json.loads(checked_file(directory, entry['capture']))
        if capture.get('key') != key or capture != records.get(key):
            raise ValueError('ownership evidence capture key mismatch')
        if index >= len(capture['executions']):
            raise ValueError('owned execution is absent')
        execution = capture['executions'][index]
        args, path = execution['args'], entry['replay_root']
        if (args.count('--replay') != 1 or args.index('--replay') + 1 >= len(args)
                or ('--' in args and args.index('--') < args.index('--replay'))
                or args[args.index('--replay') + 1] != path):
            raise ValueError('owned root is not the actual replay argument')
        if not isinstance(path, str) or not path.startswith('/') or any(
                part in ('', '.', '..') for part in path.split('/')[1:]):
            raise ValueError('invalid owned replay root')
        if entry['source']['helper'] not in ('checked_case', 'checked_steps'):
            raise ValueError('invalid reviewed response-case helper')
        checked_file(evidence, entry['source'])
        checked_file(evidence, entry['helper_source'])
        roots[context] = {path: {'replay': Path(path).name}}
    return roots


def context_roots(roots, key, index):
    return dict(roots.get(owner(key), {}), **roots.get(key + '/execution/' + str(index), {}))


def record_roots(records):
    roots = {}
    for key, record in records.items():
        for execution in record['executions']:
            args = execution['args']
            for flag in ('--record', '--replay'):
                if flag in args:
                    value = args[args.index(flag) + 1]
                    root = execution.get('root', '')
                    if root and value.startswith(root + '/'):
                        roots.setdefault(owner(key), {})[value] = value[len(root):]
    return roots


def declared_args(execution, roots):
    args = execution['args']
    result = []
    for i, arg in enumerate(args):
        for path, name in roots.items():
            # A declared record directory may be passed by another isolated process.
            if isinstance(name, dict):
                # Supplemental response-case ownership covers this exact argv slot.
                if i and args[i - 1] == '--replay' and arg == path:
                    arg = '<response-cases>/' + name['replay']
            elif arg == path:
                arg = '<record-root>' + name
        result.append(arg)
    return result


def align(base, head, normalize, bindings=None, owned=None):
    """Alias changed trial hashes through declared paths and bound opaque IDs."""
    owned = owned if owned is not None else ({}, {})
    roots = tuple(dict(record_roots(records), **extra)
                  for records, extra in zip((base, head), owned))
    bindings = bindings if bindings is not None else Bindings()
    # Shared capture keys declare runtime IDs before hashed requests are aligned.
    for key in sorted(base.keys() & head.keys()):
        for i, (a, b) in enumerate(zip(base[key]['executions'], head[key]['executions'])):
            declared = [context_roots(r, key, i) for r in roots]
            pair(a, b, owner(key), bindings, declared, normalize)
    def signature(key, record, index):
        values = []
        for i, e in enumerate(record['executions']):
            n = normalize(dict(e, args=declared_args(e, context_roots(roots[index], key, i))))
            for i, arg in enumerate(n['args']):
                if i and n['args'][i-1] in ('--pool', '--snapshot'):
                    kind = (owner(key), n['args'][i-1][2:])
                    if index == 0 and (kind, arg) in bindings.forward:
                        n['args'][i] = '<bound>/' + arg
                    elif index == 1 and (kind, arg) in bindings.reverse:
                        n['args'][i] = '<bound>/' + bindings.reverse[kind, arg]
            values.append({k: n[k] for k in ('args', 'surface')})
        return owner(key), key.rsplit('/', 1)[-1], json.dumps(values, sort_keys=True)
    def valid_hash(key, record):
        parts = key.rsplit('/', 2)
        def stable(e):
            args = e['args']
            fields = [('root', '<root>'), ('checkout', '<checkout>/crates/biomcp')]
            for field, replacement in fields:
                if e.get(field):
                    args = [a.replace(e[field], replacement) for a in args]
            return args
        return (key.startswith('trial/') and len(parts) == 3 and
                all(hashlib.sha256(json.dumps(stable(e), ensure_ascii=False,
                    separators=(',', ':')).encode()).hexdigest() == parts[1]
                    for e in record['executions']))
    unmatched = (base.keys() - head.keys(), head.keys() - base.keys())
    candidates = {}
    for key in unmatched[1]:
        if valid_hash(key, head[key]):
            candidates.setdefault(signature(key, head[key], 1), []).append(key)
    aliases = {}
    for key in unmatched[0]:
        if not valid_hash(key, base[key]):
            continue
        matches = candidates.get(signature(key, base[key], 0), [])
        if len(matches) == 1:
            target = matches.pop()
            aliases[key] = target
    return roots, aliases


def pair(left, right, scope, bindings, roots, normalize):
    external = [[path for path, name in declared.items() if isinstance(name, dict)]
                for declared in roots]
    if all(len(paths) == 1 for paths in external):
        bindings.bind((scope, 'response-cases'), external[0][0], external[1][0])
    a, b = [normalize(dict(e, args=declared_args(e, declared)))
            for e, declared in zip((left, right), roots)]
    args = bindings.values(a['args'], b['args'], scope, clocks=False)
    for n, changes in ((a, args[0]), (b, args[1])):
        for p, value in changes.items():
            # Executed argv can also contain a JSON MCP request token.
            n['args'][p[0]] = value
    # Only captured MCP request frames own JSON-encoded argument vectors.
    if (left['surface'] == right['surface'] == 'mcp'
            and '<requests>' in a['args'] and '<requests>' in b['args']):
        start = a['args'].index('<requests>') + 1
        if start == b['args'].index('<requests>') + 1:
            for i in range(start, min(len(a['args']), len(b['args']))):
                try:
                    frames = [json.loads(n['args'][i]) for n in (a, b)]
                    if not all(f.get('jsonrpc') == '2.0' and f.get('method') == 'tools/call'
                               and f['params'].get('name') == 'biomcp' for f in frames):
                        continue
                    vectors = [f['params']['arguments']['args'] for f in frames]
                except (ValueError, KeyError, TypeError, AttributeError):
                    continue
                path = ('params', 'arguments', 'args')
                changes = bindings.values(*vectors, scope, path, clocks=False)
                for n, change in zip((a, b), changes):
                    n['args'][i] = rewrite(n['args'][i], change)
    try:
        values = [json.loads(n['stdout']) for n in (a, b)]
    except ValueError:
        # Markdown clocks bind only to a JSON fetched declaration in this owner.
        for before, after in bindings.forward.items():
            kind, value = before
            if isinstance(kind, tuple) and kind[0] == scope and kind[1] == 'fetched':
                for n, clock in ((a, value), (b, after)):
                    n['stdout'] = n['stdout'].replace(
                        '**fetched:** ' + str(clock) + '\n', '**fetched:** <runtime>\n')
        return a, b
    def embedded(value, execution):
        if (execution['surface'] != 'mcp' or not isinstance(value, dict)
                or value.get('jsonrpc') != '2.0'):
            return set()
        result = value.get('result', {})
        if not isinstance(result, dict):
            return set()
        paths = set()
        for i, item in enumerate(result.get('content', [])):
            if isinstance(item, dict) and item.get('type') == 'text':
                try:
                    if json.loads(item.get('text', '')) == result.get('structuredContent'):
                        paths.add(('result', 'content', i, 'text'))
                except (ValueError, TypeError):
                    pass
        return paths
    encoded = embedded(values[0], left) & embedded(values[1], right)
    def catalog_root(execution):
        args = execution['args']
        if args.count('--case-cache') == 1 and args.index('--case-cache') + 1 < len(args):
            return args[args.index('--case-cache') + 1]
        return execution.get('root')
    changes = bindings.values(*values, scope, encoded=encoded,
                              catalog=tuple(catalog_root(e) for e in (left, right)))
    # Saved paths are owned only when the literal destination was declared.
    for index, execution in enumerate((left, right)):
        args = execution['args']
        if '--save' in args:
            name = args[args.index('--save') + 1]
            path = execution.get('root', '') + '/' + name
            target = ('result', 'file', 'data', 'path')
            value = values[index]
            for key in target:
                value = value.get(key, {}) if isinstance(value, dict) else None
            if value == path and '..' not in name.split('/') and not name.startswith('/'):
                changes[index][target] = '<saved>/' + name
    for n, change in ((a, changes[0]), (b, changes[1])):
        if change:
            n['stdout'] = rewrite(n['stdout'], change)
    return a, b
