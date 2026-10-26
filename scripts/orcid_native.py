"""Derive exact public-field assertions from accepted ORCID captures outside Git."""
import json
import re
from pathlib import Path


def step(args, expect, exit=0):
    return {'run': args, 'mcp': args, 'expect': expect, 'exit': exit,
            'is_error': False, 'full_output_bytes': True}


def prepare(records, cases, replay):
    for identifier, total in [('0000-0002-4299-3018', 380), ('0000-0002-7433-2740', 2)]:
        person = records[identifier + '-person'][1]
        name = person['name']
        display = (name['credit-name'] or {}).get('value') or ' '.join(
            name[key]['value'] for key in ['given-names', 'family-name'] if name[key])
        steps = [step(['get', 'author', 'orcid:' + identifier], {
            '/status': 'complete', '/result/items/0/card/header/name': display,
            '/result/items/0/card/header/id': 'orcid:' + identifier})]
        works = records[identifier + '-works'][1]['group']
        if sum(len(g['work-summary']) for g in works) != (453 if total == 380 else 2):
            raise ValueError('native assertion count differs')
        steps.append(step(['author', 'papers', 'orcid:' + identifier], {
            '/status': 'complete', '/result/observed_groups': total,
            '/result/collection_scope': 'whole_public_works_reply',
            '/page/total': total, '/page/returned': min(5, total)}))
        # Find a native multi-assertion group, then compare all native assertion IDs
        # and external identifier facts on that group without copying source text to Git.
        index = next((i for i, g in enumerate(works) if len(g['work-summary']) > 1), 0)
        steps.append(group_step(identifier, total, index, works[index]))
        # A complete bounded local page walk retains one collection. Its exact native
        # put-code sequence proves stable group boundaries, no gaps and no duplicates.
        for offset in range(0, total, 25):
            expect = {'/status': 'complete', '/page/offset': offset,
                      '/page/returned': min(25, total - offset)}
            for i, group in enumerate(works[offset:offset + 25]):
                for j, assertion in enumerate(group['work-summary']):
                    expect[f'/result/groups/{i}/assertions/{j}/put_code'] = assertion['put-code']
            expect['/result/groups'] = {'count': min(25, total - offset)}
            for i, group in enumerate(works[offset:offset + 25]):
                expect[f'/result/groups/{i}/assertions'] = {'count': len(group['work-summary'])}
            owner = step(['author', 'papers', 'orcid:' + identifier, '--limit', '25'], expect)
            if offset:
                owner.pop('run'); owner.pop('mcp')
                owner['follow'] = 'page'
                owner['offline'] = True
            steps.append(owner)
        case = {'feature': 'author-orcid', 'story': 'Retained exact public ORCID names and grouped works',
                'harvest': 'sdlc/planning/slices/author.md', 'replay': str(replay), 'steps': steps}
        save(case, cases, 'orcid-' + identifier + '.yaml')
    identifier = '0000-0002-4299-3018'
    works = records[identifier + '-works'][1]['group']
    case = {'feature': 'author-orcid', 'story': 'Native PMID and DOI claims retain provenance',
            'harvest': 'sdlc/planning/slices/author.md', 'replay': str(replay),
            'steps': [group_step(identifier, len(works), 42, works[42])]}
    save(case, cases, 'orcid-native-identifiers.yaml')
    absent = '0000-0000-0000-0001'
    case = {'feature': 'author-orcid', 'story': 'Native ORCID absence differs from failure',
            'harvest': 'sdlc/planning/slices/author.md', 'replay': str(replay),
            'steps': [step(['get', 'author', 'orcid:' + absent], {
                '/status': 'absent', '/result/items/0/status': 'absent', '/sources/0/state': 'ok'}, 1)]}
    save(case, cases, 'orcid-absent.yaml')


def save(case, cases, name):
    canonical = Path(__file__).resolve().parent.parent / 'tests/native/orcid' / name
    owner = json.loads(canonical.read_text())
    if len(owner['steps']) != len(case['steps']):
        raise ValueError('native ORCID step inventory differs')
    for source, target in zip(owner['steps'], case['steps']):
        for field in ['max_bytes', 'max_tokens', 'full_output_bytes']:
            target[field] = source[field]
    (cases / name).write_text(json.dumps(case, indent=2) + '\n')


def controls(cases):
    root = Path(__file__).resolve().parent.parent
    for source in (root / 'tests/cases/orcid').glob('*.yaml'):
        case = json.loads(source.read_text())
        case['replay'] = str(root / 'tests/replies' / case['replay'])
        (cases / ('control-' + source.name)).write_text(json.dumps(case, indent=2) + '\n')
    for name, source, indices in [
        ('papers-help', 'tests/cases/author/papers-help.yaml', [0]),
        ('search-help', 'tests/cases/author/help.yaml', [0]),
        ('get-help', 'tests/cases/author/help.yaml', [1]),
        ('list-help', 'tests/cases/author/help.yaml', [2]),
        ('whole-help', 'tests/cases/gene/help.yaml', [0]),
    ]:
        case = json.loads((root / source).read_text())
        case['steps'] = [case['steps'][index] for index in indices]
        case.pop('replay', None)
        # Measure proposed help metadata before the PM rules its canonical caps.
        # This staging-only measurement never certifies an existing output ceiling.
        for step in case['steps']:
            step['max_bytes'] = None
            step['max_tokens'] = None
        (cases / ('measurement-' + name + '.yaml')).write_text(json.dumps(case, indent=2) + '\n')


def identifiers(expect, prefix, native):
    expect[prefix] = {'count': len(native)}
    for index, external in enumerate(native):
        for source, normalized in [('type', 'kind'), ('value', 'value'), ('relationship', 'relationship')]:
            expect[f'{prefix}/{index}/{normalized}'] = external['external-id-' + source]


def group_step(identifier, total, index, group):
    expect = {'/status': 'complete', '/result/observed_groups': total}
    identifiers(expect, '/result/groups/0/external_ids', group['external-ids']['external-id'])
    pivots, pivot_ids = [], set()
    for i, assertion in enumerate(group['work-summary']):
        prefix = f'/result/groups/0/assertions/{i}'
        expect[prefix + '/put_code'] = assertion['put-code']
        expect[prefix + '/work_type'] = assertion['type']
        expect[prefix + '/visibility'] = 'public'
        identifiers(expect, prefix + '/external_ids', assertion['external-ids']['external-id'])
        for name, value in assertion['source'].items():
            if value is not None:
                expect[prefix + '/source/' + name.replace('-', '_')] = value.get('value', value.get('path'))
        for external in assertion['external-ids']['external-id']:
            kind, value = external['external-id-type'], external['external-id-value']
            pattern = r'10\.[0-9]+/[^\s\x00-\x1f\x7f]+' if kind == 'doi' else r'[1-9][0-9]*'
            if external['external-id-relationship'] == 'self' and kind in ['pmid', 'doi'] and re.fullmatch(pattern, value):
                identity = (kind, value.lower() if kind == 'doi' else value)
                if identity not in pivot_ids:
                    pivot_ids.add(identity)
                    pivots.append(['get', 'article', value])
    expect['/result/groups/0/assertions'] = {'count': len(group['work-summary'])}
    reserved = 1 if index + 1 < total else 0
    for i, args in enumerate(pivots[:5 - reserved]):
        expect[f'/next/{i}/args'] = args
    return step(['author', 'papers', 'orcid:' + identifier, '--limit', '1', '--offset', str(index)], expect)
