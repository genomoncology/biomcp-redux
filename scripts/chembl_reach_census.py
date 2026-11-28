"""Check that ChEMBL parents reach mechanisms recorded on their child forms."""
import json
import os
import re
import subprocess
import tempfile
import time
from collections import defaultdict
from urllib.parse import urlencode
from urllib.request import urlopen

from nightly_live_check import PASSED_ENVIRONMENT, build

MECHANISMS = 'https://www.ebi.ac.uk/chembl/api/data/mechanism.json'
PAGE_SIZE = 100
MOLECULE = re.compile(r'CHEMBL[0-9]+\Z')


def mechanism_page(params):
    with urlopen(f'{MECHANISMS}?{urlencode(params)}', timeout=30) as response:
        page = json.load(response)
    time.sleep(1)
    return page


def child_only_parents(read_page=mechanism_page):
    """Read native mechanism identities to identify parents without direct rows."""
    families = defaultdict(set)
    seen = set()
    total = None
    offset = 0
    while total is None or offset < total:
        page = read_page({'limit': PAGE_SIZE, 'offset': offset, 'order_by': 'mec_id'})
        meta = page['page_meta']
        if total is None:
            total = meta['total_count']
        rows = page['mechanisms']
        if (meta['offset'] != offset or meta['total_count'] != total
                or len(rows) != min(PAGE_SIZE, total - offset) or not rows):
            raise ValueError(f'inconsistent ChEMBL mechanism page at offset {offset}')
        for row in rows:
            molecule = row['molecule_chembl_id']
            parent = row['parent_molecule_chembl_id'] or molecule
            if (row['mec_id'] in seen or not MOLECULE.fullmatch(parent)
                    or not MOLECULE.fullmatch(molecule)):
                raise ValueError(f'invalid ChEMBL mechanism identity at offset {offset}')
            seen.add(row['mec_id'])
            families[parent].add(molecule)
        offset += len(rows)
    parents = {parent: forms for parent, forms in families.items() if parent not in forms}
    if not parents:
        raise ValueError('ChEMBL mechanism census found no child-only parents')
    print(f'ChEMBL mechanism census found {len(parents)} child-only parents in {total} rows',
          flush=True)
    return parents


def check_parent(executable, environment, parent, forms):
    reply = subprocess.run(
        [executable, '--format', 'json', 'get', 'drug', parent, 'targets',
         '--source', 'chembl', '--section-limit', '1'],
        env=environment, capture_output=True, check=False, timeout=45,
    )
    document = json.loads(reply.stdout)
    section = (document.get('result') or {}).get('targets') or {}
    records = (section.get('data') or {}).get('records', [])
    total = (document.get('page') or {}).get('total', 0)
    # A targetless native mechanism is counted and disclosed as a refusal.
    target_refused = any(problem['field'] == 'target_chembl_id'
                         for problem in document.get('problems', []))
    if (reply.returncode != 0 or section.get('state') != 'data' or total == 0
            or not (records or target_refused)
            or any(record['drug']['id'] not in forms for record in records)):
        raise ValueError(f'{parent} could not reach its child mechanisms '
                         f'(status {document.get("status")}, targets {section.get("state")})')


def main():
    executable = build()
    parents = child_only_parents()
    with tempfile.TemporaryDirectory(prefix='chembl-reach-') as home:
        environment = {name: os.environ[name] for name in PASSED_ENVIRONMENT if name in os.environ}
        environment.update(HOME=home, XDG_CACHE_HOME=home + '/cache', XDG_CONFIG_HOME=home + '/config')
        for parent, forms in sorted(parents.items()):
            check_parent(executable, environment, parent, forms)
            time.sleep(1)
    print(f'BioMCP reached child mechanisms for all {len(parents)} ChEMBL parents.')


if __name__ == '__main__':
    try:
        main()
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        raise SystemExit(f'ChEMBL reach census failed: {error}') from None
