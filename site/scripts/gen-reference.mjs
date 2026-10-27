// Writes the reference pages under src/content/docs/reference/ from the built program:
//   biomcp help                  every command, argument, section and example (biomcp.help/2)
//   biomcp --offline health      the source registry: names, use groups and terms (no requests)
//   biomcp help key SOURCE       each credential's configuration key and environment variable
//   biomcp serve (tools/list)    the MCP tools and their input schemas
//   ../docs/reference/commands.md  the clap grammar's root options (written by make docs-generate)
//   ../crates/biomcp/src/exit.rs   the process exit codes
// Test-harness flags (--replay, --record, --case-cache, --observe) are left out.
//   node scripts/gen-reference.mjs           write the pages
//   node scripts/gen-reference.mjs --check   fail if the committed pages differ
// Hand-written notes about each source live in src/data/sources.yaml and are merged in.
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import YAML from 'yaml';
import product from '../product.config.mjs';
import site from '../src/data/site.json' with { type: 'json' };
import { VERIFIED, same, stamp } from '../src/lib/verified.mjs';

const check = process.argv.includes('--check');
const OUT = path.resolve('src/content/docs/reference');
const bin = path.join(path.resolve(product.binDir), product.binary);
const HARNESS = ['--replay', '--record', '--case-cache', '--observe'];
const files = new Map();
// Source pages live at /sources/SLUG/, the addresses the 0.9 site used where a 0.9 page existed.
const SOURCES_OUT = path.resolve('src/content/docs/sources');
const SLUG_09 = { ctgov: 'clinicaltrials-gov', car: 'clingen-allele-registry', erepo: 'clingen-erepo', hpa: 'human-protein-atlas', monarch: 'monarch-initiative', 'openfda-label': 'openfda', clinpgx: 'pharmgkb', 'who-pq': 'who-prequalification' };
const srcSlug = (s) => SLUG_09[s] ?? s;
const srcRoute = (s) => `/sources/${srcSlug(s)}/`;

// SITE_EXAMPLE_CONFIG, the machine's config file for the examples, applies here too: the help and
// health replies need no network, but a proxy setting changes whether BioMCP starts at all.
const baseEnv = { ...process.env, NO_COLOR: '1' };
if (process.env.SITE_EXAMPLE_CONFIG) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'biomcp-reference-'));
  fs.mkdirSync(path.join(dir, path.dirname(product.exampleConfig)), { recursive: true });
  fs.copyFileSync(path.resolve(process.env.SITE_EXAMPLE_CONFIG), path.join(dir, product.exampleConfig));
  baseEnv.XDG_CONFIG_HOME = dir;
  process.on('exit', () => fs.rmSync(dir, { recursive: true, force: true }));
}
function run(args, input, env = baseEnv) {
  const r = spawnSync(bin, args, { input, encoding: 'utf8', env, timeout: 60000 });
  if (r.error) throw r.error;
  return r.stdout;
}
// A reply BioMCP could not give (status usage or local) stops the script with BioMCP's own message.
function json(args, input, env) {
  const reply = JSON.parse(run(args, input, env));
  if (reply.status === 'usage' || reply.status === 'local') {
    console.error(`reference: biomcp ${args.join(' ')} replied ${reply.status}: ${reply.error?.code ?? ''} ${reply.error?.message ?? JSON.stringify(reply)}`);
    console.error('On a network that needs a proxy, set SITE_EXAMPLE_CONFIG to a BioMCP config with proxy and ca_bundle (see site/README.md).');
    process.exit(1);
  }
  return reply;
}

// ---------- inputs ----------
const help = json(['help']).result.commands;
// Help topics that `biomcp help` does not list but replies name in `next`, such as `help network proxy`.
const TOPICS = [['network', 'proxy'], ['network', 'ca_bundle']];
const topics = TOPICS.map((t) => json(['help', ...t]).result.commands[0]);
const registry = json(['--offline', 'health']).result.sources.map(({ source, usage, terms, terms_url }) => ({ source, usage, terms, terms_url }));
const keyCommands = help.filter((c) => c.command[0] === 'help' && c.command[1] === 'key' && c.command.length === 3);
const keys = keyCommands.map((c) => ({ name: c.command[2], ...json(['help', 'key', c.command[2]]).result }));
const tools = (() => {
  const msgs = [
    { jsonrpc: '2.0', id: 1, method: 'initialize', params: { protocolVersion: '2025-06-18', capabilities: {}, clientInfo: { name: 'site-reference', version: '1' } } },
    { jsonrpc: '2.0', method: 'notifications/initialized' },
    { jsonrpc: '2.0', id: 2, method: 'tools/list' },
  ];
  const out = run(['serve'], msgs.map((m) => JSON.stringify(m)).join('\n') + '\n');
  const reply = out.split('\n').filter(Boolean).map((l) => JSON.parse(l)).find((m) => m.id === 2);
  return reply.result.tools;
})();
const notes = YAML.parse(fs.readFileSync(path.resolve('src/data/sources.yaml'), 'utf8')) ?? {};
const meanings = YAML.parse(fs.readFileSync(path.resolve('src/data/envelope.yaml'), 'utf8'));

const grammar = fs.readFileSync(path.resolve('../docs/reference/commands.md'), 'utf8');
const rootBlock = grammar.split('\n## biomcp author')[0];
const globalOptions = [...rootBlock.matchAll(/^ {6}(--[a-z-]+)(?: <([a-z-]+)>)?\n(?: {10}(.*)\n)?/gm)]
  .map(([, name, value, extra]) => ({ name, value: value ?? null, choices: (extra ?? '').match(/\[possible values: ([^\]]+)\]/)?.[1] ?? null }))
  .filter((o) => !HARNESS.includes(o.name));

const exitSrc = fs.readFileSync(path.resolve('../crates/biomcp/src/exit.rs'), 'utf8');
const exitCodes = [{ code: 0, status: 'Success' }, ...[...exitSrc.matchAll(/Status::(\w+) => ExitCode::from\((\d+)\)/g)].map(([, status, code]) => ({ code: Number(code), status }))]
  .sort((a, b) => a.code - b.code);

