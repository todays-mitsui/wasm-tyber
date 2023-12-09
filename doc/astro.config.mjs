import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import remarkBreaks from 'remark-breaks';

// https://astro.build/config
export default defineConfig({
	base: '/doc/',
	outDir: '../dist/doc',
	integrations: [
		starlight({
			title: 'Tuber',
			social: {
				github: 'https://github.com/todays-mitsui/wasm-mogul',
			},
			customCss: [
				'./src/assets/custom.css',
			],
			sidebar: [
				{
					label: 'Guides',
					items: [
						// Each item here is one entry in the navigation menu.
						{ label: 'Example Guide', link: '/guides/example/' },
					],
				},
				{
					label: 'Reference',
					autogenerate: { directory: 'reference' },
				},
			],
			components: {
				Hero: './src/components/MyHero.astro',
			}
		}),
	],
  markdown: {
    remarkPlugins: [
      // 改行を <br> に変換する
      remarkBreaks,
    ],
    extendDefaultPlugins: true,
  }
});
