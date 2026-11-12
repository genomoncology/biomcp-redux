"""One-off for ticket 0843: how often a real drug name reaches ChEMBL in a trial search, and what it waits.

Runs each name once with a cold cache, then again with the same cache, one search at a time against
live sources. Prints one JSON line per search and a summary. Not a gate; it does not land.
"""
import json
import os
import statistics
import subprocess
import sys
import tempfile
import time

TERMS = (
    'imatinib', 'Gleevec', 'pembrolizumab', 'Keytruda', 'nivolumab', 'trastuzumab', 'rituximab',
    'osimertinib', 'metformin', 'aspirin', 'tamoxifen', 'cisplatin', 'paclitaxel', 'lenacapavir',
    'dolutegravir', 'bictegravir', 'sotorasib', 'olaparib', 'abrocitinib', 'Cibinqo', 'argatroban',
    'vemurafenib', 'ibrutinib', 'venetoclax', 'apixaban', 'tucatinib', 'selpercatinib',
    'capivasertib', 'elacestrant', 'zongertinib',
)
PASSED = ('PATH', 'LANG')


def kind(row):
    url = row.get('url', '')
    if row['source'] != 'chembl':
        return row['source']
    if 'pref_name__iexact' in url:
        return 'chembl-preferred'
    if 'molecule_synonym__iexact' in url:
        return 'chembl-synonym'
    return 'chembl-molecule'


def search(binary, run, label, term):
    os.makedirs(run, exist_ok=True)
    env = {name: os.environ[name] for name in PASSED if name in os.environ}
    env.update(HOME=run, XDG_CACHE_HOME=run + '/cache', XDG_CONFIG_HOME=run + '/config')
    observed = os.path.join(run, label + '.jsonl')
    args = [binary, '--format', 'json', '--observe', observed, 'search', 'trial', term]
    begun = time.monotonic()
    done = subprocess.run(args, env=env, capture_output=True, text=True, timeout=120)
    wall = round((time.monotonic() - begun) * 1000)
    try:
        reply = json.loads(done.stdout)
    except ValueError:
        reply = {}
    wire = []
    if os.path.exists(observed):
        with open(observed, encoding='utf-8') as lines:
            for line in lines:
                row = json.loads(line)
                if 'source' in row:
                    wire.append([kind(row), row.get('status'), row['elapsed_ms'], row.get('cached', False)])
    names = []
    for source in reply.get('sources', []):
        for notice in source.get('notices', []):
            if isinstance(notice, dict) and 'names' in notice:
                names = notice['names']
    chembl = [w for w in wire if w[0].startswith('chembl') and not w[3]]
    line = {
        'label': label, 'term': term, 'status': reply.get('status'), 'wall_ms': wall,
        'chembl_requests': len(chembl), 'chembl_ms': sum(w[2] for w in chembl),
        'widened': len(names) > 1, 'names': len(names),
        'sources': [[s.get('source'), s.get('state'), s.get('reason')]
                    for s in reply.get('sources', []) if s.get('source') in
                    ('mychem', 'chembl', 'openfda-label', 'ctgov')],
        'wire': wire,
    }
    print(json.dumps(line, separators=(',', ':')), flush=True)
    return line


def summary(label, lines):
    reached = [l for l in lines if l['chembl_requests']]
    others = [l for l in lines if not l['chembl_requests']]
    print('SUMMARY {} searches={} reached_chembl={} widened={} statuses={}'.format(
        label, len(lines), len(reached), sum(l['widened'] for l in lines),
        sorted({l['status'] for l in lines})), flush=True)
    if reached:
        print('SUMMARY {} reached: wall median_ms={} max_ms={}; chembl wire median_ms={} max_ms={}; '
              'widened={}'.format(
                  label, round(statistics.median(l['wall_ms'] for l in reached)),
                  max(l['wall_ms'] for l in reached),
                  round(statistics.median(l['chembl_ms'] for l in reached)),
                  max(l['chembl_ms'] for l in reached), sum(l['widened'] for l in reached)),
              flush=True)
    if others:
        print('SUMMARY {} not reached: wall median_ms={} max_ms={}'.format(
            label, round(statistics.median(l['wall_ms'] for l in others)),
            max(l['wall_ms'] for l in others)), flush=True)
    for name in ('chembl-preferred', 'chembl-synonym', 'chembl-molecule'):
        rows = [w for l in lines for w in l['wire'] if w[0] == name and not w[3]]
        if rows:
            print('SUMMARY {} {} requests={} answered={} median_ms={} max_ms={} statuses={}'.format(
                label, name, len(rows), sum(1 for w in rows if w[1] == 200),
                round(statistics.median(w[2] for w in rows)), max(w[2] for w in rows),
                sorted({str(w[1]) for w in rows})), flush=True)


def main():
    target = os.environ.get('CARGO_TARGET_DIR', 'target')
    binary = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else os.path.join(target, 'debug', 'biomcp'))
    with tempfile.TemporaryDirectory(prefix='biomcp-0843-') as home:
        cold, warm = [], []
        for index, term in enumerate(TERMS):
            run = os.path.join(home, str(index))
            cold.append(search(binary, run, 'cold', term))
            warm.append(search(binary, run, 'repeat', term))
        summary('cold', cold)
        summary('repeat', warm)
    return 0


if __name__ == '__main__':
    sys.exit(main())
