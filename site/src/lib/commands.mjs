// The command rows of one entity, one home: src/data/commands.json, written by gen-reference.mjs
// from the program's help. Hand-written pages render them with <CommandTable entity="..." />.
import rows from '../data/commands.json' with { type: 'json' };

export const commandRows = (entity) => rows.filter((r) => r.entity === entity);

export function commandTableMarkdown(entity) {
  return ['| Command | Description |', '| --- | --- |',
    ...commandRows(entity).map((r) => `| [\`biomcp ${r.command}\`](${r.route}) | ${r.description.replace(/\|/g, '\\|')} |`)].join('\n');
}
