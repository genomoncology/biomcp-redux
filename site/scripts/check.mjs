// The site checks that run on the built site:
//   front matter is complete, no two files make one page, the word list holds, prose never
//   types the version, the README's install lines match, no broken internal link or anchor,
//   every page has an incoming link, the agent and host files exist, every old address
//   (and its Markdown twin) still resolves, and each redirect lands on a built page.
// The other checks run earlier in npm run build: examples, generated reference, astro warnings.
//   node scripts/check.mjs             the build check
//   node scripts/check.mjs --publish   also fail on addresses the publishing step must serve
//                                      (product.hostFiles), once they are placed in dist/
import fs from 'node:fs';
import path from 'node:path';
import YAML from 'yaml';
import product from '../product.config.mjs';
import site from '../src/data/site.json' with { type: 'json' };
import words from '../src/data/words.json' with { type: 'json' };
import redirects from '../src/data/redirects.json' with { type: 'json' };
import { pages } from './pages.mjs';

const publish = process.argv.includes('--publish');
const dist = path.resolve('dist');
const all = pages();
const errors = [];
const notices = [];

// Front matter.
const FIELDS = ['title', 'description', 'goal', 'reader', 'kind', 'verified'];
for (const p of all) for (const f of FIELDS) if (!p.data[f]) errors.push(`${p.file}: front matter lacks ${f}`);

// Two source files that make the same page: Astro keeps one and drops the other quietly.
const byRoute = new Map();
for (const p of all) byRoute.set(p.route, [...(byRoute.get(p.route) ?? []), p.file]);
for (const [route, files] of byRoute) if (files.length > 1) errors.push(`${route}: made by ${files.length} files; keep one: ${files.map((f) => path.relative('.', f)).join(', ')}`);

// Prose only, so code, output, link targets, quoted words and blockquotes pass.
const prose = (body) =>
  body
    .replace(/```[\s\S]*?```/g, '')
    .replace(/`[^`]*`/g, '')
    .replace(/\]\([^)]*\)/g, ']')
    .replace(/^import .*$/gm, '')
    .replace(/^\s*>.*$/gm, '')
    .replace(/<[^>]+>/g, '')
    .replace(/"[^"\n]*"/g, '')
    .replace(/“[^”\n]*”/g, '');
// Pages a generator writes from the program or the question list (product.generatedPages, a pattern
// on the path under src/content/docs) are checked at their source.
const generatedRe = new RegExp(product.generatedPages ?? '^(reference|answers)(/|$)');
const generated = (p) => generatedRe.test(path.relative(path.resolve('src/content/docs'), p.file));

// Word list.
for (const p of all) {
  const text = prose(`${p.data.title}\n${p.data.description}\n${p.body}`);
  for (const w of words.banned) {
    // A word marked generated may appear in generated pages, which quote the program's own help and schemas.
    if (w.generated && generated(p)) continue;
    const re = new RegExp(`\\b${w.word}\\b`, 'i');
    if (re.test(text)) errors.push(`${p.file}: "${w.word}" is not used on this site. ${w.use}`);
  }
}

// The version lives in src/data/site.json. Hand-written prose and the question list name it
// through <Fact k="version" /> or {version}; code, output and link targets may hold it.
// "1.0.0." at the end of a sentence is prose; "1.0.0.1" is another version.
const versionRe = new RegExp(`(?<![\\w.])${site.version.replaceAll('.', '\\.')}(?!\\w|\\.\\d)`);
for (const p of all) {
  if (generated(p)) continue;
  if (versionRe.test(prose(p.body))) errors.push(`${p.file}: prose names version ${site.version}; use <Fact k="version" />`);
  for (const f of ['title', 'description']) if (versionRe.test(String(p.data[f] ?? ''))) errors.push(`${p.file}: front matter ${f} names version ${site.version}; leave the version to the page body's <Fact k="version" />`);
}
const strings = (v) => (typeof v === 'string' ? [v] : v && typeof v === 'object' ? Object.values(v).flatMap(strings) : []);
for (const f of fs.readdirSync(path.resolve('questions')).filter((f) => f.endsWith('.yaml'))) {
  const entries = YAML.parse(fs.readFileSync(path.resolve('questions', f), 'utf8')) ?? [];
  if (strings(entries).some((t) => versionRe.test(prose(t)))) errors.push(`questions/${f}: prose names version ${site.version}; write {version}`);
}

// With product.readmeInstallLines, the repository README repeats every install line of site.json
// exactly. (A README rendered from site.json by scripts/readme.mjs is checked there instead.)
const readme = path.resolve('../README.md');
if (product.readmeInstallLines && fs.existsSync(readme)) {
  const text = fs.readFileSync(readme, 'utf8');
  for (const [k, line] of Object.entries(site.install ?? {})) if (typeof line === 'string' && !text.includes(line)) errors.push(`README.md: install line install.${k} differs from src/data/site.json`);
}

