"""Assert positive native OR totals from bounded, retained trial captures."""
import gzip
import hashlib
import io
import json
import os
from pathlib import Path
import sys
import tarfile
from urllib.parse import parse_qs, urlsplit

DECODED_LIMIT = 1024 * 1024


def totals(archive):
    result = {}
    with tarfile.open(archive) as source:
        for entry in source.getmembers():
            if not entry.name.endswith('.receipt.json'):
                continue
            receipt = json.load(source.extractfile(entry))
            if receipt['status'] != 200 or receipt['outcome'] != 'success':
                print('excluded failed capture: ' + receipt['id'] + ' ' + receipt['outcome'], file=sys.stderr)
                continue
            body = source.extractfile(entry.name.replace('.receipt.json', '.body')).read()
            if hashlib.sha256(body).hexdigest() != receipt['body_sha256']:
                raise ValueError('source body hash mismatch')
            if body.startswith(b'\x1f\x8b'):
                with gzip.GzipFile(fileobj=io.BytesIO(body)) as compressed:
                    body = compressed.read(DECODED_LIMIT + 1)
            if len(body) > DECODED_LIMIT:
                raise ValueError('decoded body exceeded bound')
            key = os.environ.get('NCI_API_KEY')
            if key and key.encode() in body:
                raise ValueError('decoded body reflected credential')
            value = json.loads(body)
            total = value.get('totalCount', value.get('total'))
            if not isinstance(total, int) or total <= 0:
                raise ValueError('native source count must be positive: ' + receipt['id'])
            if receipt['source'] == 'nci':
                requested = set(parse_qs(urlsplit(receipt['url']).query).get('diseases.nci_thesaurus_concept_id', []))
                rows = value.get('data', [])
                if not requested or not rows:
                    raise ValueError('missing NCI disease witness')
                for row in rows:
                    codes = {disease['nci_thesaurus_concept_id'] for disease in row.get('diseases', [])}
                    if not requested.intersection(codes):
                        raise ValueError('returned NCI trial lacks requested disease code')
            result[receipt['id']] = total
    return result


def main():
    result = {}
    for archive in sys.argv[1:]:
        result.update(totals(Path(archive)))
    for option in ['condition', 'intervention', 'keyword', 'nci']:
        counts = [result[option + '-' + suffix] for suffix in ['first', 'second', 'or']]
        if counts[2] < max(counts[:2]):
            raise ValueError(option + ' OR lost single-value hits')
        print(option + ': ' + json.dumps(counts))


if __name__ == '__main__':
    main()
