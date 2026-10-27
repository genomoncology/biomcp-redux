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
// Glossary terms (src/data/glossary.json) a newcomer would not know: on each page, the first use
// in prose links to the page that defines the term, unless a link to that page comes first.
const glossary = JSON.parse(fs.readFileSync(path.resolve('src/data/glossary.json'), 'utf8')).terms;
function linkTerms(text) {
  const m = text.match(/^---\n[\s\S]*?\n---\n/);
  const head = m ? m[0] : '';
  const lines = text.slice(head.length).split('\n');
  for (const [term, g] of Object.entries(glossary)) {
    const pages = Array.isArray(g.page) ? g.page : [g.page];
    const re = new RegExp(g.pattern ?? `\\b${term}s?\\b`, 'i');
    let fence = false;
    for (let i = 0; i < lines.length; i++) {
      const l = lines[i];
      if (/^\s*(```|~~~)/.test(l)) { fence = !fence; continue; }
      if (fence || /^#/.test(l) || /^import /.test(l)) continue;
      // In a schema's field tables "entity" is usually a source's own word, not BioMCP's entity.
      if (term === 'entity' && /^\|/.test(l)) continue;
      // Prose only: leave inline code, link text and link targets alone.
      const parts = l.split(/(`[^`]*`|\[[^\]]*\]\([^)]*\))/);
      let done = false;
      for (let k = 0; k < parts.length && !done; k++) {
        if (pages.some((pg) => parts[k].includes(`](${pg}`))) { done = true; break; }
        if (k % 2) continue;
        const hit = re.exec(parts[k]);
        if (!hit) continue;
        parts[k] = parts[k].slice(0, hit.index) + `[${hit[0]}](${pages[0]})` + parts[k].slice(hit.index + hit[0].length);
        lines[i] = parts.join('');
        done = true;
      }
      if (done) break;
    }
  }
  return head + lines.join('\n');
}
const write = (rel, text) => files.set(path.join(OUT, rel), linkTerms(text).replace(/\n{3,}/g, '\n\n').trimEnd() + '\n');
const firstSentence = (s) => (s.split(/(?<=\.)\s/)[0] ?? s).trim();
const ENTITY_OF = (c) => (['search', 'get'].includes(c.command[0]) ? c.command[1] : c.command[0]);
const entities = [...new Set(help.filter((c) => ['search', 'get'].includes(c.command[0])).map((c) => c.command[1]))];
// Plain one-sentence descriptions of each command, in src/data/command-notes.yaml.
const commandNotes = YAML.parse(fs.readFileSync(path.resolve('src/data/command-notes.yaml'), 'utf8'))?.plain ?? {};
const plainOf = (c) => {
  const t = commandNotes[name(c)];
  if (!t) { console.error(`reference: src/data/command-notes.yaml has no plain sentence for ${name(c)}`); process.exit(1); }
  return t;
};
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

// ---------- reply schemas: where each type is defined ----------
// Every reply shares the envelope and many types. A type that more than one reply schema uses is
// defined once, on the shared types page; a type only one schema uses is defined on that schema's
// page. Types that differ only in the types they hold (Section of X, Capped of Y) are one generic
// type on the shared page; each schema page names what it holds.
const schemaSlug = (id) => id.replace(/[^A-Za-z0-9]+/g, '-').toLowerCase();
const schemaRoute = (id) => `/reference/schemas/${schemaSlug(id)}/`;
const TYPES_ROUTE = '/reference/schemas/types/';
const refName = (r) => r.split('/').pop();
const anchor = (n) => n.toLowerCase().replace(/[^a-z0-9]+/g, '-');
// The schema text's own field and type descriptions, with a plain gloss where src/data/schema-notes.yaml has one.
const schemaNotes = YAML.parse(fs.readFileSync(path.resolve('src/data/schema-notes.yaml'), 'utf8')) ?? {};
const GLOSS = schemaNotes.glosses ?? {};
const descText = (d) => String(d ?? '').replace(/```[\s\S]*?```/g, '').trim();
const GLOSS_PATTERNS = (schemaNotes.gloss_patterns ?? []).map((g) => [new RegExp(g.match), g.plain]);
const plainDesc = (d) => {
  const t = descText(d);
  if (GLOSS[t]) return GLOSS[t];
  for (const [re, plain] of GLOSS_PATTERNS) if (re.test(t)) return t.replace(re, plain);
  return t;
};
// Each $ref under a node, in order, without following definitions.
const directRefs = (node) => {
  const out = [];
  const walk = (n) => { if (!n || typeof n !== 'object') return; if (Array.isArray(n)) return n.forEach(walk); if (typeof n.$ref === 'string') out.push(refName(n.$ref)); for (const [k, v] of Object.entries(n)) if (k !== '$ref') walk(v); };
  walk(node);
  return out;
};
const typeHome = (() => {
  const shape = (d) => JSON.stringify(d, (k, v) => (k === '$ref' ? '#T' : v));
  const groups = new Map();
  const concrete = new Map();
  for (const { id, schema } of replySchemas) {
    for (const [n, d] of Object.entries(schema.$defs ?? {})) {
      const base = n.replace(/\d+$/, '');
      const gk = `${base}\u0000${shape(d)}`;
      if (!groups.has(gk)) groups.set(gk, { base, defs: new Map() });
      const json = JSON.stringify(d);
      groups.get(gk).defs.set(json, d);
      const ck = `${n}\u0000${json}`;
      if (!concrete.has(ck)) concrete.set(ck, { name: n, def: d, users: new Set(), group: gk });
      concrete.get(ck).users.add(id);
    }
  }
  // A generic group: one shape, more than one definition. Its parameters are the $ref positions
  // whose target differs between definitions.
  for (const g of groups.values()) {
    g.generic = g.defs.size > 1;
    if (!g.generic) continue;
    const lists = [...g.defs.values()].map(directRefs);
    g.positions = lists[0].map((_, i) => i).filter((i) => lists.some((l) => l[i] !== lists[0][i]));
  }
  const of = new Map();
  for (const c of concrete.values()) {
    const g = groups.get(c.group);
    const home = g.generic || c.users.size > 1 ? 'shared' : [...c.users][0];
    const params = g.generic ? [...new Set(g.positions.map((i) => directRefs(c.def)[i]))] : [];
    for (const id of c.users) of.set(`${id}\u0000${c.name}`, { ...c, home, generic: g.generic, display: g.generic ? g.base : c.name, params, groupDef: g.generic ? [...g.defs.values()][0] : c.def, positions: g.positions ?? [] });
  }
  return { of, shared: () => {
    const seen = new Map();
    for (const t of of.values()) if (t.home === 'shared' && !seen.has(t.display)) seen.set(t.display, t);
    return [...seen.values()].sort((a, b) => a.display.localeCompare(b.display));
  } };
})();
// The envelope page gives the values and meanings of these enums; the shared types page links there.
const ENVELOPE_ENUMS = { Status: 'status', SourceState: 'sourcesstate', Reason: 'sourcesreason', Use: 'sourcesuse', ProblemCode: 'problemscode', StepKind: 'nextkind' };

