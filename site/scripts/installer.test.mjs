import assert from 'node:assert/strict';
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
