// Everything that differs between the BioMCP and ThinkThen sites.
export default {
  name: 'BioMCP',
  site: 'https://biomcp.org',
  description: 'BioMCP is a command line tool and MCP server that searches and fetches biomedical data from public sources: genes, variants, trials, articles, drugs, diseases and more.',
  binary: 'biomcp',
  binDir: '../target/release',
  // The command that prints the version (and the build commit, when the program reports one).
  versionArgs: ['version', '--verbose'],
  exampleMode: 'live',
  exampleEnv: {},
  exampleWorkers: 3,
  // SITE_EXAMPLE_CONFIG, when set, is copied here inside the examples' own config folder.
  exampleConfig: 'biomcp/config.json',
  // A reply with top-level status "failed" fails its example unless examples/<id>/expect-status names it.
  exampleFailedStatus: true,
  // Examples that synchronize a bundle run one at a time before the others start.
  exampleFirst: '\\bbiomcp sync\\b',
  // Keyless requests to some sources are rate limited; a failed example runs again after a pause.
  exampleRetries: 2,
  exampleRetryDelayMs: 20000,
  // Pages written by scripts/gen-reference.mjs and scripts/gen-answers.mjs, under src/content/docs.
  generatedPages: '^(reference|answers)(/|$)|^sources/(?!index\\.)',
  // Old addresses the build does not make; the publishing step must serve each. npm run check:publish fails while one is missing from dist/.
  hostFiles: {
    '/install.sh': "Publishing must copy the installer from release/install.sh in this repository to /install.sh; every install line on the site runs it.",
  },
  logo: { light: './src/assets/biomcp-mark-light.svg', dark: './src/assets/biomcp-mark-dark.svg', alt: 'BioMCP microscope mark' },
  favicon: '/favicon.svg',
  // The link-preview card; scripts/social-card.mjs renders it to public/social-card.png.
  socialCard: {
    mark: 'src/assets/biomcp-mark-dark.svg',
    markHeight: 150,
    background: '#110A1D',
    text: '#F5F1FA',
    muted: '#A89BBE',
    accent: '#CDB4FF',
    fonts: [
      { family: 'BMCP Fraunces', file: 'public/fonts/Fraunces-Bold.ttf', weight: 700 },
      { family: 'BMCP Noto', file: 'public/fonts/NotoSans-Regular.ttf', weight: 400 },
      { family: 'BMCP Mono', file: 'public/fonts/IoskeleyMono-Regular.woff2', weight: 400 },
    ],
    nameFont: "'BMCP Fraunces'",
    bodyFont: "'BMCP Noto'",
    monoFont: "'BMCP Mono'",
    tagline: 'A command line tool and MCP server for biomedical data from public sources: genes, variants, trials, articles and drugs.',
    command: 'biomcp get gene BRAF',
  },
  social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/genomoncology/biomcp' }],
  // The spine in reading order, for llms.txt. Slugs.
  spine: [
    '', 'getting-started/installation', 'getting-started/first-query', 'getting-started/mcp-clients', 'concepts/progressive-disclosure', 'results',
    'how-to/find-articles', 'how-to/find-trials', 'how-to/annotate-variants', 'genes-and-proteins', 'drugs-and-safety',
    'diseases-and-phenotypes', 'studies-and-cell-lines',
    'sources', 'getting-started/api-keys', 'local-data', 'getting-started/remote-http', 'troubleshooting', 'upgrade-from-0-9', 'reference',
  ],
  sectionNames: { answers: 'Answers', reference: 'Reference', blog: 'Blog', articles: 'Articles', 'user-guide': 'Entities', sources: 'Sources', 'how-to': 'How-to', 'getting-started': 'Getting started', concepts: 'Concepts' },
  sidebar: [
    { label: 'Start', items: ['getting-started/installation', 'getting-started/first-query', 'getting-started/mcp-clients'] },
    { label: 'Understand', items: ['concepts/progressive-disclosure', 'results'] },
    { label: 'Capabilities', items: ['how-to/find-articles', 'how-to/find-trials', 'how-to/annotate-variants', 'genes-and-proteins', 'drugs-and-safety', 'diseases-and-phenotypes', 'studies-and-cell-lines'] },
    { label: 'Set up and run', items: ['sources', 'getting-started/api-keys', 'local-data', 'getting-started/remote-http', 'troubleshooting', 'upgrade-from-0-9', 'changes'] },
    { label: 'Entities', collapsed: true, items: [{ autogenerate: { directory: 'user-guide' } }] },
    { label: 'Sources', collapsed: true, items: [{ autogenerate: { directory: 'sources' } }] },
    { label: 'Reference', collapsed: true, items: [{ autogenerate: { directory: 'reference' } }] },
    { label: 'How-to', collapsed: true, items: ['how-to/guide-workflows'] },
    { label: 'Answers', link: '/answers/' },
    { label: 'Blog', collapsed: true, items: [{ autogenerate: { directory: 'blog' } }] },
    { label: 'Articles', collapsed: true, items: [{ autogenerate: { directory: 'articles' } }] },
  ],
};
