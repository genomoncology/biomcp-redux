"""Prepare external protein command cases from the accepted native archives."""
import email
import hashlib
import io
import json
from pathlib import Path
import sys
import shutil
import tarfile
from urllib.parse import parse_qsl, urlencode, urlsplit, urlunsplit, quote

archives, output = map(Path, sys.argv[1:])
output.mkdir()
replay = output / 'replay' / 'uniprot'
replay.mkdir(parents=True)
shutil.copytree(Path(__file__).resolve().parent.parent / 'tests/replies/gene/main/hgnc', output / 'replay/hgnc')


def record(url, body, headers):
    parts = urlsplit(url)
    params = dict(parse_qsl(parts.query))
    pairs = [(key, params[key]) for key in ('query', 'format', 'size', 'fields', 'cursor') if key in params]
    canonical = urlunsplit(parts._replace(query=urlencode(pairs, quote_via=quote, safe='~')))
    headers = email.message_from_string(headers.decode().replace('\r\n', '\n').split('\n', 1)[1])
    exchange = {'status': 200, 'content_type': 'application/json', 'delay_ms': 0,
                'retry_after_ms': None, 'body': {'kind': 'json', 'value': json.loads(body)}}
    if headers.get('x-total-results') is not None:
        link = headers.get('link', '')
        next_url = link.split('>', 1)[0].removeprefix('<') if 'rel="next"' in link else None
        exchange['pagination'] = {'total': int(headers['x-total-results']), 'next': next_url}
    name = hashlib.sha256(b'GET\0' + canonical.encode()).hexdigest() + '.json'
    (replay / name).write_text(json.dumps({'source': 'uniprot', 'method': 'GET',
                                          'url': canonical, 'exchanges': [exchange]}))


with tarfile.open(archives / 'j14202-native-sources.tar.gz') as archive:
    for accession in ['P00533', 'Q9UIF7', 'P42771', 'Q8N726', 'P15056']:
        record(f'https://rest.uniprot.org/uniprotkb/{accession}.json',
               archive.extractfile(f'uniprot-{accession}.body').read(),
               archive.extractfile(f'uniprot-{accession}.headers').read())
with tarfile.open(archives / 'j14211-j14221-identity-followup.tar.gz') as archive:
    receipts = json.load(archive.extractfile('receipts.json'))
    for gene in ['BRAF', 'CDKN2A']:
        url = next(row['url'] for row in receipts if row['source'] == 'search' and row['accession'] == gene)
        record(url, archive.extractfile(f'search-{gene}.body').read(),
               archive.extractfile(f'search-{gene}.headers').read())
with tarfile.open(archives / 'j14362-native-witnesses.tar.gz') as archive:
    for name in ['isoform', 'reviewed', 'reviewed-second']:
        with tarfile.open(fileobj=io.BytesIO(archive.extractfile(f'capture-{name}.tar.gz').read())) as inner:
            receipt = json.load(inner.extractfile('0000.receipt.json'))
            record(receipt['url'], inner.extractfile('0000.body').read(),
                   inner.extractfile('0000.headers').read())


def case(name, steps):
    ceilings = {
        'P00533': [768], 'Q9UIF7': [741], 'P38398-2': [789],
        'CDKN2A': [1267], 'search-BRAF': [749], 'search-CDKN2A': [1076],
        'text-pages': [2330, 2250], 'gene-crossrefs': [796, 970],
    }
    for step, ceiling in zip(steps, ceilings[name], strict=True):
        step['max_bytes'] = ceiling
        step['max_tokens'] = (ceiling + 3) // 4
        if 'run' in step:
            step['mcp'] = step['run']
    (output / f'{name}.yaml').write_text(json.dumps({'feature': 'protein-search',
        'story': 'Accepted native protein facts retain exact identities and source ordering',
        'replay': str(replay.parent.resolve()), 'steps': steps}))


for accession, length in [('P00533', 1210), ('Q9UIF7', 546), ('P38398-2', 63)]:
    args = ['get', 'protein', accession]
    case(accession, [{'run': args, 'mcp': args, 'expect': {
        '/status': 'complete', '/result/items/0/header/protein/accession': accession,
        '/result/items/0/header/protein/length': length}}])
p00533 = output / 'P00533.yaml'
card = json.loads(p00533.read_text())
card['steps'][0]['expect'].update({
    '/result/items/0/audit/entry_version': 301,
    '/result/items/0/audit/sequence_version': 2,
    '/result/items/0/audit/first_public': '1986-07-21',
    '/result/items/0/audit/last_annotation_update': '2026-09-02',
    '/result/items/0/audit/last_sequence_update': '1997-11-01'})
p00533.write_text(json.dumps(card))
case('CDKN2A', [{'run': ['get', 'protein', 'P42771', 'Q8N726'], 'expect': {
    '/result/items/*/header/protein/accession': ['P42771', 'Q8N726'],
    '/result/items/*/header/protein/length': [156, 132]}}])
for gene, accessions in [('BRAF', ['P15056']), ('CDKN2A', ['P42771', 'Q8N726'])]:
    case('search-' + gene, [{'run': ['search', 'protein', '--gene', gene], 'expect': {
        '/status': 'complete', '/result/items/*/protein/accession': accessions,
        '/next/*/kind': ['disambiguate'] * len(accessions)}}])
case('text-pages', [
    {'run': ['search', 'protein', 'EGFR'], 'mcp': ['search', 'protein', 'EGFR'],
     'expect': {'/status': 'complete', '/page/total': 399,
        '/result/items/*/protein/accession': ['P00533', 'Q86T13', 'Q9ULV8', 'O00750', 'O14744']}},
    {'follow': 'page', 'expect': {'/status': 'complete', '/page/offset': 5,
        '/result/items/*/protein/accession': ['P04626', 'P01133', 'P22681', 'Q8NFJ5', 'P12004']}}])

case('gene-crossrefs', [
    {'run': ['get', 'protein', 'BRAF'], 'expect': {'/status': 'complete',
        '/result/items/0/header/protein/accession': 'P15056'}},
    {'run': ['get', 'protein', 'CDKN2A'], 'expect': {'/status': 'complete',
        '/result/items/0/candidates/*/protein/accession': ['P42771', 'Q8N726'],
        '/next/*/kind': ['disambiguate', 'disambiguate']}}])
