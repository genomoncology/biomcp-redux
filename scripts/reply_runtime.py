"""Bind declared runtime values across one complete pair of captures."""
import hashlib
import json


def owner(key):
    return key.rsplit('/step/', 1)[0] if key.startswith('case/') else '/'.join(key.split('/')[:3])


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

    def values(self, left, right, scope, path=()):
        """Return only authorized scalar substitutions, indexed by JSON path."""
        changes = ({}, {})
        def put(p, a, b, kind):
            if type(a) is not type(b):
                return
            self.bind(kind, a, b)
            changes[0][p] = changes[1][p] = '<runtime>'
        def visit(a, b, p):
            if isinstance(a, dict) and isinstance(b, dict):
                pool = p[-1:] == ('pool',) and {'id', 'created', 'expires', 'size'} <= a.keys() & b.keys()
                if pool:
                    if not all(type(x[k]) is int for x in (a, b) for k in ('created', 'expires')):
                        raise ValueError('invalid pool clocks')
                    if a['expires'] - a['created'] != b['expires'] - b['created']:
                        raise ValueError('changed pool lifetime')
                    for index, value in enumerate((a, b)):
                        clocks = (value['created'], value['expires'])
                        if self.pools[index].setdefault(value['id'], clocks) != clocks:
                            raise ValueError('inconsistent pool clocks')
                    put(p + ('id',), a['id'], b['id'], 'pool')
                    for k in ('created', 'expires'):
                        changes[0][p + (k,)] = changes[1][p + (k,)] = '<runtime>'
                for k in a.keys() & b.keys():
                    child = p + (k,)
                    if pool and k in ('id', 'created', 'expires'):
                        continue
                    if k == 'fetched' or (k == 'acquired' and p[-1:] == ('snapshot',)):
                        if type(a[k]) is int and type(b[k]) is int:
                            put(child, a[k], b[k], (scope, k))
                    elif k == 'pool' and p[-1:] == ('query',) and isinstance(a[k], str) and isinstance(b[k], str):
                        put(child, a[k], b[k], 'pool')
                    else:
                        visit(a[k], b[k], child)
            elif isinstance(a, list) and isinstance(b, list):
                for i, (x, y) in enumerate(zip(a, b)):
                    if (not p or p[-1:] == ('args',)) and i and a[i-1] == b[i-1] == '--pool' and isinstance(x, str) and isinstance(y, str):
                        put(p + (i,), x, y, 'pool')
                    else:
                        visit(x, y, p + (i,))
            elif isinstance(a, str) and isinstance(b, str):
                try:
                    aa, bb = json.loads(a), json.loads(b)
                except ValueError:
                    return
                # MCP embeds complete JSON replies inside text content.
                if isinstance(aa, (dict, list)) and isinstance(bb, (dict, list)):
                    nested = self.values(aa, bb, scope, p)
                    for index, value in enumerate((a, b)):
                        if nested[index]:
                            changes[index][p] = rewrite(value, nested[index], p)
        visit(left, right, path)
        return changes


def rewrite(text, changes, path=()):
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
        _, end = decoder.raw_decode(text, i)
        if p in changes:
            edits.append((i, end, json.dumps(changes[p], ensure_ascii=False)))
        return end
    parse(0, path)
    for start, end, value in reversed(edits):
        text = text[:start] + value + text[end:]
    return text


def record_roots(records):
    roots = {}
    for key, record in records.items():
        for execution in record['executions']:
            args = execution['args']
            if '--record' in args:
                value = args[args.index('--record') + 1]
                root = execution.get('root', '')
                if root and value.startswith(root + '/'):
                    roots.setdefault(owner(key), set()).add(root)
    return roots


def declared_args(execution, roots):
    args = execution['args']
    result = []
    for arg in args:
        for root in roots:
            # A declared record directory may be passed by another isolated process.
            if arg.startswith(root + '/'):
                arg = '<record-root>' + arg[len(root):]
        result.append(arg)
    return result


def align(base, head, normalize):
    """Alias changed trial hashes only through their declared record path arguments."""
    roots = (record_roots(base), record_roots(head))
    def signature(key, record, index):
        values = []
        for e in record['executions']:
            n = normalize(dict(e, args=declared_args(e, roots[index].get(owner(key), ()))))
            values.append({k: n[k] for k in ('args', 'surface', 'exit')})
        return owner(key), json.dumps(values, sort_keys=True)
    def valid_hash(key, record):
        parts = key.rsplit('/', 2)
        return (key.startswith('trial/') and len(parts) == 3 and
                all(hashlib.sha256(json.dumps(e['args'], ensure_ascii=False,
                    separators=(',', ':')).encode()).hexdigest() == parts[1]
                    for e in record['executions']))
    unmatched = (base.keys() - head.keys(), head.keys() - base.keys())
    candidates = {}
    for key in unmatched[1]:
        if owner(key) in roots[1] and valid_hash(key, head[key]):
            candidates.setdefault(signature(key, head[key], 1), []).append(key)
    aliases = {}
    for key in unmatched[0]:
        if owner(key) not in roots[0] or not valid_hash(key, base[key]):
            continue
        matches = candidates.get(signature(key, base[key], 0), [])
        if len(matches) == 1:
            target = matches.pop()
            aliases[key] = target
    return roots, aliases


def pair(left, right, scope, bindings, roots, normalize):
    a, b = [normalize(dict(e, args=declared_args(e, declared)))
            for e, declared in zip((left, right), roots)]
    args = bindings.values(a['args'], b['args'], scope)
    for n, changes in ((a, args[0]), (b, args[1])):
        for p, value in changes.items():
            # Executed argv can also contain a JSON MCP request token.
            n['args'][p[0]] = value
    try:
        values = [json.loads(n['stdout']) for n in (a, b)]
    except ValueError:
        # Markdown clocks bind only to a JSON fetched declaration in this owner.
        for before, after in bindings.forward.items():
            kind, value = before
            if isinstance(kind, tuple) and kind[0] == scope and kind[1] == 'fetched':
                for n, clock in ((a, value), (b, after)):
                    n['stdout'] = n['stdout'].replace('**fetched:** ' + str(clock) + '\n', '**fetched:** <runtime>\n')
        return a, b
    changes = bindings.values(*values, scope)
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
