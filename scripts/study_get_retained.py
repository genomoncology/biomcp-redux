"""Prepare external study metadata cases from the verified retained archive; make no requests."""
import argparse
import hashlib
import json
from pathlib import Path
import tarfile

ARCHIVE_SHA = '3c6c25ccc194e1822506c6caaf260403aa00a32770c0d210b1b65d135166588a'
BASE = 'https://www.cbioportal.org/api/studies'
STUDIES = ['brca_tcga_pan_can_atlas_2018', 'luad_tcga_pan_can_atlas_2018', 'msk_impact_2017']


def compact(value):
    return json.dumps(value, ensure_ascii=False, separators=(',', ':'))


def inputs(path):
    assert hashlib.sha256(path.read_bytes()).hexdigest() == ARCHIVE_SHA
    with tarfile.open(path) as archive:
        members = archive.getmembers()
        assert len(members) == 41 and len({m.name for m in members}) == 41
        assert all(m.isfile() and '/' not in m.name for m in members)
        contents = {m.name: archive.extractfile(m).read() for m in members}
    manifest = json.loads(contents['manifest.json'])
    assert set(contents) == set(manifest) | {'manifest.json'}
    for name, digest in manifest.items():
        assert hashlib.sha256(contents[name]).hexdigest() == digest
    return contents


def recording(root, url, body):
    target = root / 'cbioportal'
    target.mkdir(parents=True, exist_ok=True)
    key = hashlib.sha256(b'GET\0' + url.encode()).hexdigest()
    value = {'source': 'cbioportal', 'method': 'GET', 'url': url,
             'exchanges': [{'status': 200, 'fetched': 1793059200,
                            'content_type': 'application/json', 'delay_ms': 0,
                            'retry_after_ms': None,
                            'body': {'kind': 'text', 'value': body.decode()}}]}
    (target / f'{key}.json').write_text(compact(value))


def step(args, expected=None, **kw):
    return {'run': args, 'mcp': args, 'expect': expected or {}, **kw}


def main(archive, out):
    assert out.is_absolute() and not out.exists()
    contents = inputs(archive)
    out.mkdir(parents=True)
    replay = out / 'replay'
    recording(replay, BASE + '?projection=DETAILED', contents['studies.body'])
    for study in STUDIES:
        for section, suffix in [('molecular-profiles', 'profiles'), ('sample-lists', 'sample-lists')]:
            recording(replay, f'{BASE}/{study}/{section}?projection=DETAILED', contents[f'{study}-{suffix}.body'])
    cases = out / 'study-get-cases'
    cases.mkdir()
    # Compare all native fields rather than interpreting clinical units or prose.
    catalog = json.loads(contents['studies.body'])
    receipts = json.loads(contents['receipts.json'])
    selected = [r['name'] for r in receipts if r['name'] + '.body' in contents
                and r['name'] not in {'studies', 'api-docs', 'faq', 'datahub-readme', 'datahub-list', 'cancer-types', 'absent'}
                and '-' not in r['name']]
    steps = []
    for identity in selected:
        native = next(row for row in catalog if row['studyId'] == identity)
        expected = {'/result/study/header/studyId': identity,
                    '/result/study/header/cancerTypeId': native['cancerTypeId'],
                    '/result/study/header/allSampleCount': native['allSampleCount'],
                    '/result/study/description': native['description'],
                    '/result/source_url': f'https://www.cbioportal.org/study/summary?id={identity}'}
        for key in ['sequencedSampleCount', 'cnaSampleCount', 'structuralVariantCount', 'treatmentCount', 'importDate']:
            expected['/result/study/' + key] = native[key]
        if native.get('pmid'):
            expected['/result/study/header/pmids'] = [x.strip() for x in native['pmid'].split(',')]
        steps.append(step(['get', 'study', identity], expected))
    for study in STUDIES:
        for section, suffix, identifier in [('profiles', 'profiles', 'molecularProfileId'),
                                             ('sample-lists', 'sample-lists', 'sampleListId')]:
            native = json.loads(contents[f'{study}-{suffix}.body'])
            for offset in range(0, len(native), 5):
                expected = {'/page/total': len(native), '/page/offset': offset,
                            '/result/acquired': 1793059200}
                for n, row in enumerate(native[offset:offset + 5]):
                    for key in [identifier, 'studyId', 'name', 'description']:
                        expected[f'/result/items/{n}/{key}'] = row[key]
                    keys = ['molecularAlterationType', 'datatype'] if section == 'profiles' else ['category', 'sampleCount']
                    for key in keys:
                        if key in row:
                            expected[f'/result/items/{n}/{key}'] = row[key]
                steps.append(step(['get', 'study', study, section, '--section-offset', str(offset)], expected))
    for n, operation in enumerate(steps):
        (cases / f'{n:03d}.yaml').write_text(compact({
            'story': 'Retained exact study metadata and complete DETAILED section walks',
            'feature': 'study-get', 'replay': str(replay),
            'steps': [step(['sync', 'cbioportal-studies']), operation]}))
    print(f'Prepared {len(steps)} CLI/MCP steps outside Git; zero provider requests')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('archive', type=Path)
    parser.add_argument('out', type=Path)
    args = parser.parse_args()
    main(args.archive, args.out)
