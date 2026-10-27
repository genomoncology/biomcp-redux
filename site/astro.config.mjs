// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import product from './product.config.mjs';
import redirects from './src/data/redirects.json' with { type: 'json' };

export default defineConfig({
  site: product.site,
  trailingSlash: 'always',
  // Old addresses that have no page of their own now. src/data/old-addresses.txt lists every old address; scripts/check.mjs fails if one no longer resolves.
  redirects: Object.fromEntries(Object.entries(redirects).map(([from, to]) => [from.replace(/\/$/, ''), to])),
  integrations: [
    starlight({
      title: product.name,
      description: product.description,
      logo: product.logo,
      favicon: product.favicon,
      social: product.social,
      customCss: ['./src/styles/brand.css'],
      pagination: false,
      disable404Route: true,
      lastUpdated: false,
      sidebar: product.sidebar,
      components: {
        Head: './src/components/Head.astro',
      },
      head: product.head ?? [],
    }),
  ],
});
