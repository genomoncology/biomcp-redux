// Runs every example in examples/<id>/run.sh against the program built from this commit.
//   npm run examples            run all, save output.txt and exit
//   npm run examples -- ID...   run some
//   --check                     the build check: exact mode compares byte for byte;
//                               live mode checks the exit code and saves fresh output
import { spawn } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import product from '../product.config.mjs';

const args = process.argv.slice(2);
const check = args.includes('--check');
const only = args.filter((a) => !a.startsWith('--'));
const root = path.resolve('examples');
const bin = path.resolve(product.binDir);
if (!fs.existsSync(path.join(bin, product.binary))) {
  console.error(`missing ${path.join(bin, product.binary)}: build the product first (cargo build --release)`);
  process.exit(1);
}
const ids = fs.readdirSync(root).filter((d) => fs.existsSync(path.join(root, d, 'run.sh')) && (!only.length || only.includes(d))).sort();

function run(id) {
  return new Promise((resolve) => {
    const cwd = path.join(root, id);
    const env = { ...process.env, ...product.exampleEnv, PATH: `${bin}:${process.env.PATH}`, NO_COLOR: '1', TERM: 'dumb', COLUMNS: '100' };
    const child = spawn('bash', ['-c', 'bash run.sh 2>&1'], { cwd, env });
    let out = '';
    child.stdout.on('data', (b) => (out += b));
    const timer = setTimeout(() => child.kill('SIGKILL'), product.exampleTimeoutMs ?? 120000);
    child.on('close', (code) => {
      clearTimeout(timer);
      resolve({ id, out: out.replace(/[ \t]+$/gm, '').replace(/\s+$/, '') + '\n', exit: code ?? 124 });
    });
  });
}

const failures = [];
let next = 0;
async function worker() {
  while (next < ids.length) {
    const id = ids[next++];
    const dir = path.join(root, id);
    const saved = (f) => (fs.existsSync(path.join(dir, f)) ? fs.readFileSync(path.join(dir, f), 'utf8') : null);
    if (check && saved('unverified') !== null) { console.log(`skip ${id} (unverified: ${saved('unverified').trim()})`); continue; }
    const r = await run(id);
    if (check && product.exampleMode === 'exact') {
      if (saved('output.txt') !== r.out || Number((saved('exit') ?? '0').trim()) !== r.exit) failures.push(`${id}: output or exit differs from the saved run; run npm run examples -- ${id} and review the diff`);
      continue;
    }
    if (check) {
      const want = Number((saved('exit') ?? '0').trim());
      if (r.exit !== want) { failures.push(`${id}: exit ${r.exit}, saved ${want}\n${r.out.slice(0, 400)}`); continue; }
      if (!r.out.trim()) { failures.push(`${id}: empty output`); continue; }
    }
    fs.writeFileSync(path.join(dir, 'output.txt'), r.out);
    fs.writeFileSync(path.join(dir, 'exit'), `${r.exit}\n`);
    console.log(`${r.exit === 0 ? 'ok  ' : `exit ${r.exit}`} ${id}`);
  }
}
await Promise.all(Array.from({ length: product.exampleWorkers ?? 4 }, worker));
if (failures.length) {
  console.error(`\n${failures.length} example(s) failed:\n` + failures.join('\n'));
  process.exit(1);
}
console.log(`${ids.length} example(s) ${check ? 'checked' : 'ran'}`);