// ---------- helpers ----------
const esc = (s) => String(s ?? '').replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
const cell = (s) => esc(s).replace(/\|/g, '\\|').replace(/\n+/g, ' ');
const code = (s) => '`' + String(s).replace(/`/g, "'") + '`';
const words = (c) => c.command.filter((w) => !/^[<[]/.test(w));
const name = (c) => words(c).join(' ');
const slug = (c) => words(c).join('-');
const route = (c) => `/reference/commands/${slug(c)}/`;
const quote = (a) => (/^[A-Za-z0-9_.:\/@=,+-]+$/.test(a) ? a : `'${a.replace(/'/g, "'\\''")}'`);
const fm = (o) => `---\n${YAML.stringify({ ...o, kind: 'reference', verified: VERIFIED }).trim()}\n---\n\n`;
const write = (rel, text) => files.set(path.join(OUT, rel), text.replace(/\n{3,}/g, '\n\n').trimEnd() + '\n');
const firstSentence = (s) => (s.split(/(?<=\.)\s/)[0] ?? s).trim();
const ENTITY_OF = (c) => (['search', 'get'].includes(c.command[0]) ? c.command[1] : c.command[0]);
const entities = [...new Set(help.filter((c) => ['search', 'get'].includes(c.command[0])).map((c) => c.command[1]))];
const genFooter = `This page is generated from the help of ${site.product} ${site.version}. Fix a wrong sentence in the help text, then regenerate.`;
const sourcesFor = (c) => c.arguments.find((a) => a.name === '--source')?.choices ?? [];
const canonical = new Set(registry.map((r) => r.source));

// Reply schemas: every schema ID that `biomcp version --verbose` lists whose root is a reply envelope.
const schemaIds = (run(['version', '--verbose']).match(/^Schemas: (.*)$/m)?.[1] ?? '').split(', ').filter(Boolean);
const replySchemas = schemaIds.map((id) => ({ id, schema: json(['schema', id]) })).filter((x) => x.schema.properties?.schema?.const === x.id);
// Which reply schema each command returns: run its help examples, plus a fixed call for commands
// without examples, with --offline in an empty cache and home, so no request leaves the machine.
const schemaOf = (() => {
  const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'biomcp-schemas-'));
  process.on('exit', () => fs.rmSync(tmp, { recursive: true, force: true }));
  const env = { ...Object.fromEntries(Object.entries(baseEnv).filter(([k]) => !/(_API_KEY|_TOKEN|_SECRET|_PASSWORD)$/.test(k))), HOME: tmp, XDG_CACHE_HOME: path.join(tmp, 'cache'), XDG_DATA_HOME: path.join(tmp, 'data') };
  if (!process.env.SITE_EXAMPLE_CONFIG) env.XDG_CONFIG_HOME = path.join(tmp, 'config');
  // Commands that start a server, write files outside the cache or change keys never run here.
  const SKIP = ['serve', 'skill', 'key set', 'key remove', 'mcp-config', 'version', 'schema'];
  const CALLS = { help: [['help']], health: [['health']], 'key list': [['key', 'list']], 'store clean': [['store', 'clean']], sync: [['sync', 'hgnc']] };
  const known = new Set(replySchemas.map((x) => x.id));
  const map = new Map();
  for (const c of [...help, ...topics]) {
    const n = name(c);
    if (SKIP.some((w) => n === w || n.startsWith(w + ' '))) continue;
    const calls = c.examples?.length ? c.examples : CALLS[n] ?? (c.command[0] === 'help' ? [c.command] : []);
    const seen = [];
    for (const e of calls) {
      const out = spawnSync(bin, ['--offline', ...e], { env, encoding: 'utf8', timeout: 60000 }).stdout;
      let id = null;
      try { id = JSON.parse(out).schema; } catch {}
      if (known.has(id) && id !== 'biomcp.error/1' && !seen.includes(id)) seen.push(id);
    }
    map.set(n, seen);
  }
  return map;
})();

