// Writes the files agents read: a Markdown twin of every page at its address plus .md,
// llms.txt (the spine and every section) and llms-full.txt (every page's full text).
import fs from 'node:fs';
import path from 'node:path';
import product from '../product.config.mjs';
import site from '../src/data/site.json' with { type: 'json' };
import redirects from '../src/data/redirects.json' with { type: 'json' };
import { pages, bodyMarkdown } from './pages.mjs';

const dist = path.resolve('dist');
const all = pages();
const abs = (p) => new URL(p, product.site).href;

for (const p of all) {
  const md = `---\n${p.frontmatter}\n---\n\n# ${p.data.title}\n\n${bodyMarkdown(p.body)}\n`;
  const out = path.join(dist, p.twin);
  fs.mkdirSync(path.dirname(out), { recursive: true });
  fs.writeFileSync(out, md);
}

// Each redirected old page address also served a Markdown twin (/a/b/ -> /a/b.md). Where the
// build has no twin at that address, write one that names the new address and holds its page.
let moved = 0;
for (const [from, to] of Object.entries(redirects)) {
  if (!from.endsWith('/')) continue;
  const twin = from === '/' ? '/index.md' : `${from.replace(/\/$/, '')}.md`;
  const out = path.join(dist, twin);
  if (fs.existsSync(out)) continue;
  const target = all.find((p) => p.route === to.split('#')[0]);
  if (!target) continue; // scripts/check.mjs reports a redirect to a missing page
  const md = `---\nmoved_to: ${abs(target.twin)}\n---\n\nThis address moved. The page is now at ${abs(target.route)} and its Markdown at ${abs(target.twin)}.\n\n# ${target.data.title}\n\n${bodyMarkdown(target.body)}\n`;
  fs.mkdirSync(path.dirname(out), { recursive: true });
  fs.writeFileSync(out, md);
  moved++;
}

const bySlug = new Map(all.map((p) => [p.slug, p]));
const line = (p) => `- [${p.data.title}](${abs(p.twin)}): ${p.data.description}`;
const spine = product.spine.map((s) => bySlug.get(s)).filter(Boolean);
const sections = new Map();
for (const p of all) {
  if (spine.includes(p)) continue;
  const top = p.slug.split('/')[0] || 'home';
  const name = product.sectionNames?.[top] ?? top;
  if (!sections.has(name)) sections.set(name, []);
  sections.get(name).push(p);
}
let llms = `# ${product.name}\n\n> ${product.description}\n\nVersion ${site.version}. Every page below is Markdown. The full text of every page is in ${abs('/llms-full.txt')}.\n\n## Start here, in order\n\n${spine.map(line).join('\n')}\n`;
for (const [name, ps] of sections) llms += `\n## ${name}\n\n${ps.map(line).join('\n')}\n`;
fs.writeFileSync(path.join(dist, 'llms.txt'), llms);

const ordered = [...spine, ...[...sections.values()].flat()];
fs.writeFileSync(
  path.join(dist, 'llms-full.txt'),
  `# ${product.name} ${site.version}: full documentation\n\n` +
    ordered.map((p) => `<page url="${abs(p.route)}">\n\n# ${p.data.title}\n\n${bodyMarkdown(p.body)}\n\n</page>`).join('\n\n') + '\n',
);
console.log(`agent files: ${all.length} Markdown twins, ${moved} moved-page twins, llms.txt, llms-full.txt`);

// Static hosts serve /404.html for a missing address.
if (fs.existsSync(path.join(dist, '404/index.html'))) fs.copyFileSync(path.join(dist, '404/index.html'), path.join(dist, '404.html'));
