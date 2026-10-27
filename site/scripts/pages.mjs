// The docs pages, their routes and front matter, read from src/content/docs.
import fs from 'node:fs';
import path from 'node:path';
import YAML from 'yaml';
import { exampleMarkdown } from '../src/lib/examples.mjs';
import { fact } from '../src/lib/facts.mjs';

export const DOCS = path.resolve('src/content/docs');

export function pages() {
  const out = [];
  const walk = (dir) => {
    for (const f of fs.readdirSync(dir, { withFileTypes: true })) {
      const p = path.join(dir, f.name);
      if (f.isDirectory()) walk(p);
      else if (/\.mdx?$/.test(f.name) && f.name !== '404.mdx') out.push(page(p));
    }
  };
  walk(DOCS);
  return out.sort((a, b) => a.route.localeCompare(b.route));
}

function page(file) {
  const raw = fs.readFileSync(file, 'utf8');
  const m = raw.match(/^---\n([\s\S]*?)\n---\n?/);
  const data = m ? YAML.parse(m[1]) : {};
  const body = m ? raw.slice(m[0].length) : raw;
  let slug = path.relative(DOCS, file).replace(/\.mdx?$/, '').toLowerCase();
  slug = slug === 'index' ? '' : slug.replace(/\/index$/, '');
  return { file, slug, route: slug ? `/${slug}/` : '/', twin: slug ? `/${slug}.md` : '/index.md', data, body, frontmatter: m ? m[1] : '' };
}

// The page body as plain Markdown: components become their text.
export function bodyMarkdown(body) {
  return body
    .replace(/^import .*$\n?/gm, '')
    .replace(/<Example\s+id="([^"]+)"(?:\s+title="[^"]*")?(?:\s+lines=\{(\d+)\})?\s*\/>/g, (_, id, lines) => exampleMarkdown(id, lines ? Number(lines) : undefined))
    .replace(/<InstallLine(?:\s+k="([^"]+)")?\s*\/>/g, (_, k) => '```sh\n' + fact(k ?? 'install.script') + '\n```')
    .replace(/<Fact\s+k="([^"]+)"\s*\/>/g, (_, k) => fact(k))
    .replace(/<\/?(Hero|CardGrid|Card|LinkCard|Tabs|TabItem|Aside|Steps|FunctionTable|Badge)[^>]*>/g, '')
    .replace(/\n{3,}/g, '\n\n')
    .trim();
}