// ---------- one page per reply schema ----------
const schemaSlug = (id) => id.replace(/[^A-Za-z0-9]+/g, '-').toLowerCase();
const schemaRoute = (id) => `/reference/schemas/${schemaSlug(id)}/`;
const refName = (r) => r.split('/').pop();
const typeOf = (p) => {
  if (!p || typeof p !== 'object') return 'any';
  if (p.$ref) return refName(p.$ref);
  if (p.const !== undefined) return JSON.stringify(p.const);
  if (p.enum) return p.enum.map((v) => JSON.stringify(v)).join(' or ');
  for (const k of ['anyOf', 'oneOf', 'allOf']) if (p[k]) return p[k].map(typeOf).join(k === 'allOf' ? ' and ' : ' or ');
  if (p.type === 'array') return `array of ${typeOf(p.items)}`;
  if (Array.isArray(p.type)) return p.type.join(' or ');
  if (p.type === 'object' && p.additionalProperties && typeof p.additionalProperties === 'object') return `map of ${typeOf(p.additionalProperties)}`;
  return p.type ? `${p.type}${p.format ? ` (${p.format})` : ''}` : 'any';
};
const target = (p, defs) => {
  // The definition a property points to, through anyOf [X, null].
  const ref = p?.$ref ?? (p?.anyOf ?? p?.oneOf ?? []).find((x) => x.$ref)?.$ref;
  return ref ? { name: refName(ref), def: defs[refName(ref)] } : null;
};
// Type names that have a section on the page become links to it (the heading's anchor).
let linkTypes = new Map();
const linked = (text) => text.replace(/\b[A-Z][A-Za-z0-9]*\b/g, (n) => (linkTypes.has(n) ? `[${n}](#${linkTypes.get(n)})` : n));
const fieldTable = (def) => {
  const props = Object.entries(def?.properties ?? {});
  if (!props.length) return null;
  const req = def.required ?? [];
  return ['| Field | Type | Required | Description |', '| --- | --- | --- | --- |', ...props.map(([k, v]) => `| ${code(k)} | ${linked(cell(typeOf(v)))} | ${req.includes(k) ? 'yes' : 'no'} | ${cell(v.description ?? '')} |`)].join('\n');
};
// Every definition a schema node refers to, directly or through other definitions.
const refsIn = (node, defs, seen = new Set()) => {
  const walk = (n) => {
    if (!n || typeof n !== 'object') return;
    if (Array.isArray(n)) { n.forEach(walk); return; }
    if (typeof n.$ref === 'string') {
      const r = refName(n.$ref);
      if (!seen.has(r) && defs[r]) { seen.add(r); walk(defs[r]); }
    }
    for (const [k, v] of Object.entries(n)) if (k !== '$ref') walk(v);
  };
  walk(node);
  return seen;
};
const variantsList = (def) => {
  const vs = def?.oneOf ?? def?.anyOf;
  if (!vs) return null;
  return vs.map((v) => `- ${cell(typeOf(v.$ref ? v : (v.properties ? { type: 'object' } : v)))}${v.description ? `: ${cell(v.description)}` : ''}`).join('\n');
};
// A schema description as prose: Rust doc tests in a description are not for readers.
const descText = (d) => esc(String(d ?? '').replace(/```[\s\S]*?```/g, '').trim());
const describeDef = (label, t) => {
  if (!t?.def) return [];
  const out = [`## ${label}: ${t.name}`, '', descText(t.def.description), ''];
  const ft = fieldTable(t.def);
  if (ft) out.push(ft, '');
  else if (variantsList(t.def)) out.push('One of:', '', linked(variantsList(t.def)), '');
  else out.push(`Type: ${linked(cell(typeOf(t.def)))}.`, '');
  return out;
};
// One section per named type, so every type a table names is defined on the page.
const describeType = (name, def) => {
  const out = [`### ${name}`, '', descText(def.description), ''];
  const ft = fieldTable(def);
  if (ft) out.push(ft, '');
  else if (def.enum) out.push(`Values: ${def.enum.map((v) => code(v)).join(', ')}.`, '');
  else if (variantsList(def)) out.push('One of:', '', linked(variantsList(def)), '');
  else out.push(`Type: ${linked(cell(typeOf(def)))}.`, '');
  return out;
};
// The commands that return each reply schema, found by scripts/map-schemas.mjs against the live sources.
const SCHEMA_CALLS = path.resolve('src/data/schema-calls.json');
const schemaCalls = fs.existsSync(SCHEMA_CALLS) ? JSON.parse(fs.readFileSync(SCHEMA_CALLS, 'utf8')) : {};
const schemaNotes = YAML.parse(fs.readFileSync(path.resolve('src/data/schema-notes.yaml'), 'utf8')) ?? {};

// ---------- one page per command ----------
const usage = (c) => {
  const pos = c.arguments.filter((a) => a.position !== undefined).sort((a, b) => a.position - b.position)
    .map((a) => (a.required ? `<${a.name}${a.value_type === 'array' ? '...' : ''}>` : `[${a.name}${a.value_type === 'array' ? '...' : ''}]`));
  const opt = c.arguments.some((a) => a.position === undefined) ? ['[OPTIONS]'] : [];
  return ['biomcp', ...c.command, ...opt, ...pos].join(' ');
};
for (const c of [...help, ...topics]) {
  const n = name(c);
  const args = [...c.arguments].sort((a, b) => (a.position ?? -1) - (b.position ?? -1) || a.name.localeCompare(b.name));
  const lines = [
    fm({ title: `biomcp ${n}`, description: `Reference for the ${site.product} ${site.version} command biomcp ${n}: ${firstSentence(c.description)}`.slice(0, 300), goal: `Look up the arguments, sections and examples of biomcp ${n}.`, reader: 'A BioMCP user or agent checking one command', sidebar: { label: n } }),
    `${site.product} ${site.version} command ${code('biomcp ' + n)}.`,
    '',
    esc(c.description),
    '',
    '## Usage',
    '',
    '```text',
    usage(c),
    '```',
    '',
  ];
  if (args.length) {
    lines.push('## Arguments', '', '| Argument | Type | Required | Limits | Choices | Description |', '| --- | --- | --- | --- | --- | --- |');
    for (const a of args) {
      const limits = [a.minimum !== undefined ? `min ${a.minimum}` : '', a.maximum !== undefined ? `max ${a.maximum}` : ''].filter(Boolean).join(', ');
      lines.push(`| ${code(a.name)}${a.position !== undefined ? ` (position ${a.position + 1})` : ''} | ${cell(a.value_type || 'value')} | ${a.required ? 'yes' : 'no'} | ${limits} | ${a.choices.map(code).join(', ')} | ${cell(a.description)} |`);
    }
    lines.push('');
  } else {
    lines.push('## Arguments', '', 'The help lists no arguments for this command.', '');
  }
  if (c.sections?.length) {
    lines.push('## Sections', '', esc(c.section_rule ?? ''), '', '| Section | Description |', '| --- | --- |');
    for (const s of c.sections) lines.push(`| ${code(s.word)} | ${cell(s.description)} |`);
    lines.push('');
  }
  const src = sourcesFor(c).filter((s) => canonical.has(s));
  if (src.length) {
    lines.push('## Sources', '', `${code('--source')} accepts these sources: ${src.map((s) => `[${s}](${srcRoute(s)})`).join(', ')}.`, '');
  }
  lines.push('## Examples', '');
  if (c.examples?.length) {
    lines.push('```sh', ...c.examples.map((e) => ['biomcp', ...e.map(quote)].join(' ')), '```', '');
  } else lines.push('The help lists no examples for this command.', '');
  const replies = schemaOf.get(n) ?? [];
  if (replies.length) {
    lines.push('## Reply schema', '', `${c.examples?.length ? 'The help examples of this command return' : 'This command returns'} ${replies.map((id) => `[${code(id)}](${schemaRoute(id)})`).join(', ')}. Print the full JSON schema with ${code('biomcp schema ' + replies[0])}.`, '');
  }
  lines.push('## Exit codes', '', `See [exit codes](/reference/error-codes/). Options that apply to every command are in [global options](/reference/global-options/).`, '');
  const e = ENTITY_OF(c);
  lines.push('## Next', '', entities.includes(e) ? `The [${e} entity](/user-guide/${e}/) page lists every ${e} command and links to a worked guide.` : 'The [commands index](/reference/commands/) lists every command.', '', genFooter);
  write(`commands/${slug(c)}.md`, lines.join('\n'));
}

