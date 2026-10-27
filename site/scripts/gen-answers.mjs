// Writes one answers page per question-list entry marked `answers_page: true`,
// and the answers index that lists them. Pages are rebuilt, never edited.
import fs from 'node:fs';
import path from 'node:path';
import YAML from 'yaml';
import site from '../src/data/site.json' with { type: 'json' };
import { ranOn } from '../src/lib/verified.mjs';
import { pages } from './pages.mjs';
import { exampleWithFiles, inputFiles } from '../src/lib/examples.mjs';

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
for (const q of answered) q.verified = (q.steps ?? [q]).map((st) => ranOn(st.example)).sort().at(-1);
const lastRun = answered.map((q) => q.verified).sort().at(-1) ?? new Date().toISOString().slice(0, 10);
// Link text is the target page's own title, so a link never names a page differently from the page.
const titles = new Map(pages().filter((p) => !p.route.startsWith('/answers/')).map((p) => [p.route, p.data.title]));
const titleOf = (q, k) => {
  const t = titles.get(q[k].split('#')[0]);
  if (!t) throw new Error(`questions/${q.area}.yaml: ${q.id} ${k} ${q[k]} is not a page`);
  return t;
};
const fm = (o) => `---\n${YAML.stringify(o).trim()}\n---\n`;
// The value a field has in an example's saved output: the first JSON value of that key in the
// output's JSON documents, or a labelled text line (field: value). The "Reading the output" table
// shows it beside its meaning.
const jsonDocs = (text) => {
  const docs = [];
  let buf = [];
  for (const line of text.split('\n')) {
    buf.push(line);
    if (/^[}\]]$/.test(line) || (buf.length === 1 && /^[{[].*[}\]]$/.test(line))) {
      try { docs.push(JSON.parse(buf.join('\n'))); } catch {}
      buf = [];
    } else if (buf.length === 1 && !/^[{[]/.test(line)) buf = [];
  }
  return docs;
};
const findKey = (v, key) => {
  if (Array.isArray(v)) { for (const x of v) { const r = findKey(x, key); if (r !== undefined) return r; } return undefined; }
  if (v && typeof v === 'object') {
    if (key in v) return v[key];
    for (const x of Object.values(v)) { const r = findKey(x, key); if (r !== undefined) return r; }
  }
  return undefined;
};
const valueIn = (ids, field) => {
  const key = field.split('.').pop().replace(/\[\]$/, '');
  for (const id of ids) {
    const out = fs.readFileSync(path.resolve('examples', id, 'output.txt'), 'utf8');
    for (const d of jsonDocs(out)) { const v = findKey(d, key); if (v !== undefined) return typeof v === 'string' ? v : JSON.stringify(v); }
    const esc = key.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const m = out.match(new RegExp(`^\\s*${esc}:\\s*(.+?)\\s*$`, 'm'));
    if (m) return m[1];
  }
  return null;
};
const shortValue = (v) => (v.length > 40 ? `${v.slice(0, 39)}…` : v);
const stepsOf = (q) => (q.steps ?? [{ example: q.example, says: q.says, lines: q.lines }]);
const PRODUCT = `${site.product} ${site.version} is a command-line tool and MCP server that reads biomedical data sources.`;
for (const q of answered) {
  for (const k of ['question', 'answer', 'why', 'limits', 'page', 'reference', 'explain']) if (!q[k]) throw new Error(`questions/${q.area}.yaml: ${q.id} lacks ${k}`);
  if (!q.example && !q.steps) throw new Error(`questions/${q.area}.yaml: ${q.id} lacks example or steps`);
  const explainFrom = stepsOf(q).map((st) => st.example).reverse();
  const rows = Object.entries(q.explain).map(([field, meaning]) => {
    const v = valueIn(explainFrom, field);
    if (v === null) throw new Error(`questions/${q.area}.yaml: ${q.id} explains ${field}, which the output of ${explainFrom.join(', ')} does not show`);
    return `| \`${field}\` | \`${shortValue(v).replace(/`/g, "'").replace(/\|/g, '\\|')}\` | ${String(meaning).trim().replace(/\|/g, '\\|').replace(/\n+/g, ' ')} |`;
  });
  const steps = stepsOf(q);
  const usesFiles = steps.some((st) => inputFiles(st.example).length);
  const body = [
    "import Example from '../../../components/Example.astro';",
    ...(usesFiles ? ["import ExampleFile from '../../../components/ExampleFile.astro';"] : []),
    '',
    `${PRODUCT} ${q.answer.trim()}`,
    '',
    `## ${steps.length > 1 ? 'Commands' : 'Command'}`,
    '',
    ...steps.flatMap((st) => [...(st.says ? [st.says.trim(), ''] : []), exampleWithFiles(st.example, st.lines), '']),
    ...(q.note ? [q.note.trim(), ''] : []),
    '## Reading the output',
    '',
    '| Field | In this run | Meaning |',
    '| --- | --- | --- |',
    ...rows,
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
  `${site.product} is a command-line tool and MCP server that reads biomedical data sources. These pages answer one question each. Each answer shows the command, its real output with a table that reads it, why it works and its limits.`,
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
