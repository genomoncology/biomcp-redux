"""Retained contained baseline recipe; no execution on import.

Derived from the immutable stage2 pass.py witness. Inputs and bodies stay outside Git.
Call with an owned experiment directory only after its capture budget is approved.
"""
import pathlib,json,hashlib,os,threading,subprocess,http.server,urllib.parse,urllib.request,urllib.error,datetime,time,socket,errno
import sys
import textwrap
from capture_outcome import classify, comparison_eligible

def setup(root):
 global ROOT,OUT,PLAN,ENVNAMES,BINARY,SHA,POLICY,PORT,CAPS,COUNTS,LOCK,PHYSICAL,LOCAL,CACHE,FAILED,CURRENT,COMMANDS,RUNNING,ALLOWED,LABELS,CHEMBL,PAGES,LAST,BASES,OVERRIDES
 ROOT=pathlib.Path(root);OUT=ROOT/'stage2';OUT.mkdir();(OUT/'captures').mkdir();(OUT/'commands').mkdir()
 PLAN=json.loads((ROOT/'inventory.json').read_text());ENVNAMES=json.loads((ROOT/'base-env-names.json').read_text());BINARY=ROOT/'baseline/biomcp';SHA='54f66769d9193c569d1b65756ad766eafd1a2c5e92d08f8c855cc8964e9a7145';assert hashlib.sha256(BINARY.read_bytes()).hexdigest()==SHA
 POLICY=ROOT/'containment-probe-2/policy.sb';assert hashlib.sha256(POLICY.read_bytes()).hexdigest()=='1024eee91361e2058c12b4b733b18c90e430780574198d462a836f3fa2307a8f';PORT=int(POLICY.read_text().split('localhost:')[1].split('"')[0])
 CAPS={('drug','mychem'):32,('drug','openfda'):16,('drug','civic'):4,('drug','chembl'):4,('drug','opentargets'):4,('disease','mydisease'):28};COUNTS={k:0 for k in CAPS};LOCK=threading.RLock();PHYSICAL=[];LOCAL=[];CACHE={};FAILED=set();CURRENT={};COMMANDS=[];RUNNING=[];ALLOWED=set();LABELS=set();CHEMBL=set();PAGES=set();LAST={}
 BASES={'mychem':'https://mychem.info','mydisease':'https://mydisease.info','openfda':'https://api.fda.gov','civic':'https://civicdb.org','chembl':'https://www.ebi.ac.uk','opentargets':'https://api.platform.opentargets.org'}
 OVERRIDES={'BIOMCP_MYCHEM_BASE':'/mychem/v1','BIOMCP_MYDISEASE_BASE':'/mydisease/v1','BIOMCP_OPENFDA_BASE':'/openfda','BIOMCP_CIVIC_BASE':'/civic/api','BIOMCP_CHEMBL_BASE':'/chembl/chembl/api/data','BIOMCP_OPENTARGETS_BASE':'/opentargets/api/v4'}

def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def dump(p,v):p.write_text(json.dumps(v,indent=2,ensure_ascii=False))
def key(method,url,body):
 u=urllib.parse.urlsplit(url);return (method,u.scheme,u.netloc,u.path,tuple(sorted(urllib.parse.parse_qsl(u.query,keep_blank_values=True))),json.dumps(body,sort_keys=True,separators=(',',':')) if body is not None else None)
def esc(s):return ''.join(('\\'+c if c in '\\+-!(){}[]^"~*?:/&|' else c) for c in s)
def declaration(method,url,body):ALLOWED.add(key(method,url,body))
def getdecl(source,path,params):declaration('GET',BASES[source]+path+'?'+urllib.parse.urlencode(params),None)
def writeledger():dump(OUT/'physical-requests.json',PHYSICAL);dump(OUT/'budget.json',dict(total=len(PHYSICAL),cap=88,source_counts={a+'/'+b:v for (a,b),v in COUNTS.items()},source_caps={a+'/'+b:v for (a,b),v in CAPS.items()}))
def admit_native_names(body):
 try:obj=json.loads(body)
 except ValueError:return
 if not isinstance(obj,dict):return
 hits=obj.get('hits',[])
 if not isinstance(hits,list):return
 for h in hits:
  if not isinstance(h,dict):continue
  c=h.get('chembl')
  if isinstance(c,dict):
   v=c.get('molecule_chembl_id')
   if isinstance(v,str) and v.startswith('CHEMBL') and v[6:].isdigit():CHEMBL.add(v)
 # Native IDs only permit the old command to send its own exact ID; no local allele/product equivalence is asserted.
