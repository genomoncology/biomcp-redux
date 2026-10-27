// The site checks that run on the built site:
//   front matter is complete, the word list holds, no broken internal link,
//   every page has an incoming link, and the agent files list every page.
// The other checks run earlier in npm run build: examples, generated reference, astro warnings.
import fs from 'node:fs';
import path from 'node:path';
import words from '../src/data/words.json' with { type: 'json' };
import { pages } from './pages.mjs';

const dist = path.resolve('dist');
const all = pages();
const errors = [];

// Front matter.
const FIELDS = ['title', 'description', 'goal', 'reader', 'kind', 'verified'];
for (const p of all) for (const f of FIELDS) if (!p.data[f]) errors.push(`${p.file}: front matter lacks ${f}`);

// Word list: prose only, so code, output and link targets pass.
const prose = (body) =>
  body
    .replace(/```[\s\S]*?```/g, '')
    .replace(/`[^`]*`/g, '')
    .replace(/\]\([^)]*\)/g, ']')
    .replace(/<[^>]+>/g, '')
    .replace(/^import .*$/gm, '');
for (const p of all) {
  const text = prose(`${p.data.title}\n${p.data.description}\n${p.body}`);
  for (const w of words.banned) {
    const re = new RegExp(`\\b${w.word}\\b`, 'i');
    if (re.test(text)) errors.push(`${p.file}: "${w.word}" is not used on this site. ${w.use}`);
  }
}

// Links, read from each page's main content in the built site.
const htmlFor = (route) => path.join(dist, route, 'index.html');
const exists = (target) => {
  const clean = decodeURIComponent(target.split('#')[0].split('?')[0]);
  if (!clean || clean === '/') return true;
  const p = path.join(dist, clean);
  return fs.existsSync(p) && (fs.statSync(p).isFile() || fs.existsSync(path.join(p, 'index.html')));
};
const incoming = new Map(all.map((p) => [p.route, 0]));
for (const p of all) {
  const file = htmlFor(p.route);
  if (!fs.existsSync(file)) { errors.push(`${p.route}: not built`); continue; }
  const html = fs.readFileSync(file, 'utf8');
  const main = html.slice(html.indexOf('<main'), html.lastIndexOf('</main>'));
  for (const [, href] of main.matchAll(/href="([^"]+)"/g)) {
    if (/^(https?:|mailto:|#)/.test(href)) continue;
    const target = new URL(href, `https://x${p.route}`).pathname;
    if (!exists(target)) { errors.push(`${p.route}: broken link ${href}`); continue; }
    const r = target.endsWith('/') ? target : `${target}/`;
    if (incoming.has(r) && r !== p.route) incoming.set(r, incoming.get(r) + 1);
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
// Old addresses: each still serves a page or a redirect.
for (const old of fs.readFileSync(path.resolve('src/data/old-addresses.txt'), 'utf8').split('\n').filter(Boolean)) {
  if (!exists(old)) errors.push(`${old}: old address no longer resolves; add it to src/data/redirects.json`);
}
if (!fs.existsSync(path.join(dist, 'sitemap-index.xml'))) errors.push('dist: sitemap-index.xml missing');

if (errors.length) {
  console.error(`${errors.length} check failure(s):\n` + errors.join('\n'));
  process.exit(1);
}
console.log(`checks passed: ${all.length} pages`);