// ---------- commands index ----------
{
  const groups = new Map();
  for (const c of help) {
    const g = entities.includes(ENTITY_OF(c)) ? ENTITY_OF(c) : 'housekeeping';
    if (!groups.has(g)) groups.set(g, []);
    groups.get(g).push(c);
  }
  const lines = [
    fm({ title: 'Commands', description: `Every ${site.product} ${site.version} command, grouped by entity, with its one-line description from the help.`, goal: 'Find the reference page for one BioMCP command.', reader: 'A BioMCP user or agent looking up a command' }),
    `${site.product} ${site.version} has ${help.length} commands in its help. Each links to its page with arguments, sections and examples.`,
    '',
  ];
  for (const [g, cs] of [...groups].sort(([a], [b]) => (a === 'housekeeping') - (b === 'housekeeping') || a.localeCompare(b))) {
    lines.push(`## ${g === 'housekeeping' ? 'Setup, keys, data and server' : g}`, '', '| Command | Description |', '| --- | --- |');
    for (const c of cs) lines.push(`| [${code('biomcp ' + name(c))}](${route(c)}) | ${cell(firstSentence(c.description))} |`);
    lines.push('');
  }
  lines.push('## Help topics', '', `${code('biomcp help')} does not list these topics. A reply names one in ${code('next')} when a source fails with reason ${code('proxy')} or ${code('tls')}.`, '', '| Topic | Description |', '| --- | --- |');
  for (const c of topics) lines.push(`| [${code('biomcp ' + name(c))}](${route(c)}) | ${cell(firstSentence(c.description))} |`);
  lines.push('', genFooter);
  write('commands/index.md', lines.join('\n'));
}

// ---------- reply schemas ----------
{
  const returnedBy = (id) => [...help, ...topics].filter((c) => (schemaOf.get(name(c)) ?? []).includes(id));
  for (const { id, schema } of replySchemas) {
    const defs = schema.$defs ?? {};
    const result = target(schema.properties.result, defs);
    const query = target(schema.properties.query, defs);
    const by = returnedBy(id);
    const calls = schemaCalls[id] ?? [];
    const note = schemaNotes[id];
    if (!by.length && !calls.length && !note) { console.error(`reference: no command returns ${id}; run npm run schema-calls, or explain why in src/data/schema-notes.yaml`); process.exit(1); }
    if (note && (by.length || calls.length)) { console.error(`reference: ${id} has a command now; remove it from src/data/schema-notes.yaml`); process.exit(1); }
    const commandOf = (args) => [...help, ...topics].filter((c) => c.command.every((w, i) => args[i] === w)).sort((a, b) => b.command.length - a.command.length)[0];
    const callLines = calls.map((a) => { const c = commandOf(a); return `- ${code(['biomcp', ...a.map(quote)].join(' '))}${c ? ` ([${code('biomcp ' + name(c))}](${route(c)}))` : ''}`; });
    const shown = new Set([result?.name, query?.name].filter(Boolean));
    const types = [...refsIn(schema.properties, defs)].filter((n) => !shown.has(n)).sort();
    linkTypes = new Map([...types.map((n) => [n, n.toLowerCase()]), ...(result ? [[result.name, `result-${result.name.toLowerCase()}`]] : []), ...(query ? [[query.name, `query-${query.name.toLowerCase()}`]] : [])]);
    const lines = [
      fm({ title: id, description: `The ${site.product} ${site.version} reply schema ${id}: its envelope fields, its result type${result ? ` ${result.name}` : ''} and the commands that return it.`, goal: `Look up the fields of a ${id} reply.`, reader: 'A developer or agent parsing BioMCP JSON', sidebar: { label: id } }),
      `${site.product} ${site.version} reply schema ${code(id)}. Every reply names its schema in ${code('schema')}. Print this one as JSON Schema with ${code('biomcp schema ' + id)}.`,
      '',
      by.length ? `These commands return it: ${by.map((c) => `[${code('biomcp ' + name(c))}](${route(c)})`).join(', ')}.` : calls.length ? `${calls.length > 1 ? 'These calls return' : 'This call returns'} it.` : note,
      '',
      ...(calls.length ? [by.length ? 'Calls that returned it:' : '', '', ...callLines, ''].filter((l, i) => i || l) : []),
      '## Envelope',
      '',
      esc(schema.description ?? ''),
      '',
      fieldTable(schema) ?? '',
      '',
      ...describeDef('Result', result),
      ...describeDef('Query', query),
      ...(types.length ? ['## Types', '', 'Every other type this schema uses.', '', ...types.flatMap((n) => describeType(n, defs[n]))] : []),
      `[Reply envelope](/reference/envelope/) gives the values of ${code('status')}, ${code('sources[].state')} and the other shared fields. [Reply schemas](/reference/schemas/) lists every schema.`,
      '',
      `This page is generated from ${code('biomcp schema ' + id)} in ${site.product} ${site.version}. Fix a wrong description in the schema, then regenerate.`,
    ];
    write(`schemas/${schemaSlug(id)}.md`, lines.join('\n'));
  }
  linkTypes = new Map();
  const lines = [
    fm({ title: 'Reply schemas', description: `Every reply schema in ${site.product} ${site.version}, with its result type and the commands that return it.`, goal: 'Find the JSON schema of a BioMCP reply and the commands that return it.', reader: 'A developer or agent parsing BioMCP JSON' }),
    `${site.product} ${site.version} has ${replySchemas.length} reply schemas. Every reply names one in its ${code('schema')} field, such as ${code('biomcp.gene_cards/4')}. ${code('biomcp schema ID')} prints one as JSON Schema, and ${code('biomcp version --verbose')} lists every schema ID, including the types the replies share.`,
    '',
    '| Schema | Result type | Returned by |',
    '| --- | --- | --- |',
    ...replySchemas.map(({ id, schema }) => {
      const cmds = [...new Set([...returnedBy(id).map(name), ...(schemaCalls[id] ?? []).map((a) => { const c = [...help, ...topics].filter((h) => h.command.every((w, i) => a[i] === w)).sort((x, y) => y.command.length - x.command.length)[0]; return c ? name(c) : null; }).filter(Boolean)])];
      return `| [${code(id)}](${schemaRoute(id)}) | ${target(schema.properties.result, schema.$defs ?? {})?.name ?? ''} | ${cmds.map((n) => { const c = [...help, ...topics].find((h) => name(h) === n); return `[${code(n)}](${route(c)})`; }).join(', ')} |`;
    }),
    '',
    `The commands come from running each command's help examples with ${code('--offline')}, and from live calls of every command the site's examples run, with each section of each ${code('get')} (${code('npm run schema-calls')}). Each schema page lists the calls that returned it.`,
    '',
    genFooter,
  ];
  write('schemas/index.md', lines.join('\n'));
}

