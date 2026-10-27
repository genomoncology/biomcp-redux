// Writes the files hosts and crawlers read at fixed addresses:
//   robots.txt     allows every crawler and names the sitemap
//   sitemap.xml    the same sitemap index as Astro's sitemap-index.xml, at the classic address
//   version.json   the version, the commit of this site's source, the program's build commit
//                  when it reports one, and the build time
// /install.sh is not written here; the publishing step places it (see site/README.md).
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import product from '../product.config.mjs';
import site from '../src/data/site.json' with { type: 'json' };
import { binaryInfo } from '../src/lib/binary.mjs';

const dist = path.resolve('dist');
const abs = (p) => new URL(p, product.site).href;

fs.writeFileSync(path.join(dist, 'robots.txt'), `User-agent: *\nAllow: /\n\nSitemap: ${abs('/sitemap.xml')}\n`);
fs.copyFileSync(path.join(dist, 'sitemap-index.xml'), path.join(dist, 'sitemap.xml'));

const git = (...a) => {
  const r = spawnSync('git', a, { encoding: 'utf8' });
  return r.status === 0 ? r.stdout.trim() : null;
};
const info = binaryInfo();
const commit = git('rev-parse', 'HEAD');
const version = {
  version: site.version,
  commit,
  short: commit ? commit.slice(0, 7) : null,
  branch: git('rev-parse', '--abbrev-ref', 'HEAD'),
  built: new Date().toISOString(),
  program: { version: info.version, commit: info.commit },
};
fs.writeFileSync(path.join(dist, 'version.json'), JSON.stringify(version, null, 2) + '\n');
console.log(`host files: robots.txt, sitemap.xml, version.json (${site.version}${info.commit ? `, program commit ${info.commit.slice(0, 7)}` : ''})`);
