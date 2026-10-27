// The version and build commit the built program reports, read with product.versionArgs.
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import product from '../../product.config.mjs';

export function binaryInfo(env = { ...process.env, NO_COLOR: '1' }) {
  const bin = path.resolve(product.binDir, product.binary);
  const r = spawnSync(bin, product.versionArgs, { encoding: 'utf8', env, timeout: 30000 });
  const text = `${r.stdout ?? ''}${r.stderr ?? ''}`;
  const version = text.match(/\b(\d+\.\d+\.\d+(?:-[\w.]+)?)\b/)?.[1];
  if (r.status !== 0 || !version) throw new Error(`${bin} ${product.versionArgs.join(' ')} printed no version:\n${text}`);
  const commit = text.match(/^\s*commit:\s*([0-9a-f]{7,40})\b/im)?.[1] ?? null;
  return { version, commit, line: text.split('\n')[0].trim() };
}