// A type expression as Markdown, with each named type linked to the page and anchor that defines it.
// `ctx` is the schema id (a schema page), or 'shared' (the shared types page); `T` names generic parameters.
function typeMd(p, ctx, id, params = new Map()) {
  if (!p || typeof p !== 'object') return 'any';
  if (p.$ref) return refMd(refName(p.$ref), ctx, id, params);
  if (p.const !== undefined) return code(JSON.stringify(p.const));
  if (p.enum) return p.enum.map((v) => code(JSON.stringify(v))).join(' or ');
  for (const k of ['anyOf', 'oneOf', 'allOf']) if (p[k]) return p[k].map((x) => typeMd(x, ctx, id, params)).join(k === 'allOf' ? ' and ' : ' or ');
  if (p.type === 'array') return `list of ${typeMd(p.items, ctx, id, params)}`;
  if (Array.isArray(p.type)) return p.type.map((t) => (t === 'array' ? `list of ${typeMd(p.items, ctx, id, params)}` : t)).join(' or ');
  if (p.type === 'object' && p.additionalProperties && typeof p.additionalProperties === 'object') return `map of ${typeMd(p.additionalProperties, ctx, id, params)}`;
  return p.type ? `${p.type}${p.format ? ` (${p.format})` : ''}` : 'any';
}
function refMd(n, ctx, id, params) {
  if (params.has(n)) return params.get(n);
  const t = typeHome.of.get(`${id}\u0000${n}`);
  if (!t) return n;
  const where = t.home === 'shared' ? (ctx === 'shared' ? '' : TYPES_ROUTE) : t.home === ctx ? '' : schemaRoute(t.home);
  const link = `[${t.display}](${where}#${anchor(t.display)})`;
  return t.generic ? `${link} of ${t.params.map((x) => refMd(x, ctx, id, params)).join(', ')}` : link;
}
// Rewrites the $ref at each parameter position (in directRefs order) to a placeholder name.
const withParams = (node, positions, letters) => {
  let i = 0;
  const walk = (n) => {
    if (!n || typeof n !== 'object') return n;
    if (Array.isArray(n)) return n.map(walk);
    const out = {};
    let ref;
    if (typeof n.$ref === 'string') { const k = positions.indexOf(i++); ref = k >= 0 ? `#/$defs/\u0000${letters[k]}` : n.$ref; }
    for (const [k, v] of Object.entries(n)) out[k] = k === '$ref' ? ref : walk(v);
    return out;
  };
  return walk(node);
};
// Text with each line wrapped at `width` characters, at spaces.
const wrapText = (text, width = 80) => String(text).split('\n').map((line) => {
  const indent = line.match(/^\s*(?:[-*] )?/)[0].replace(/[-*]/, ' ');
  const out = [];
  let cur = '';
  // A word longer than a line, such as a list of section words, breaks after a | or a comma.
  const pieces = line.split(' ').flatMap((w) => (w.length > width - indent.length ? w.split(/(?<=[|,])/) : [w]).map((p, i) => (i ? `\u0000${p}` : p)));
  for (const p of pieces) {
    const glued = p.startsWith('\u0000');
    const w = glued ? p.slice(1) : p;
    const joined = glued ? `${cur}${w}` : cur ? `${cur} ${w}` : w;
    if (cur && joined.length > width) { out.push(cur); cur = indent + w; } else cur = joined;
  }
  out.push(cur);
  return out.join('\n');
}).join('\n');
// A shell command as lines of at most `width` characters, broken before an option with \.
const wrapCommand = (words, width = 80) => {
  const tokens = [];
  for (const w of words) { const last = tokens.at(-1); if (last && /^--/.test(last) && !last.includes(' ') && !/^--/.test(w)) tokens[tokens.length - 1] = `${last} ${w}`; else tokens.push(w); }
  const lines = [];
  let cur = '';
  for (const t of tokens) {
    if (cur && `${cur} ${t}`.length > width - 2) { lines.push(`${cur} \\`); cur = `  ${t}`; } else cur = cur ? `${cur} ${t}` : t;
  }
  lines.push(cur);
  return lines.join('\n');
};
const fieldRows = (def, ctx, id, params) => {
  const props = Object.entries(def?.properties ?? {});
  const req = def?.required ?? [];
  return props.map(([k, v]) => `| ${code(k)} | ${cell(typeMd(v, ctx, id, params))} | ${req.includes(k) ? 'yes' : 'no'} | ${cell(plainDesc(v.description))} |`);
};
const FIELD_HEAD = ['| Field | Type | Always present | Meaning |', '| --- | --- | --- | --- |'];
// One type's definition: its meaning, then its fields, values or forms.
function describeType(name, def, ctx, id, params = new Map()) {
  const out = [`### ${name}`, ''];
  const d = plainDesc(def.description);
  if (d) out.push(cell(d), '');
  if (def.properties) out.push(...FIELD_HEAD, ...fieldRows(def, ctx, id, params), '');
  else if (def.enum) {
    const xs = def['x-enum-descriptions'] ?? [];
    if (xs.length) out.push('| Value | Meaning |', '| --- | --- |', ...def.enum.map((v, i) => `| ${code(v)} | ${cell(plainDesc(xs[i]))} |`), '');
    else out.push(`Values: ${def.enum.map((v) => code(v)).join(', ')}.`, '');
  } else if (def.oneOf ?? def.anyOf) {
    out.push('One of these forms:', '');
    for (const v of def.oneOf ?? def.anyOf) {
      if (v.properties) {
        const disc = Object.entries(v.properties).find(([, x]) => x.const !== undefined);
        const rest = Object.entries(v.properties).filter(([k]) => k !== disc?.[0]);
        const head = disc ? `${code(disc[0])} is ${code(JSON.stringify(disc[1].const))}` : 'An object';
        const fields = rest.map(([k, x]) => `${code(k)} (${typeMd(x, ctx, id, params)})`);
        out.push(`- ${head}: ${cell(plainDesc(v.description)) || 'no description.'}${fields.length ? ` Fields: ${fields.join(', ')}.` : ''}`);
      } else out.push(`- ${typeMd(v, ctx, id, params)}${v.description ? `: ${cell(plainDesc(v.description))}` : ''}`);
    }
    out.push('');
  } else out.push(`Type: ${typeMd(def, ctx, id, params)}.`, '');
  return out;
}

