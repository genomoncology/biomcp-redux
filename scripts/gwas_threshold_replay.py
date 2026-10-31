"""Prepare exact threshold cases from retained native pages without requests."""
import json
from pathlib import Path
import sys
import tarfile
from gwas_search_replay import archive_rows, record, write


def step(args, count, total):
    return {'run': args, 'mcp': args, 'expect': {
        '/status': 'complete', '/result/items': {'count': count},
        '/result/qualifying_total': {'equals': total}, '/page/total': 6}}


def case(root, name, args, first_count, first_total, second_count=None, second_total=None):
    steps = [step(args, first_count, first_total)]
    if second_count is not None:
        follow = step([], second_count, second_total)
        follow.pop('run')
        follow.pop('mcp')
        follow['follow'] = 'page'
        steps.append(follow)
    budgets = json.loads((Path(__file__).resolve().parent.parent / 'tests/gwas-threshold-budgets.yaml').read_text())
    first_key = name.split('-')[0] + '-crossing-first'
    for index, item in enumerate(steps):
        measured = budgets[first_key if index == 0 and len(steps) > 1 else name][1]
        cap = (measured * 115 + 99) // 100
        item.update(max_bytes=cap, max_tokens=(cap + 3) // 4)
    write(root / 'fixtures/gwas-threshold-cases' / f'{name}.yaml', {
        'feature': 'gwas-search', 'story': name,
        'live_check': 'retained restricted source replies outside Git',
        'replay': 'native', 'steps': steps})


def main():
    root = Path(sys.argv[1])
    previous, current, original = map(Path, sys.argv[2:5])
    for name, url, body in archive_rows(previous):
        if name in {'study-positive', 'study-page-one'}:
            record(root, url, body)
    for name, url, body in archive_rows(current):
        if name == 'pubmed-second':
            record(root, url, body)
    with tarfile.open(original) as archive:
        prefix = '2292-0193-gwas/metadata/'
        receipts = json.load(archive.extractfile(prefix + 'receipts.json'))
        row = next(r for r in receipts if r['name'] == '003')
        record(root, row['url'], archive.extractfile(prefix + '003.body').read().decode())
    for selector, value in [('pubmed', '17529967'), ('study', 'GCST000035')]:
        for label, threshold, count, total, second, final in [
            ('crossing', '1e-15', 3, None, 0, 3),
            ('crossing-first', '1e-15', 3, None, None, None),
            ('all-first', '1e0', 3, None, None, None),
            ('all-last', '1e0', 3, None, 3, 6),
            ('below-first', '1e-100', 0, 0, None, None)]:
            args = ['search', 'gwas', f'--{selector}', value, '--p-value', threshold]
            case(root, f'{selector}-{label}', args, count, total, second, final)
    print('Prepared ten threshold cases from four retained pages; zero source requests')


if __name__ == '__main__':
    main()
