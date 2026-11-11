"""One-off for ticket 0832: time live drug-name lookups at 1, 5 and 25 searches at once.

Prints one JSON line per search and a per-source summary per round. Not a gate; it does not land.
"""
import collections
import json
import os
import statistics
import subprocess
import sys
import tempfile
import time

TERMS = (
    'imatinib', 'GS-US-621-6289', 'lenacapavir', 'pembrolizumab', 'osimertinib', 'Keytruda',
    'dolutegravir', 'sotorasib', 'olaparib', 'trastuzumab', 'abrocitinib', 'argatroban',
    'vemurafenib', 'nivolumab', 'ibrutinib', 'venetoclax', 'apixaban', 'metformin', 'Cibinqo',
    'bictegravir', 'tucatinib', 'selpercatinib', 'capivasertib', 'elacestrant', 'zongertinib',
)
PASSED = ('PATH', 'LANG')


def launch(binary, home, index, entity, term):
    run = os.path.join(home, str(index))
    os.makedirs(run)
    env = {name: os.environ[name] for name in PASSED if name in os.environ}
    env.update(HOME=run, XDG_CACHE_HOME=run + '/cache', XDG_CONFIG_HOME=run + '/config')
    observed = os.path.join(run, 'observed.jsonl')
    args = [binary, '--format', 'json', '--observe', observed, 'search', entity, term]
    process = subprocess.Popen(args, env=env, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
                               text=True)
    return process, observed, time.monotonic(), entity, term


def round_of(binary, home, label, searches):
    started = [launch(binary, home, '{}-{}'.format(label, index), entity, term)
               for index, (entity, term) in enumerate(searches)]
    by_source = collections.defaultdict(list)
    statuses = collections.Counter()
    for process, observed, begun, entity, term in started:
        out, _ = process.communicate(timeout=120)
        wall = round((time.monotonic() - begun) * 1000)
        try:
            reply = json.loads(out)
        except ValueError:
            reply = {}
        wire = []
        if os.path.exists(observed):
            with open(observed, encoding='utf-8') as lines:
                for line in lines:
                    row = json.loads(line)
                    if 'source' in row:
                        wire.append([row['source'], row.get('status'), row['elapsed_ms']])
                        by_source[row['source']].append(row['elapsed_ms'])
        status = reply.get('status')
        statuses[status] += 1
        print(json.dumps({
            'round': label, 'search': entity, 'term': term, 'status': status, 'wall_ms': wall,
            'sources': [[s.get('source'), s.get('state'), s.get('reason')]
                        for s in reply.get('sources', [])],
            'wire': wire,
        }, separators=(',', ':')), flush=True)
    for source, values in sorted(by_source.items()):
        print('SUMMARY {} {} requests={} median_ms={} max_ms={}'.format(
            label, source, len(values), round(statistics.median(values)), max(values)), flush=True)
    print('SUMMARY {} statuses={}'.format(label, dict(statuses)), flush=True)


def main():
    binary = os.path.expanduser(sys.argv[1])
    with tempfile.TemporaryDirectory(prefix='biomcp-0832-') as home:
        trials = [('trial', term) for term in TERMS]
        round_of(binary, home, 'one', trials[:1])
        time.sleep(3)
        round_of(binary, home, 'five', trials[1:6])
        time.sleep(3)
        mixed = [('article' if index % 5 == 4 else 'trial', term)
                 for index, term in enumerate(TERMS)]
        round_of(binary, home, 'twentyfive', mixed)
    return 0


if __name__ == '__main__':
    sys.exit(main())
