"""Derive exact public-field assertions from accepted ORCID captures outside Git."""
import json
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
        group = works[index]
        expect = {'/status': 'complete', '/result/observed_groups': total}
        for i, assertion in enumerate(group['work-summary']):
            prefix = f'/result/groups/0/assertions/{i}'
            expect[prefix + '/put_code'] = assertion['put-code']
            expect[prefix + '/work_type'] = assertion['type']
            expect[prefix + '/visibility'] = 'public'
            for j, external in enumerate(assertion['external-ids']['external-id']):
                base = prefix + f'/external_ids/{j}'
                for native, normalized in [('type', 'kind'), ('value', 'value'), ('relationship', 'relationship')]:
                    expect[base + '/' + normalized] = external['external-id-' + native]
        expect['/result/groups/0/assertions'] = {'count': len(group['work-summary'])}
        pivots = []
        for assertion in group['work-summary']:
            for external in assertion['external-ids']['external-id']:
                if external['external-id-relationship'] == 'self' and external['external-id-type'] in ['pmid', 'doi']:
                    args = ['get', 'article', external['external-id-value']]
                    if args not in pivots:
                        pivots.append(args)
        reserved = 1 if index + 1 < total else 0
        for i, args in enumerate(pivots[:5 - reserved]):
            expect[f'/next/{i}/args'] = args
        steps.append(step(['author', 'papers', 'orcid:' + identifier, '--limit', '1', '--offset', str(index)], expect))
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
