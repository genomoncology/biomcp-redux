"""Validate the reviewed public fixture digests; keep all key bytes out of Git."""
import json
from pathlib import Path
import re

POLICY = Path(__file__).with_name('public-fixtures.json')
UPSTREAM = 'https://raw.githubusercontent.com/gnutls/gnutls/3.7.9/lib/crypto-selftests-pk.c'


def reviewed():
    policy = json.loads(POLICY.read_text())
    if (set(policy) != {'upstream', 'tag', 'file', 'fixtures'} or policy['upstream'] != UPSTREAM
            or policy['tag'] != '3.7.9' or policy['file'] != 'lib/crypto-selftests-pk.c'
            or len(policy['fixtures']) != 10):
        raise ValueError('invalid reviewed public fixture policy')
    symbols, digests = set(), set()
    for row in policy['fixtures']:
        if (set(row) != {'symbol', 'sha256'} or not re.fullmatch('[a-zA-Z0-9_]+', row['symbol'])
                or row['symbol'] in symbols or not re.fullmatch('[a-f0-9]{64}', row['sha256'])
                or row['sha256'] in digests):
            raise ValueError('invalid reviewed public fixture entry')
        symbols.add(row['symbol'])
        digests.add(row['sha256'])
    return digests
