// One home per fact: src/data/site.json. Pages read facts through <Fact k="..."/>.
import site from '../data/site.json' with { type: 'json' };

export function fact(key) {
  const v = key.split('.').reduce((o, k) => (o == null ? o : o[k]), site);
  if (v === undefined) throw new Error(`unknown fact: ${key}`);
  return String(v);
}