// One real reply as pretty JSON, cut so no line passes 80 characters: lists keep their first
// items and end with a "…" line, long text ends with "…", and deep values become {…}.
function sketch(reply, { width = 80, maxLines = 60 } = {}) {
  const isCut = (x) => x && typeof x === 'object' && !Array.isArray(x) && Object.keys(x).length === 1 && '…' in x;
  // One line of JSON, or null when the value holds a cut list.
  const compact = (v) => {
    if (v === null || typeof v !== 'object') return JSON.stringify(v);
    if (isCut(v)) return null;
    const parts = Array.isArray(v) ? v.map(compact) : Object.entries(v).map(([k, x]) => { const c = compact(x); return c === null ? null : `${JSON.stringify(k)}: ${c}`; });
    if (parts.includes(null)) return null;
    return Array.isArray(v) ? `[${parts.join(', ')}]` : `{${parts.join(', ')}}`;
  };
  const fitString = (s, room) => {
    if (JSON.stringify(s).length <= room) return JSON.stringify(s);
    let n = Math.max(4, room - 3);
    while (n > 4 && JSON.stringify(s.slice(0, n) + '…').length > room) n--;
    return JSON.stringify(s.slice(0, n).trimEnd() + '…');
  };
  const render = (items, depth) => {
    const lines = [];
    const emit = (v, ind, prefix, suffix, d) => {
      const pad = ' '.repeat(ind);
      const room = width - ind - prefix.length - suffix.length;
      if (typeof v === 'string') return lines.push(pad + prefix + fitString(v, room) + suffix);
      if (v === null || typeof v !== 'object') return lines.push(pad + prefix + JSON.stringify(v) + suffix);
      const c = compact(v);
      if (c !== null && c.length <= room && (c.length <= 50 || d >= depth - 1)) return lines.push(pad + prefix + c + suffix);
      const arr = Array.isArray(v);
      // A list of plain values that does not fit one line: several values to a line.
      if (arr && v.length && v.every((x) => isCut(x) || x === null || typeof x !== 'object')) {
        lines.push(pad + prefix + '[');
        const vals = v.filter((x) => !isCut(x)).map((x) => (typeof x === 'string' ? fitString(x, width - ind - 3) : JSON.stringify(x)));
        const left = v.filter(isCut).reduce((n, x) => n + x['…'], 0);
        let cur = '';
        vals.forEach((x, i) => {
          const item = x + (i < vals.length - 1 || left ? ',' : '');
          if (cur && `${cur} ${item}`.length > width - ind - 2) { lines.push(`${pad}  ${cur}`); cur = item; } else cur = cur ? `${cur} ${item}` : item;
        });
        if (cur) lines.push(`${pad}  ${cur}`);
        if (left) lines.push(`${pad}  … ${left} more`);
        return lines.push(pad + ']' + suffix);
      }
      const entries = arr ? v.map((x) => [null, x]) : Object.entries(v);
      if (!entries.length) return lines.push(pad + prefix + (arr ? '[]' : '{}') + suffix);
      if (d >= depth) return lines.push(pad + prefix + (arr ? '[…]' : '{…}') + suffix);
      lines.push(pad + prefix + (arr ? '[' : '{'));
      let left = 0;
      const shown = [];
      for (const [k, x] of entries) {
        if (isCut(x)) left += x['…'];
        else if (arr && shown.length >= items) left += 1;
        else shown.push([k, x]);
      }
      shown.forEach(([k, x], i) => emit(x, ind + 2, k === null ? '' : `${JSON.stringify(k)}: `, i < shown.length - 1 || left ? ',' : '', d + 1));
      if (left) lines.push(`${pad}  … ${left} more`);
      lines.push(pad + (arr ? ']' : '}') + suffix);
    };
    emit(reply, 0, '', '', 0);
    return lines;
  };
  for (const [items, depth] of [[2, 8], [1, 8], [1, 6], [1, 5], [1, 4], [1, 3]]) {
    const lines = render(items, depth);
    if (lines.length <= maxLines) return lines.join('\n');
  }
  const lines = render(1, 3);
  return [...lines.slice(0, maxLines), '…'].join('\n');
}
const schemaReplies = (() => { const f = path.resolve('src/data/schema-replies.json'); return fs.existsSync(f) ? JSON.parse(fs.readFileSync(f, 'utf8')) : {}; })();
// The commands that return each reply schema, found by scripts/map-schemas.mjs against the live sources.
const SCHEMA_CALLS = path.resolve('src/data/schema-calls.json');
const schemaCalls = fs.existsSync(SCHEMA_CALLS) ? JSON.parse(fs.readFileSync(SCHEMA_CALLS, 'utf8')) : {};
const noCommand = schemaNotes.no_command ?? {};

