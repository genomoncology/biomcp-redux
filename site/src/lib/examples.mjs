// One home for reading an example from examples/<id>/.
// run.sh is what the build runs; show.<ext> (if present) is what the page shows.
import fs from 'node:fs';
import path from 'node:path';
import { xlsxMarkdown } from './xlsx.mjs';

const LANGS = { sh: 'sh', md: 'markdown', jsonl: 'json', txt: 'text', log: 'text', tsv: 'text', csproj: 'xml', mod: 'text', jq: 'text', py: 'python', ts: 'ts', js: 'js', sql: 'sql', go: 'go', java: 'java', kt: 'kotlin', cs: 'csharp', r: 'r', R: 'r', c: 'c', cpp: 'cpp', rb: 'ruby', swift: 'swift', zig: 'zig', rs: 'rust', json: 'json', csv: 'csv', yaml: 'yaml', toml: 'toml' };

export const EXAMPLES = path.resolve(process.cwd(), 'examples');

export function readExample(id) {
  const dir = path.join(EXAMPLES, id);
  if (!fs.existsSync(path.join(dir, 'run.sh'))) throw new Error(`example ${id}: missing examples/${id}/run.sh`);
  const shown = fs.readdirSync(dir).find((f) => f.startsWith('show.'));
  const file = shown ?? 'run.sh';
  const ext = file.split('.').pop();
  const show = fs.readFileSync(path.join(dir, file), 'utf8').replace(/^#!.*\n/, '').trimEnd();
  const read = (f) => (fs.existsSync(path.join(dir, f)) ? fs.readFileSync(path.join(dir, f), 'utf8') : null);
  const output = read('output.txt');
  if (output === null && read('unverified') === null) throw new Error(`example ${id}: no output.txt; run npm run examples`);
  return {
    id,
    show,
    lang: LANGS[ext] ?? 'text',
    output: (output ?? '').replace(/\s+$/, ''),
    exit: Number((read('exit') ?? '0').trim()),
    unverified: read('unverified') !== null,
    unverifiedReason: (read('unverified') ?? '').trim(),
    ran: (read('ran') ?? '').trim(),
  };
}

// The label of an example the build does not rerun: the saved run's day when it has one, then the reason.
export function unverifiedLabel(ex) {
  const head = ex.output && ex.ran ? `Output saved from a run on ${ex.ran}; the build does not rerun this example.` : 'The build does not run this example, and no output is shown.';
  return `${head} ${ex.unverifiedReason}`;
}

// Trim long output to the first `lines` lines, marking the cut.
export function trimOutput(out, lines) {
  if (!lines) return out;
  const all = out.split('\n');
  return all.length > lines ? [...all.slice(0, lines), '…'].join('\n') : out;
}

// A Markdown code fence longer than any backtick run in the text, so output that holds
// its own fences (Markdown replies) stays one block.
export function fenced(text, lang = '') {
  const longest = Math.max(2, ...[...text.matchAll(/`+/g)].map((m) => m[0].length));
  const fence = '`'.repeat(longest + 1);
  return `${fence}${lang}\n${text}\n${fence}`;
}

export function exampleMarkdown(id, lines) {
  const ex = readExample(id);
  if (ex.unverified) return `${fenced(ex.show, ex.lang)}\n\n${unverifiedLabel(ex)}\n${ex.output ? `\n${fenced(trimOutput(ex.output, lines), 'text')}\n` : ''}`;
  const exit = ex.exit === 0 ? '' : `\nExit code ${ex.exit}.\n`;
  return `${fenced(ex.show, ex.lang)}\n\nOutput:\n\n${fenced(trimOutput(ex.output, lines), 'text')}\n${exit}`;
}

// The files of the example folder that make the build run it, not the files the example reads.
const HARNESS = /^(run\.sh|show\..+|output\.txt|exit|ran|unverified|needs|expect-status|exact|build-only)$/;

// examples/<id>/build-only lists files that point the build at the packages staged from this
// commit, one `FILE: reason` per line. A reader writes those files differently, with the
// published package, so pages need not show them.
export function buildOnly(id) {
  const f = path.join(EXAMPLES, id, 'build-only');
  if (!fs.existsSync(f)) return new Map();
  return new Map(fs.readFileSync(f, 'utf8').split('\n').filter((l) => l.trim()).map((l) => {
    const [file, ...why] = l.split(':');
    if (!why.join(':').trim()) throw new Error(`examples/${id}/build-only: ${file} gives no reason`);
    if (!fs.existsSync(path.join(EXAMPLES, id, file.trim()))) throw new Error(`examples/${id}/build-only: ${file} does not exist`);
    return [file.trim(), why.join(':').trim()];
  }));
}

// The files an example reads from its own folder, as paths relative to it (subfolders included).
export function inputFiles(id) {
  const dir = path.join(EXAMPLES, id);
  const out = [];
  const walk = (rel) => {
    for (const f of fs.readdirSync(path.join(dir, rel), { withFileTypes: true })) {
      const r = rel ? `${rel}/${f.name}` : f.name;
      if (f.isDirectory()) walk(r);
      else if (rel || !HARNESS.test(f.name)) out.push(r);
    }
  };
  walk('');
  const skip = buildOnly(id);
  return out.filter((f) => !skip.has(f)).sort();
}

export const isSpreadsheet = (file) => /\.xlsx$/i.test(file);
export const fileLang = (file, lang) => lang ?? LANGS[file.split('.').pop()] ?? 'text';

// One input file as Markdown: a titled code block, or a table per worksheet for a spreadsheet.
export function fileMarkdown(id, file, lang) {
  const p = path.join(EXAMPLES, id, file);
  if (isSpreadsheet(file)) return xlsxMarkdown(p, file);
  return '```' + fileLang(file, lang) + ' title="' + file + '"\n' + fs.readFileSync(p, 'utf8').trimEnd() + '\n```';
}

// MDX that shows an example's input files and then the example, for generated pages.
// The page must import Example, and ExampleFile when the example reads files.
export function exampleWithFiles(id, lines) {
  const files = inputFiles(id);
  const list = files.map((f) => `\`${f}\``);
  const lead = files.length ? `The example reads ${list.length > 1 ? `${list.slice(0, -1).join(', ')} and ${list.at(-1)}` : list[0]}:\n\n${files.map((f) => `<ExampleFile id="${id}" file="${f}" />`).join('\n\n')}\n\n` : '';
  return `${lead}<Example id="${id}"${lines ? ` lines={${lines}}` : ''} />`;
}
