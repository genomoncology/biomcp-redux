"""Run every code in a pinned Orphadata product 1 XML release through disease get.

Supply the XML and the matching MONDO SSSOM release with their SHA-256 digests.
Replay the XML through sync and derive MyDisease-shaped replies from the MONDO
mapping, so the check needs no network and can run on a schedule. The floor is
100% for each of the mapped and unmapped groups. Keep source files outside Git.
"""
import argparse
import csv
import hashlib
import io
import json
from pathlib import Path
import re
import subprocess
import tempfile
import xml.etree.ElementTree as ET


PRODUCT1 = 'https://www.orphadata.com/data/xml/en_product1.xml'
MYDISEASE = 'https://mydisease.info/v1/disease/ORPHANET%3A{}?fields=mondo%2Cdisease_ontology'


def pinned(path, digest):
    data = path.read_bytes()
    if hashlib.sha256(data).hexdigest() != digest:
        raise ValueError(f'{path.name} does not match its pinned SHA-256')
    return data.decode('utf-8')


def mondos_by_code(tsv):
    mapped = {}
    rows = csv.DictReader((line for line in io.StringIO(tsv) if not line.startswith('#')),
                          delimiter='\t')
    for row in rows:
        left, right = row['subject_id'], row['object_id']
        if right.startswith('MONDO:'):
            left, right = right, left
        orphanet = re.fullmatch(r'(?:ORPHA|Orphanet):([0-9]+)', right, re.I)
        if orphanet and re.fullmatch(r'MONDO:[0-9]+', left):
            mapped.setdefault(orphanet[1], left)
    if not mapped:
        raise ValueError('MONDO SSSOM has no Orphanet mappings')
    return mapped


def capture(folder, source, url, status, media, kind, body):
    key = hashlib.sha256(f'GET\0{url}'.encode()).hexdigest()
    directory = folder / source
    directory.mkdir(exist_ok=True)
    record = {'method': 'GET', 'source': source, 'url': url, 'exchanges': [{
        'status': status, 'content_type': media, 'delay_ms': 0,
        'retry_after_ms': None, 'body': {'kind': kind, 'value': body},
    }]}
    (directory / f'{key}.json').write_text(json.dumps(record, ensure_ascii=False),
                                          encoding='utf-8')


def invoke(command, *args):
    run = subprocess.run([*command, *args], capture_output=True, timeout=60, check=False)
    return run.returncode, json.loads(run.stdout)


def check_disorders(command, replay, disorders, mapped):
    totals = {'mapped': 0, 'unmapped': 0}
    failures = totals.copy()
    examples = []
    for row in disorders:
        code = row.findtext('OrphaCode')
        orphanet = f'ORPHA:{code}'
        mondo = mapped.get(code)
        group = 'mapped' if mondo else 'unmapped'
        totals[group] += 1
        if mondo:
            body = {'_id': mondo, 'mondo': {'mondo': mondo,
                    'xrefs': {'orphanet': f'ORPHANET:{code}'}}}
        else:
            body = {'code': 404, 'success': False, 'error': 'Not Found.'}
        capture(replay, 'mydisease', MYDISEASE.format(code), 200 if mondo else 404,
                'application/json', 'json', body)
        rc, reply = invoke(command, 'get', 'disease', orphanet)
        card = reply.get('result') or {}
        valid = (rc == 0 and reply.get('status') == 'complete'
                 and card.get('returned_id') == (mondo or orphanet))
        if not mondo:
            valid &= (card.get('concept', {}).get('label') == row.findtext('Name')
                      and card.get('disorder_type') == row.findtext('DisorderType/Name')
                      and card.get('attribution', {}).get('source') == 'orphadata')
        if not valid:
            failures[group] += 1
            if len(examples) < 10:
                examples.append(orphanet)
    for group, total in totals.items():
        print(f'Orphadata {group} cards {total - failures[group]}/{total}, floor 100%')
    if examples:
        print('Unreached or incorrect cards include ' + ', '.join(examples))
    return int(not all(totals.values()) or any(failures.values()))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('biomcp', type=Path)
    parser.add_argument('product1', type=Path)
    parser.add_argument('product1_sha256')
    parser.add_argument('mondo_sssom', type=Path)
    parser.add_argument('mondo_sha256')
    args = parser.parse_args()
    try:
        xml = pinned(args.product1, args.product1_sha256)
        mapped = mondos_by_code(pinned(args.mondo_sssom, args.mondo_sha256))
        catalog = ET.fromstring(xml)
        disorders = catalog.findall('DisorderList/Disorder')
        codes = [row.findtext('OrphaCode') for row in disorders]
        if not codes or len(set(codes)) != len(codes) or any(
                not re.fullmatch(r'[0-9]+', code or '') for code in codes):
            raise ValueError('Orphadata product 1 must contain distinct numeric disorder codes')
        with tempfile.TemporaryDirectory(prefix='orphadata-census-') as scratch:
            root = Path(scratch)
            replay = root / 'replay'
            replay.mkdir()
            config = root / 'config.json'
            config.write_text('{}\n', encoding='utf-8')
            command = [str(args.biomcp.resolve()), '--config', str(config),
                       '--replay', str(replay), '--case-cache', str(root / 'cache')]
            capture(replay, 'orphadata', PRODUCT1, 200, 'application/xml', 'text', xml)
            rc, reply = invoke(command, 'sync', 'orphadata', '--xrefs')
            if rc or (reply.get('result') or {}).get('rows') != len(disorders):
                raise ValueError('BioMCP did not synchronize every Orphadata disorder')
            print(f'Orphadata release {catalog.attrib["date"]}')
            return check_disorders(command, replay, disorders, mapped)
    except (KeyError, OSError, ValueError, ET.ParseError, subprocess.TimeoutExpired) as error:
        parser.exit(1, f'Orphadata census failed: {error}\n')


if __name__ == '__main__':
    raise SystemExit(main())
