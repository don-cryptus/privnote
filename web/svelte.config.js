import { vitePreprocess } from '@sveltejs/kit/vite';
import adapaterNode from '@sveltejs/adapter-node';
import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';

const file = fileURLToPath(new URL('package.json', import.meta.url));
const json = readFileSync(file, 'utf8');
const pkg = JSON.parse(json);

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: [vitePreprocess({})],
  kit: {
    version: {
      name: pkg.version
    },
    adapter: adapaterNode(),
    csrf: {
      checkOrigin: false
    }
  }
};

export default config;