// ---------- one page per command ----------
const usage = (c) => {
  const pos = c.arguments.filter((a) => a.position !== undefined).sort((a, b) => a.position - b.position)
    .map((a) => (a.required ? `<${a.name}${a.value_type === 'array' ? '...' : ''}>` : `[${a.name}${a.value_type === 'array' ? '...' : ''}]`));
  const opt = c.arguments.some((a) => a.position === undefined) ? ['[OPTIONS]'] : [];
  return ['biomcp', ...c.command, ...opt, ...pos].join(' ');
};
const PRODUCT_LINE = `${site.product} ${site.version}, the command-line tool and MCP server that reads biomedical data sources`;
for (const c of [...help, ...topics]) {
  const n = name(c);
  const args = [...c.arguments].sort((a, b) => (a.position ?? -1) - (b.position ?? -1) || a.name.localeCompare(b.name));
  const plain = plainOf(c);
  const replies = schemaOf.get(n) ?? [];
  const parts = [args.length ? 'arguments' : null, c.sections?.length ? 'sections' : null, c.examples?.length ? 'examples' : null, replies.length ? 'reply' : null].filter(Boolean);
  const lines = [
    fm({ title: `biomcp ${n}`, description: `The ${site.product} ${site.version} command biomcp ${n}: ${plain}`.slice(0, 300), goal: `Look up the arguments, sections and examples of biomcp ${n}.`, reader: 'A BioMCP user or agent checking one command', sidebar: { label: n } }),
    `${code('biomcp ' + n)} is a command of ${PRODUCT_LINE}. It ${plain[0].toLowerCase()}${plain.slice(1)} ${c.sections?.length ? 'A [section word](/concepts/progressive-disclosure/) after the identifier adds one more part of the record to the reply. ' : ''}This page lists its ${parts.length > 1 ? `${parts.slice(0, -1).join(', ')} and ${parts.at(-1)}` : parts[0] ?? 'usage'}.`,
    '',
    'The program\'s help describes it:',
    '',
    ...esc(c.description).split('\n').map((l) => `> ${l}`),
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
      lines.push(`| ${code(a.name)}${a.position !== undefined ? ` (position ${a.position + 1})` : ''} | ${cell(a.value_type || 'value')} | ${a.required ? 'yes' : 'no'} | ${limits} | ${a.choices.map(code).join(', ')} | ${cell(plainDesc(a.description))} |`);
    }
    lines.push('');
  } else {
    lines.push('## Arguments', '', 'The command takes no arguments beyond those in its usage line.', '');
  }
  if (c.sections?.length) {
    lines.push('## Sections', '', esc(c.section_rule ?? ''), '', '| Section | Description |', '| --- | --- |');
    for (const s of c.sections) lines.push(`| ${code(s.word)} | ${cell(plainDesc(s.description))} |`);
    lines.push('');
  }
  const src = sourcesFor(c).filter((s) => canonical.has(s));
  if (src.length) {
    lines.push('## Sources', '', `${code('--source')} picks the source to ask: ${src.map((s) => `[${s}](${srcRoute(s)})`).join(', ')}.`, '');
  }
  if (c.examples?.length) {
    lines.push('## Examples', '', `The program's help gives ${c.examples.length > 1 ? 'these examples' : 'this example'}.`, '');
    for (const e of c.examples) lines.push('```sh', wrapCommand(['biomcp', ...e.map(quote)]), '```', '');
  }
  if (replies.length) {
    lines.push('## Reply', '', `The reply is one JSON object. ${replies.length > 1 ? 'Its schema is one of' : 'Its schema is'} ${replies.map((id) => `[${code(id)}](${schemaRoute(id)})`).join(', ')}, whose page shows a real reply and its fields.`, '');
  }
  const e = ENTITY_OF(c);
  lines.push('## Next', '', `[Exit codes](/reference/error-codes/) gives what each exit code means, and [global options](/reference/global-options/) the options every command takes. ${entities.includes(e) ? `The [${e} guide](/user-guide/${e}/) shows every ${e} command at work.` : 'The [commands index](/reference/commands/) lists every command.'}`);
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
    fm({ title: 'Commands', description: `Every ${site.product} ${site.version} command, grouped by entity, with what it does.`, goal: 'Find the reference page for one BioMCP command.', reader: 'A BioMCP user or agent looking up a command' }),
    `${PRODUCT_LINE[0].toUpperCase()}${PRODUCT_LINE.slice(1)}, has ${help.length} commands. They are grouped below by [entity](/reference/entities/), the kind of record they read, such as gene or trial. Each links to its page with arguments, sections and examples.`,
    '',
  ];
  for (const [g, cs] of [...groups].sort(([a], [b]) => (a === 'housekeeping') - (b === 'housekeeping') || a.localeCompare(b))) {
    lines.push(`## ${g === 'housekeeping' ? 'Setup, keys, data and server' : g}`, '', '| Command | Description |', '| --- | --- |');
    for (const c of cs) lines.push(`| [${code('biomcp ' + name(c))}](${route(c)}) | ${cell(plainOf(c))} |`);
    lines.push('');
  }
  lines.push('## Help topics', '', `${code('biomcp help')} does not list these topics. A reply names one in ${code('next')} when a source fails with reason ${code('proxy')} or ${code('tls')}.`, '', '| Topic | Description |', '| --- | --- |');
  for (const c of topics) lines.push(`| [${code('biomcp ' + name(c))}](${route(c)}) | ${cell(plainOf(c))} |`);
  lines.push('', '## Next', '', '[Command grammar](/concepts/progressive-disclosure/) explains how search, get, sections and relation commands fit together.');
  write('commands/index.md', lines.join('\n'));
}

