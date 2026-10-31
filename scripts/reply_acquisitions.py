"""Resolve catalog acquisition events only from captured execution boundaries."""
import json


def catalog_root(execution):
    args = execution['args']
    args = args[:args.index('--')] if '--' in args else args
    if args.count('--case-cache') == 1 and args.index('--case-cache') + 1 < len(args):
        return args[args.index('--case-cache') + 1]
    return execution.get('root')


def document(execution):
    try:
        value = json.loads(bytes(execution['stdout']))
        if isinstance(value, dict) and value.get('jsonrpc') == '2.0':
            value = value.get('result', {}).get('structuredContent')
        return value if isinstance(value, dict) else {}
    except (ValueError, TypeError, KeyError, AttributeError):
        return {}


def captured_command(args, command):
    """Accept the capture's two path globals and one exact command, fail closed otherwise."""
    if args[-len(command):] != command:
        return False
    prefix = args[:-len(command)]
    return (len(prefix) % 2 == 0 and all(
        prefix[i] in ('--replay', '--case-cache')
        and isinstance(prefix[i + 1], str) and prefix[i + 1].startswith('/')
        for i in range(0, len(prefix), 2)))


def accepted_sync(execution, value, mcp_arguments):
    args = execution['args']
    if execution['surface'] == 'mcp':
        frames = list(mcp_arguments(execution))
        if (len(frames) != 1 or args.count('<requests>') != 1
                or len(args) - args.index('<requests>') != 2
                or args[:1] != ['serve']
                or not captured_command(args[1:args.index('<requests>')] + ['serve'], ['serve'])):
            return False
        args = frames[0][2]
    elif execution['surface'] != 'cli':
        return False
    # This Document catalog cannot revalidate with 304: download passes old=None.
    # One CLI command / MCP frame has no prior Fetch-local replay import to reuse.
    # Other sources and unsupported launch syntax receive no event declaration.
    query, sources = value.get('query'), value.get('sources')
    return (isinstance(query, dict) and isinstance(sources, list)
            and captured_command(args, ['sync', 'cbioportal-studies'])
            and execution['exit'] == 0 and value.get('schema') == 'biomcp.sync/7'
            and query.get('source') == 'cbioportal'
            and any(isinstance(source, dict) and source.get('source') == 'cbioportal' and source.get('state') == 'ok'
                    for source in sources))


def acquisition_events(records, owner, mcp_arguments):
    """Map an execution to its observed active sync ordinal; never infer old order."""
    owners = {}
    for key, record in records.items():
        owners.setdefault(owner(key), []).extend(record['executions'])
    result = {}
    for executions in owners.values():
        relevant = []
        for execution in executions:
            value = document(execution)
            body = value.get('result')
            snapshot = body.get('snapshot') if isinstance(body, dict) else None
            if isinstance(snapshot, dict) and type(snapshot.get('acquired')) is int:
                relevant.append((execution, value, snapshot))
        if not any('sequence_start' in e or 'sequence_end' in e for e, _, _ in relevant):
            continue
        boundaries = set()
        groups = {}
        for execution, value, snapshot in relevant:
            start, end = (execution.get(k) for k in ('sequence_start', 'sequence_end'))
            if (type(start) is not int or type(end) is not int or not 0 <= start < end
                    or start in boundaries or end in boundaries):
                raise ValueError('invalid or incomplete captured execution sequence')
            boundaries.update((start, end))
            groups.setdefault(catalog_root(execution), []).append(
                (start, end, execution, value, snapshot))
        for entries in groups.values():
            entries.sort(key=lambda entry: entry[0])
            syncs = [entry for entry in entries if accepted_sync(entry[2], entry[3], mcp_arguments)]
            if not syncs:
                continue
            active, ordinal = None, -1
            for start, end, execution, value, snapshot in entries:
                for sync_start, sync_end, sync, _, _ in syncs:
                    if sync is not execution and start < sync_end and sync_start < end:
                        raise ValueError('overlapping catalog acquisition and execution')
                if accepted_sync(execution, value, mcp_arguments):
                    active = snapshot
                    ordinal += 1
                elif active is None or snapshot != active:
                    raise ValueError('catalog read differs from latest accepted sync acquisition')
                result[id(execution)] = ordinal
    return result