def declare_drug_branches():
 n=CURRENT.get('name');mode=CURRENT.get('mode')
 if not n:return
 e=esc(n)
 if mode in ('card','label','search'):
  getdecl('openfda','/drug/label.json',{'search':f'openfda.generic_name:"{e}" OR openfda.brand_name:"{e}"','limit':'5','sort':'effective_time:desc'})
  getdecl('openfda','/drug/label.json',{'search':f'openfda.generic_name.exact:"{esc(n.upper())}"','limit':'5','sort':'effective_time:desc'})
  getdecl('openfda','/drug/label.json',{'search':f'spl_product_data_elements:"{e}"','limit':'10'})
 if mode=='card':
  for c in PLAN['conditional_request_graphs']:
   if c.get('source')=='civic' and c.get('body'):
    b=json.loads(json.dumps(c['body']));b['variables']['therapyName']=n;declaration('POST',c['url'],b)
   if c.get('source')=='opentargets' and c.get('body') and 'chemblId' in c['body'].get('variables',{}):
    for native in CHEMBL:
     b=json.loads(json.dumps(c['body']));b['variables']['chemblId']=native;declaration('POST',c['url'],b)
  for native in CHEMBL:getdecl('chembl','/chembl/api/data/mechanism.json',{'molecule_chembl_id':native,'limit':'15'})
  getdecl('openfda','/drug/event.json',{'search':f'(patient.drug.openfda.generic_name:"{e}" OR patient.drug.openfda.brand_name:"{e}" OR patient.drug.medicinalproduct:"{e}") AND patient.drug.drugcharacterization:1','count':'patient.reaction.reactionmeddrapt.exact','limit':'50'})

class NoRedirect(urllib.request.HTTPRedirectHandler):
 def redirect_request(self,*a,**k):return None
OPENER=urllib.request.build_opener(urllib.request.ProxyHandler({}),NoRedirect())
class Handler(http.server.BaseHTTPRequestHandler):
 protocol_version='HTTP/1.0'
 def log_message(self,*a):pass
 def handle_one_request(self):
  try:super().handle_one_request()
  except (BrokenPipeError,ConnectionResetError):pass
 def do_GET(self):self.record()
 def do_POST(self):self.record()
 def do_HEAD(self):self.record()
 def do_PUT(self):self.record()
 def do_DELETE(self):self.record()
 def record(self):
  start=time.monotonic();parsed=urllib.parse.urlsplit(self.path);parts=parsed.path.split('/',2);source=parts[1] if len(parts)>1 else '';path='/'+parts[2] if len(parts)>2 else '/';url=BASES.get(source,'https://invalid.invalid')+path+('?' +parsed.query if parsed.query else '')
  length=int(self.headers.get('Content-Length','0'));raw=self.rfile.read(length) if length<1048576 else b''
  try:body=json.loads(raw) if raw else None
  except ValueError:body=None
  identity=key(self.command,url,body);status=409;content=b'{"error":"undeclared_or_budget_exhausted"}';headers={'Content-Type':'application/json'};reason=None;capture=None;reused=False
  with LOCK:
   local={'number':len(LOCAL)+1,'command_index':CURRENT.get('index'),'questions':CURRENT.get('question_ids'),'source':source,'local_method':self.command,'local_url':f'http://127.0.0.1:{PORT}'+self.path,'provider_url':url,'request_body':body,'request_body_sha256':hashlib.sha256(raw).hexdigest(),'seen_at':now()};LOCAL.append(local)
   declare_drug_branches()
   safe=not self.headers.get('Authorization') and not self.headers.get('Proxy-Authorization') and length<1048576 and self.headers.get('Host')==f'127.0.0.1:{PORT}'
   bucket=(CURRENT.get('slice'),source)
   if not safe:reason='credential_or_header_refusal'
   elif identity not in ALLOWED:reason='undeclared_identity'
   elif identity in CACHE:
    capture=CACHE[identity];status,content,headers=capture['status'],(OUT/capture['file']).read_bytes(),capture['delivery_headers'];reused=True
   elif bucket in FAILED:reason='source_stopped_after_failure'
   elif len(PHYSICAL)>=88 or COUNTS.get(bucket,0)>=CAPS.get(bucket,0):reason='budget_exhausted'
   else:
    COUNTS[bucket]+=1;capture={'number':len(PHYSICAL)+1,'slice':bucket[0],'source':source,'method':self.command,'url':url,'body':body,'request_body_sha256':hashlib.sha256(raw).hexdigest(),'command_index':CURRENT['index'],'retrieved_at':now(),'state':'reserved_before_forward','fresh_provider_call':True};PHYSICAL.append(capture);writeledger()
  if capture and not reused and capture.get('state')=='reserved_before_forward':
   request_headers={k:v for k,v in self.headers.items() if k.lower() in ('accept','content-type','user-agent')}
   request=urllib.request.Request(url,data=raw if raw else None,headers=request_headers,method=self.command)
   truncated=False
   try:
    try:response=OPENER.open(request,timeout=20)
    except urllib.error.HTTPError as ex:response=ex
    with response:
     status=response.status;content=response.read(8*1024*1024+1);truncated=len(content)>8*1024*1024;headers={k:v for k,v in response.headers.items() if k.lower() in ('content-type','retry-after')};response_headers=dict(response.headers)
    if truncated:reason='body_cap_exceeded'
   except Exception as ex:
    status=502;content=json.dumps({'capture_error':type(ex).__name__}).encode();headers={'Content-Type':'application/json'};response_headers={};reason='provider_transport_failure'
   with LOCK:
    file=f'captures/{capture["number"]:04d}.body';(OUT/file).write_bytes(content);capture.update(status=status,file=file,bytes=len(content),sha256=hashlib.sha256(content).hexdigest(),response_headers=response_headers,delivery_headers=headers,elapsed_seconds=time.monotonic()-start,truncated=truncated,state='complete',transport_failure=reason=='provider_transport_failure')
    outcome=classify(source,url,status,content,truncated=truncated,transport_failure=reason=='provider_transport_failure')
    capture['outcome']=outcome.kind.value
    if outcome.stop_source:FAILED.add(bucket)
    if outcome.comparison_eligible:CACHE[identity]=capture
    if source=='mychem' and status==200 and not reason:
     admit_native_names(content)
     if CURRENT.get('mode')=='search':
      try:
       obj=json.loads(content);hits=obj['hits'];q=dict(urllib.parse.parse_qsl(parsed.query));offset=int(q['from']);total=obj['total'];nextoffset=offset+len(hits)
       if len(hits)>0 and nextoffset<total and total<=10000:q['from']=str(nextoffset);getdecl(source,path,q)
       if len(hits)==0 or nextoffset>=total:
        # Empty candidate ranking may reissue its separate size10 search; exact q/fields remain unchanged.
        q['size']='10';q['from']='0';getdecl(source,path,q)
      except (ValueError,KeyError,TypeError):pass
    writeledger()
  with LOCK:
   outcome=classify(source,url,status,content,truncated=capture.get('truncated',False) if capture else False,transport_failure=capture.get('transport_failure',False) if capture else False,refusal=reason if not capture else None)
   local.update(outcome=outcome.kind.value,reason=reason,capture_number=capture.get('number') if capture else None,reused_retained_body=reused,status=status,provider_fresh=bool(capture and not reused),response_sha256=hashlib.sha256(content).hexdigest());dump(OUT/'local-requests.json',LOCAL)
  try:self.send_response(status);[self.send_header(k,v) for k,v in headers.items()];self.send_header('Content-Length',str(len(content)));self.end_headers();self.wfile.write(content)
  except (BrokenPipeError,ConnectionResetError):pass

