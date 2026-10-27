// The landing-page checks: each page must work for a reader who lands on it from search or an
// agent link and has read nothing else. They read the built site (dist/ HTML and the .md twins)
// in Chromium at 1280 and 390 px, and the page sources, and report the [auto] items of the
// landing-page checklist. The build does not run them yet; see site/README.md.
//
//   npm run landing                        write landing-report.json and landing-shots/, print a summary
//   npm run landing -- --strict            also exit 1 on any violation not in the allowlist
//   npm run landing -- --worklist FILE     also write FILE: the violations grouped by the file to fix
//   npm run landing -- --pages REGEX       check only the routes that match REGEX (no duplicates, no shots)
//   npm run landing -- --no-shots          skip the screenshots
//
// Run it after npm run build, in site/. It needs Playwright with Chromium, from the site's packages
// or a global install (NODE_PATH, or `npm root -g`).
//
// Rules. Each violation names its rule, the offending text, the source line when one is found,
// and a suggested fix. Limits default to the numbers below; product.config.mjs `landing.limits`
// may change them.
//   L1 code-line-length   A line of a rendered code or output block (each <pre>, line by line as
//                         rendered, counted in characters) is longer than limits.line (80).
//   L2 one-line-json      A line of an output block (an example's output, or a text or JSON
//                         block) is a JSON object or array (it parses, or starts with { or [ and
//                         ends with } or ]) longer than limits.json (80).
//                         A line reported under L2 is not reported again under L1.
//   L3 first-paragraph    The first paragraph of body text (the first <p> in the main content
//                         outside examples, asides and code) does not name the product
//                         (product.name, case-sensitive), or starts with a caveat opening from
//                         words.json `landing.caveatStarts` (case-insensitive, at the start).
//   L4 maintainer-text    A phrase from words.json `landing.maintainerPhrases` (case-insensitive,
//                         whole words) appears in reader-visible text: the title, the description,
//                         or the main content outside code blocks and inline code; or in the
//                         page's Markdown twin outside code.
//   L5 page-length        The main content holds more than limits.words (1,500) words outside code
//                         blocks, or the page renders taller than limits.height (4,000) px at 1280 px.
//   L6 near-duplicate     More than limits.duplicate (60%) of a page's word 5-shingles (main
//                         content, code included) also occur in one other page. Pages with fewer
//                         than limits.minShingles (40) shingles are skipped. Pairs are grouped into
//                         families of pages that share text.
//   L7 title-is-id        The title, less a trailing generic word (error, schema, command, code,
//                         option, type, field), is one token with no spaces that holds . / _ or :.
//   L8 undefined-jargon   The first use of a glossary term (src/data/glossary.json) in body text
//                         (headings, code blocks and the title excluded; inline code counts when it
//                         is the whole term) is neither linked nor defined. Linked: a link to the
//                         term's page sits in the same block or earlier. Defined: the page is the
//                         term's page, the term is set in bold or <dfn>, or the block says
//                         "A term is/are/means".
//   L9 overflow-390       At 390 px the page scrolls sideways (code blocks may scroll; long lines are L1)
//                         sideways inside its box (code).
//
// Allowlist: src/data/landing-allow.json, {"allow": [{rule, route, match?, reason}]}. `route` is a
// route or a glob with *; `match` is a substring of the violation's text. Each entry needs a reason.
// --strict ignores allowed violations and fails on the rest.
//
// Each violation also names its cause: the page itself, an example (examples/<id>/), a generator
// (product.config.mjs `landing.generators`), a shared template (src/components, src/lib), or data a
// generator reads. A violation caused by a generator, template or example is fixed there, once.
import fs from 'node:fs';
import path from 'node:path';
import http from 'node:http';
import { execSync } from 'node:child_process';
import { createRequire } from 'node:module';
import product from '../product.config.mjs';
import words from '../src/data/words.json' with { type: 'json' };
import { pages, splitCode } from './pages.mjs';

const args = process.argv.slice(2);
const opt = (name) => { const i = args.indexOf(name); return i >= 0 ? args[i + 1] : undefined; };
const strict = args.includes('--strict');
const worklistPath = opt('--worklist');
const only = opt('--pages') ? new RegExp(opt('--pages')) : null;
const shots = !args.includes('--no-shots') && !only;

const SITE = process.cwd();
const DIST = path.join(SITE, 'dist');
const rel = (f) => path.relative(SITE, f);
const cfg = product.landing ?? {};
const LIMITS = { line: 80, json: 80, words: 1500, height: 4000, duplicate: 0.6, shingle: 5, minShingles: 40, ...(cfg.limits ?? {}) };
const LW = words.landing ?? {};
const PHRASES = (LW.maintainerPhrases ?? []).map((p) => (typeof p === 'string' ? { phrase: p } : p));
const CAVEATS = LW.caveatStarts ?? [];
const GENERIC_TITLE_WORDS = /\s+(error|schema|command|code|option|type|field)s?$/i;
const readJson = (f, fallback) => (fs.existsSync(f) ? JSON.parse(fs.readFileSync(f, 'utf8')) : fallback);
const glossary = readJson(path.join(SITE, 'src/data/glossary.json'), { terms: {} }).terms ?? {};
const allow = readJson(path.join(SITE, 'src/data/landing-allow.json'), { allow: [] }).allow ?? [];
for (const a of allow) if (!a.rule || !a.route || !a.reason) throw new Error(`src/data/landing-allow.json: each entry needs rule, route and reason: ${JSON.stringify(a)}`);

