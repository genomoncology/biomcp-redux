import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { copyInstaller, checkInstaller } from '../src/lib/installer.mjs';

test('the built site requires the exact release installer', () => {
  const dist = fs.mkdtempSync(path.join(os.tmpdir(), 'biomcp-site-installer-'));
  copyInstaller(dist);
  assert.deepEqual(checkInstaller(dist), []);
  fs.writeFileSync(path.join(dist, 'install.sh'), 'changed');
  assert.deepEqual(checkInstaller(dist), ['dist: install.sh differs from release/install.sh']);
  fs.unlinkSync(path.join(dist, 'install.sh'));
  assert.deepEqual(checkInstaller(dist), ['dist: install.sh missing']);
});

const builtInstaller = new URL('../dist/install.sh', import.meta.url);
test('the site checker rejects a built site without its installer', {
  skip: !fs.existsSync(builtInstaller),
}, () => {
  const backup = new URL('../dist/install.sh-test-backup', import.meta.url);
  fs.renameSync(builtInstaller, backup);
  try {
    const result = spawnSync(process.execPath, ['scripts/check.mjs'], {
      cwd: new URL('../', import.meta.url), encoding: 'utf8',
    });
    assert.notEqual(result.status, 0);
    assert.match(result.stdout + result.stderr, /dist: install\.sh missing/);
  } finally {
    fs.renameSync(backup, builtInstaller);
  }
});
