"""Replay retained GPML and exact HGNC rows, without any provider request."""
import csv
import hashlib
import io
import json
import os
from pathlib import Path
import subprocess
import tarfile
import time
import xml.etree.ElementTree as ET

from source_capture import verify_archive


STORE = '/home/ian/workspace/data/biomcp-redux/'
GPML = STORE + '0224-wikipathways-genes/native-1793004917879261000.tar.gz'
HGNC = STORE + '0102-article-short-pool/zero/hgnc/0887a33cc809657a2a564d65e8cbfcddf45adc3810a8bfb0653e03b418cf2953.json'
CATALOG = STORE + '0191-pathway-investigation/0191-pathway-investigation-evidence.tar.gz'


def read_remote(path):
    return subprocess.check_output(['ssh', 'yellow', 'cat', path])


def member(archive, name):
    with tarfile.open(fileobj=io.BytesIO(archive)) as packed:
        with packed.extractfile(name) as stream:
            return stream.read()


def main():
    root = Path.home() / 'job-evidence' / 'biomcp-redux-0224' / ('replay-' + str(time.time_ns()))
    native = root / 'fixtures' / 'wikipathways-genes-native'
    native.mkdir(parents=True)
    archive = read_remote(GPML)
    if hashlib.sha256(archive).hexdigest() != 'eeb794004a09a28cd022a81dfe3c10a255dbbed367d9f3dd8b2be744a2e0ca4c':
        raise ValueError('retained GPML archive mismatch')
    references = set()
    nodes = {}
    for identity in ['WP382', 'WP100']:
        body = member(member(archive, identity + '.tar.gz'), '0000.body')
        (native / (identity + '.gpml')).write_bytes(body)
        nodes[identity] = []
        for node in ET.fromstring(body).findall('{http://pathvisio.org/GPML/2013a}DataNode'):
            xref = node.find('{http://pathvisio.org/GPML/2013a}Xref')
            if node.get('Type') == 'GeneProduct':
                nodes[identity].append(node.get('GraphId'))
                if xref is not None:
                    references.add((xref.get('Database'), xref.get('ID')))
    (native / 'nodes.json').write_text(json.dumps(nodes))
    catalog = read_remote(CATALOG)
    if hashlib.sha256(catalog).hexdigest() != '7d7a46ccaf538be9bf10393d6d5a336072a75e1fdd2cdc2fd271bef2888f26a3':
        raise ValueError('retained catalogs archive mismatch')
    inner = member(catalog, '2290-pathway-investigation.tar.gz')
    for role, name in [('search', 'wiki-search.body'), ('get', 'wiki-detail.body')]:
        (native / (role + '.json')).write_bytes(member(inner, name))
    hgnc = json.loads(read_remote(HGNC))['exchanges'][0]['body']['value']
    if hashlib.sha256(hgnc.encode()).hexdigest() != '12363916be36f8f6f91879b0faade4eeaac973722de7136ec61329b77f2c2f91':
        raise ValueError('retained HGNC body mismatch')
    rows = csv.DictReader(io.StringIO(hgnc), delimiter='\t')
    with (native / 'hgnc.tsv').open('w') as stream:
        writer = csv.DictWriter(stream, fieldnames=rows.fieldnames, delimiter='\t', lineterminator='\n')
        writer.writeheader()
        for row in rows:
            if (('Entrez Gene', row['entrez_id']) in references
                    or ('Ensembl', row['ensembl_gene_id']) in references
                    or ('HGNC', row['hgnc_id']) in references
                    or ('HGNC', row['symbol']) in references):
                writer.writerow(row)
    environment = dict(os.environ, BIOMCP_TEST_FIXTURES=str(native.parent))
    result = subprocess.run(['cargo', 'test', '--locked', '-p', 'biomcp', '--test', 'cases', '--',
                             'wikipathways-genes/retained-native-measurements'], env=environment)
    (native / 'runner-exit.json').write_text(json.dumps({'exit': result.returncode}))
    files = list(native.iterdir())
    manifest = {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in files}
    (native / 'manifest.json').write_text(json.dumps(manifest, indent=2))
    packed_path = root / 'native-outputs.tar.gz'
    with tarfile.open(packed_path, 'w:gz') as packed:
        for path in sorted(native.iterdir()):
            packed.add(path, arcname=path.name)
    verify_archive(packed_path)
    destination = STORE + '0224-wikipathways-genes/' + root.name + '.tar.gz'
    subprocess.run(['scp', str(packed_path), 'yellow:' + destination], check=True)
    digest = hashlib.sha256(packed_path.read_bytes()).hexdigest()
    output = subprocess.check_output(['ssh', 'yellow', 'sha256sum', destination], text=True)
    if output.split()[0] != digest:
        raise ValueError('stored native output mismatch')
    print(json.dumps({'archive': destination, 'sha256': digest, 'runner_exit': result.returncode}))
    raise SystemExit(result.returncode)


if __name__ == '__main__':
    main()