// ---------- entities ----------
{
  const lines = [
    fm({ title: 'Entities and sections', description: `The ${entities.length} ${site.product} ${site.version} entities, their search and get commands, relation commands and section words.`, goal: 'Find which commands and sections each BioMCP entity has.', reader: 'A BioMCP user or agent choosing a command' }),
    `${site.product} ${site.version} has ${entities.length} entities. Each row lists the entity's commands and the section words its ${code('get')} command accepts.`,
    '',
    '| Entity | Commands | Sections of get |',
    '| --- | --- | --- |',
  ];
  for (const e of entities) {
    const cs = help.filter((c) => ENTITY_OF(c) === e);
    const get = help.find((c) => c.command[0] === 'get' && c.command[1] === e);
    lines.push(`| ${e} | ${cs.map((c) => `[${name(c)}](${route(c)})`).join('<br/>')} | ${(get?.sections ?? []).map((s) => code(s.word)).join(' ')} |`);
  }
  lines.push('', genFooter);
  write('entities.md', lines.join('\n'));
}

// ---------- sources ----------
// Key use: a source whose key is required reports reason no_key in an offline health check run
// without keys. Every other source that a key belongs to takes it as optional.
const KEY_VARS = keys.map((k) => k.environment);
const requiredKey = (() => {
  const env = Object.fromEntries(Object.entries(baseEnv).filter(([k]) => !KEY_VARS.includes(k)));
  return new Set(json(['--offline', 'health'], undefined, env).result.sources.filter((x) => x.reason === 'no_key').map((x) => x.source));
})();
const keyNotes = YAML.parse(fs.readFileSync(path.resolve('src/data/keys.yaml'), 'utf8'));
const keyOf = (s) => keys.find((k) => k.source === s || k.name === s || k.name === notes[s]?.key);
const keyUse = (s) => (requiredKey.has(s) ? 'Required' : keyNotes[keyOf(s)?.name]?.optional ?? 'Optional');
const commandsFor = (s) => help.filter((c) => sourcesFor(c).includes(s));
const mdx = (s) => String(s ?? '').split(/(`[^`]*`)/).map((part, i) => (i % 2 ? part : part.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/\{/g, '&#123;').replace(/\}/g, '&#125;'))).join('');
const mcell = (s) => mdx(s).replace(/\|/g, '\\|').replace(/\n+/g, ' ');
const pageTitle = (route) => {
  for (const ext of ['.mdx', '.md']) {
    const f = path.resolve('src/content/docs', route.replace(/^\/|\/$/g, '') + ext);
    if (fs.existsSync(f)) return YAML.parse(fs.readFileSync(f, 'utf8').split('---')[1]).title;
  }
  throw new Error(`reference: no page for guide ${route}`);
};
const exitOfExample = (id) => { const f = path.resolve('examples', id, 'exit'); return fs.existsSync(f) ? Number(fs.readFileSync(f, 'utf8').trim()) : 0; };
const sourceRows = [];
for (const r of registry) {
  const n = notes[r.source] ?? {};
  const label = n.name ?? r.source;
  const selectable = commandsFor(r.source);
  const reading = (n.commands ?? []).map((w) => help.find((c) => name(c) === w)).filter((c) => c && !selectable.includes(c));
  const key = keyOf(r.source);
  const use = key ? keyUse(r.source) : null;
  const exampleId = `src-${r.source}`;
  if (!fs.existsSync(path.resolve('examples', exampleId, 'run.sh'))) { console.error(`reference: source ${r.source} has no example; add examples/${exampleId}/run.sh`); process.exit(1); }
  if (!n.limits || !n.guide) { console.error(`reference: src/data/sources.yaml ${r.source} needs limits and guide`); process.exit(1); }
  sourceRows.push({ source: r.source, name: label, use: r.usage, key: key ? `${use}: ${key.name}` : '', keyUse: use, keyName: key?.name ?? null, terms: r.terms, terms_url: r.terms_url, route: srcRoute(r.source) });
  const lines = [
    fm({ title: n.name ? `${n.name} (${r.source})` : r.source, description: `How ${site.product} ${site.version} uses ${label}: what it reads, an example, the key, the use group, terms and limits.`, goal: `Find what BioMCP reads from ${label}, which commands reach it, its key, its terms and its limits.`, reader: 'A BioMCP user checking one data source', sidebar: { label: r.source } }),
    "import Example from '../../../components/Example.astro';",
    '',
    `${site.product} ${site.version} names this source ${code(r.source)}.${n.what ? ' ' + mdx(n.what) : ''}`,
    '',
    '| Field | Value |',
    '| --- | --- |',
    `| Source name | ${code(r.source)} |`,
    `| Use group | ${code(r.usage)} |`,
    `| Terms | ${mcell(r.terms)} |`,
    `| Terms page | [${r.terms_url}](${r.terms_url}) |`,
    `| Key | ${key ? `${use}. Set it with ${code('biomcp key set ' + key.name)} or the environment variable ${code(key.environment)}. See [${code('help key ' + key.name)}](/reference/commands/help-key-${key.name}/).` : 'None.'} |`,
    ...(n.bundle ? [`| Local bundle | ${code('biomcp sync ' + n.bundle)} |`] : []),
    '',
    '## Example',
    '',
    `The first line of output is the reply's status and source outcomes. The lines after it are a few fields of the result.`,
    '',
    `<Example id="${exampleId}" />`,
    '',
    ...(exitOfExample(exampleId) === 4 ? [`The source did not answer this run, so the reply's status is ${code('failed')} and BioMCP exits with code 4. [Limits](#limits) says what the source needs.`, ''] : []),
    '## Commands',
    '',
  ];
  const table = (cs) => ['| Command | Description |', '| --- | --- |', ...cs.map((c) => `| [${code('biomcp ' + name(c))}](${route(c)}) | ${mcell(firstSentence(c.description))} |`)];
  if (selectable.length) lines.push(`Pick this source with ${code('--source ' + r.source)} on these commands:`, '', ...table(selectable), '');
  if (reading.length) lines.push(`These commands read ${mdx(label)} as part of their reply. They do not take ${code('--source ' + r.source)}:`, '', ...table(reading), '');
  if (!selectable.length && !reading.length) lines.push(`No command takes ${code('--source ' + r.source)}. The example above shows the command that reads it.`, '');
  lines.push('## Limits', '', mdx(n.limits), '',
    '## Turn it off', '', `Leave this source out of one command with ${code('--disable-sources ' + r.source)}, or add ${code(JSON.stringify(r.source))} to ${code('disable_sources')} in ${code('config.json')}. The group ${code(r.usage)} switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).`, '',
    '## Next', '', `The task guide for ${mdx(label)} is [${mdx(pageTitle(n.guide))}](${n.guide}). [Data sources](/sources/) lists every source.`, '',
    genFooter);
  files.set(path.join(SOURCES_OUT, `${srcSlug(r.source)}.mdx`), lines.join('\n').replace(/\n{3,}/g, '\n\n').trimEnd() + '\n');
}
files.set(path.resolve('src/data/sources.json'), JSON.stringify(sourceRows, null, 2) + '\n');
{
  const groups = ['open', 'attribution', 'noncommercial', 'restricted'];
  const lines = [
    fm({ title: 'Data sources', description: `Every data source in ${site.product} ${site.version}, with its use group, publisher terms and key.`, goal: 'Find a BioMCP source, its terms, its key and the commands that read it.', reader: 'A BioMCP user choosing or checking sources' }),
    `${site.product} ${site.version} registers ${registry.length} sources. This table comes from ${code('biomcp --offline health')}, which reports the registry without sending requests. The [data sources guide](/sources/) explains use groups and how to turn sources off. Each source has its own page under [/sources/](/sources/).`,
    '',
    '| Source | Use group | Key | Terms |',
    '| --- | --- | --- | --- |',
    ...registry.map((r) => `| [${r.source}](${srcRoute(r.source)}) | ${r.usage} | ${keyOf(r.source) ? `${keyUse(r.source).split(';')[0].toLowerCase()}, ${code(keyOf(r.source).environment)}` : ''} | [${cell(r.terms)}](${r.terms_url}) |`),
    '',
    '## Counts by use group',
    '',
    '| Use group | Sources |',
    '| --- | --- |',
    ...groups.map((g) => `| ${code(g)} | ${registry.filter((r) => r.usage === g).length} |`),
    '',
    genFooter,
  ];
  write('data-sources.md', lines.join('\n'));
}

