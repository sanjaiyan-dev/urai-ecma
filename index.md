> For AI agents: the complete documentation index is available at https://sanjaiyan-dev.github.io/urai-ecma/llms.txt, the full documentation bundle is available at https://sanjaiyan-dev.github.io/urai-ecma/llms-full.txt.

# urai-ecma

AST-Aware Code-to-Prompt Engine


> Strip styling bloat, summarize method bodies, and feed complete JS/TS codebases into LLMs with up to 80% token reduction.

[Get Started](/guide/quick-start) | [Explore Features](/guide/introduction) | [View on GitHub](https://github.com/sanjaiyan-dev/urai-ecma)

## Features

- [⚡ **Multi-Threaded SWC Traversal**](/guide/introduction#swc-pipeline): Built on Rust's SWC AST parser and Rayon. Concurrently parses and optimizes large monorepos in milliseconds.
- [🎨 **Smart Tailwind CSS Pruning**](/guide/tailwind-pruning): Strips static styling strings while strictly preserving dynamic clsx, cn(), and template literal logic.
- [🧱 **Structural Stub Retention**](/guide/function-summarization#structural-stubs): Retains essential statements (React hooks, JSX returns, event listeners) while pruning internal function bloat.
- [⚛️ **React Component Inspector**](/guide/react-analysis): Extracts typed props, useState variables, effect counters, and hierarchy into clean Markdown summaries.
- [🦙 **JSDoc & Local Ollama AI**](/guide/function-summarization#jsdoc-first): Prioritizes instant JSDoc extraction. Falls back to local Ollama models with persistent disk caching.
- [📊 **Built-in Tiktoken Analytics**](/guide/quick-start#token-savings-report): Real-time before-and-after token savings report calculated via OpenAI o200k_base and Llama 3 BPE.
