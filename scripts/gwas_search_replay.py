"""Build external cases and recordings from the retained GWAS archives, without requests."""
import hashlib
import json
from pathlib import Path
import sys
import tarfile
from urllib.parse import parse_qsl, quote, urlencode, urlsplit, urlunsplit


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n')


def archive_rows(path, plan_name='plan.json', prefix=''):
    with tarfile.open(path) as archive:
        plan = json.load(archive.extractfile(prefix + plan_name))
        for index, request in enumerate(plan['requests']):
            yield request['id'], request['url'], archive.extractfile(prefix + f'{index:04d}.body').read().decode()


def canonical(url):
    parts = urlsplit(url)
    query = urlencode(sorted(parse_qsl(parts.query)), quote_via=quote)
    return urlunsplit((parts.scheme, parts.netloc, parts.path, query, ''))


def command(url):
    flags = {'rs_id': '--variant', 'efo_trait': '--trait', 'efo_id': '--trait-id',
             'mapped_gene': '--gene', 'accession_id': '--study', 'pubmed_id': '--pubmed'}
    query = dict(parse_qsl(urlsplit(url).query))
    args = ['search', 'gwas']
    for key, flag in flags.items():
        if key in query:
            args += [flag, query[key]]
    for key, flag in [('show_child_trait', '--include-child-traits'), ('extended_geneset', '--extended-geneset')]:
        if query.get(key) == 'true':
            args.append(flag)
    if query['size'] != '3':
        args += ['--limit', query['size']]
    if query['page'] != '0':
        args += ['--offset', str(int(query['page']) * int(query['size']))]
    return args


def record(root, url, text):
    url = canonical(url)
    key = hashlib.sha256(('GET\0' + url).encode()).hexdigest()
    write(root / 'fixtures/replies/native/gwas' / (key + '.json'), {
        'source': 'gwas', 'method': 'GET', 'url': url, 'exchanges': [{
            'status': 200, 'content_type': 'application/json', 'delay_ms': 0,
            'retry_after_ms': None, 'body': {'kind': 'text', 'value': text}}]})


def case(root, name, url, body, expected_ids=None):
    text = json.loads(body)
    rows = text.get('_embedded', {}).get('associations', [])
    args = command(url)
    expect = {'/status': 'complete', '/page/total': text['page']['totalElements'],
              '/query/query/order': 'p_value,asc', '/result/items': {'count': len(rows)}}
    if expected_ids is not None:
        expect['/result/items/*/id' if rows else '/result/items'] = expected_ids
    record(root, url, body)
    budgets = json.loads((Path(__file__).resolve().parent.parent / 'tests/gwas-search-budgets.yaml').read_text())
    _, comparable = budgets[name]
    cap = ((comparable + 1) * 115 + 99) // 100
    step = {'run': args, 'mcp': args, 'expect': expect, 'max_bytes': cap, 'max_tokens': (cap + 3) // 4}
    if name == 'pubmed-second':
        expect.update({'/result/items/2/alleles/0/rs_id':'rs981782',
                       '/result/items/0/traits':['Breast cancer'],
                       '/result/items/0/ontology_traits':[{'id':'MONDO_0004989','label':'breast carcinoma'}],
                       '/result/items/0/alleles':[{'rs_id':'rs13281615','allele':'C'}],
                       '/result/items/0/exact_p_value':{'equals':{'mantissa':'5','exponent':-12}},
                       '/result/items/0/beta':'-', '/result/items/0/interval':'[1.05-1.11]',
                       '/result/items/0/risk_frequency':'0.40'})
    if name == 'tomlinson':
        expect.update({'/result/items/0/odds_ratio':1.27, '/result/items/0/traits':['Colorectal cancer'],
                       '/result/items/0/alleles':[{'rs_id':'rs6983267','allele':'G'}],
                       '/result/items/0/exact_p_value':{'equals':{'mantissa':'1','exponent':-14}},
                       '/result/items/0/beta':'-', '/result/items/0/interval':'[1.16-1.39]',
                       '/result/items/0/risk_frequency':'0.49'})
    # Projection ceilings are starting measurements, not claims about shipped output.
    write(root / 'fixtures/gwas-native-cases' / (name + '.yaml'), {
        'feature': 'gwas-search', 'live_check': 'retained restricted source replies outside Git',
        'story': name, 'replay': 'native', 'steps': [step]})


def main():
    root = Path(sys.argv[1])
    previous, current, original = map(Path, sys.argv[2:5])
    known = {'variant-positive':[6110,191182970,217418586],
             'trait-text-positive':[41310323,93928402,41311109], 'trait-id-positive':[41310323,93928402,41311109],
             'gene-positive':[41310323,93928402,105594966], 'study-positive':[6110,6111,6112],
             'study-page-one':[6113,6114,11346], 'gene-trait-positive':[41310323,93928402,41311109]}
    selected = {'variant-positive', 'trait-text-positive', 'trait-id-positive', 'gene-positive',
                'study-positive', 'study-page-one', 'gene-trait-positive', 'variant-negative',
                'trait-text-negative', 'trait-id-negative', 'gene-negative', 'study-negative',
                'pubmed-negative'}
    for name, url, body in archive_rows(previous):
        if name in selected:
            case(root, name, url, body, known.get(name, []))
    fixed = {'pubmed-second': [6113, 6114, 11346], 'stacey': [6117, 6116], 'broderick': [5949],
             'tomlinson': [6063], 'samani': [11386, 11850, 6056, 11851, 6057, 11852, 11385],
             'children': [41310323, 93928402, 41311109],
             'extended': [41310323, 93928402, 105594966], 'variant-publication': [6110]}
    for name, url, body in archive_rows(current):
        case(root, name, url, body, fixed[name])
    with tarfile.open(original) as archive:
        prefix = '2292-0193-gwas/metadata/'
        receipts = json.load(archive.extractfile(prefix + 'receipts.json'))
        row = next(r for r in receipts if r['name'] == '003')
        body = archive.extractfile(prefix + '003.body').read().decode()
        case(root, 'pubmed-first', row['url'], body, [6110, 6111, 6112])
    print('Prepared 22 external native cases; zero source requests')


if __name__ == '__main__':
    main()
