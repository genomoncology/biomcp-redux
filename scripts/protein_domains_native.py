"""Prepare exact retained source replies and fixed protein relation stories."""
import json
from pathlib import Path
import sys
import tarfile
from urllib.parse import quote, urlencode, urlsplit
archives, output = map(Path, sys.argv[1:])
output.mkdir()
requests = {}
with tarfile.open(archives/'j14202-native-sources.tar.gz') as archive:
    receipts = json.load(archive.extractfile('receipts.json'))
    for row in receipts:
        if row['source'] not in ['interpro', 'complexportal', 'uniprot']:
            continue
        parts = urlsplit(row['url'])
        if row['source'] == 'interpro':
            target = parts.path + '?page_size=25'
        elif row['source'] == 'complexportal':
            target = parts.path + '?' + urlencode([('number', '25'), ('filters', 'species_f:("Homo sapiens")')], quote_via=quote)
        else:
            target = parts.path
        requests[target] = archive.extractfile(row['source']+'-'+row['accession']+'.body').read().decode()
with tarfile.open(archives/'j14211-j14221-identity-followup.tar.gz') as archive:
    requests['/intact/complex-ws/complex/CPX-715'] = archive.extractfile('complex-CPX-715.body').read().decode()
(output/'requests.json').write_text(json.dumps(requests))
cases = output/'cases'
cases.mkdir()
budgets = json.loads((Path(__file__).resolve().parent.parent / 'tests/protein-domain-budgets.yaml').read_text())

def case(name, args, expect):
    ceiling = (budgets[name][1] * 115 + 99) // 100
    (cases/(name+'.yaml')).write_text(json.dumps({'feature':'protein-domains-complexes',
        'story':'A caller reads native protein source facts with exact participant admission',
        'replay':str((output/'replay').resolve()), 'steps':[{'run':args, 'mcp':args, 'expect':expect, 'max_bytes':ceiling, 'max_tokens':(ceiling+3)//4}]}))

for accession, total in [('P15056',11),('P04637',9),('Q8N726',1)]:
    case('domains-'+accession,['protein','domains',accession],{'/status':'complete','/result/native_total':total,'/page/total':total,'/page/returned':min(5,total)})
case('domains-Q8N726',['protein','domains','Q8N726'],{'/status':'complete','/result/native_total':1,'/page/total':1,'/result/items/0/id':'IPR010868','/result/items/0/entry_type':'family'})
case('tp53-family',['protein','domains','P04637','--type','family'],{'/status':'complete','/result/items/*/id':['IPR002117'],'/result/items/*/entry_type':['family']})
case('tp53-domain',['protein','domains','P04637','--type','domain'],{'/status':'complete','/result/items/0/id':'IPR010991','/result/items/0/entry_type':'domain','/result/native_total':9})
case('mutyh-domain-empty',['protein','domains','Q8N726','--type','domain'],{'/status':'complete','/result/items':[],'/result/native_total':1,'/page/total':0})
for accession, native, admitted in [('P04637',18,11),('P38398',9,6),('P15056',18,12),('P13569',0,0)]:
    case('complexes-'+accession,['protein','complexes',accession],{'/status':'complete','/result/native_total':native,'/page/total':admitted,'/page/returned':min(5,admitted)})
case('members-CPX-715',['protein','complex-members','P38398','--complex','CPX-715'],{'/status':'complete','/page/total':2,'/result/items/*/id':['P38398','Q99728'],'/result/items/*/stoichiometry':['minValue: 1, maxValue: 1']*2})
for accession, section in [('P15056','domains'),('P38398','complexes')]:
    case('get-'+section,['get','protein',accession,section],{'/status':'complete',f'/result/items/0/sections/{section}/state':'data'})
case('get-all',['get','protein','P00533','all'],{'/status':'complete','/result/items/0/sections/domains/state':'data','/result/items/0/sections/complexes/state':'empty',**{f'/result/items/0/sections/{s}/state':'data' for s in ['features','isoforms','structures','sequence','function']}})
