import * as path from 'node:path';
import { defineConfig } from '@rspress/core';

const SITE_URL = 'https://sanjaiyan-dev.github.io/urai-ecma';

export default defineConfig({
  root: path.join(__dirname, 'docs'),
  globalStyles: path.join(__dirname, 'styles/index.css'),
  siteOrigin: 'https://sanjaiyan-dev.github.io',
  base: '/urai-ecma/',
  title: 'Urai Ecma',
  description: 'A multilingual Rspress documentation site.',
  lang: 'en',
  icon: '/urai-icon.png',
  logo: '/urai.jpg',
  logoText: 'URAI (உரை)',
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

  builderConfig: {
    output: {
      assetPrefix: '/urai-ecma/',
    },
    html: {
      tags: [
        // Open Graph Meta Tags
        {
          tag: 'meta',
          attrs: { property: 'og:site_name', content: 'URAI Documentation' },
        },
        { tag: 'meta', attrs: { property: 'og:type', content: 'website' } },
        { tag: 'meta', attrs: { property: 'og:url', content: `${SITE_URL}/` } },
        {
          tag: 'meta',
          attrs: {
            property: 'og:title',
            content: 'URAI (உரை) | AST-Aware LLM Prompt Engine',
          },
        },
        {
          tag: 'meta',
          attrs: {
            property: 'og:description',
            content:
              'Compiler that compresses JavaScript and TypeScript repositories into token-optimized LLM prompts without semantic loss.',
          },
        },
        {
          tag: 'meta',
          attrs: { property: 'og:image', content: `${SITE_URL}/urai-og.jpg` },
        },
        // Twitter Cards
        {
          tag: 'meta',
          attrs: { name: 'twitter:card', content: 'summary_large_image' },
        },
        {
          tag: 'meta',
          attrs: { name: 'twitter:image', content: `${SITE_URL}/urai.jpg` },
        },
        // Schema.org SoftwareApplication JSON-LD
        {
          tag: 'script',
          attrs: { type: 'application/ld+json' },
          children: JSON.stringify({
            '@context': 'https://schema.org',
            '@type': 'SoftwareApplication',
            name: 'URAI',
            alternateName: 'உரை',
            applicationCategory: 'DeveloperApplication',
            operatingSystem: 'Cross-platform',
            isAccessibleForFree: true,

            disambiguatingDescription:
              'A high-performance Rust CLI compiler that compresses JavaScript and TypeScript codebases into token-efficient LLM context prompts.',

            abstract:
              'AST-aware codebase-to-prompt compiler reducing full-stack JS/TS token consumption by up to 80% through syntax pruning and local AI summarization.',

            description:
              'URAI (உரை) is a high-throughput static analysis compiler written in Rust that transforms full-stack JavaScript and TypeScript repositories into hyper-dense, token-optimized LLM prompts. By combining SWC AST visitors, aggressive Tailwind CSS pruning, Foyer hybrid 2-tier disk/memory caching, and local Ollama semantic summarization, URAI eliminates up to 80% of boilerplate context tokens while preserving structural APIs, types, and architectural topologies.',

            keywords:
              'AST, LLM prompt optimizer, token reduction, context window compression, Tailwind CSS pruner, SWC, TypeScript static analysis, Ollama prompt compressor, Rust CLI',

            featureList: [
              'AST-level static Tailwind CSS utility pruning (remove, summarize, aggressive, preserve)',
              'Semantic function and class method summarization via local Ollama inference or zero-latency JSDoc extraction',
              'Preservation of structural stubs: nested functions, React hooks, event listeners, and JSX return trees',
              'Automated route extraction for Express, Fastify, Next.js App Router, and NestJS controllers',
              'React component architecture inspection: props, state variables, hooks, and rendered JSX topologies',
              'Petgraph-powered ASCII tree and Mermaid module dependency graph generation',
              'Foyer hybrid 2-tier memory and disk cache with Zstd block compression and Sha512-256 content addressing',
              "Integrated prompt token benchmarking using OpenAI's o200k_base tiktoken tokenizer",
            ],

            softwareRequirements:
              'Optional: Ollama daemon (http://localhost:11434) for offline local function summarization',
            url: 'https://sanjaiyan-dev.github.io/urai-ecma/',
            codeRepository: 'https://github.com/sanjaiyan-dev/urai-ecma',
            license: 'https://opensource.org/licenses/MIT',
            offers: {
              '@type': 'Offer',
              price: '0',
            },
          } as const),
        },
      ],
    } as const,
  },
});
