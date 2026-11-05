#!/bin/sh
# Exercise FAERS reaction counts at the caller boundary.
set -eu
cd "$(dirname "$0")/.."
faers_python_bin=$(python3.13 -c 'from pathlib import Path; import sys; print(Path(sys.executable).resolve().parent)')
export PATH="$faers_python_bin:$HOME/.cargo/bin:$PATH"
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-focused}" in
  baseline)
    cargo build --locked -p biomcp --bin biomcp
    python3.13 - "${CARGO_TARGET_DIR:-target}/debug/biomcp" <<'PY'
import hashlib,json,subprocess,sys,tempfile
from pathlib import Path
from release.smoke import environment
binary=Path(sys.argv[1]).resolve()
output=Path('target/faers-counts-baseline')
output.mkdir(parents=True,exist_ok=True)
command=['search','adverse-event','daraxonrasib','--source','faers','--limit','10']
summary={}
for mode in ('single','typed'):
    with tempfile.TemporaryDirectory(prefix='faers-counts-') as directory:
        root=Path(directory).resolve()
        config=root/'config.json'
        config.write_text(json.dumps({'mcp_tool_mode':mode,'mcp_reply_form':'both'}))
        tool='biomcp' if mode=='single' else 'search_adverse_event'
        arguments={'args':command} if mode=='single' else {'text':'daraxonrasib','source':'faers','limit':10}
        counted={'args':command+['--aggregate','reactions']} if mode=='single' else {**arguments,'aggregate':'reactions'}
        frames=[{'jsonrpc':'2.0','id':1,'method':'initialize','params':{'protocolVersion':'2025-11-25','capabilities':{},'clientInfo':{'name':'faers-counts-baseline','version':'1'}}},
                {'jsonrpc':'2.0','method':'notifications/initialized','params':{}},
                {'jsonrpc':'2.0','id':2,'method':'tools/list','params':{}},
                {'jsonrpc':'2.0','id':3,'method':'tools/call','params':{'name':tool,'arguments':arguments}},
                {'jsonrpc':'2.0','id':4,'method':'tools/call','params':{'name':tool,'arguments':counted}}]
        request=''.join(json.dumps(frame)+'\n' for frame in frames)
        result=subprocess.run([str(binary),'serve','--hosted','--config',str(config)],input=request,text=True,capture_output=True,env=environment(root),cwd=root,timeout=120)
        (output/f'{mode}-requests.jsonl').write_text(request)
        (output/f'{mode}-replies.jsonl').write_text(result.stdout)
        (output/f'{mode}.stderr').write_text(result.stderr)
        if result.returncode: raise RuntimeError(f'{mode} server failed; see retained output')
        replies={frame['id']:frame.get('result',{}) for line in result.stdout.splitlines() if (frame:=json.loads(line)).get('id')}
        ordinary=replies[3].get('structuredContent',{})
        aggregate=replies[4].get('structuredContent',{})
        summary[mode]={'report_status':ordinary.get('status'),'report_row_fields':sorted(ordinary.get('result',{}).get('items',[{}])[0]) if ordinary.get('result',{}).get('items') else [],'aggregation_status':aggregate.get('status'),'aggregation_error':aggregate.get('error')}
        if aggregate.get('status')!='usage': raise RuntimeError(f'{mode}: aggregation is already supported; inspect before implementing')
summary.update(source_commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),evaluation_command=command)
(output/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
PY
    ;;
  generate)
    cargo fmt --all
    make schema-generate docs-generate
    ;;
  focused)
    python3.13 scripts/trials.py --case-file adverse-event/faers-reaction-counts.yaml
    ;;
  *) exit 2 ;;
esac
