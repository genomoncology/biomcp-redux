import fs from 'node:fs';
import path from 'node:path';

const source = new URL('../../../release/install.sh', import.meta.url);

export function copyInstaller(dist) {
  fs.copyFileSync(source, path.join(dist, 'install.sh'));
}

export function checkInstaller(dist) {
  const file = path.join(dist, 'install.sh');
  if (!fs.existsSync(file)) return ['dist: install.sh missing'];
  return fs.readFileSync(file).equals(fs.readFileSync(source))
    ? [] : ['dist: install.sh differs from release/install.sh'];
}
