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
  spine: ['', 'quickstart'],
  sectionNames: { answers: 'Answers', reference: 'Reference', blog: 'Blog' },
  sidebar: [
    { label: 'Start', items: ['quickstart'] },
  ],
};
