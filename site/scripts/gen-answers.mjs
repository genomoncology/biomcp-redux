// Writes one answers page per question-list entry marked `answers_page: true`,
// and the answers index that lists them. Pages are rebuilt, never edited.
import fs from 'node:fs';
import path from 'node:path';
import YAML from 'yaml';
import site from '../src/data/site.json' with { type: 'json' };
import { ranOn } from '../src/lib/verified.mjs';
import { pages } from './pages.mjs';

const qdir = path.resolve('questions');
const out = path.resolve('src/content/docs/answers');
// The question list writes the version as {version}; site.json holds it.
const withFacts = (v) => (typeof v === 'string' ? v.replaceAll('{version}', site.version) : v);
const questions = fs.readdirSync(qdir).filter((f) => f.endsWith('.yaml')).flatMap((f) =>
  (YAML.parse(fs.readFileSync(path.join(qdir, f), 'utf8')) ?? []).map((q) => ({ ...Object.fromEntries(Object.entries(q).map(([k, v]) => [k, withFacts(v)])), area: f.replace(/\.yaml$/, '') })),
);
const seen = new Set();
for (const q of questions) {
  if (seen.has(q.id)) throw new Error(`questions: duplicate id ${q.id}`);
  seen.add(q.id);
}
const answered = questions.filter((q) => q.answers_page).sort((a, b) => a.rank - b.rank);
fs.rmSync(out, { recursive: true, force: true });
fs.mkdirSync(out, { recursive: true });

// An answer is verified on the day its example last ran (examples/<id>/ran), not on a typed date.
for (const q of answered) q.verified = ranOn(q.example);
const lastRun = answered.map((q) => q.verified).sort().at(-1) ?? new Date().toISOString().slice(0, 10);
// Link text is the target page's own title, so a link never names a page differently from the page.
const titles = new Map(pages().filter((p) => !p.route.startsWith('/answers/')).map((p) => [p.route, p.data.title]));
const titleOf = (q, k) => {
  const t = titles.get(q[k].split('#')[0]);
  if (!t) throw new Error(`questions/${q.area}.yaml: ${q.id} ${k} ${q[k]} is not a page`);
  return t;
};
const fm = (o) => `---\n${YAML.stringify(o).trim()}\n---\n`;
for (const q of answered) {
  for (const k of ['question', 'answer', 'example', 'why', 'limits', 'page', 'reference']) if (!q[k]) throw new Error(`questions/${q.area}.yaml: ${q.id} lacks ${k}`);
  const body = [
    "import Example from '../../../components/Example.astro';",
    '',
    q.answer.trim(),
    '',
    '## Command',
    '',
    `<Example id="${q.example}"${q.lines ? ` lines={${q.lines}}` : ''} />`,
    '',
    '## Why it works',
    '',
    q.why.trim(),
    '',
    '## Limits',
    '',
    q.limits.trim(),
    '',
    '## Read next',
    '',
    `- [${titleOf(q, 'page')}](${q.page})`,
    `- [${titleOf(q, 'reference')}](${q.reference})`,
    '',
    `Verified on ${q.verified} with ${site.product} ${site.version}.`,
    '',
  ].join('\n');
  fs.writeFileSync(
    path.join(out, `${q.id}.mdx`),
    fm({ title: q.question, description: q.answer.trim().split('\n')[0].slice(0, 300), goal: `Answer: ${q.question}`, reader: q.reader ?? 'Someone who searched for this question', kind: 'answer', verified: q.verified, sidebar: { hidden: true }, pagefind: true }) + '\n' + body,
  );
}
const index = [
  `${site.product} answers, one question per page. Each answer shows a command, its real output from the build, why it works and its limits.`,
  '',
  ...[...new Set(answered.map((q) => q.area))].flatMap((area) => [
    `## ${area[0].toUpperCase()}${area.slice(1).replace(/-/g, ' ')}`,
    '',
    ...answered.filter((q) => q.area === area).map((q) => `- [${q.question}](/answers/${q.id}/)`),
    '',
  ]),
].join('\n');
fs.writeFileSync(
  path.join(out, 'index.mdx'),
  fm({ title: 'Answers', description: `Short answers to common ${site.product} questions, each with a command and its real output.`, goal: 'List every answers page so a reader or agent can find one question.', reader: 'Anyone with one specific question', kind: 'index', verified: lastRun }) + '\n' + index,
);
console.log(`answers: ${answered.length} pages from ${questions.length} questions`);