// ---------- reply schemas ----------
{
  const all = [...help, ...topics];
  const returnedBy = (id) => all.filter((c) => (schemaOf.get(name(c)) ?? []).includes(id));
  const commandOf = (args) => all.filter((c) => c.command.every((w, i) => args[i] === w)).sort((a, b) => b.command.length - a.command.length)[0];
  // The command a call runs, with its section word when it reads one section of a get.
  const callCommand = (args) => {
    const c = commandOf(args);
    if (!c) return null;
    const section = c.command[0] === 'get' ? (c.sections ?? []).map((s) => s.word).find((w) => args.slice(c.command.length).includes(w)) : null;
    return { c, section };
  };
  const shownCommand = (x) => `biomcp ${name(x.c)}${x.section ? ` ${x.section}` : ''}`;
  const readable = (id) => id.replace(/^biomcp\./, '').replace(/\/\d+$/, '').replace(/_/g, ' ');
  const index = [];
  for (const { id, schema } of replySchemas) {
    const defs = schema.$defs ?? {};
    const by = returnedBy(id);
    const calls = schemaCalls[id] ?? [];
    const note = noCommand[id];
    if (!by.length && !calls.length && !note) { console.error(`reference: no command returns ${id}; run npm run schema-calls, or explain why in src/data/schema-notes.yaml no_command`); process.exit(1); }
    if (note && (by.length || calls.length)) { console.error(`reference: ${id} has a command now; remove it from src/data/schema-notes.yaml no_command`); process.exit(1); }
    // The commands, each once: the live calls first, then the help examples' commands.
    const cmds = [];
    for (const x of [...calls.map(callCommand).filter(Boolean), ...by.map((c) => ({ c, section: null }))]) if (!cmds.some((y) => shownCommand(y) === shownCommand(x))) cmds.push(x);
    const main = cmds[0];
    const title = main ? `Reply of ${shownCommand(main)}${main.section ? ' section' : ''} (${id})` : `${readable(id)[0].toUpperCase()}${readable(id).slice(1)} reply (${id})`;
    index.push({ id, cmds, title });
    const resultRef = directRefs(schema.properties.result)[0];
    const queryRef = directRefs(schema.properties.query)[0];
    const local = Object.keys(defs).filter((n) => typeHome.of.get(`${id}\u0000${n}`)?.home === id);
    const order = [...new Set([resultRef, queryRef, ...local.sort()])].filter((n) => n && local.includes(n));
    const ex = schemaReplies[id];
    const cmdList = cmds.map((x) => `[${code(shownCommand(x))}](${route(x.c)})`);
    const lines = [
      fm({ title, description: `The JSON reply ${site.product} ${site.version} prints for ${main ? shownCommand(main) : `no command (${readable(id)})`}: a real example and the fields of schema ${id}.`, goal: `Parse the ${id} reply: see a real example and what each field means.`, reader: 'A developer or agent parsing BioMCP JSON', sidebar: { label: id } }),
      `${site.product} ${site.version} prints one JSON object, the reply, for each command. ${main ? `${cmdList.length > 1 ? `${cmdList.slice(0, -1).join(', ')} and ${cmdList.at(-1)} reply` : `${cmdList[0]} replies`} with ${code('schema')} set to ${code(id)}.` : `The reply schema ${code(id)} is in the program, but no command returns it as a whole reply.`} Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](${TYPES_ROUTE}).`,
      '',
    ];
    if (!main) lines.push(note, '');
    if (calls.length) {
      lines.push('## Commands that return it', '', 'These commands returned this reply:', '', '```sh', ...calls.map((a) => wrapCommand(['biomcp', ...a.map(quote)])), '```', '');
      lines.push(`${code('biomcp schema ' + id)} prints the full JSON Schema.`, '');
    } else if (main) lines.push(`${code('biomcp schema ' + id)} prints the full JSON Schema.`, '');
    if (ex) {
      lines.push('## Example reply', '', `${code(['biomcp', ...ex.args.map(quote)].join(' '))} returned this reply on ${ex.ran}. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.`, '', '```json', sketch(ex.reply), '```', '');
      if (ex.reply.status === 'failed') lines.push(`The source did not answer that run, so ${code('status')} is ${code('failed')} and ${code('result')} is missing. A reply that succeeds fills ${code('result')} with the fields below.`, '');
    }
    lines.push('## Fields', '');
    const say = (field, n) => {
      if (!n) return null;
      const t = typeHome.of.get(`${id}\u0000${n}`);
      return `${code(field)} holds ${refMd(n, id, id, new Map())}${t?.home === id ? '' : ', a shared type'}.`;
    };
    lines.push([say('result', resultRef), say('query', queryRef)].filter(Boolean).join(' ') + ` ${code('query')} repeats the request as BioMCP read it.`, '');
    if (!order.length) lines.push('Every type this reply uses is shared with other replies, so its fields are on the shared types page.', '');
    for (const n of order) lines.push(...describeType(n, defs[n], id, id));
    lines.push('## Next', '', `[Reply envelope and shared types](${TYPES_ROUTE}) defines ${code('status')}, ${code('sources')}, ${code('next')} and every shared type. [Reply schemas](/reference/schemas/) lists every reply.${main ? ` [${code(shownCommand({ c: main.c, section: null }))}](${route(main.c)}) gives the command's arguments.` : ''}`);
    write(`schemas/${schemaSlug(id)}.md`, lines.join('\n'));
  }

  // The shared page: the envelope's fields, then every type more than one reply uses.
  {
    const env = replySchemas[0].schema;
    const envRows = Object.entries(env.properties).map(([k, v]) => {
      const t = k === 'result' ? 'the result type of each schema, or null' : k === 'query' ? 'the query type of each schema' : k === 'schema' ? 'string' : cell(typeMd(v, 'shared', replySchemas[0].id));
      return `| ${code(k)} | ${t} | ${(env.required ?? []).includes(k) ? 'yes' : 'no'} | ${cell(plainDesc(v.description) || (k === 'schema' ? 'The reply schema ID, such as `biomcp.gene_cards/4`.' : ''))} |`;
    });
    const words = Object.entries(schemaNotes.words ?? {}).map(([w, m]) => `| ${w} | ${cell(m)} |`);
    const lines = [
      fm({ title: 'Reply envelope and shared types', description: `The fields every ${site.product} ${site.version} JSON reply carries, and every type that more than one reply schema uses.`, goal: 'Look up an envelope field or a type that several BioMCP replies share.', reader: 'A developer or agent parsing BioMCP JSON', sidebar: { label: 'Shared types' } }),
      `${site.product} ${site.version} prints one JSON object, the reply, for each command. Every reply has the same outer fields, the **envelope**: ${code('schema')}, ${code('status')}, ${code('query')}, ${code('result')}, ${code('sources')} and ${code('next')}. This page defines those fields and every type that more than one reply uses. Each [reply schema](/reference/schemas/) page shows a real reply and the types only that reply has.`,
      '',
      '## Envelope',
      '',
      `${cell(plainDesc(env.description))} [Reply envelope values](/reference/envelope/) gives the meaning of each ${code('status')}, source state and reason.`,
      '',
      ...FIELD_HEAD,
      ...envRows,
      '',
      ...(words.length ? ['## Words in the field descriptions', '', 'The field descriptions come from the program and use a few words in a fixed sense:', '', '| Word | Meaning |', '| --- | --- |', ...words, ''] : []),
      '## Types',
      '',
      'Each type below appears in more than one reply. A generic type holds another type, named T; each schema page says what T is, as in "Section of SupplementContent".',
      '',
    ];
    for (const t of typeHome.shared()) {
      if (ENVELOPE_ENUMS[t.display]) {
        lines.push(`### ${t.display}`, '', `${cell(plainDesc(t.def.description))} [Reply envelope values](/reference/envelope/#${ENVELOPE_ENUMS[t.display]}) lists each value and its meaning.`, '');
        continue;
      }
      const params = new Map();
      if (t.generic) {
        const letters = t.positions.length > 1 ? t.positions.map((_, i) => `T${i + 1}`) : ['T'];
        const def = withParams(t.groupDef, t.positions, letters);
        for (const l of letters) params.set(`\u0000${l}`, l);
        const users = [...typeHome.of.values()].filter((x) => x.display === t.display);
        lines.push(...describeType(t.display, def, 'shared', [...users[0].users][0], params));
        continue;
      }
      lines.push(...describeType(t.display, t.def, 'shared', [...t.users][0], params));
    }
    lines.push('## Next', '', `[Reply schemas](/reference/schemas/) lists every reply and the commands that return it. [Results](/results/) explains how to read a reply from the command line.`);
    write('schemas/types.md', lines.join('\n'));
  }

  const lines = [
    fm({ title: 'Reply schemas', description: `Every reply schema in ${site.product} ${site.version}, with the commands that return it.`, goal: 'Find the JSON schema of a BioMCP reply and the commands that return it.', reader: 'A developer or agent parsing BioMCP JSON' }),
    `${site.product} ${site.version} prints one JSON object, the reply, for each command. Its ${code('schema')} field names the reply schema, such as ${code('biomcp.gene_cards/4')}, so code that parses replies can pick the right fields. Each schema below has a page with a real reply and its fields. The fields every reply shares are on [reply envelope and shared types](${TYPES_ROUTE}). ${code('biomcp schema ID')} prints one schema as JSON Schema.`,
    '',
    '| Schema | Returned by |',
    '| --- | --- |',
    ...index.map(({ id, cmds }) => `| [${code(id)}](${schemaRoute(id)}) | ${cmds.map((x) => `[${code(shownCommand(x).replace(/^biomcp /, ''))}](${route(x.c)})`).join(', ')} |`),
    '',
    `## Next`,
    '',
    `[Results](/results/) explains how to read a reply. [Commands](/reference/commands/) lists every command.`,
  ];
  write('schemas/index.md', lines.join('\n'));
}

// ---------- entities ----------
{
  const lines = [
    fm({ title: 'Entities and sections', description: `The ${entities.length} ${site.product} ${site.version} entities, their search and get commands, relation commands and section words.`, goal: 'Find which commands and sections each BioMCP entity has.', reader: 'A BioMCP user or agent choosing a command' }),
    `${PRODUCT_LINE[0].toUpperCase()}${PRODUCT_LINE.slice(1)}, groups its commands by entity. An **entity** is a kind of record, such as a gene, a trial or an article. BioMCP has ${entities.length}. Each row lists the entity's commands and the [section words](/concepts/progressive-disclosure/) its ${code('get')} command accepts. A section word asks for one more part of the record.`,
    '',
    '| Entity | Commands | Sections of get |',
    '| --- | --- | --- |',
  ];
  for (const e of entities) {
    const cs = help.filter((c) => ENTITY_OF(c) === e);
    const get = help.find((c) => c.command[0] === 'get' && c.command[1] === e);
    lines.push(`| ${e} | ${cs.map((c) => `[${name(c)}](${route(c)})`).join('<br/>')} | ${(get?.sections ?? []).map((s) => code(s.word)).join(' ')} |`);
  }
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
    `${mdx(label)} is one of the data sources of ${PRODUCT_LINE}.${n.what ? ' ' + mdx(n.what) : ''}${n.bundle || /bundle/i.test(n.what ?? '') ? ` A [bundle](/local-data/) is a local copy of the source that ${code('biomcp sync' + (n.bundle ? ' ' + n.bundle : ''))} downloads.` : ''} Replies name it ${code(r.source)}, and so do ${code('--source')} and ${code('--disable-sources')}.`,
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
    `The command pipes the reply through ${code('jq')} to keep its ${code('status')}, the state of each source it asked, and a few fields of the result. Leave out the ${code('jq')} step to see the whole reply.`,
    '',
    `<Example id="${exampleId}" />`,
    '',
    ...(exitOfExample(exampleId) === 4 ? [`The source did not answer this run, so the reply's status is ${code('failed')} and BioMCP exits with code 4. [Limits](#limits) says what the source needs.`, ''] : []),
    '## Commands',
    '',
  ];
  const table = (cs) => ['| Command | Description |', '| --- | --- |', ...cs.map((c) => `| [${code('biomcp ' + name(c))}](${route(c)}) | ${mcell(plainOf(c))} |`)];
  if (selectable.length) lines.push(`Pick this source with ${code('--source ' + r.source)} on these commands:`, '', ...table(selectable), '');
  if (reading.length) lines.push(`These commands read ${mdx(label)} as part of their reply. They do not take ${code('--source ' + r.source)}:`, '', ...table(reading), '');
  if (!selectable.length && !reading.length) lines.push(`No command takes ${code('--source ' + r.source)}. The example above shows the command that reads it.`, '');
  lines.push('## Limits', '', mdx(n.limits), '',
    '## Turn it off', '', `Leave this source out of one command with ${code('--disable-sources ' + r.source)}, or add ${code(JSON.stringify(r.source))} to ${code('disable_sources')} in ${code('config.json')}. The group ${code(r.usage)} switches off every source in that use group. See [keys and configuration](/getting-started/api-keys/).`, '',
    '## Next', '', `The task guide for ${mdx(label)} is [${mdx(pageTitle(n.guide))}](${n.guide}). [Data sources](/sources/) lists every source.`, '',
    );
  files.set(path.join(SOURCES_OUT, `${srcSlug(r.source)}.mdx`), linkTerms(lines.join('\n')).replace(/\n{3,}/g, '\n\n').trimEnd() + '\n');
}
files.set(path.resolve('src/data/sources.json'), JSON.stringify(sourceRows, null, 2) + '\n');
{
  const groups = ['open', 'attribution', 'noncommercial', 'restricted'];
  const lines = [
    fm({ title: 'Data sources', description: `Every data source in ${site.product} ${site.version}, with its use group, publisher terms and key.`, goal: 'Find a BioMCP source, its terms, its key and the commands that read it.', reader: 'A BioMCP user choosing or checking sources' }),
    `${PRODUCT_LINE[0].toUpperCase()}${PRODUCT_LINE.slice(1)}, reads ${registry.length} sources. Each belongs to one use group, a label for its terms: ${groups.map((g) => `${registry.filter((r) => r.usage === g).length} ${code(g)}`).join(', ')}. The table lists every source with its use group, key and publisher terms, as ${code('biomcp --offline health')} reports them without sending a request. The [data sources guide](/sources/) explains use groups and how to turn sources off, and each source links to its own page.`,
    '',
    '| Source | Use group | Key | Terms |',
    '| --- | --- | --- | --- |',
    ...registry.map((r) => `| [${r.source}](${srcRoute(r.source)}) | ${r.usage} | ${keyOf(r.source) ? `${keyUse(r.source).split(';')[0].toLowerCase()}, ${code(keyOf(r.source).environment)}` : ''} | [${cell(r.terms)}](${r.terms_url}) |`),
    '',
    '## Next',
    '',
    'The [data sources guide](/sources/) shows how to check that sources answer and how to turn a use group off.',
  ];
  write('data-sources.md', lines.join('\n'));
}

