import staticAdapter from "@sveltejs/adapter-static";
import { vitePreprocess } from '@sveltejs/kit/vite';
import path from 'path';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: staticAdapter(),
    alias: {
      '@components': path.resolve('./src/components'),
      '@stores': path.resolve('./src/stores'),
    }
  },
  preprocess: vitePreprocess()
};

export default config;
