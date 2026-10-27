// The key table, one home: the keys in src/data/reference.json (written by gen-reference.mjs
// from `key list` and `help key`) and the sources that take each key in src/data/sources.json.
import reference from '../data/reference.json' with { type: 'json' };
import sources from '../data/sources.json' with { type: 'json' };

export function keyRows() {
  return reference.keys.map((k) => {
    const srcs = sources.filter((s) => s.keyName === k.source);
    if (!srcs.length) throw new Error(`key ${k.source}: no source in src/data/sources.json takes it`);
    return { name: k.source, environment: k.environment, sources: srcs.map((s) => ({ source: s.source, route: s.route })), use: srcs[0].keyUse };
  });
}

export function keyTableMarkdown() {
  return ['| Key name | Environment variable | Sources | Needed? |', '| --- | --- | --- | --- |',
    ...keyRows().map((r) => `| \`${r.name}\` | \`${r.environment}\` | ${r.sources.map((s) => `[\`${s.source}\`](${s.route})`).join(', ')} | ${r.use} |`)].join('\n');
}
