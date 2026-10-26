"""Prepare external protein command cases from the accepted native archives."""
import email
import hashlib
import io
import json
from pathlib import Path
import shutil
import tarfile
from urllib.parse import parse_qsl, urlencode, urlsplit, urlunsplit, quote

def record(replay, url, body, headers, text=False):
    parts = urlsplit(url)
    params = dict(parse_qsl(parts.query))
    pairs = [(key, params[key]) for key in ('query', 'format', 'size', 'fields', 'cursor') if key in params]
    canonical = urlunsplit(parts._replace(query=urlencode(pairs, quote_via=quote, safe='~')))
    headers = email.message_from_string(headers.decode().replace('\r\n', '\n').split('\n', 1)[1])
    exchange = {'status': 200, 'content_type': headers.get('Content-Type', 'application/octet-stream'), 'delay_ms': 0,
                'retry_after_ms': None, 'body': {'kind': 'text' if text else 'json', 'value': body.decode() if text else json.loads(body)}}
    if headers.get('x-total-results') is not None:
        link = headers.get('link', '')
        next_url = link.split('>', 1)[0].removeprefix('<') if 'rel="next"' in link else None
        exchange['pagination'] = {'total': int(headers['x-total-results']), 'next': next_url}
    name = hashlib.sha256(b'GET\0' + canonical.encode()).hexdigest() + '.json'
    (replay / name).write_text(json.dumps({'source': 'uniprot', 'method': 'GET',
                                          'url': canonical, 'exchanges': [exchange]}))


def prepare(archives, output, sections=False):
    output.mkdir()
    replay = output / 'replay' / 'uniprot'
    replay.mkdir(parents=True)
    shutil.copytree(Path(__file__).resolve().parent.parent / 'tests/replies/gene/main/hgnc', output / 'replay/hgnc')
    with tarfile.open(archives / 'j14202-native-sources.tar.gz') as archive:
        for accession in (['P00533', 'P15056', 'P38398', 'P04637', 'P13569', 'P31749', 'Q9UIF7', 'P42771', 'Q8N726'] if sections else ['P00533', 'Q9UIF7', 'P42771', 'Q8N726', 'P15056']):
            record(replay, f'https://rest.uniprot.org/uniprotkb/{accession}.json',
                   archive.extractfile(f'uniprot-{accession}.body').read(),
                   archive.extractfile(f'uniprot-{accession}.headers').read())
    with tarfile.open(archives / 'j14211-j14221-identity-followup.tar.gz') as archive:
        receipts = json.load(archive.extractfile('receipts.json'))
        if sections:
            for accession in ['P38398-2', 'Q9UIF7-2']:
                record(replay, f'https://rest.uniprot.org/uniprotkb/{accession}.fasta',
                       archive.extractfile(f'isoform-{accession}.body').read(),
                       archive.extractfile(f'isoform-{accession}.headers').read(), text=True)
        for gene in ['BRAF', 'CDKN2A']:
            url = next(row['url'] for row in receipts if row['source'] == 'search' and row['accession'] == gene)
            record(replay, url, archive.extractfile(f'search-{gene}.body').read(),
                   archive.extractfile(f'search-{gene}.headers').read())
    with tarfile.open(archives / 'j14362-native-witnesses.tar.gz') as archive:
        for name in ['isoform', 'reviewed', 'reviewed-second']:
            with tarfile.open(fileobj=io.BytesIO(archive.extractfile(f'capture-{name}.tar.gz').read())) as inner:
                receipt = json.load(inner.extractfile('0000.receipt.json'))
                record(replay, receipt['url'], inner.extractfile('0000.body').read(),
                       inner.extractfile('0000.headers').read())

    return replay
