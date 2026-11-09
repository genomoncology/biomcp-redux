// The status, source-state and next-kind rows, one home: src/data/reference.json (written by gen-reference.mjs).
import reference from '../data/reference.json' with { type: 'json' };

export function envelopeRows(field) {
  const rows = reference.envelope?.[field];
  if (!rows) throw new Error(`unknown envelope field: ${field}`);
  return rows;
}

const esc = (s) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
export const inlineCode = (s) => esc(s).replace(/`([^`]+)`/g, '<code>$1</code>');

export function envelopeMarkdown(field) {
  const rows = envelopeRows(field);
  const head = field === 'status' ? '| Status | Meaning | Exit code |\n| --- | --- | --- |' : `| ${field === 'kind' ? 'Kind' : 'State'} | Meaning |\n| --- | --- |`;
  return [head, ...rows.map((r) => `| \`${r.value}\` | ${r.meaning} |${field === 'status' ? ` ${r.exit} |` : ''}`)].join('\n');
}