if (!fs.existsSync(path.join(DIST, 'index.html'))) {
  console.error('dist/ has no built site; run npm run build first');
  process.exit(2);
}

// ---------- pages, families, generators ----------
const DOCS = path.join(SITE, 'src/content/docs');
const generators = (cfg.generators ?? []).map((g) => ({ ...g, re: new RegExp(g.pattern) }));
const spine = new Set((product.spine ?? []).map((s) => (s ? `/${s}/` : '/')));
const familyRules = (cfg.families ?? []).map(([re, name]) => [new RegExp(re), name]);
function familyOf(p) {
  for (const [re, name] of familyRules) if (re.test(p.route)) return name;
  if (spine.has(p.route)) return 'spine';
  const seg = p.slug.split('/');
  if (seg[0] === 'reference' && seg.length >= 3) return `reference/${seg[1]}/*`;
  if (seg.length >= 2) return `${seg[0]}/*`;
  return 'other top-level';
}
const all = pages()
  .filter((p) => !only || only.test(p.route))
  .map((p) => {
    const relDocs = path.relative(DOCS, p.file);
    const gen = generators.find((g) => g.re.test(relDocs)) ?? null;
    return { ...p, relFile: rel(p.file), family: familyOf(p), gen };
  });

// ---------- where a text comes from ----------
const fileCache = new Map();
const readText = (f) => {
  if (!fileCache.has(f)) fileCache.set(f, fs.existsSync(f) && fs.statSync(f).isFile() && fs.statSync(f).size < 2e6 ? fs.readFileSync(f, 'utf8') : '');
  return fileCache.get(f);
};
const listFiles = (dir, re) => {
  const out = [];
  const walk = (d) => { if (!fs.existsSync(d)) return; for (const e of fs.readdirSync(d, { withFileTypes: true })) { const f = path.join(d, e.name); if (e.isDirectory()) walk(f); else if (re.test(e.name)) out.push(f); } };
  walk(dir);
  return out.sort();
};
const TEMPLATES = [...listFiles(path.join(SITE, 'src/components'), /\.(astro|mjs|ts)$/), ...listFiles(path.join(SITE, 'src/lib'), /\.(mjs|js|ts)$/)];
const DATA = [...listFiles(path.join(SITE, 'questions'), /\.ya?ml$/), ...listFiles(path.join(SITE, 'src/data'), /\.(json|ya?ml|txt)$/)];
// Curly quotes as straight ones, character for character, so offsets stay the same.
const plain = (s) => s.replace(/[‘’]/g, "'").replace(/[“”]/g, '"').toLowerCase();
// The 1-based line of `file` that holds `text`, or failing that its longest prefix (cut at a word
// boundary, 14 characters at least) that the file holds; null when none is found.
function lineOf(file, text) {
  const t = plain(String(text).replace(/\s+/g, ' ').trim());
  if (!t) return null;
  const src = plain(readText(file));
  if (!src) return null;
  const tries = [t];
  for (let n = Math.min(t.length, 60); n >= 14; n--) if (t[n] === ' ' || n === t.length) tries.push(t.slice(0, n).trim());
  for (const needle of tries) {
    const i = src.indexOf(needle);
    if (i >= 0) return src.slice(0, i).split('\n').length;
  }
  return null;
}
const firstIn = (files, text) => { for (const f of files) { const line = lineOf(f, text); if (line) return { file: rel(f), line }; } return null; };
const exampleFiles = (id) => {
  const dir = path.join(SITE, 'examples', id);
  if (!fs.existsSync(dir)) return [];
  const meta = ['output.txt', 'exit', 'ran', 'needs', 'exact', 'expect-status', 'unverified', 'build-only'];
  const out = [];
  const walk = (d) => {
    for (const e of fs.readdirSync(d, { withFileTypes: true })) {
      if (e.isDirectory() && !['node_modules', 'target', 'bin', 'obj', 'build', '.zig-cache', 'zig-out'].includes(e.name) && !e.name.startsWith('.')) walk(path.join(d, e.name));
      else if (e.isFile() && !(d === dir && meta.includes(e.name))) out.push(path.join(d, e.name));
    }
  };
  walk(dir);
  return out.sort((a, b) => (path.basename(b).startsWith('show.') - path.basename(a).startsWith('show.')) || a.localeCompare(b));
};
// The cause of a violation on page p: {type, file, line}. type is page, example, generator,
// template or data. Generator, template and example causes are fixed once, at that file.
function causeOf(p, text, { example, block } = {}) {
  if (example) {
    const files = exampleFiles(example);
    const run = path.join(SITE, 'examples', example, 'run.sh');
    if (block === 'output') return { type: 'example', file: rel(run), line: null, note: `output of example ${example}; change what run.sh prints` };
    return { type: 'example', ...(firstIn(files, text) ?? { file: rel(files[0] ?? run), line: null }) };
  }
  const own = text ? lineOf(p.file, text) : null;
  // A block the page shows from an example's folder (<ExampleFile>, or an example's code).
  if (text && block && !own) {
    const ids = [...new Set([...p.body.matchAll(/<Example(?:File)?\b[^>]*\bid="([^"]+)"/g)].map((m) => m[1]))];
    const hit = firstIn(ids.flatMap(exampleFiles), text);
    if (hit) return { type: 'example', ...hit };
  }
  if (!p.gen) {
    if (own) return { type: 'page', file: p.relFile, line: own };
    const t = text && firstIn(TEMPLATES, text);
    if (t) return { type: 'template', ...t };
    return { type: 'page', file: p.relFile, line: null };
  }
  // A generated page: the text comes from the generator, a template, data it reads, or the program.
  const genFile = path.join(SITE, p.gen.generator);
  const g = text && lineOf(genFile, text);
  if (g) return { type: 'generator', file: p.gen.generator, line: g, template: p.gen.template };
  const t = text && firstIn(TEMPLATES, text);
  if (t) return { type: 'template', ...t };
  const d = text && firstIn(DATA, text);
  if (d) return { type: 'data', ...d, note: `read by ${p.gen.generator}` };
  return { type: 'generator', file: p.gen.generator, line: null, template: p.gen.template, note: p.gen.source ?? 'text from the program, written by the generator' };
}

// ---------- a static server for dist/ ----------
const TYPES = { '.html': 'text/html; charset=utf-8', '.css': 'text/css', '.js': 'text/javascript', '.mjs': 'text/javascript', '.svg': 'image/svg+xml', '.png': 'image/png', '.jpg': 'image/jpeg', '.webp': 'image/webp', '.woff2': 'font/woff2', '.woff': 'font/woff', '.ttf': 'font/ttf', '.json': 'application/json', '.txt': 'text/plain', '.md': 'text/markdown', '.xml': 'application/xml', '.ico': 'image/x-icon' };
const server = http.createServer((req, res) => {
  let p = decodeURIComponent(new URL(req.url, 'http://x').pathname);
  let f = path.join(DIST, p);
  if (!f.startsWith(DIST)) { res.writeHead(403).end(); return; }
  if (fs.existsSync(f) && fs.statSync(f).isDirectory()) f = path.join(f, 'index.html');
  if (!fs.existsSync(f)) { res.writeHead(404).end(); return; }
  res.writeHead(200, { 'content-type': TYPES[path.extname(f)] ?? 'application/octet-stream' });
  fs.createReadStream(f).pipe(res);
});
await new Promise((r) => server.listen(0, '127.0.0.1', r));
const ORIGIN = `http://127.0.0.1:${server.address().port}`;

// ---------- Chromium ----------
async function loadPlaywright() {
  try { return await import('playwright'); } catch {}
  for (const root of [process.env.NODE_PATH, (() => { try { return execSync('npm root -g', { encoding: 'utf8' }).trim(); } catch { return ''; } })()]) {
    if (!root) continue;
    try { return createRequire(path.join(root, 'x'))('playwright'); } catch {}
  }
  throw new Error('Playwright not found: npm install playwright, or set NODE_PATH=$(npm root -g)');
}
const { chromium } = await loadPlaywright();
const browser = await chromium.launch();

// Runs `fn(page, item)` over items with a few pages at once.
async function pool(items, width, fn, { colorScheme = 'light', workers = 4 } = {}) {
  const ctx = await browser.newContext({ viewport: { width, height: 900 }, colorScheme });
  await ctx.route('**/*', (route) => (route.request().url().startsWith(ORIGIN) ? route.continue() : route.abort()));
  let next = 0;
  const run = async () => {
    const page = await ctx.newPage();
    while (next < items.length) {
      const item = items[next++];
      await page.goto(ORIGIN + item.route, { waitUntil: 'load' });
      await fn(page, item);
    }
    await page.close();
  };
  await Promise.all(Array.from({ length: Math.min(workers, items.length) }, run));
  await ctx.close();
}

// In the browser: everything the rules read from one page at 1280 px.
function extract({ glossary, product }) {
  const main = document.querySelector('main');
  const md = main.querySelector('.sl-markdown-content') ?? main;
  const SKIP = 'pre, .expressive-code, .sr-only, script, style, svg, nav, .sl-anchor-link, [data-pagefind-ignore]';
  const visible = (n) => !(n.parentElement?.closest(SKIP));
  const texts = (root, { code = true } = {}) => {
    const out = [];
    const w = document.createTreeWalker(root, NodeFilter.SHOW_TEXT);
    for (let n; (n = w.nextNode());) if (visible(n) && (code || !n.parentElement.closest('code'))) out.push(n);
    return out;
  };
  const title = document.querySelector('h1')?.textContent.trim() ?? document.title;
  const description = document.querySelector('meta[name="description"]')?.content ?? '';
  // Code and output blocks, line by line as rendered.
  const blocks = [...main.querySelectorAll('pre')].map((pre) => {
    const ex = pre.closest('[data-example]');
    const lines = pre.querySelector('.ec-line') ? [...pre.querySelectorAll('.ec-line')].map((l) => l.textContent) : pre.textContent.replace(/\n$/, '').split('\n');
    const lang = pre.dataset.language ?? '';
    const box = pre.closest('.expressive-code') ?? pre;
    let kind = 'code';
    if (ex) { if (box.previousElementSibling?.classList.contains('example-label')) kind = 'output'; } else if (/^(text|txt|plaintext|output|console|json|jsonl)$/.test(lang)) kind = 'output';
    return { lines, lang, kind, example: ex?.dataset.example ?? null };
  });
  // Body text outside code blocks.
  const proseNodes = texts(md);
  const prose = proseNodes.filter((n) => !n.parentElement.closest('code')).map((n) => n.textContent).join(' ').replace(/\s+/g, ' ');
  const words = (prose.match(/[\p{L}\p{N}][\p{L}\p{N}'’.-]*/gu) ?? []).length;
  const allText = (md.textContent ?? '').replace(/Section titled “[^”]*”/g, ' ').replace(/\s+/g, ' ');
  // The first paragraph of body text.
  const firstP = [...md.querySelectorAll('p')].find((p) => !p.closest('[data-example], aside, .expressive-code, figure, nav, .starlight-aside, details') && !p.classList.contains('example-label') && p.textContent.trim());
  const hero = document.querySelector('.hero .tagline');
  const firstParagraph = (firstP?.textContent ?? hero?.textContent ?? '').replace(/\s+/g, ' ').trim();
  // Glossary: the first use of each term, and whether it is linked or defined there.
  const BLOCK = 'p, li, td, th, dd, dt, figcaption, summary, blockquote';
  const here = location.pathname;
  const links = [...md.querySelectorAll('a[href]')].map((a) => ({ a, path: new URL(a.href).pathname }));
  const strongText = [...md.querySelectorAll('strong, b, dfn')].map((e) => e.textContent.toLowerCase());
  const jargon = [];
  for (const [term, g] of Object.entries(glossary)) {
    const targets = (Array.isArray(g.page) ? g.page : [g.page]).map((u) => new URL(u, location.origin).pathname);
    if (targets.includes(here)) continue;
    const re = new RegExp(g.pattern ?? `\\b${term.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}s?\\b`, 'i');
    let first = null;
    for (const n of proseNodes) {
      if (n.parentElement.closest('h1, h2, h3, h4, h5, h6, .example-label')) continue;
      const code = n.parentElement.closest('code');
      const t = code ? code.textContent.trim().replace(/\[\]$/, '') : n.textContent;
      const m = code ? (t.toLowerCase() === term.toLowerCase() || (re.exec(t)?.[0].length === t.length) ? [t] : null) : re.exec(t);
      if (m) { first = { node: n, match: m[0] }; break; }
    }
    if (!first) continue;
    const block = first.node.parentElement.closest(BLOCK) ?? first.node.parentElement;
    const linked = links.some(({ a, path }) => targets.includes(path) && (block.contains(a) || (a.compareDocumentPosition(block) & Node.DOCUMENT_POSITION_FOLLOWING)));
    const bold = strongText.some((s) => re.test(s) && s.length <= term.length + 20);
    const sentence = new RegExp(`(^|[.!?:]\\s+)(an?|the)?\\s*${re.source}\\s+(is|are|means)\\b`, 'i').test(block.textContent);
    if (linked || bold || sentence) continue;
    const text = block.textContent.replace(/\s+/g, ' ').trim();
    const at = text.toLowerCase().indexOf(first.match.toLowerCase());
    jargon.push({ term, match: first.match, context: text.slice(Math.max(0, at - 60), at + 80), page: g.page });
  }
  return { title, description, blocks, prose, words, allText, firstParagraph, jargon, height: document.documentElement.scrollHeight };
}

// In the browser at 390 px: page-level sideways scroll and code blocks that scroll.
function overflow() {
  const vw = document.documentElement.clientWidth;
  const pageScroll = document.documentElement.scrollWidth > vw + 1 || document.body.scrollWidth > vw + 1;
  const wide = [];
  if (pageScroll) {
    for (const el of document.body.querySelectorAll('*')) {
      const r = el.getBoundingClientRect();
      if (r.right <= vw + 1 || r.width === 0) continue;
      let clipped = false;
      for (let a = el.parentElement; a && a !== document.body; a = a.parentElement) {
        const ox = getComputedStyle(a).overflowX;
        if (ox !== 'visible' && a.getBoundingClientRect().right <= vw + 1) { clipped = true; break; }
      }
      if (clipped) continue;
      // Report the outermost offenders only.
      if (wide.some((w) => w.el.contains(el))) continue;
      wide.push({ el, desc: `${el.tagName.toLowerCase()}${el.className && typeof el.className === 'string' ? '.' + el.className.trim().split(/\s+/).slice(0, 2).join('.') : ''}`, width: Math.round(r.right), text: (el.textContent ?? '').replace(/\s+/g, ' ').trim().slice(0, 80) });
      if (wide.length >= 5) break;
    }
  }
  const code = [...document.querySelectorAll('main pre')].map((pre, i) => ({ i, pre })).filter(({ pre }) => pre.scrollWidth > pre.clientWidth + 1).map(({ i, pre }) => {
    const lines = pre.querySelector('.ec-line') ? [...pre.querySelectorAll('.ec-line')].map((l) => l.textContent) : pre.textContent.split('\n');
    const longest = lines.reduce((a, b) => ([...b].length > [...a].length ? b : a), '');
    return { i, scroll: pre.scrollWidth, box: pre.clientWidth, longest, example: pre.closest('[data-example]')?.dataset.example ?? null };
  });
  return { pageScroll, scrollWidth: document.documentElement.scrollWidth, vw, wide: wide.map(({ el, ...w }) => w), code };
}

// ---------- run ----------
const info = new Map();
const familyFirst = new Map();
for (const p of all) if (!familyFirst.has(p.family)) familyFirst.set(p.family, p);
const famSlug = (f) => f.replace(/\/\*$/, '').replace(/[^a-z0-9]+/gi, '-').replace(/^-|-$/g, '').toLowerCase() || 'home';
const SHOTS = path.join(SITE, 'landing-shots');
if (shots) { fs.rmSync(SHOTS, { recursive: true, force: true }); fs.mkdirSync(SHOTS, { recursive: true }); }
const reps = new Set([...familyFirst.values()].map((p) => p.route));

const t0 = Date.now();
await pool(all, 1280, async (page, p) => {
  info.set(p.route, await page.evaluate(extract, { glossary, product: { name: product.name } }));
  if (shots && reps.has(p.route)) await page.screenshot({ path: path.join(SHOTS, `${famSlug(p.family)}-1280.png`), fullPage: true });
});
await pool(all, 390, async (page, p) => {
  info.get(p.route).overflow = await page.evaluate(overflow);
  if (shots && reps.has(p.route)) await page.screenshot({ path: path.join(SHOTS, `${famSlug(p.family)}-390.png`), fullPage: true });
});
if (shots) {
  const repPages = all.filter((p) => reps.has(p.route));
  for (const width of [1280, 390]) await pool(repPages, width, async (page, p) => page.screenshot({ path: path.join(SHOTS, `${famSlug(p.family)}-${width}-dark.png`), fullPage: true }), { colorScheme: 'dark' });
}
await browser.close();
server.close();

// ---------- rules ----------
const violations = new Map(all.map((p) => [p.route, []]));
const add = (p, v) => violations.get(p.route).push(v);
const len = (s) => [...s].length;
const isJsonLine = (s) => {
  const t = s.trim();
  if (!/^[[{]/.test(t)) return false;
  try { const v = JSON.parse(t); return v !== null && typeof v === 'object'; } catch { return /^[[{][\s\S]*[\]}],?$/.test(t); }
};
const escapeRe = (s) => s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

for (const p of all) {
  const d = info.get(p.route);
  // L1, L2
  d.blocks.forEach((b, bi) => {
    b.lines.forEach((line, li) => {
      const n = len(line.replace(/\s+$/, ''));
      if (b.kind === 'output' && isJsonLine(line) && n > LIMITS.json) {
        add(p, { rule: 'L2', where: `${b.kind} block ${bi + 1}, line ${li + 1}`, text: line.length > 160 ? `${line.slice(0, 157)}...` : line, detail: { chars: n, block: b.kind, example: b.example }, fix: `Pretty-print the JSON (jq . rather than jq -c) or select fewer fields, so no line passes ${LIMITS.json} characters.`, cause: causeOf(p, line.slice(0, 60), { example: b.example, block: b.kind }) });
      } else if (n > LIMITS.line) {
        add(p, { rule: 'L1', where: `${b.kind} block ${bi + 1}, line ${li + 1}`, text: line.length > 160 ? `${line.slice(0, 157)}...` : line, detail: { chars: n, block: b.kind, example: b.example }, fix: b.kind === 'output' ? `Trim the output so no line passes ${LIMITS.line} characters: select fewer fields or columns, or cut long values with a visible "…" and a note.` : `Break the line at an option or argument boundary (\\ in a shell command) so no line passes ${LIMITS.line} characters.`, cause: causeOf(p, line.trim().slice(0, 60), { example: b.example, block: b.kind }) });
      }
    });
  });
  // L3
  const fp = d.firstParagraph;
  if (!fp) add(p, { rule: 'L3', where: 'first paragraph', text: '(no paragraph of body text)', fix: `Open the page with one plain paragraph that names ${product.name} and what this page covers.`, cause: causeOf(p, null) });
  else {
    if (!fp.includes(product.name)) add(p, { rule: 'L3', where: 'first paragraph', text: fp.slice(0, 200), detail: { reason: 'product not named' }, fix: `Name ${product.name} in the first sentence and say what the command, function or concept on this page does.`, cause: causeOf(p, fp.slice(0, 60)) });
    const caveat = CAVEATS.find((c) => fp.toLowerCase().startsWith(c.toLowerCase()) && !/\w/.test(fp[c.length] ?? ' '));
    if (caveat) add(p, { rule: 'L3', where: 'first paragraph', text: fp.slice(0, 200), detail: { reason: `starts with "${caveat}"` }, fix: 'Give the direct answer or outcome first; move the caveat or syntax warning after it.', cause: causeOf(p, fp.slice(0, 60)) });
  }
  // L4: one violation per passage, naming every phrase in it; the twin adds only phrases the HTML lacks.
  const seen = new Set();
  const twinFile = path.join(DIST, p.twin);
  const twinProse = fs.existsSync(twinFile) ? splitCode(fs.readFileSync(twinFile, 'utf8')).filter((_, i) => i % 2 === 0).join('') : '';
  for (const [where, text] of [['title', d.title], ['description', d.description], ['body', d.prose], ['Markdown twin', twinProse]]) {
    const hits = [];
    for (const ph of PHRASES) {
      if (where === 'Markdown twin' && seen.has(ph.phrase)) continue;
      for (const m of plain(text).matchAll(new RegExp(`(?<![\\w-])${escapeRe(plain(ph.phrase))}(?![\\w-])`, 'gi'))) hits.push({ ph, at: m.index, end: m.index + m[0].length });
    }
    hits.sort((a, b) => a.at - b.at);
    const passages = [];
    for (const h of hits) {
      const last = passages.at(-1);
      if (last && h.at - last.end < 80) { last.end = Math.max(last.end, h.end); last.phrases.add(h.ph); } else passages.push({ at: h.at, end: h.end, phrases: new Set([h.ph]) });
    }
    for (const ps of passages) {
      for (const ph of ps.phrases) seen.add(ph.phrase);
      const ctx = text.slice(Math.max(0, ps.at - 70), ps.end + 70).replace(/\s+/g, ' ').trim();
      const phrases = [...ps.phrases];
      add(p, { rule: 'L4', where, text: ctx, detail: { phrases: phrases.map((x) => x.phrase) }, fix: phrases[0].use ?? 'Remove the text about how the site is made; say what the reader does or sees.', cause: causeOf(p, text.slice(ps.at, ps.at + 60)) });
    }
  }
  // L5
  if (d.words > LIMITS.words) add(p, { rule: 'L5', where: 'main content', text: `${d.words} words outside code`, detail: { words: d.words, height: d.height }, fix: 'Put a summary at the top and move detail to linked pages or a collapsed section.', cause: causeOf(p, null) });
  if (d.height > LIMITS.height) add(p, { rule: 'L5', where: 'page at 1280 px', text: `${d.height} px tall`, detail: { words: d.words, height: d.height }, fix: 'Put a summary at the top; move long tables, repeated types or long output to linked pages or a collapsed section.', cause: causeOf(p, null) });
  // L7
  const bare = d.title.replace(GENERIC_TITLE_WORDS, '').trim();
  if (!/\s/.test(bare) && /[./_:]/.test(bare)) add(p, { rule: 'L7', where: 'title', text: d.title, fix: `Title the page with what the reader wants from it, such as "${bare} reply: fields and an example"; keep the bare ID in the first paragraph.`, cause: causeOf(p, null) });
  // L8
  for (const j of d.jargon) add(p, { rule: 'L8', where: 'first use', text: `"${j.match}": ${j.context}`, detail: { term: j.term, match: j.match, page: j.page }, fix: `Link the first use of "${j.match}" to ${Array.isArray(j.page) ? j.page[0] : j.page}, or define it in that sentence.`, cause: causeOf(p, j.context.slice(Math.max(0, j.context.toLowerCase().indexOf(j.match.toLowerCase()) - 20), j.context.toLowerCase().indexOf(j.match.toLowerCase()) + 30)) });
  // L9
  const o = d.overflow;
  if (o.pageScroll) add(p, { rule: 'L9', where: 'page at 390 px', text: `page is ${o.scrollWidth} px wide at ${o.vw} px; widest: ${o.wide.map((w) => `${w.desc} (${w.width} px) "${w.text}"`).join('; ')}`, detail: { kind: 'page', wide: o.wide }, fix: 'Let the listed element wrap, or give it its own scrolling box, so the page never scrolls sideways at 390 px.', cause: causeOf(p, o.wide[0]?.text?.slice(0, 40) ?? null) });
  // Code cannot wrap, so a code block may scroll on a phone. Only page-level sideways scroll fails;
  // long lines are L1's job. Code-block scroll is kept as a note in the report, not a violation.
  for (const c of []) add(p, { rule: 'L9', where: `code block ${c.i + 1} at 390 px`, text: c.longest.length > 160 ? `${c.longest.slice(0, 157)}...` : c.longest, detail: { kind: 'code', scroll: c.scroll, box: c.box, example: c.example }, fix: 'Shorten the longest lines (break commands with \\, trim output) so the block scrolls only where it must.', cause: causeOf(p, c.longest.trim().slice(0, 60), { example: c.example, block: d.blocks[c.i]?.kind }) });
}

// L6: shingles of the main-content text.
const dup = { pairs: [], groups: [] };
if (!only) {
  const ids = new Map();
  const shingles = all.map((p) => {
    const w = info.get(p.route).allText.toLowerCase().match(/[\p{L}\p{N}]+/gu) ?? [];
    const set = new Set();
    for (let i = 0; i + LIMITS.shingle <= w.length; i++) {
      const s = w.slice(i, i + LIMITS.shingle).join(' ');
      if (!ids.has(s)) ids.set(s, ids.size);
      set.add(ids.get(s));
    }
    return set;
  });
  const postings = new Map();
  shingles.forEach((set, i) => { for (const s of set) { if (!postings.has(s)) postings.set(s, []); postings.get(s).push(i); } });
  const shared = new Map();
  shingles.forEach((set, i) => {
    if (set.size < LIMITS.minShingles) return;
    const counts = new Int32Array(all.length);
    for (const s of set) for (const j of postings.get(s)) counts[j]++;
    let best = -1;
    for (let j = 0; j < all.length; j++) if (j !== i && shingles[j].size >= LIMITS.minShingles && (best < 0 || counts[j] > counts[best])) best = j;
    for (let j = 0; j < all.length; j++) {
      if (j === i || shingles[j].size < LIMITS.minShingles) continue;
      const share = counts[j] / set.size;
      if (share > LIMITS.duplicate) {
        const key = i < j ? `${i}:${j}` : `${j}:${i}`;
        if (!shared.has(key)) shared.set(key, { a: all[Math.min(i, j)].route, b: all[Math.max(i, j)].route, share: {} });
        shared.get(key).share[all[i].route] = Math.round(share * 100) / 100;
      }
    }
  });
  dup.pairs = [...shared.values()];
  // Families: connected groups of near-duplicate pages.
  const parent = new Map(all.map((p) => [p.route, p.route]));
  const find = (x) => (parent.get(x) === x ? x : (parent.set(x, find(parent.get(x))), parent.get(x)));
  for (const { a, b } of dup.pairs) parent.set(find(a), find(b));
  const groups = new Map();
  for (const { a, b } of dup.pairs) for (const r of [a, b]) { const k = find(r); if (!groups.has(k)) groups.set(k, new Set()); groups.get(k).add(r); }
  dup.groups = [...groups.values()].map((s) => [...s].sort()).sort((x, y) => y.length - x.length).map((routes) => ({ size: routes.length, families: [...new Set(routes.map((r) => all.find((p) => p.route === r).family))], routes }));
  const groupOf = new Map();
  dup.groups.forEach((g, gi) => g.routes.forEach((r) => groupOf.set(r, gi)));
  for (const p of all) {
    const mine = dup.pairs.filter((x) => x.share[p.route] !== undefined).sort((x, y) => y.share[p.route] - x.share[p.route]);
    if (!mine.length) continue;
    const top = mine[0];
    const other = top.a === p.route ? top.b : top.a;
    const g = dup.groups[groupOf.get(p.route)];
    const otherPage = all.find((q) => q.route === other);
    add(p, { rule: 'L6', where: 'main content', text: `${Math.round(top.share[p.route] * 100)}% of this page's text also appears on ${other}${mine.length > 1 ? ` (and ${mine.length - 1} more page${mine.length > 2 ? 's' : ''})` : ''}`, detail: { other, share: top.share[p.route], matches: mine.length, group: groupOf.get(p.route), groupSize: g?.size }, fix: 'Keep the shared text on one home page, link it from here, and leave only what is particular to this page.', cause: p.gen && otherPage?.gen?.generator === p.gen.generator ? { type: 'generator', file: p.gen.generator, line: null, template: p.gen.template, note: 'pages from one generator repeat the same text' } : causeOf(p, null) });
  }
}

// ---------- allowlist ----------
const globRe = (g) => new RegExp(`^${g.split('*').map(escapeRe).join('.*')}$`);
const used = new Set();
for (const p of all) for (const v of violations.get(p.route)) {
  const i = allow.findIndex((a) => a.rule === v.rule && globRe(a.route).test(p.route) && (!a.match || v.text.includes(a.match)));
  if (i >= 0) { v.allowed = allow[i].reason; used.add(i); }
}

// ---------- report ----------
const RULES = { L1: 'code-line-length', L2: 'one-line-json', L3: 'first-paragraph-context', L4: 'maintainer-text', L5: 'page-length', L6: 'near-duplicate', L7: 'title-is-id', L8: 'undefined-jargon', L9: 'overflow-390' };
const ruleKey = (v) => (v.rule === 'L9' ? `L9 ${v.detail.kind}` : v.rule);
const count = (vs) => vs.reduce((m, v) => ((m[ruleKey(v)] = (m[ruleKey(v)] ?? 0) + 1), m), {});
const open = (vs) => vs.filter((v) => !v.allowed);
const report = {
  product: product.name,
  checked: new Date().toISOString(),
  limits: LIMITS,
  rules: RULES,
  totals: { pages: all.length, violations: 0, allowed: 0, pagesWithViolations: 0, byRule: {}, byCause: {} },
  families: {},
  top: [],
  duplicates: dup,
  unusedAllow: allow.filter((_, i) => !used.has(i)),
  pages: [],
};
for (const p of all) {
  const vs = violations.get(p.route);
  const d = info.get(p.route);
  report.pages.push({ route: p.route, file: p.relFile, kind: p.data.kind ?? null, family: p.family, generated: !!p.gen, generator: p.gen ? p.gen.generator : null, template: p.gen?.template ?? null, title: d.title, firstParagraph: d.firstParagraph, words: d.words, height: d.height, counts: count(open(vs)), violations: vs });
  const f = (report.families[p.family] ??= { pages: 0, pagesWithViolations: 0, violations: 0, byRule: {}, generated: 0 });
  f.pages++;
  if (p.gen) f.generated++;
  const o = open(vs);
  if (o.length) f.pagesWithViolations++;
  f.violations += o.length;
  for (const [k, n] of Object.entries(count(o))) f.byRule[k] = (f.byRule[k] ?? 0) + n;
  report.totals.violations += o.length;
  report.totals.allowed += vs.length - o.length;
  if (o.length) report.totals.pagesWithViolations++;
  for (const [k, n] of Object.entries(count(o))) report.totals.byRule[k] = (report.totals.byRule[k] ?? 0) + n;
  for (const v of o) report.totals.byCause[v.cause.type] = (report.totals.byCause[v.cause.type] ?? 0) + 1;
}
report.top = report.pages.map((p) => ({ route: p.route, family: p.family, violations: open(p.violations).length, counts: p.counts })).filter((p) => p.violations).sort((a, b) => b.violations - a.violations || a.route.localeCompare(b.route)).slice(0, 15);
fs.writeFileSync(path.join(SITE, 'landing-report.json'), JSON.stringify(report, null, 2) + '\n');

// Worklist: open violations grouped by the file where the fix goes.
if (worklistPath) {
  const groups = new Map();
  for (const p of report.pages) for (const v of open(p.violations)) {
    const k = `${v.cause.file}\t${v.cause.template ?? ''}`;
    if (!groups.has(k)) groups.set(k, { file: v.cause.file, cause: v.cause.type, fixOnce: ['generator', 'template', 'example'].includes(v.cause.type), template: v.cause.template ?? null, pages: new Set(), byRule: {}, patterns: {}, violations: [] });
    const g = groups.get(k);
    if (g.cause !== v.cause.type) g.cause = [...new Set([...String(g.cause).split('+'), v.cause.type])].join('+');
    g.fixOnce ||= ['generator', 'template', 'example'].includes(v.cause.type);
    g.pages.add(p.route);
    g.byRule[ruleKey(v)] = (g.byRule[ruleKey(v)] ?? 0) + 1;
    const pattern = `${ruleKey(v)}: ${v.detail?.phrases?.join(', ') ?? v.detail?.term ?? v.detail?.block ?? v.detail?.reason ?? v.where.replace(/\d+/g, 'N')}`;
    g.patterns[pattern] = (g.patterns[pattern] ?? 0) + 1;
    g.violations.push({ route: p.route, rule: v.rule, where: v.where, line: v.cause.line, text: v.text, fix: v.fix, ...(v.cause.note ? { note: v.cause.note } : {}) });
  }
  const list = [...groups.values()].map((g) => ({ ...g, pages: [...g.pages], count: g.violations.length, patterns: Object.entries(g.patterns).sort((a, b) => b[1] - a[1]).map(([pattern, count]) => ({ pattern, count })) })).sort((a, b) => b.fixOnce - a.fixOnce || b.count - a.count);
  fs.mkdirSync(path.dirname(path.resolve(worklistPath)), { recursive: true });
  fs.writeFileSync(path.resolve(worklistPath), JSON.stringify({ product: product.name, checked: report.checked, note: 'Open landing-check violations grouped by the file where the fix goes. fixOnce marks a generator, shared template or example: fix it there, never on the generated or including page.', totals: report.totals, groups: list }, null, 2) + '\n');
}

// Summary.
const fmt = (o) => Object.entries(o).sort().map(([k, n]) => `${k} ${n}`).join(', ') || 'none';
console.log(`landing check, ${product.name}: ${all.length} pages, ${report.totals.violations} violations on ${report.totals.pagesWithViolations} pages (${report.totals.allowed} allowed), ${Math.round((Date.now() - t0) / 1000)} s`);
console.log(`  by rule: ${fmt(report.totals.byRule)}`);
console.log(`  by cause: ${fmt(report.totals.byCause)}`);
console.log('  by family:');
for (const [f, s] of Object.entries(report.families).sort((a, b) => b[1].violations - a[1].violations)) console.log(`    ${f.padEnd(26)} ${String(s.pages).padStart(4)} pages, ${String(s.violations).padStart(5)} violations: ${fmt(s.byRule)}`);
console.log('  top pages:');
for (const t of report.top.slice(0, 10)) console.log(`    ${String(t.violations).padStart(4)}  ${t.route}  (${fmt(t.counts)})`);
if (dup.groups.length) console.log(`  near-duplicate groups: ${dup.groups.length}, largest ${dup.groups[0].size} pages (${dup.groups[0].families.join(', ')})`);
if (report.unusedAllow.length) console.log(`  allowlist entries that matched nothing: ${report.unusedAllow.length}`);
console.log(`  report: landing-report.json${shots ? `; screenshots: landing-shots/ (${reps.size} families)` : ''}${worklistPath ? `; worklist: ${worklistPath}` : ''}`);
if (strict && report.totals.violations) {
  console.error(`--strict: ${report.totals.violations} violation(s) are not in src/data/landing-allow.json`);
  process.exit(1);
}