// ---------- global options ----------
write('global-options.md', [
  fm({ title: 'Global options', description: `Options that every ${site.product} ${site.version} command accepts: output format, disabled sources, configuration file, offline and fresh.`, goal: 'Look up the options shared by every BioMCP command.', reader: 'A BioMCP user or agent' }),
  `Every ${site.product} ${site.version} command accepts these options. They come from the command grammar's root options. The help gives them no descriptions, so the meanings below link to the guide that verified each one.`,
  '',
  '| Option | Value | Choices | Guide |',
  '| --- | --- | --- | --- |',
  ...globalOptions.map((o) => `| ${code(o.name)} | ${o.value ? code(o.value) : 'none'} | ${o.choices ? o.choices.split(', ').map(code).join(', ') : ''} | ${{ '--format': '[Results](/results/)', '--disable-sources': '[Data sources](/sources/)', '--config': '[Keys and configuration](/getting-started/api-keys/)', '--offline': '[Local data](/local-data/)', '--fresh': '[Local data](/local-data/)' }[o.name] ?? ''} |`),
  '',
  genFooter,
].join('\n'));

// ---------- exit codes ----------
const MEANING = {
  Success: 'The command finished with status `complete` or `partial`. Read the source outcomes: a partial result names the sources that failed or were skipped.',
  Absent: 'The command finished and the sources returned no matching record. Status `absent`. This is the answer for that source, query and filters, not proof that the thing does not exist.',
  Usage: 'The arguments were wrong, a selector is retired, or a pool cursor does not match its search. The error names a code such as `usage.arguments` and often the replacement command. Nothing was sent to a source.',
  Expired: 'A retained pool or snapshot expired. The reply carries the command that restarts the search.',
  Failed: 'No source the command needed answered: each failed or was skipped. Status `failed`. Each source outcome names its reason, such as `deadline_exceeded` or `rejected`.',
  Local: 'A local step failed: the configuration file, the cache, a saved file or writing the output. The error code is `local.failure`.',
};
write('error-codes.md', [
  fm({ title: 'Error and exit codes', description: `What each ${site.product} ${site.version} process exit code means, from 0 for a result to 5 for a local failure.`, goal: 'Find what a BioMCP exit code means and what to do next.', reader: 'A developer or agent scripting BioMCP' }),
  `${site.product} ${site.version} sets the process exit code from the reply's status. Scripts can branch on it without reading the output.`,
  '',
  '| Exit code | Status | Meaning |',
  '| --- | --- | --- |',
  ...exitCodes.map((e) => `| ${e.code} | ${e.status} | ${MEANING[e.status] ?? ''} |`),
  '',
  'Read [results](/results/) for the reply envelope and [troubleshooting](/troubleshooting/) for the common failures.',
  '',
  genFooter,
].join('\n'));