class Server(http.server.ThreadingHTTPServer):daemon_threads=False

def start():
 global SERVER,THREAD
 SERVER=Server(('127.0.0.1',PORT),Handler);THREAD=threading.Thread(target=SERVER.serve_forever);THREAD.start()
 # Recheck the exact policy before the first old execution or provider forwarding.
 forbidden=socket.socket();forbidden.bind(('127.0.0.1',0));forbidden.listen(1);forbidden.settimeout(.2)
 probe=textwrap.dedent("""
 import socket,errno,json,sys
 p,o=map(int,sys.argv[1:]);r=[]
 for proto,host,port in [('tcp','127.0.0.1',p),('tcp','127.0.0.1',o),('tcp','192.0.2.1',9),('udp','192.0.2.1',9)]:
  s=socket.socket(socket.AF_INET,socket.SOCK_DGRAM if proto=='udp' else socket.SOCK_STREAM);s.settimeout(1)
  try:s.sendto(b'probe',(host,port)) if proto=='udp' else s.connect((host,port));r.append({'allowed':True})
  except OSError as e:r.append({'allowed':False,'errno':e.errno})
  finally:s.close()
 print(json.dumps(r));assert r[0]['allowed'] and all(not x['allowed'] and x['errno']==errno.EPERM for x in r[1:])
 """)
 (OUT/'probe.py').write_text(probe);probe_cmd=['/usr/bin/sandbox-exec','-f',str(POLICY),'/usr/bin/python3',str(OUT/'probe.py'),str(PORT),str(forbidden.getsockname()[1])];dump(OUT/'probe-command.json',probe_cmd)
 p=subprocess.run(probe_cmd,capture_output=True,timeout=10);(OUT/'probe.stdout').write_bytes(p.stdout);(OUT/'probe.stderr').write_bytes(p.stderr)
 try:forbidden.accept();denied_accept=True
 except socket.timeout:denied_accept=False
 forbidden.close();dump(OUT/'probe-result.json',dict(exit=p.returncode,denied_listener_accepted=denied_accept,policy_sha256=hashlib.sha256(POLICY.read_bytes()).hexdigest()));assert p.returncode==0 and not denied_accept
 print('Containment probe passed; no provider request yet.',flush=True)

