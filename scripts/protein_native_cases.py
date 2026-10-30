"""Prepare external protein command cases from the accepted native archives."""
import json
from pathlib import Path
import sys
from protein_native_replay import prepare
archives, output = map(Path, sys.argv[1:])
replay = prepare(archives, output)
budgets = json.loads((Path(__file__).resolve().parent.parent / 'tests/protein-section-budgets.yaml').read_text())
def card_ceiling(name):
    return (budgets['card-' + name][1] * 115 + 99) // 100

def case(name, steps):
    ceilings = {
        'P00533': [card_ceiling('P00533')], 'Q9UIF7': [card_ceiling('Q9UIF7')], 'P38398-2': [card_ceiling('P38398-2')],
        'CDKN2A': [1267], 'search-BRAF': [749], 'search-CDKN2A': [1076],
        'text-pages': [2330, 2250], 'gene-crossrefs': [card_ceiling('BRAF'), 970],
    }
    if len(steps) != len(ceilings[name]):
        raise ValueError('every native step needs its measured ceiling')
    for step, ceiling in zip(steps, ceilings[name]):
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
