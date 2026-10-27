// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import product from './product.config.mjs';

export default defineConfig({
  site: product.site,
  trailingSlash: 'always',
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