def run():
 try:
  for index,command in enumerate(PLAN['commands'],1):
   # A first-run per command cache directory is intentionally empty; disease warm controls share the original disease home.
   home=OUT/('disease-home' if command['question_ids'][0].startswith('disease:') else f'commands/{index:03d}/home');home.mkdir(parents=True,exist_ok=True);work=OUT/'commands'/f'{index:03d}';work.mkdir(exist_ok=True)
   slice='disease' if command['question_ids'][0].startswith('disease:') else 'drug';CURRENT.clear();CURRENT.update(command,index=index,slice=slice)
   if slice=='drug':CURRENT['name']=command['argv'][4] if command['mode']=='search' else command['argv'][5]
   ALLOWED.clear();CHEMBL.clear()
   for id in command['initial_request_ids']:
    r=next(x for x in PLAN['statically_determined_requests'] if x['request_identity_sha256']==id);declaration(r['method'],r['url'],r['body'])
   env={k:os.environ[k] for k in ['PATH','LANG','LC_ALL','LC_CTYPE','SYSTEM_VERSION_COMPAT','__CF_USER_TEXT_ENCODING'] if k in os.environ};env.update(HOME=str(home),XDG_CONFIG_HOME=str(home/'config'),XDG_CACHE_HOME=str(home/'cache'),XDG_DATA_HOME=str(home/'data'),TMPDIR=str(home/'tmp'))
   (home/'tmp').mkdir(exist_ok=True)
   for name in ENVNAMES:env[name]=f'http://127.0.0.1:{PORT}/denied/{name}'
   for name,path in OVERRIDES.items():env[name]=f'http://127.0.0.1:{PORT}'+path
   cmd=['/usr/bin/sandbox-exec','-f',str(POLICY),str(BINARY),*command['argv']];dump(work/'command.json',cmd);dump(work/'environment-names.json',sorted(env));start=time.monotonic();before=len(LOCAL);beforephysical=len(PHYSICAL)
   try:p=subprocess.run(cmd,env=env,cwd=home,capture_output=True,timeout=30);code=p.returncode;stdout=p.stdout;stderr=p.stderr
   except subprocess.TimeoutExpired as ex:code=124;stdout=ex.stdout or b'';stderr=ex.stderr or b''
   # Wait for every handler before changing the command context.
   while threading.active_count()>2:time.sleep(.05)
   (work/'stdout.json').write_bytes(stdout);(work/'stderr.txt').write_bytes(stderr)
   rows=LOCAL[before:];reasons=sorted({r['reason'] for r in rows if r.get('reason')})
   result=dict(index=index,question_ids=command['question_ids'],mode=command['mode'],argv=command['argv'],exit=code,bytes=len(stdout),estimated_tokens_bytes_div_4=len(stdout)/4,actual_tokenizer_count=None,sha256=hashlib.sha256(stdout).hexdigest(),local_requests=len(rows),new_provider_requests=len(PHYSICAL)-beforephysical,local_reused_body_deliveries=sum(r.get('reused_retained_body',False) for r in rows),refusal_reasons=reasons,elapsed_seconds=time.monotonic()-start,comparison_eligible=comparison_eligible(code,rows),facts_review='fresh clinical review required; no completeness inferred from exit or HTTP status',path=f'commands/{index:03d}/stdout.json')
   COMMANDS.append(result);dump(OUT/'commands.json',COMMANDS);print(json.dumps(result),flush=True)
 finally:
  SERVER.shutdown();SERVER.server_close();THREAD.join();writeledger();dump(OUT/'local-requests.json',LOCAL)
  dump(OUT/'run.json',{'published_main':'d45ef78f6e1bd9e99215810079061150a74e575d','old_source_tag_commit':'5641de7d7b79b53872f8599dbb4dba22e611dcaf','binary_sha256':SHA,'finished_at':now(),'commands':len(COMMANDS),'new_physical_requests':len(PHYSICAL),'policy_sha256':hashlib.sha256(POLICY.read_bytes()).hexdigest(),'source_budget':{a+'/'+b:v for (a,b),v in COUNTS.items()},'all_caps_observed':len(PHYSICAL)<=88 and all(COUNTS[k]<=CAPS[k] for k in CAPS),'provider_redirects_followed':0,'source_refreshes':0,'retained_body_reuse':'Only an exact method/URL/query/JSONbody identity from this pass; immutable captured bytes and original retrieval time retained. No earlier unmatched body used.','comparison_scope':'mechanical outputs only; actual tokenizer count and clinical completeness unmeasured pending review'})

if __name__ == '__main__':
 setup(sys.argv[1])
 start()
 run()
