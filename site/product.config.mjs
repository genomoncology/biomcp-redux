// Everything that differs between the BioMCP and ThinkThen sites.
export default {
  name: 'BioMCP',
  site: 'https://biomcp.org',
  description: 'BioMCP is a command line tool and MCP server that searches and fetches biomedical data from public sources: genes, variants, trials, articles, drugs, diseases and more.',
  binary: 'biomcp',
  binDir: '../target/release',
  exampleMode: 'live',
  exampleEnv: {},
  exampleWorkers: 3,
  logo: { light: './src/assets/biomcp-mark-light.svg', dark: './src/assets/biomcp-mark-dark.svg', alt: 'BioMCP microscope mark' },
  favicon: '/favicon.svg',
  social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/genomoncology/biomcp' }],
  // The spine in reading order, for llms.txt. Slugs.
  spine: [
    '', 'install', 'quickstart', 'connect-an-agent', 'how-biomcp-works', 'results',
    'literature', 'clinical-trials', 'variants', 'genes-and-proteins', 'drugs-and-safety',
    'diseases-and-phenotypes', 'studies-and-cell-lines',
    'sources', 'keys-and-configuration', 'local-data', 'run-as-a-server', 'troubleshooting', 'upgrade-from-0-9', 'reference',
  ],
  sectionNames: { answers: 'Answers', reference: 'Reference', blog: 'Blog', articles: 'Articles', entities: 'Entities', guides: 'Guides' },
  sidebar: [
    { label: 'Start', items: ['install', 'quickstart', 'connect-an-agent'] },
    { label: 'Understand', items: ['how-biomcp-works', 'results'] },
    { label: 'Capabilities', items: ['literature', 'clinical-trials', 'variants', 'genes-and-proteins', 'drugs-and-safety', 'diseases-and-phenotypes', 'studies-and-cell-lines'] },
    { label: 'Set up and run', items: ['sources', 'keys-and-configuration', 'local-data', 'run-as-a-server', 'troubleshooting', 'upgrade-from-0-9'] },
    { label: 'Entities', collapsed: true, items: [{ autogenerate: { directory: 'entities' } }] },
    { label: 'Reference', collapsed: true, items: [{ autogenerate: { directory: 'reference' } }] },
    { label: 'Answers', link: '/answers/' },
  ],
};
