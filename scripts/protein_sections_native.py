"""Reuse accepted UniProt replies for source section command cases outside Git."""
import json
from pathlib import Path
import sys
from protein_native_replay import prepare
archives, output = map(Path, sys.argv[1:])
replay = prepare(archives, output, sections=True)
cases = output / 'cases'
cases.mkdir()
budget_path = Path(__file__).resolve().parent.parent / 'tests/protein-section-budgets.yaml'
budgets = json.loads(budget_path.read_text()) if budget_path.is_file() else {}


def case(name, args, expect, exit=0):
    step = {'run': args, 'mcp': args, 'expect': expect, 'exit': exit}
    step['max_bytes'] = (budgets[name][1] * 115 + 99) // 100
    step['max_tokens'] = (step['max_bytes'] + 3) // 4
    cases.joinpath(name + '.yaml').write_text(json.dumps({
        'feature': 'protein-sections', 'story': 'A caller reads exact native protein section facts',
        'replay': str(replay.parent.resolve()), 'steps': [step]}))


for accession, point, description, start, end in [
        ('P00533', 858, 'Protein kinase', 712, 979),
        ('P00533', 797, 'Protein kinase', 712, 979),
        ('P00533', 790, 'Protein kinase', 712, 979),
        ('P15056', 600, 'Protein kinase', 457, 717),
        ('P13569', 508, 'ABC transporter 1', 423, 646),
        ('P31749', 17, 'PH', 5, 108)]:
    for position in sorted({point, start, end}):
        case(f'position-{accession}-{position}', ['protein', 'features', accession,
             '--position', str(position), '--type', 'Domain'], {
            '/status': 'complete', '/page/total': 1,
            '/result/items/0/feature/protein': accession,
            '/result/items/0/feature/description': description,
            '/result/items/0/feature/span/start/min': start - 1,
            '/result/items/0/feature/span/start/max': start - 1,
            '/result/items/0/feature/span/end/min': end,
            '/result/items/0/feature/span/end/max': end,
            '/result/items/0/feature/kind/label': 'Domain'})
case('brct-domains', ['protein', 'features', 'P38398', '--type', 'Domain'], {
    '/page/total': 2, '/result/items/*/feature/description': ['BRCT 1', 'BRCT 2'],
    '/result/items/*/feature/span/start/min': [1641, 1755],
    '/result/items/*/feature/span/end/min': [1736, 1855]})
case('brct-gap', ['protein', 'features', 'P38398', '--type', 'Domain', '--position', '1740'], {
    '/page/total': 0, '/result/items': []})
case('ring', ['protein', 'features', 'P38398', '--type', 'Zinc finger'], {
    '/result/items/0/feature/description': 'RING-type', '/result/items/0/feature/kind/label': 'Zinc finger',
    '/result/items/0/feature/span/start/min': 23, '/result/items/0/feature/span/end/min': 65})
case('dna-binding', ['protein', 'features', 'P04637', '--type', 'DNA binding'], {
    '/result/items/0/feature/span/start/min': 101, '/result/items/0/feature/span/end/min': 292})
case('active-site', ['protein', 'features', 'P00533', '--type', 'Active site'], {
    '/result/items/0/feature/span/start/min': 836, '/result/items/0/feature/span/end/min': 837})
for accession, ids in [
        ('P00533', ['P00533-1', 'P00533-2', 'P00533-3', 'P00533-4']),
        ('P04637', [f'P04637-{n}' for n in range(1, 6)]),
        ('Q9UIF7', [f'Q9UIF7-{n}' for n in range(1, 6)])]:
    case('isoforms-' + accession, ['protein', 'isoforms', accession], {
        '/result/items/*/ids/0': ids})
for accession in ['P42771', 'Q8N726']:
    case('external-' + accession, ['protein', 'isoforms', accession], {'/status': 'complete'})
for accession, length in [('P38398-2', 63), ('Q9UIF7-2', 536), ('Q9UIF7', 546), ('P38398', 1863)]:
    case('sequence-' + accession, ['protein', 'sequence', accession], {
        '/result/sequence/accession': accession, '/result/sequence/length': length})
for accession in ['P00533', 'P38398', 'Q9UIF7']:
    for section in ['features', 'isoforms', 'structures', 'sequence', 'function']:
        case(f'get-{accession}-{section}', ['get', 'protein', accession, section], {
            '/result/items/0/header/protein/accession': accession,
            f'/result/items/0/sections/{section}/state': 'data'})
case('structures', ['protein', 'structures', 'P00533'], {
    '/result/items/0/id': '1IVO', '/result/items/0/database': 'PDB',
    '/result/items/0/url': 'https://www.rcsb.org/structure/1IVO',
    '/result/items/0/properties/0/key': 'Method',
    '/result/items/0/properties/0/value': 'X-ray'})
for accession in ['P00533', 'Q9UIF7', 'P38398-2', 'BRAF']:
    case('card-' + accession, ['get', 'protein', accession], {'/status': 'complete'})
for accession, remainder in [('P04637', ['P04637-6', 'P04637-7', 'P04637-8', 'P04637-9']),
                             ('Q9UIF7', ['Q9UIF7-6'])]:
    path = cases / ('isoforms-' + accession + '.yaml')
    value = json.loads(path.read_text())
    follow = {'follow': 'page', 'expect': {'/result/items/*/ids/0': remainder}}
    follow['max_bytes'] = (budgets['walk-isoforms-' + accession][1] * 115 + 99) // 100
    follow['max_tokens'] = (follow['max_bytes'] + 3) // 4
    value['steps'].append(follow)
    walks = output / 'walks'
    walks.mkdir(exist_ok=True)
    (walks / path.name).write_text(json.dumps(value))
case('features-default', ['protein', 'features', 'P00533'], {
    '/page/total': 321, '/page/returned': 5,
    '/result/items/*/feature/kind/label': ['Signal', 'Chain', 'Topological domain', 'Transmembrane', 'Topological domain']})
for point, description, start, end in [(1642, 'BRCT 1', 1642, 1736), (1736, 'BRCT 1', 1642, 1736),
                                       (1756, 'BRCT 2', 1756, 1855), (1855, 'BRCT 2', 1756, 1855)]:
    case(f'brct-endpoint-{point}', ['protein', 'features', 'P38398', '--type', 'Domain', '--position', str(point)], {
        '/page/total': 1, '/result/items/0/feature/description': description,
        '/result/items/0/feature/span/start/min': start - 1, '/result/items/0/feature/span/end/min': end})
# The changed composite all proof belongs to protein_domains_native.py.
