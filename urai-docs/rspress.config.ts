import * as path from 'node:path';
import { defineConfig } from '@rspress/core';

export default defineConfig({
  root: path.join(__dirname, 'docs'),
  title: 'Urai Ecma',
  description: 'A multilingual Rspress documentation site.',
  lang: 'en',
  icon: '/urai-icon.png',
  logo: '/urai.jpg',
  themeConfig: {
    socialLinks: [
      {
        icon: 'github',
        mode: 'link',
        content: 'https://github.com/sanjaiyan-dev/urai-ecma',
      },
    ],
    enableContentAnimation: true,
    enableAppearanceAnimation: true,
  },
  route: {
    useTransitions: true,
  },
  llms: true,
});
