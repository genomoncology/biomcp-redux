// Renders the facts and example output in the repository README from the site's data,
// so the README never carries a hand-copied count, install line or reply.
//   <!--fact:KEY-->VALUE<!--/fact-->            inline: VALUE becomes the fact from src/data/site.json
//   <!--fact:KEY-->\n```sh\nVALUE\n```\n<!--/fact-->   block: the code block holds the fact
//   <!--example:ID-->\n```json\n...\n```\n<!--/example-->  block: the saved output of examples/ID
//   node scripts/readme.mjs           write ../README.md
//   node scripts/readme.mjs --check   fail if ../README.md differs
import fs from 'node:fs';
import path from 'node:path';
import { fact } from '../src/lib/facts.mjs';
import { readExample } from '../src/lib/examples.mjs';

const check = process.argv.includes('--check');
const file = path.resolve('../README.md');
const before = fs.readFileSync(file, 'utf8');
const block = (inner, value) => {
  const lang = inner.match(/```(\w*)/)?.[1] ?? '';
  return `\n\`\`\`${lang}\n${value}\n\`\`\`\n`;
};
let n = 0;
const after = before
  .replace(/<!--fact:([\w.]+)-->([\s\S]*?)<!--\/fact-->/g, (_, k, inner) => {
    n++;
    return `<!--fact:${k}-->${inner.startsWith('\n```') ? block(inner, fact(k)) : fact(k)}<!--/fact-->`;
  })
  .replace(/<!--example:([\w-]+)-->([\s\S]*?)<!--\/example-->/g, (_, id, inner) => {
    n++;
    const ex = readExample(id);
    if (ex.unverified) throw new Error(`readme: example ${id} is unverified`);
    return `<!--example:${id}-->${block(inner, ex.output)}<!--/example-->`;
  });
if (check) {
  if (after !== before) { console.error('readme: README.md differs from site.json or the examples; run node scripts/readme.mjs'); process.exit(1); }
  console.log(`readme: ${n} facts match`);
} else {
  if (after !== before) fs.writeFileSync(file, after);
  console.log(`readme: ${n} facts ${after !== before ? 'written' : 'unchanged'}`);
}
