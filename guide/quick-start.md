> For AI agents: the complete documentation index is available at https://sanjaiyan-dev.github.io/urai-ecma/llms.txt, the full documentation bundle is available at https://sanjaiyan-dev.github.io/urai-ecma/llms-full.txt.

# Quick Start விரைவு தொடக்கம்


v0.1.1 Active
Multi-Platform Binaries
\< 60s Setup
Zero Dependencies

Transform raw enterprise repositories into token-dense, architecturally intact Markdown prompts. Go from zero to your first synthesized LLM prompt in **4 simple steps**.

***

## ⚡ Installation Matrix

Select your target operating system or package ecosystem:


**🐧 macOS & Linux (Shell)**

```bash
# Downloads and installs prebuilt binary to /usr/local/bin or ~/.cargo/bin
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/sanjaiyan-dev/urai-ecma/releases/download/v0.1.1/urai-ecma-installer.sh | sh
```


**🪟 Windows (PowerShell)**

```powershell
# Installs prebuilt Windows x86_64 binary
powershell -ExecutionPolicy Bypass -c "irm https://github.com/sanjaiyan-dev/urai-ecma/releases/download/v0.1.1/urai-ecma-installer.ps1 | iex"
```


**📦 Node.js (npm / pnpm / bun)**


```sh [npm]
npm install -g urai-ecma
```

```sh [yarn]
yarn add -g urai-ecma
```

```sh [pnpm]
pnpm add -g urai-ecma
```

```sh [bun]
bun add -g urai-ecma
```

```sh [deno]
deno add -g npm:urai-ecma
```


**🦀 Rust (Cargo)**

```bash
# Build from source via crates.io with native CPU optimizations
cargo install urai-ecma
```


### Verify Installation

Run the version diagnostic in your terminal to confirm the SWC runtime is ready:

```bash
urai-ecma --version
# Output: urai-ecma 0.1.1
```

***

## 🚀 Step-by-Step Workflow


### Step 1: Scaffold Configuration
Run `urai-ecma create` in the root of your project directory:
```bash
urai-ecma create
```
This generates a commented `urai.config.jsonc` file formatted in **JSON5** (allowing comments and trailing commas):
```jsonc
{
    "$schema": "https://sanjaiyan-dev.github.io/urai-ecma/json-schema/v0/config.schema.json",
    
    // Path to the project directory or single source file
    "input_project": "./src",

    // Target output Markdown file path
    "output_file": "./output.md",

    // Ollama local endpoint URL (Optional, e.g., "http://localhost:11434")
    "ollama_endpoint": "http://localhost:11434",

    // Ollama Model Name (e.g., "gemma4", "ornith")
    "ollama_modelname": "gemma4",

    // Tailwind CSS / className pruning mode: "remove" | "remove_aggr" | "summarize" | "preserve"
    // "remove": strips static class strings exceeding threshold while keeping dynamic expressions.
    // "remove_aggr": aggressively removes class strings even if below character threshold.
    // "summarize": sends class strings exceeding threshold to Ollama for 1-line style descriptions.
    // "preserve": keeps classNames untouched.
    "tailwind_mode": "remove",

    // Character length threshold for Tailwind pruning (default: 96 characters)
    "tailwind_threshold": 96,

    // Summarize function block bodies using local Ollama or fallback to JSDoc comments
    "summarize_functions": true,

    // Line count threshold to trigger function summarization (default: 5 lines)
    "summarize_functions_threshold": 5,

    // Extract and generate Express/Fastify/Next.js/NestJS API Route Table
    "generate_route_table": true,

    // Analyze React / React Native components and output detailed explanations
    "analyze_react_components": true,

    // Generate ASCII File Structure & Module Dependency Graph
    "generate_file_graph": true
}
```

Precedence Hierarchy

CLI runtime flags always override settings in `urai.config.jsonc`. If no config file is detected, `urai-ecma` uses defaults.

