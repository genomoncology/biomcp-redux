// Finds the commands that return each reply schema, by running them against the live sources,
// and writes src/data/schema-calls.json for gen-reference.mjs. Offline calls are not enough:
// a section reply that fails offline falls back to its entity's card schema.
//   npm run schema-calls
// It runs every help example and every command in the site's examples, and each `get` among
// them once per section word, in a fresh cache and home after synchronizing every bundle, with no keys (as the examples run).
// Rerun it when the program's commands, sections or schemas change; the build fails while
// a reply schema has no recorded command.
import { spawn, spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import product from '../product.config.mjs';

const bin = path.resolve(product.binDir, product.binary);
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'biomcp-schema-calls-'));
process.on('exit', () => fs.rmSync(tmp, { recursive: true, force: true }));
const env = Object.fromEntries(Object.entries({ ...process.env, NO_COLOR: '1' }).filter(([k]) => !/(_API_KEY|_TOKEN|_SECRET|_PASSWORD)$/.test(k)));
Object.assign(env, { HOME: tmp, XDG_CACHE_HOME: path.join(tmp, 'cache'), XDG_CONFIG_HOME: path.join(tmp, 'config'), XDG_DATA_HOME: path.join(tmp, 'data') });
if (process.env.SITE_EXAMPLE_CONFIG) {
  fs.mkdirSync(path.join(tmp, 'config', 'biomcp'), { recursive: true });
  fs.copyFileSync(path.resolve(process.env.SITE_EXAMPLE_CONFIG), path.join(tmp, 'config', 'biomcp', 'config.json'));
}
const json = (args) => JSON.parse(spawnSync(bin, args, { env, cwd: tmp, encoding: 'utf8' }).stdout);
const call = (args) => new Promise((resolve) => {
  // Commands that save a file write it in the temporary folder.
  const child = spawn(bin, args, { env, cwd: tmp });
  let out = '';
  child.stdout.on('data', (b) => (out += b));
  const timer = setTimeout(() => child.kill('SIGKILL'), 90000);
  child.on('close', () => { clearTimeout(timer); try { resolve(JSON.parse(out)); } catch { resolve(null); } });
});

const schemaIds = (spawnSync(bin, ['version', '--verbose'], { env, encoding: 'utf8' }).stdout.match(/^Schemas: (.*)$/m)?.[1] ?? '').split(', ');
const replyIds = new Set(schemaIds.filter((id) => json(['schema', id]).properties?.schema?.const === id));

const found = new Map();
const fallback = new Map();
const record = (map, id, args) => {
  if (!map.has(id)) map.set(id, []);
  const have = map.get(id);
  if (have.length < 2 && !have.some((a) => a[0] === args[0] && a[1] === args[1] && a.length === args.length)) have.push(args);
};
for (const b of ['hgnc', 'clingen', 'wikipathways', 'ema', 'ddinter', 'who-pq', 'who-ivd', 'gtr', 'fda-cdx']) {
  const r = json(['sync', b]);
  if (replyIds.has(r.schema) && r.status !== 'failed') record(found, r.schema, ['sync', b]);
  console.log(`sync ${b}: ${r.status}`);
}
for (const args of [['store', 'clean']]) { const r = json(args); if (replyIds.has(r.schema)) record(found, r.schema, args); }

const help = json(['help']).result.commands;
const SKIP = ['serve', 'skill', 'key set', 'key remove', 'mcp-config', 'sync'];
const calls = [];
for (const c of help) {
  const n = c.command.join(' ');
  if (SKIP.some((w) => n === w || n.startsWith(w + ' '))) continue;
  for (const e of c.examples ?? []) {
    calls.push(e);
    if (c.command[0] !== 'get') continue;
    // A section word follows the identifier: get ENTITY ID SECTION [OPTIONS].
    for (const s of c.sections ?? []) if (s.word !== 'all') calls.push([...e.slice(0, 3), s.word, ...e.slice(3)]);
  }
}

