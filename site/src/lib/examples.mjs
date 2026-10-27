// One home for reading an example from examples/<id>/.
// run.sh is what the build runs; show.<ext> (if present) is what the page shows.
import fs from 'node:fs';
import path from 'node:path';

const LANGS = { sh: 'sh', py: 'python', ts: 'ts', js: 'js', sql: 'sql', go: 'go', java: 'java', kt: 'kotlin', cs: 'csharp', r: 'r', R: 'r', c: 'c', cpp: 'cpp', rb: 'ruby', swift: 'swift', zig: 'zig', rs: 'rust', json: 'json', csv: 'csv', yaml: 'yaml', toml: 'toml' };

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
  };
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
  if (ex.unverified) return `${fenced(ex.show, ex.lang)}\n\nNot run in the build: ${ex.unverifiedReason}\n`;
  const exit = ex.exit === 0 ? '' : `\nExit code ${ex.exit}.\n`;
  return `${fenced(ex.show, ex.lang)}\n\nOutput:\n\n${fenced(trimOutput(ex.output, lines), 'text')}\n${exit}`;
}
