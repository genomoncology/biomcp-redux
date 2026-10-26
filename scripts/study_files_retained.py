"""Build external file-reference replay cases from verified native archives, without requests."""
import argparse
import hashlib
import json
import io
from pathlib import Path
import tarfile


def archive(path):
    options = {"fileobj": path} if hasattr(path, "read") else {"name": path}
    with tarfile.open(**options) as packed:
        members = packed.getmembers()
        assert all(m.isfile() and '/' not in m.name for m in members)
        assert len({m.name for m in members}) == len(members)
        data = {m.name: packed.extractfile(m).read() for m in members}
    expected = json.loads(data['manifest.json'])
    assert set(data) == set(expected) | {'manifest.json'}
    for name, digest in expected.items():
        assert hashlib.sha256(data[name]).hexdigest() == digest
    return data


def compact(value):
    return json.dumps(value, ensure_ascii=False, separators=(',', ':'))


def recording(replay, url, raw, status=200):
    key = hashlib.sha256(b'GET\0' + url.encode()).hexdigest()
    value = {'source': 'cbioportal', 'method': 'GET', 'url': url,
             'exchanges': [{'status': status, 'fetched': 0, 'content_type': 'application/json',
                            'delay_ms': 0, 'retry_after_ms': None,
                            'body': {'kind': 'text', 'value': raw.decode()}}]}
    (replay / (key + '.json')).write_text(compact(value))


def main(native, catalog, admission, out):
    source = archive(native)
    catalog = archive(catalog)
    assert out.is_absolute() and not out.exists()
    out.mkdir(parents=True)
    replay = out / 'replay' / 'cbioportal'
    replay.mkdir(parents=True)
    recording(replay, 'https://www.cbioportal.org/api/studies?projection=DETAILED', catalog['studies.body'])
    cases = out / 'study-files-cases'
    cases.mkdir()
    count = 0
    for identity, total in [('brca_tcga_pan_can_atlas_2018', 58), ('msk_impact_2017', 18)]:
        requests = json.loads(source['plan.json'])['requests']
        index = next(n for n, r in enumerate(requests) if r['id'] == identity)
        name = f'{index:04d}.body'
        raw = source[name]
        entries = json.loads(raw)
        assert len(entries) == total
        assert len({r['path'] for r in entries}) == total
        recording(replay, f'https://api.github.com/repos/cBioPortal/datahub/contents/public/{identity}', raw)
        notice = next(r['html_url'] for r in entries if r['name'] == 'LICENSE')
        for section, suffix in [('molecular-profiles', 'profiles'), ('sample-lists', 'sample-lists')]:
            recording(replay, f'https://www.cbioportal.org/api/studies/{identity}/{section}?projection=DETAILED', catalog[f'{identity}-{suffix}.body'])
        operations = [(['get', 'study', identity], {'/result/license_notice': notice}),
                      (['get', 'study', identity, 'all'], {'/result/license_notice': notice,
                       '/result/files/data/page/total': total,
                       '/result/files/data/page/returned': 5})]
        for offset in range(0, total, 5):
            expected = {'/page/total': total, '/page/offset': offset, '/result/license_notice': notice}
            for n, row in enumerate(entries[offset:offset + 5]):
                for key in ['name', 'path', 'sha']:
                    expected[f'/result/items/{n}/{key}'] = row[key]
                for native_key, wire_key in [('html_url', 'htmlUrl'), ('download_url', 'downloadUrl'), ('size', 'repositoryBlobSize'), ('type', 'kind')]:
                    if row.get(native_key) is not None:
                        expected[f'/result/items/{n}/{wire_key}'] = row[native_key]
            operations.append((['get', 'study', identity, 'files', '--section-offset', str(offset)], expected))
        for args, expected in operations:
            value = {'story': 'Retained exact DataHub directory facts and license notice', 'feature': 'study-files',
                     'replay': str(replay.parent), 'steps': [{'run': ['sync', 'cbioportal-studies']},
                     {'run': args, 'mcp': args, 'expect': expected}]}
            (cases / f'{count:03d}.yaml').write_text(compact(value))
            count += 1
    absent = archive(io.BytesIO(archive(admission)['absent-files-capture.tar.gz']))
    receipt = json.loads(absent['0000.receipt.json'])
    assert receipt['status'] == 404
    recording(replay, receipt['url'], absent['0000.body'], 404)
    args = ['get', 'study', 'biomcp_absent_study_0207', 'files']
    value = {'story': 'Exact retained directory absence does not require a catalog lookup',
             'feature': 'study-files', 'replay': str(replay.parent),
             'steps': [{'run': args, 'mcp': args, 'exit': 1,
                        'expect': {'/status': 'absent', '/sources/0/use': 'restricted'}}]}
    (cases / f'{count:03d}.yaml').write_text(compact(value))
    count += 1
    print(f'Prepared {count} native CLI/MCP file-reference cases; no provider request')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('native', type=Path)
    parser.add_argument('catalog', type=Path)
    parser.add_argument('admission', type=Path)
    parser.add_argument('out', type=Path)
    args = parser.parse_args()
    main(args.native, args.catalog, args.admission, args.out)
