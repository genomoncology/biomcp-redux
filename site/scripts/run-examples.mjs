// Runs every example in examples/<id>/run.sh against the program built from this commit.
//   npm run examples            run all, save output.txt, exit and ran
//   npm run examples -- ID...   run some (an example marked unverified runs only when named)
//   --check                     the build check, which never writes a tracked file:
//                               exact mode compares output and exit byte for byte;
//                               live mode writes fresh output to .example-runs/<id>/ and
//                               compares its exit and shape with the saved run, and compares
//                               output and exit byte for byte for an example with an `exact`
//                               file (one whose output reads no live source)
//
// Each example runs as `bash -o pipefail run.sh` with stdout and stderr merged, so a failed
// command anywhere in a pipe fails it. An example also fails when its output holds `jq: error`
// and, with product.exampleFailedStatus, when a JSON reply's top-level status is "failed"
// unless examples/<id>/expect-status names that status.
//
// The examples run in an isolated environment, one per build: a fresh HOME, XDG_CACHE_HOME,
// XDG_CONFIG_HOME and TMPDIR, shared by the examples of that build, PATH with the built
// program first, no key or token variables, and only the proxy and certificate variables of
// the shell. SITE_EXAMPLE_CONFIG names a config file copied to product.exampleConfig inside
// the isolated config folder. product.exampleSetup may add variables (see product.config.mjs).
//
// Order: examples/<id>/needs lists example ids that must run first. Those, and examples whose
// script matches product.exampleFirst, run one at a time before the others start.
import { spawn, spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import product from '../product.config.mjs';
import site from '../src/data/site.json' with { type: 'json' };
import { binaryInfo } from '../src/lib/binary.mjs';
import { jsonValues } from '../src/lib/json-values.mjs';

const args = process.argv.slice(2);
const check = args.includes('--check');
const only = args.filter((a) => !a.startsWith('--'));
const root = path.resolve('examples');
const runs = path.resolve('.example-runs');
const bin = path.resolve(product.binDir);
const today = new Date().toISOString().slice(0, 10);
if (!fs.existsSync(path.join(bin, product.binary))) {
  console.error(`missing ${path.join(bin, product.binary)}: build the product first (cargo build --release)`);
  process.exit(1);
}
const ids = fs.readdirSync(root).filter((d) => fs.existsSync(path.join(root, d, 'run.sh')) && (!only.length || only.includes(d))).sort();
const file = (id, f) => (fs.existsSync(path.join(root, id, f)) ? fs.readFileSync(path.join(root, id, f), 'utf8') : null);
const list = (text) => (text ?? '').split('\n').map((l) => l.trim()).filter((l) => l && !l.startsWith('#'));

// ---- The isolated environment ----------------------------------------------------------
// Variables never passed to an example, whatever the shell holds.
const SECRET = /(_API_KEY|_TOKEN|_SECRET|_PASSWORD)$/;
const KNOWN_KEYS = ['NCBI_API_KEY', 'S2_API_KEY', 'NCI_API_KEY', 'ONCOKB_TOKEN', 'OPENFDA_API_KEY', 'UMLS_API_KEY', 'DISGENET_API_KEY', 'ALPHAGENOME_API_KEY', 'ORCID_ACCESS_TOKEN', 'THINKTHEN_API_KEY', 'OPENAI_API_KEY'];
// Variables passed through from the shell.
const PASS = ['HTTPS_PROXY', 'https_proxy', 'HTTP_PROXY', 'http_proxy', 'NO_PROXY', 'no_proxy', 'SSL_CERT_FILE'];

const sandbox = fs.mkdtempSync(path.join(os.tmpdir(), `${product.binary}-examples-`));
const dirs = { home: path.join(sandbox, 'home'), cache: path.join(sandbox, 'cache'), config: path.join(sandbox, 'config'), tmp: path.join(sandbox, 'tmp') };
for (const d of Object.values(dirs)) fs.mkdirSync(d, { recursive: true });
const cleanup = () => {
  // Go and other tools write read-only folders; make them writable before removing.
  spawnSync('chmod', ['-R', 'u+w', sandbox]);
  fs.rmSync(sandbox, { recursive: true, force: true });
};
process.on('exit', cleanup);
for (const s of ['SIGINT', 'SIGTERM']) process.on(s, () => process.exit(130));

if (process.env.SITE_EXAMPLE_CONFIG) {
  if (!product.exampleConfig) {
    console.error(`SITE_EXAMPLE_CONFIG is set, but ${product.name} names no exampleConfig in product.config.mjs`);
    process.exit(1);
  }
  const to = path.join(dirs.config, product.exampleConfig);
  fs.mkdirSync(path.dirname(to), { recursive: true });
  fs.copyFileSync(path.resolve(process.env.SITE_EXAMPLE_CONFIG), to);
}

const env = {
  PATH: `${bin}:${process.env.PATH}`,
  HOME: dirs.home,
  XDG_CACHE_HOME: dirs.cache,
  XDG_CONFIG_HOME: dirs.config,
  XDG_DATA_HOME: path.join(dirs.home, '.local/share'),
  TMPDIR: dirs.tmp,
  LANG: 'C.UTF-8',
  TZ: 'UTC',
  NO_COLOR: '1',
  TERM: 'dumb',
  COLUMNS: '100',
  ...Object.fromEntries(PASS.filter((k) => process.env[k] !== undefined).map((k) => [k, process.env[k]])),
  ...(product.exampleSetup?.({ ...dirs, sandbox, bin, shell: process.env }) ?? {}),
  ...(product.exampleEnv ?? {}),
};
for (const k of Object.keys(env)) if (SECRET.test(k) || KNOWN_KEYS.includes(k)) delete env[k];

// ---- The version the site states must be the version of the program it runs -------------
const info = binaryInfo(env);
if (info.version !== site.version) {
  console.error(`src/data/site.json version is ${site.version}, but ${product.binary} ${product.versionArgs.join(' ')} prints ${info.version}: build the program from this commit or update site.json`);
  process.exit(1);
}

// ---- Running one example ---------------------------------------------------------------
function run(id) {
  return new Promise((resolve) => {
    const cwd = path.join(root, id);
    // detached: the example gets its own process group, so a timeout kills every process it started.
    const child = spawn('bash', ['-c', 'exec bash -o pipefail run.sh 2>&1'], { cwd, env, detached: true, stdio: ['ignore', 'pipe', 'inherit'] });
    let out = '';
    let timedOut = false;
    child.stdout.on('data', (b) => (out += b));
    const killGroup = () => { try { process.kill(-child.pid, 'SIGKILL'); } catch {} };
    const timer = setTimeout(() => { timedOut = true; killGroup(); }, product.exampleTimeoutMs ?? 120000);
    child.on('close', (code) => {
      clearTimeout(timer);
      killGroup();
      resolve({ id, out: out.replace(/[ \t]+$/gm, '').replace(/\s+$/, '') + '\n', exit: timedOut ? 124 : (code ?? 124), timedOut });
    });
  });
}

// Problems a run's output shows whatever was saved before.
function outputProblems(id, r) {
  const p = [];
  if (r.timedOut) p.push(`timed out after ${(product.exampleTimeoutMs ?? 120000) / 1000} s`);
  if (r.out.includes('jq: error')) p.push('output holds "jq: error"');
  // A page never shows the build machine's paths: the program's folder, the sandbox or the repository.
  for (const dir of [bin, sandbox, path.resolve('..')]) if (r.out.includes(dir)) p.push(`output holds the machine path ${dir}; replace it with an example path`);
  if (product.exampleFailedStatus) {
    const statuses = jsonValues(r.out).filter((v) => v && typeof v === 'object' && !Array.isArray(v) && typeof v.status === 'string').map((v) => v.status);
    // A Markdown reply ends with its status line.
    for (const m of r.out.matchAll(/^\*\*status:\*\* (\w+)$/gm)) statuses.push(m[1]);
    const expected = list(file(id, 'expect-status'));
    if (statuses.includes('failed') && !expected.includes('failed')) p.push('a reply has status "failed"; fix the example, or add an expect-status file that names failed when the page shows a failure');
    for (const s of expected) if (!statuses.includes(s)) p.push(`expect-status names "${s}", but no reply has that status`);
  }
  return p;
}

// The shape of an output: for each top-level JSON reply, its schema and status, or its keys.
// Live data changes values and counts; a changed shape means a changed reply.
function shape(text) {
  const sig = (v) => {
    if (v === null || typeof v !== 'object') return typeof v;
    if (Array.isArray(v)) return 'array';
    if (typeof v.schema === 'string') return `${v.schema} ${v.status ?? ''}`.trim();
    return `{${Object.keys(v).sort().join(',')}}`;
  };
  return [...new Set(jsonValues(text).map(sig))].sort();
}

// Everything wrong with one run.
function judge(id, r) {
  const problems = outputProblems(id, r);
  if (!check) return problems;
  const want = Number((file(id, 'exit') ?? '0').trim());
  // An example marked exact reads no live source, so any change in its output is a change in the program.
  if (product.exampleMode === 'exact' || file(id, 'exact') !== null) {
    if (file(id, 'output.txt') !== r.out || want !== r.exit) problems.push(`output or exit differs from the saved run; run npm run examples -- ${id}, review the diff against the page's prose, and commit both`);
    return problems;
  }
  if (r.exit !== want) problems.push(`exit ${r.exit}, saved ${want}`);
  if (!r.out.trim()) problems.push('empty output');
  const [was, now] = [shape(file(id, 'output.txt') ?? ''), shape(r.out)];
  if (was.join('\n') !== now.join('\n')) problems.push(`reply shape changed: saved [${was.join('; ')}], now [${now.join('; ')}]`);
  return problems;
}

// ---- Order -----------------------------------------------------------------------------
const needs = new Map();
const errors = [];
const addNeeds = (id, from) => {
  if (needs.has(id)) return;
  if (!fs.existsSync(path.join(root, id, 'run.sh'))) { errors.push(`${from}: needs ${id}, which is not an example`); return; }
  if (!ids.includes(id)) ids.push(id); // a named example still runs what it needs
  needs.set(id, list(file(id, 'needs')));
  for (const n of needs.get(id)) addNeeds(n, id);
};
for (const id of [...ids]) addNeeds(id);
if (errors.length) { console.error(errors.join('\n')); process.exit(1); }
const first = new Set();
const visiting = new Set();
const addFirst = (id) => {
  if (first.has(id)) return;
  if (visiting.has(id)) { console.error(`examples: needs files form a cycle through ${id}`); process.exit(1); }
  visiting.add(id);
  for (const n of needs.get(id)) addFirst(n);
  first.add(id);
};
const script = (id) => fs.readdirSync(path.join(root, id)).filter((f) => f === 'run.sh' || f.startsWith('show.')).map((f) => file(id, f)).join('\n');
for (const id of ids) for (const n of needs.get(id)) addFirst(n);
if (product.exampleFirst) for (const id of ids) if (new RegExp(product.exampleFirst).test(script(id))) addFirst(id);

// ---- Run -------------------------------------------------------------------------------
const failures = [];
const failed = new Set();
let ran = 0;
async function one(id) {
  const unverified = file(id, 'unverified');
  if (unverified !== null && !only.includes(id)) { console.log(`skip ${id} (unverified: ${unverified.trim()})`); return; }
  const blocked = needs.get(id).filter((n) => failed.has(n));
  if (blocked.length) { failed.add(id); failures.push(`${id}: not run; it needs ${blocked.join(', ')}, which failed`); return; }
  let r;
  let problems;
  // Live sources limit the rate of keyless requests; product.exampleRetries reruns a failed example after a pause.
  for (let attempt = 0; ; attempt++) {
    r = await run(id);
    problems = judge(id, r);
    // An exact example reads no live source, so a rerun cannot change its output.
    if (!problems.length || attempt >= (product.exampleRetries ?? 0) || file(id, 'exact') !== null) break;
    console.log(`retry ${id}: ${problems[0]}`);
    await new Promise((ok) => setTimeout(ok, product.exampleRetryDelayMs ?? 15000));
  }
  ran++;
  if (check && product.exampleMode !== 'exact') {
    fs.mkdirSync(path.join(runs, id), { recursive: true });
    fs.writeFileSync(path.join(runs, id, 'output.txt'), r.out);
    fs.writeFileSync(path.join(runs, id, 'exit'), `${r.exit}\n`);
  } else if (!check && !problems.length) {
    // A failed run never replaces the saved one.
    fs.writeFileSync(path.join(root, id, 'output.txt'), r.out);
    fs.writeFileSync(path.join(root, id, 'exit'), `${r.exit}\n`);
    fs.writeFileSync(path.join(root, id, 'ran'), `${today}\n`);
  }
  if (problems.length) {
    failed.add(id);
    failures.push(`${id}: ${problems.join('; ')}\n${r.out.slice(0, 400)}`);
  }
  console.log(`${problems.length ? 'FAIL' : 'ok  '} ${r.exit === 0 ? '      ' : `exit ${String(r.exit).padEnd(2)}`} ${id}`);
}

if (check && product.exampleMode !== 'exact') fs.rmSync(runs, { recursive: true, force: true });
for (const id of first) await one(id);
const rest = ids.filter((id) => !first.has(id));
let next = 0;
async function worker() {
  while (next < rest.length) await one(rest[next++]);
}
await Promise.all(Array.from({ length: product.exampleWorkers ?? 4 }, worker));
if (failures.length) {
  console.error(`\n${failures.length} example(s) failed:\n` + failures.join('\n'));
  process.exit(1);
}
console.log(`${ran} example(s) ${check ? 'checked' : 'ran'} with ${product.binary} ${info.version}${info.commit ? ` (commit ${info.commit.slice(0, 12)})` : ''}`);