// ---------- MCP tools ----------
{
  const lines = [
    fm({ title: 'MCP server tools', description: `The three MCP tools that ${site.product} ${site.version} serves: biomcp, search and fetch, with their input schemas.`, goal: 'Look up the MCP tools, their arguments and descriptions.', reader: 'A developer connecting an MCP client to BioMCP' }),
    `${code('biomcp serve')} in ${site.product} ${site.version} answers ${code('tools/list')} with ${tools.length} tools. This page is generated from that reply. [Connect an agent](/getting-started/mcp-clients/) shows how to add the server to a client.`,
    '',
  ];
  for (const t of tools) {
    const props = Object.entries(t.inputSchema?.properties ?? {});
    lines.push(`## ${t.name}`, '', '| Argument | Type | Required |', '| --- | --- | --- |',
      ...props.map(([k, v]) => `| ${code(k)} | ${v.type}${v.items ? ` of ${v.items.type}` : ''} | ${(t.inputSchema.required ?? []).includes(k) ? 'yes' : 'no'} |`), '');
    if (t.annotations) lines.push(`Annotations: ${Object.entries(t.annotations).map(([k, v]) => `${code(k)} ${v}`).join(', ')}.`, '');
    lines.push('Description sent to the client:', '', '```text', t.description, '```', '');
  }
  lines.push(genFooter);
  write('mcp-server.md', lines.join('\n'));
}

// ---------- reply envelope: the enums every reply uses, from the JSON schemas ----------
const envelopeFacts = {};
{
  const defs = { ...json(['schema', 'biomcp.gene_cards/4'])['$defs'], ...json(['schema', 'biomcp.error/1'])['$defs'] };
  const HIDE = ['replay_miss'];
  const table = (d, head) => {
    const xs = d['x-enum-descriptions'] ?? [];
    return [`| ${head} | Meaning |`, '| --- | --- |', ...d.enum.map((v, i) => [v, (xs[i] ?? '').trim()]).filter(([v]) => !HIDE.includes(v)).map(([v, m]) => `| ${code(v)} | ${cell(m)} |`)].join('\n');
  };
  const list = (d) => d.enum.filter((v) => !HIDE.includes(v)).map(code).join(', ');
  // status and sources[].state: the schema gives the values, src/data/envelope.yaml their meanings.
  for (const [field, def] of [['status', defs.Status], ['state', defs.SourceState], ['kind', defs.StepKind]]) {
    const want = def.enum.filter((v) => !HIDE.includes(v));
    const have = Object.keys(meanings[field] ?? {});
    const missing = want.filter((v) => !have.includes(v));
    const extra = have.filter((v) => !want.includes(v));
    if (missing.length || extra.length) { console.error(`reference: src/data/envelope.yaml ${field} differs from the schema: missing ${missing.join(', ') || 'none'}, extra ${extra.join(', ') || 'none'}`); process.exit(1); }
  }
  const exitOf = (v) => exitCodes.find((e) => e.status.toLowerCase() === v)?.code ?? 0;
  envelopeFacts.status = defs.Status.enum.map((v) => ({ value: v, meaning: meanings.status[v], exit: exitOf(v) }));
  envelopeFacts.state = defs.SourceState.enum.filter((v) => !HIDE.includes(v)).map((v) => ({ value: v, meaning: meanings.state[v] }));
  envelopeFacts.kind = defs.StepKind.enum.map((v) => ({ value: v, meaning: meanings.kind[v] }));
  const meaningTable = (field) => [field === 'status' ? '| Status | Meaning | Exit code |' : `| ${field === 'kind' ? 'Kind' : 'State'} | Meaning |`, field === 'status' ? '| --- | --- | --- |' : '| --- | --- |',
    ...envelopeFacts[field].map((r) => `| ${code(r.value)} | ${cell(r.meaning)} |${field === 'status' ? ` ${r.exit} |` : ''}`)].join('\n');
  write('envelope.md', [
    fm({ title: 'Reply envelope', description: `The values a ${site.product} ${site.version} reply uses for status, source state, failure reason, use group, next-command kind and error code.`, goal: 'Look up the exact values of the fields every BioMCP reply carries.', reader: 'A developer or agent parsing BioMCP JSON' }),
    `Every ${site.product} ${site.version} reply is one JSON object. Data replies carry ${code('schema')}, ${code('status')}, ${code('query')}, ${code('result')}, ${code('sources')} and ${code('next')}, and pages add ${code('page')}. Errors carry ${code('schema')} ${code('biomcp.error/1')}, ${code('status')} and ${code('error')}. The values below come from the reply schemas; ${code('biomcp schema ID')} prints a full schema. [Results](/results/) explains how to read them.`,
    '',
    '## status',
    '',
    `${esc(defs.Status.description)} Values: ${list(defs.Status)}. Error replies use ${list(defs.ErrorStatus ?? { enum: [] }) || code('usage') + ' or ' + code('local')}.`,
    '',
    meaningTable('status'),
    '',
    '## sources[].state',
    '',
    `${esc(defs.SourceState.description)} Values: ${list(defs.SourceState)}.`,
    '',
    meaningTable('state'),
    '',
    '## sources[].reason',
    '',
    esc(defs.Reason.description),
    '',
    table(defs.Reason, 'Reason'),
    '',
    '## sources[].use',
    '',
    `${esc(defs.Use.description)} Values: ${list(defs.Use)}. [Data sources](/sources/) explains each group.`,
    '',
    '## problems[].code',
    '',
    `${esc(defs.ProblemCode?.description ?? '')} Values: ${defs.ProblemCode ? list(defs.ProblemCode) : ''}. Each problem also names its ${code('source')}, ${code('record')} and ${code('field')}.`,
    '',
    '## next[].kind',
    '',
    `${esc(defs.StepKind.description)} Values: ${list(defs.StepKind)}. Each entry's ${code('args')} is a complete command without the leading ${code('biomcp')}.`,
    '',
    meaningTable('kind'),
    '',
    '## error.code',
    '',
    `${esc(defs.ErrorCode.description)} Values: ${list(defs.ErrorCode)}. [Exit codes](/reference/error-codes/) maps each status to a process exit code.`,
    '',
    genFooter,
  ].join('\n'));
}