// Links and anchors, read from each page's main content in the built site.
const fileFor = (target) => {
  const clean = decodeURIComponent(target.split('#')[0].split('?')[0]) || '/';
  const p = path.join(dist, clean);
  if (fs.existsSync(p) && fs.statSync(p).isFile()) return p;
  if (fs.existsSync(path.join(p, 'index.html'))) return path.join(p, 'index.html');
  return null;
};
const exists = (target) => fileFor(target) !== null;
const isRedirect = (file) => /<meta http-equiv="refresh"/i.test(fs.readFileSync(file, 'utf8').slice(0, 2000));
const idCache = new Map();
const ids = (file) => {
  if (!idCache.has(file)) idCache.set(file, new Set([...fs.readFileSync(file, 'utf8').matchAll(/\sid="([^"]+)"/g)].map((m) => m[1])));
  return idCache.get(file);
};
const anchorOk = (file, hash) => !hash || ids(file).has(decodeURIComponent(hash));
const incoming = new Map(all.map((p) => [p.route, 0]));
let anchors = 0;
for (const p of all) {
  const file = path.join(dist, p.route, 'index.html');
  if (!fs.existsSync(file)) { errors.push(`${p.route}: not built`); continue; }
  const html = fs.readFileSync(file, 'utf8');
  const main = html.slice(html.indexOf('<main'), html.lastIndexOf('</main>'));
  for (const [, raw] of main.matchAll(/href="([^"]+)"/g)) {
    const href = raw.replaceAll('&amp;', '&');
    if (/^(https?:|mailto:)/.test(href)) continue;
    const url = new URL(href, `https://x${p.route}`);
    const target = url.pathname;
    const hash = url.hash.slice(1);
    const tfile = href.startsWith('#') ? file : fileFor(target);
    if (!tfile) { errors.push(`${p.route}: broken link ${href}`); continue; }
    if (hash && tfile.endsWith('.html')) {
      anchors++;
      if (!anchorOk(tfile, hash)) errors.push(`${p.route}: link ${href} names an anchor its page lacks`);
    }
    const r = target.endsWith('/') ? target : `${target}/`;
    if (!href.startsWith('#') && incoming.has(r) && r !== p.route) incoming.set(r, incoming.get(r) + 1);
  }
}
for (const [route, n] of incoming) if (n === 0 && route !== '/') errors.push(`${route}: no page links here`);

// Agent files.
const llms = path.join(dist, 'llms.txt');
if (!fs.existsSync(llms) || !fs.existsSync(path.join(dist, 'llms-full.txt'))) errors.push('dist: llms.txt or llms-full.txt missing');
else {
  const text = fs.readFileSync(llms, 'utf8');
  for (const p of all) {
    if (!fs.existsSync(path.join(dist, p.twin))) errors.push(`${p.twin}: Markdown twin missing`);
    if (!text.includes(p.twin)) errors.push(`llms.txt: does not list ${p.twin}`);
  }
}

// Redirects: each lands on a built page, not on another redirect or a missing address.
for (const [from, to] of Object.entries(redirects)) {
  const [toPath, hash] = to.split('#');
  const file = fileFor(toPath);
  if (!file) errors.push(`redirect ${from} -> ${to}: the target is not a built page`);
  else if (file.endsWith('.html') && isRedirect(file)) errors.push(`redirect ${from} -> ${to}: the target is itself a redirect; point at the page`);
  else if (hash && !anchorOk(file, hash)) errors.push(`redirect ${from} -> ${to}: the target page has no anchor #${hash}`);
}

// Old addresses: each still serves a page, a redirect or a file. A page's Markdown twin
// (/a/b/ -> /a/b.md) was served too, so each twin must still resolve.
// Addresses in product.hostFiles are files the publishing step places; see site/README.md.
const hostFiles = product.hostFiles ?? {};
const twinOf = (a) => (a === '/' ? '/index.md' : `${a.replace(/\/$/, '')}.md`);
const old = fs.readFileSync(path.resolve('src/data/old-addresses.txt'), 'utf8').split('\n').map((l) => l.trim()).filter((l) => l && !l.startsWith('#'));
for (const a of old) {
  if (hostFiles[a] !== undefined) {
    if (!exists(a)) (publish ? errors : notices).push(`${a}: the build does not make this old address. ${hostFiles[a]}`);
    continue;
  }
  if (!exists(a)) errors.push(`${a}: old address no longer resolves; add it to src/data/redirects.json`);
  if (a.endsWith('/') && !exists(twinOf(a))) errors.push(`${twinOf(a)}: the Markdown twin of old address ${a} no longer resolves; scripts/agent-files.mjs writes one for each redirect`);
}

// Host files: sitemaps, robots.txt, version.json and the social card.
for (const f of ['sitemap-index.xml', 'sitemap.xml', 'robots.txt', 'version.json', 'social-card.png']) if (!fs.existsSync(path.join(dist, f))) errors.push(`dist: ${f} missing`);
if (fs.existsSync(path.join(dist, 'robots.txt')) && !fs.readFileSync(path.join(dist, 'robots.txt'), 'utf8').includes(`Sitemap: ${new URL('/sitemap.xml', product.site).href}`)) errors.push('dist/robots.txt: does not name the sitemap');
if (fs.existsSync(path.join(dist, 'version.json'))) {
  const v = JSON.parse(fs.readFileSync(path.join(dist, 'version.json'), 'utf8'));
  if (v.version !== site.version) errors.push(`dist/version.json: version ${v.version}, site.json ${site.version}`);
}
const card = path.join(dist, 'social-card.png');
if (fs.existsSync(card)) {
  const b = fs.readFileSync(card);
  if (b.readUInt32BE(16) !== 1200 || b.readUInt32BE(20) !== 630) errors.push(`public/social-card.png: ${b.readUInt32BE(16)}x${b.readUInt32BE(20)}, not 1200x630`);
}
for (const p of all) {
  const file = path.join(dist, p.route, 'index.html');
  if (!fs.existsSync(file)) continue;
  const head = fs.readFileSync(file, 'utf8').split('</head>')[0];
  if (!/property="og:image"/.test(head) || !/name="twitter:image"/.test(head)) errors.push(`${p.route}: lacks og:image or twitter:image`);
}

if (notices.length) console.log(`publishing must serve:\n${notices.join('\n')}`);
if (errors.length) {
  console.error(`${errors.length} check failure(s):\n` + errors.join('\n'));
  process.exit(1);
}
console.log(`checks passed: ${all.length} pages, ${anchors} anchors, ${Object.keys(redirects).length} redirects, ${old.length} old addresses`);
