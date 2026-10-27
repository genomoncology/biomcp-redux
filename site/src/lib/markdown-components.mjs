// The Markdown form of this site's own components, for the Markdown twins and llms-full.txt.
// scripts/pages.mjs renders the shared components; a component missing from both fails the build.
import { exampleMarkdown } from './examples.mjs';
import { fact } from './facts.mjs';
import { envelopeMarkdown } from './envelope.mjs';
import { sourceTableMarkdown } from './sources.mjs';
import { commandTableMarkdown } from './commands.mjs';
import { keyTableMarkdown } from './keys.mjs';

export default {
  TerminalHero: (a) => exampleMarkdown(a.id, a.lines ? Number(a.lines) : undefined),
  InstallLine: (a) => '```sh\n' + fact(a.k ?? 'install.script') + '\n```',
  EnvelopeTable: (a) => envelopeMarkdown(a.field),
  SourceTable: (a) => sourceTableMarkdown(a.field, a.group),
  CommandTable: (a) => commandTableMarkdown(a.entity),
  KeyTable: () => keyTableMarkdown(),
};