// ---------- 0.9 to 1.0 command map, from the parity ledger's generated table ----------
{
  const up = fs.readFileSync(path.resolve('../sdlc/planning/upgrade.md'), 'utf8');
  const block = up.split('<!-- upgrade-commands -->')[1].split('<!-- /upgrade-commands -->')[0];
  const rows = block.split('\n').filter((l) => l.startsWith('| `')).map((l) => l.split('|').slice(1, -1).map((c) => c.trim()));
  const counts = {};
  for (const r of rows) counts[r[1]] = (counts[r[1]] ?? 0) + 1;
  write('upgrade-map.md', [
    fm({ title: 'Command map from 0.9 to 1.0', description: `Every BioMCP 0.9 command and what ${site.product} ${site.version} does with it: ported or renamed, dropped, or deferred, with the options it drops.`, goal: 'Find what happened to one 0.9 command or option in BioMCP 1.0.', reader: 'A BioMCP 0.9 user moving scripts or agents to 1.0' }),
    `This map lists ${rows.length} ${site.product} 0.9 commands and their outcome in ${site.version}: ${Object.entries(counts).map(([k, v]) => `${v} ${k.toLowerCase()}`).join(', ')}. It is generated from the release's parity ledger. [Upgrading from 0.9](/upgrade-from-0-9/) explains the main changes.`,
    '',
    '| 0.9 command | Outcome in 1.0 | Dropped options | Deferred options |',
    '| --- | --- | --- | --- |',
    ...rows.map((r) => `| ${r[0]} | ${r[1]} | ${r[2]} | ${r[3]} |`),
    '',
    genFooter,
  ].join('\n'));
}

// ---------- reference index ----------
write('index.md', [
  fm({ title: 'Reference', description: `Generated reference for ${site.product} ${site.version}: commands, entities and sections, sources, global options, exit codes and MCP tools.`, goal: 'Find the generated reference page for a command, entity, source, option, exit code or MCP tool.', reader: 'A BioMCP user or agent looking up an exact fact' }),
  `The ${site.product} ${site.version} reference is generated from the program's own help and registry. It covers:`,
  '',
  `- [Commands](/reference/commands/): all ${help.length} commands, one page each.`,
  `- [Entities and sections](/reference/entities/): the ${entities.length} entities and their section words.`,
  `- [Sources](/reference/data-sources/): the ${registry.length} sources, their terms and keys.`,
  '- [Global options](/reference/global-options/): options every command accepts.',
  '- [Reply envelope](/reference/envelope/): statuses, source states, reasons, use groups and error codes.',
  '- [Exit codes](/reference/error-codes/): what each exit code means.',
  `- [Reply schemas](/reference/schemas/): the ${replySchemas.length} reply schemas and the commands that return them.`,
  '- [Command map from 0.9](/reference/upgrade-map/): what happened to each 0.9 command.',
  `- [MCP tools](/reference/mcp-server/): the ${tools.length} tools the server offers.`,
  '',
  `Run ${code('biomcp help')} for the same command data as JSON, or ${code('biomcp help get gene')} for one command. ${code('biomcp schema ID')} prints a reply's JSON schema, such as ${code('biomcp schema biomcp.gene_cards/4')}.`,
  '',
  genFooter,
].join('\n'));

// ---------- the command rows that hand-written pages render with <CommandTable /> ----------
files.set(path.resolve('src/data/commands.json'), JSON.stringify(help.map((c) => ({ command: name(c), entity: ENTITY_OF(c), route: route(c), description: firstSentence(c.description), schemas: schemaOf.get(name(c)) ?? [] })), null, 2) + '\n');

// ---------- facts for the guide pages ----------
const facts = { envelope: envelopeFacts, commands: help.length, entities: entities.length, entity_names: entities, sources: registry.length, tools: tools.length, keys: keys.map((k) => ({ source: k.name, environment: k.environment, config: k.config })) };
files.set(path.resolve('src/data/reference.json'), JSON.stringify(facts, null, 2) + '\n');

// ---------- the counts in site.json must match the build ----------
for (const [k, v] of Object.entries({ entities: entities.length, sources: registry.length, commands: help.length, mcp_tools: tools.length, keys: keys.length })) {
  if (site.counts?.[k] !== v) { console.error(`reference: site.json counts.${k} is ${site.counts?.[k]}, the build has ${v}`); process.exit(1); }
}

// The tools sentence in site.json names every tool the server lists.
for (const t of tools) if (!String(site.mcp_tools ?? '').includes(`\`${t.name}\``)) { console.error(`reference: site.json mcp_tools does not name the tool ${t.name}`); process.exit(1); }

// ---------- write or check ----------
const existing = (dir) => (fs.existsSync(dir) ? fs.readdirSync(dir, { recursive: true }).map((f) => path.join(dir, f)).filter((f) => fs.statSync(f).isFile()) : []);
if (check) {
  const diffs = [];
  for (const [f, text] of files) if (!fs.existsSync(f) || !same(fs.readFileSync(f, 'utf8'), text)) diffs.push(path.relative('.', f));
  for (const f of [...existing(OUT), ...existing(SOURCES_OUT).filter((f) => !f.endsWith('index.mdx'))]) if (!files.has(f)) diffs.push(`${path.relative('.', f)} (stale)`);
  if (diffs.length) {
    console.error(`reference: ${diffs.length} page(s) differ from the build; run npm run reference\n` + diffs.slice(0, 20).join('\n'));
    process.exit(1);
  }
  console.log(`reference: ${files.size} files match the build`);
} else {
  const before = new Map([...files.keys()].filter((f) => fs.existsSync(f)).map((f) => [f, fs.readFileSync(f, 'utf8')]));
  fs.rmSync(OUT, { recursive: true, force: true });
  for (const f of existing(SOURCES_OUT)) if (!f.endsWith('index.mdx')) fs.rmSync(f);
  for (const [f, text] of files) {
    fs.mkdirSync(path.dirname(f), { recursive: true });
    fs.writeFileSync(f, stamp(text, before.get(f) ?? null));
  }
  console.log(`reference: wrote ${files.size} files`);
}
