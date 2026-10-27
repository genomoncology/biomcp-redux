// Writes one answers page per question-list entry marked `answers_page: true`,
// and the answers index that lists them. Pages are rebuilt, never edited.
import fs from 'node:fs';
import path from 'node:path';
import YAML from 'yaml';
import site from '../src/data/site.json' with { type: 'json' };

const qdir = path.resolve('questions');
const out = path.resolve('src/content/docs/answers');
const questions = fs.readdirSync(qdir).filter((f) => f.endsWith('.yaml')).flatMap((f) =>
  (YAML.parse(fs.readFileSync(path.join(qdir, f), 'utf8')) ?? []).map((q) => ({ ...q, area: f.replace(/\.yaml$/, '') })),
);
const seen = new Set();
for (const q of questions) {
  if (seen.has(q.id)) throw new Error(`questions: duplicate id ${q.id}`);
  seen.add(q.id);
}
const answered = questions.filter((q) => q.answers_page).sort((a, b) => a.rank - b.rank);
fs.rmSync(out, { recursive: true, force: true });
fs.mkdirSync(out, { recursive: true });

const fm = (o) => `---\n${YAML.stringify(o).trim()}\n---\n`;
for (const q of answered) {
  for (const k of ['question', 'answer', 'example', 'why', 'limits', 'page', 'reference', 'verified']) if (!q[k]) throw new Error(`questions/${q.area}.yaml: ${q.id} lacks ${k}`);
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
    `- [${q.page_title ?? 'Main guide'}](${q.page})`,
    `- [${q.reference_title ?? 'Reference'}](${q.reference})`,
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
  fm({ title: 'Answers', description: `Short answers to common ${site.product} questions, each with a command and its real output.`, goal: 'List every answers page so a reader or agent can find one question.', reader: 'Anyone with one specific question', kind: 'index', verified: site.verified }) + '\n' + index,
);
console.log(`answers: ${answered.length} pages from ${questions.length} questions`);