// ---------- global options ----------
write('global-options.md', [
  fm({ title: 'Global options', description: `Options that every ${site.product} ${site.version} command accepts: output format, disabled sources, configuration file, offline and fresh.`, goal: 'Look up the options shared by every BioMCP command.', reader: 'A BioMCP user or agent' }),
  `Every command of ${PRODUCT_LINE} accepts these options, placed before or after the command. Each row links to the guide that shows the option at work.`,
  '',
  '| Option | Value | Choices | Guide |',
  '| --- | --- | --- | --- |',
  ...globalOptions.map((o) => `| ${code(o.name)} | ${o.value ? code(o.value) : 'none'} | ${o.choices ? o.choices.split(', ').map(code).join(', ') : ''} | ${{ '--format': '[Results](/results/)', '--disable-sources': '[Data sources](/sources/)', '--config': '[Keys and configuration](/getting-started/api-keys/)', '--offline': '[Local data](/local-data/)', '--fresh': '[Local data](/local-data/)' }[o.name] ?? ''} |`),
  '',
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
  `${PRODUCT_LINE[0].toUpperCase()}${PRODUCT_LINE.slice(1)}, sets each command's exit code from the reply's ${code('status')}. A script can branch on the exit code without reading the output.`,
  '',
  '| Exit code | Status | Meaning |',
  '| --- | --- | --- |',
  ...exitCodes.map((e) => `| ${e.code} | ${e.status} | ${MEANING[e.status] ?? ''} |`),
  '',
  'Read [results](/results/) for the reply envelope and [troubleshooting](/troubleshooting/) for the common failures.',
  '',
].join('\n'));