// The commands the site's examples run, read from each run.sh and show.* (a page's own commands).
const words = (text) => [...text.matchAll(/'([^']*)'|"([^"]*)"|(\S+)/g)].map((m) => m[1] ?? m[2] ?? m[3]);
const exampleCalls = [];
const examplesDir = path.resolve('examples');
for (const id of fs.readdirSync(examplesDir).sort()) {
  const dir = path.join(examplesDir, id);
  if (!fs.statSync(dir).isDirectory()) continue;
  for (const f of fs.readdirSync(dir).filter((f) => f === 'run.sh' || /^show\.(sh|bash)$/.test(f))) {
    const text = fs.readFileSync(path.join(dir, f), 'utf8').replace(/\\\n/g, ' ');
    for (const m of text.matchAll(/(?:^|[|;(]|\$\()\s*biomcp\s+([^|>&;)\n]+)/gm)) {
      if (/\$/.test(m[1])) continue;
      // JSON output only: drop --format and its value, and the example's own --config and --offline.
      const args = words(m[1].trim()).filter((w, i, a) => !['--offline', '--config', '--format'].includes(w) && !['--config', '--format'].includes(a[i - 1]));
      if (!args.length || SKIP.some((w) => args.join(' ').startsWith(w))) continue;
      exampleCalls.push(args);
      // A get example also reads each section of the entity it names.
      const c = help.find((h) => h.command[0] === 'get' && h.command[1] === args[1] && args[0] === 'get');
      if (c) for (const s of c.sections ?? []) if (s.word !== 'all') calls.push([...args.slice(0, 3), s.word, ...args.slice(3).filter((a) => !(c.sections ?? []).some((x) => x.word === a))]);
    }
  }
}
// Commands for schemas that neither the help examples nor the site's examples reach,
// with identifiers taken from the examples' saved output.
const EXTRA = [
  ['get', 'adverse-event', '50941416', '--source', 'maude'],
  ['get', 'adverse-event', '50941416', 'devices', '--source', 'maude'],
  ['get', 'adverse-event', '50941416', 'narrative', '--source', 'maude'],
  ['get', 'adverse-event', '26907716', 'drugs'],
  ['get', 'drug', '2033', 'civic', '--source', 'civic'],
  ['get', 'drug', 'EMA Art 58 H-W-2319-2', '--source', 'who-pq'],
  ['protein', 'complex-members', 'P15056', '--complex', 'CPX-10863'],
  ['get', 'pgx', '--gene', 'DPYD', '--source', 'cpic', 'recommendations'],
  ['get', 'pgx', '--annotation-id', 'PA166169507', '--source', 'clinpgx'],
  ['author', 'papers', 'orcid:0000-0002-1825-0097'],
  ['article', 'pdf', '41351423', '--save', 'article.pdf', '--replace'],
  ['trial', 'document', 'NCT04303780', 'SAP_001.pdf', '--save', 'document.pdf', '--replace'],
  ['search', 'adverse-event', 'MMR', '--source', 'vaers', '--limit', '2'],
  ['search', 'trial', '--keyword', 'melanoma', '--source', 'nci', '--limit', '1'],
  ['get', 'trial', 'NCT06887088', '--source', 'nci'],
  ['get', 'study', 'skcm_tcga'],
  ['get', 'study', 'skcm_tcga', 'profiles'],
  ['get', 'study', 'skcm_tcga', 'sample-lists'],
  ['get', 'study', 'skcm_tcga', 'files'],
  ['search', 'study', 'melanoma', '--limit', '1'],
];

// The site's own commands come first, so a schema is named by a command a page shows; each call runs once.
const seenCall = new Set();
const queue = [...exampleCalls, ...calls, ...EXTRA].filter((a) => { const k = a.join('\u0000'); return !seenCall.has(k) && seenCall.add(k); });

let next = 0;
async function worker() {
  while (next < queue.length) {
    const args = queue[next++];
    const r = await call(args);
    const id = r?.schema;
    if (!replyIds.has(id) || id === 'biomcp.error/1') continue;
    // A failed reply still names the schema its command returns, but a failed section can fall
    // back to its card's schema; failed replies only fill schemas no answered call reached.
    record(r.status === 'failed' ? fallback : found, id, args);
  }
}
await Promise.all(Array.from({ length: 4 }, worker));

for (const [id, calls] of fallback) if (!found.has(id)) found.set(id, calls);
const out = Object.fromEntries([...found].sort(([a], [b]) => a.localeCompare(b)));
fs.writeFileSync(path.resolve('src/data/schema-calls.json'), JSON.stringify(out, null, 2) + '\n');
const missing = [...replyIds].filter((id) => id !== 'biomcp.error/1' && !found.has(id));
console.log(`schema-calls: ${queue.length} calls, ${found.size} of ${replyIds.size} reply schemas found${missing.length ? `; none for ${missing.join(', ')}` : ''}`);