***
### Step 2: Execute Codebase Synthesis
Run the compiler against your project directory:
```bash
# Using the active configuration file:
urai-ecma

# Or via explicit CLI flags:
urai-ecma -i ./src -o ./prompt.md --tailwind-mode remove
```
Watch the multi-threaded **Rayon + SWC engine** process your codebase:
```text
🚀 [urai-ecma] Starting AST Analysis on project: ./src
🔍 Found 48 source file(s) for analysis.
✅ [urai-ecma] Prompt successfully generated at: ./prompt.md
📊 [urai-ecma] Estimated Tokens in ./prompt.md: 24,190 tokens

============================================================
📊 TOKEN SAVINGS & OPTIMIZATION REPORT
============================================================
📁 Raw Source Code (All JS/TS):     132,450 tokens
⚡ Optimized Output (prompt.md):     24,190 tokens
------------------------------------------------------------
🎉 Reduction: -81.74% tokens saved! (Saved ~108,260 tokens)
============================================================
```
***
### Step 3: Inspect the Synthesized Prompt
Open `prompt.md`. Instead of noisy utility strings and imperative loops, you will find:
1. **`package.json` Project Architecture**: Version, dependencies, and stack overview.
2. **ASCII Project Hierarchy**: Clean directory layout honoring `.gitignore`.
3. **Mermaid Dependency Graph**: Dynamic import/export relationship maps.
4. **Backend API Route Table**: Auto-discovered endpoints (Express, Fastify, Next.js App Router, NestJS).
5. **React Component Breakdowns**: Typed props, hook side-effects, state variables, and handlers.
6. **AST-Pruned Source Code**: Core functions converted into structural stubs with dynamic JSX intact.
```markdown
## Backend API Route Table
| Framework | Method | Path | Handler | File Location |
| :--- | :--- | :--- | :--- | :--- |
| **Next.js** | `POST` | `/api/v1/checkout` | `POST` | `app/api/v1/checkout/route.ts:14` |
| **NestJS**  | `GET`  | `/users/:id` | `UserController::getProfile` | `src/controllers/user.ts:32` |

## React Component Breakdown: `<UserProfileCard>`
- **Props**: `user` (User), `onSelect` ((id: string) => void)
- **State**: Manages `isHovered` via setter `setIsHovered`
- **Hooks**: Uses `useState, useEffect` (Side-Effects: 1)
- **Rendered Tree**: `<div>, <Avatar>, <Badge>, <Button>`
```
***
### Step 4: Dispatch to Your Frontier LLM
Copy `prompt.md` into your LLM workflow:

💬 Claude 3.5 / 3.7 Sonnet

Upload <code>prompt.md</code> directly to Projects or chat. Prevents attention dilution and cuts prefill costs by \~80%.

🧠 ChatGPT (GPT-4o / o3)

Fits entire enterprise systems into prompt memory with zero rate-limit (TPM) throttling.

⚡ Cursor & Copilot

Reference <code>@prompt.md</code> in your agent context for high-precision, repo-wide refactors.

***

## 🛠️ Tactical Command Recipes

Copy-paste these CLI commands for common development workflows:

### Recipe A: Extreme Frontend Pruning (Design Systems & Tailwind)

Strips all static styling while preserving dynamic `clsx` and conditional logic:

```bash
urai-ecma -i ./apps/web -o ./frontend-prompt.md \
  --tailwind-mode remove_aggr \
  --tailwind-threshold 32 \
  --analyze-react-components
```

### Recipe B: Air-Gapped / Offline CI Pipeline (No Network, No Ollama)

Relies strictly on AST structural stubbing and existing JSDoc comments:

```bash
urai-ecma -i ./src -o ./ci-prompt.md \
  --summarize-functions=false \
  --tailwind-mode remove
```

### Recipe C: Local AI Semantic Mode (Using Ollama)

Summarizes complex function bodies into 1-sentence explanations using local neural models:

```bash
# Start your local Ollama daemon first:
ollama run gemma4

# Run urai-ecma targeting local Ollama:
urai-ecma -i ./src -o ./ai-prompt.md \
  --ollama-endpoint "http://localhost:11434" \
  --ollama-modelname "gemma4" \
  --summarize-functions-threshold 6
```

***

## 💡 Developer Pro-Tips

### 1. Zero-Config Git Awareness

`urai-ecma` uses Rust's `ignore::WalkBuilder` under the hood. It automatically respects your `.gitignore`, `.ignore`, and hidden files, while proactively pruning heavy directories (`node_modules`, `dist`, `build`, `.next`, `target`).

### 2. Persistent Foyer Hybrid Caching

When Ollama summarization is enabled, results are cached in `.urai-cache/` at your project root using **Sha512\_256** hash keys and **Zstd** block compression. Subsequent runs complete in **milliseconds** because identical functions skip LLM inference entirely.

```bash
# To clear cached AI function summaries:
rm -rf ./src/.urai-cache
```

***

## 🧭 Next Steps

Continue exploring the technical guides to get the most out of `urai-ecma`:


[✂️ Tailwind Optimization Modes

Compare <code>remove</code>, <code>remove\_aggr</code>, <code>summarize</code>, and <code>preserve</code> with real code samples.


](/guide/tailwind-pruning)[⚛️ React & Route Extraction

Learn how Next.js, Fastify, Express, and NestJS routing tables are automatically cataloged.


](/guide/react-and-routes)[🛠️ CLI Flags & Variables

Comprehensive reference of all CLI parameters, environment variables, and config keys.


](/reference/cli)