// ---------- MCP tools ----------
{
  const lines = [
    fm({ title: 'MCP server tools', description: `The three MCP tools that ${site.product} ${site.version} serves: biomcp, search and fetch, with their input schemas.`, goal: 'Look up the MCP tools, their arguments and descriptions.', reader: 'A developer connecting an MCP client to BioMCP' }),
    `${code('biomcp serve')} runs ${PRODUCT_LINE}, as an MCP server. An agent that connects asks it for its tools with ${code('tools/list')}, and it answers with ${tools.length}, listed below with their arguments and the description each one sends. [Connect an agent](/getting-started/mcp-clients/) shows how to add the server to a client.`,
    '',
  ];
  for (const t of tools) {
    const props = Object.entries(t.inputSchema?.properties ?? {});
    lines.push(`## ${t.name}`, '', '| Argument | Type | Required |', '| --- | --- | --- |',
      ...props.map(([k, v]) => `| ${code(k)} | ${v.type}${v.items ? ` of ${v.items.type}` : ''} | ${(t.inputSchema.required ?? []).includes(k) ? 'yes' : 'no'} |`), '');
    if (t.annotations) lines.push(`Annotations: ${Object.entries(t.annotations).map(([k, v]) => `${code(k)} ${v}`).join(', ')}.`, '');
    lines.push('Description sent to the client, with its lines wrapped at 80 characters:', '', '```text', wrapText(t.description), '```', '');
  }
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
    `This map lists ${rows.length} ${site.product} 0.9 commands and their outcome in ${site.version}: ${Object.entries(counts).map(([k, v]) => `${v} ${k.toLowerCase()}`).join(', ')}. [Upgrading from 0.9](/upgrade-from-0-9/) explains the main changes.`,
    '',
    '| 0.9 command | Outcome in 1.0 | Dropped options | Deferred options |',
    '| --- | --- | --- | --- |',
    ...rows.map((r) => `| ${r[0]} | ${r[1]} | ${r[2]} | ${r[3]} |`),
    '',
  ].join('\n'));
}

// ---------- reference index ----------
write('index.md', [
  fm({ title: 'Reference', description: `The ${site.product} ${site.version} reference: commands, entities and sections, sources, global options, exit codes, reply schemas and MCP tools.`, goal: 'Find the reference page for a command, entity, source, option, exit code or MCP tool.', reader: 'A BioMCP user or agent looking up an exact fact' }),
  `The reference of ${PRODUCT_LINE} gives the exact facts behind each guide, taken from the program's own help and source list. It covers:`,
  '',
  `- [Commands](/reference/commands/): all ${help.length} commands, one page each.`,
  `- [Entities and sections](/reference/entities/): the ${entities.length} kinds of record and the parts each one can add.`,
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
].join('\n'));

// ---------- the command rows that hand-written pages render with <CommandTable /> ----------
files.set(path.resolve('src/data/commands.json'), JSON.stringify(help.map((c) => ({ command: name(c), entity: ENTITY_OF(c), route: route(c), description: plainOf(c), schemas: schemaOf.get(name(c)) ?? [] })), null, 2) + '\n');

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
