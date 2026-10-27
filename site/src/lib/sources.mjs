// The source overview rows, one home: src/data/sources.json, written by gen-reference.mjs
// from the program's registry, its key help and src/data/sources.yaml.
import rows from '../data/sources.json' with { type: 'json' };

export const GROUPS = ['open', 'attribution', 'noncommercial', 'restricted'];
export const sourceRows = () => rows;
export const groupCounts = () => GROUPS.map((g) => ({ group: g, count: rows.filter((r) => r.use === g).length }));
const keyText = (r) => (r.keyName ? `${r.keyUse.split(';')[0]}: \`${r.keyName}\`` : 'none');

export const groupRows = (group) => rows.filter((r) => r.use === group);

export function sourceTableMarkdown(field = 'sources', group) {
  if (group) return ['| Source | Name | Terms |', '| --- | --- | --- |', ...groupRows(group).map((r) => `| [\`${r.source}\`](${r.route}) | ${r.name} | [${r.terms.replace(/\|/g, '\\|')}](${r.terms_url}) |`)].join('\n');
  if (field === 'groups') return ['| Use group | Sources |', '| --- | --- |', ...groupCounts().map((g) => `| \`${g.group}\` | ${g.count} |`)].join('\n');
  return ['| Source | Name | Use group | Key | Terms |', '| --- | --- | --- | --- | --- |',
    ...rows.map((r) => `| [\`${r.source}\`](${r.route}) | ${r.name} | \`${r.use}\` | ${keyText(r)} | [${r.terms.replace(/\|/g, '\\|')}](${r.terms_url}) |`)].join('\n');
}
