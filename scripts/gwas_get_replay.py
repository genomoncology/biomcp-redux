"""Prepare only external retained GWAS get inputs, without source requests."""
import json
from pathlib import Path
import sys
import tarfile


def main():
    out = Path(sys.argv[1]) / 'gwas-get-native'
    out.mkdir(parents=True)
    rows = []
    with tarfile.open(sys.argv[2]) as archive:
        plan = json.load(archive.extractfile('plan.json'))
        for index, request in enumerate(plan['requests']):
            name, url = request['id'], request['url']
            body = archive.extractfile(f'{index:04d}.body').read().decode()
            native = json.loads(body)
            args = ['get', 'gwas']
            expect = {'/status': 'complete'}
            if name.startswith('association-'):
                identity = name.removeprefix('association-')
                args += ['association:' + identity]
                expect.update({'/result/association/id': int(identity),
                               '/result/association/study': native['accession_id'],
                               '/result/association/alleles': [{'rs_id': a['rs_id'], 'allele': a['effect_allele']} for a in native['snp_allele']]})
                if identity == '6114':
                    expect.update({'/result/association/alleles/0/rs_id': 'rs3817198',
                                   '/result/association/alleles/0/allele': 'C',
                                   '/result/association/study': 'GCST000035',
                                   '/result/association/publication': '17529967'})
                if identity == '6063':
                    expect.update({'/result/association/odds_ratio': 1.27,
                                   '/result/association/interval': '[1.16-1.39]',
                                   '/result/association/alleles/0/rs_id': 'rs6983267'})
                if identity == '11386':
                    expect.update({'/result/association/odds_ratio': 1.36,
                                   '/result/association/exact_p_value': {'equals': {'mantissa': '3', 'exponent': -19}},
                                   '/result/association/alleles/0/rs_id': 'rs1333049'})
            elif name.startswith('study-'):
                identity = name.removeprefix('study-')
                args += [identity]
                expect.update({'/result/id': identity,
                               '/result/initial_sample_description': native['initial_sample_size'],
                               '/result/replication_sample_description': native['replication_sample_size'],
                               '/result/discovery_ancestries': native['discovery_ancestry'],
                               '/result/replication_ancestries': native['replication_ancestry'],
                               '/result/reported_traits': [native['disease_trait']]})
            elif name == 'loci':
                args += ['association:6110', 'loci']
                expect.update({'/result/items/0/author_reported_genes/0/name': 'FGFR2',
                               '/result/items/0/author_reported_genes/0/ensembl_ids': ['ENSG00000066468'],
                               '/result/items/0/author_reported_genes/0/entrez_ids': ['2263']})
            elif name == 'ancestries':
                args += ['GCST000035', 'ancestries']
                expect.update({'/result/items': {'count': 3}, '/page/total': 3})
            else:
                page = int(name.rsplit('-', 1)[1])
                args += ['GCST000035', 'associations']
                if page:
                    args += ['--section-offset', '3']
                expect.update({'/result/items': {'count': 3}, '/page/total': 6,
                               '/result/items/*/id': [r['association_id'] for r in native['_embedded']['associations']],
                               '/query/operation': 'get', '/query/query/id': 'GCST000035'})
            rows.append({'name': name, 'url': url, 'body': body, 'args': args, 'expect': expect})
    (out / 'input.json').write_text(json.dumps(rows, ensure_ascii=False) + '\n')
    print(f'Prepared {len(rows)} native get responses; no provider requests')


if __name__ == '__main__':
    main()
