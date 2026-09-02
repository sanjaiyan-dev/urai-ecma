
<div align="center">

# 🏛️ URAI (உரை)

### **AST-Aware JS/TS Codebase-to-Prompt Engine for LLMs**

*Transform bloated JavaScript & TypeScript repositories into hyper-dense, token-optimized context prompts.*


![Rust](https://img.shields.io/badge/Language-Rust-orange?logo=rust&style=for-the-badge)
[![Parser: SWC](https://img.shields.io/badge/Parser-SWC_ECMA-yellow.svg?style=for-the-badge&logo=swc)](https://swc.rs/)
[![Parallelism: Rayon](https://img.shields.io/badge/Concurrency-Rayon-blue.svg?style=for-the-badge)](https://github.com/rayon-rs/rayon)
![Ollama](https://img.shields.io/badge/AI_Engine-Ollama-white?logo=ollama&style=for-the-badge)
[![Tokenizer: Tiktoken](https://img.shields.io/badge/Tokens-tiktoken_o200k-green.svg?style=for-the-badge&logo=openai)](https://github.com/openai/tiktoken)
[![License: MIT](https://img.shields.io/badge/License-MIT-purple.svg?style=for-the-badge)](LICENSE)

<br />

<img src="./assets/hero-urai.png" alt="Urai Architecture Banner" width="700" style="border-radius: 12px; box-shadow: 0 8px 32px rgba(0,0,0,0.25);" />

<br />

---

## 🏛️ The Name Inspiration: The Art of *"உரை எழுதுதல்"* (Urai Ezhuthudhal)

In classical Tamil literary heritage, monumental epics and ancient treatises—such as the *Thirukkuṛaḷ*, *Tolkāppiyam*, and *Cilappatikāram*—span vast volumes of dense, poetic, and complex thought. To make these monumental texts intelligible without losing their depth, classical scholars (*Uraiyāsiriyars*) practiced **உரை எழுதுதல் (*Urai Ezhuthudhal*)**: the disciplined art of writing a lucid, structured, and insightful commentary that distills the core essence, syntax, and architectural meaning of vast literature.


### The Modern Parallel
Today, enterprise JavaScript and TypeScript codebases are the **epic literatures of modern software**. Spanning thousands of files across Next.js, React, Node.js, and TypeScript, they are laden with boilerplate, repetitive utility classes, and nested syntax.

When feeding these systems to Large Language Models:
* **The LLM does not need raw syntactic exhaustion**—it needs the structural anatomy, the API contracts, the state flows, and the architectural intent.
* **`urai-ecma` acts as the modern *Uraiyāsiriyar***: it reads your massive JS/TS/TSX codebase through its Abstract Syntax Tree (AST), strips the repetitive noise, captures component signatures and route tables, and writes a pristine, authoritative **"உரை" (Prompt Commentary)** engineered specifically for AI reasoning.

---




<br/>

> **“உரை” (*Urai*)** in classical Tamil translates to *commentary, exposition, reasoned narrative, or discourse*. 
> 
> **urai-ecma** translates your entire JavaScript, TypeScript, and React codebases into an information-dense, noise-free Markdown prompt engineered specifically for LLM context windows (GPT-4o, Claude 3.5 Sonnet, Gemini 1.5 Pro, Llama 3).

<br />

[**Key Features**](#-key-features) •
[**Benchmark & Token Savings**](#-token-savings--benchmark) •
[**Quick Start**](#-quick-start) •
[**Tailwind Pruning**](#-tailwind-css-optimization-modes) •
[**Configuration**](#-configuration-uraiconfigjsonc) •
[**Architecture**](#-architecture--pipeline)

</div>

---

## ⚡ Why URAI?

Modern AI context windows are large, but feeding raw repositories into LLMs introduces three critical bottlenecks:

1. **Massive Token Waste**: In modern frontend apps, repetitive Tailwind utility strings (`className="..."`) and verbose internal implementation details often constitute **50%–70% of total tokens**.
2. **Context Degradation & Hallucinations**: LLMs lose track of overarching system architecture when drowning in hundreds of lines of mechanical loops and styling classes.
3. **Escalating API Costs**: Developers pay for every token ingested. Sending 150k raw tokens costs significantly more and runs noticeably slower than sending a curated 25k architectural prompt.

**`urai-ecma` solves this at the compiler level.** Powered by the blazing-fast **SWC** Rust parser, it analyzes Abstract Syntax Trees (AST), prunes CSS bloat, converts internal logic into structural stubs, extracts backend routes and React component signatures, and summarizes functions offline using local Ollama instances.

---

## 📊 Token Savings & Benchmark

Every execution prints an instant BPE telemetry report comparing raw files against your generated prompt using OpenAI's **`o200k_base`** and Meta's **`llama3`** encodings:

```text
============================================================
📊 TOKEN SAVINGS & OPTIMIZATION REPORT
============================================================
📁 Raw Source Code (All JS/TS):     148,290 tokens
⚡ Optimized Output (output.md):     28,410 tokens
------------------------------------------------------------
🎉 Reduction: -80.84% tokens saved! (Saved ~119,880 tokens)
============================================================
```

### The Difference: Raw Code vs. URAI Optimized Prompt

#### ❌ Before Urai (Noisy, Token-Exhaustive)
```tsx
export function UserProfileCard({ user, onSelect }: ProfileCardProps) {
  const [isHovered, setIsHovered] = useState(false);
  useEffect(() => {
    trackImpression(user.id);
  }, [user.id]);

  // 40 lines of heavy validation, event binding, and formatting calculations
  const formattedDate = new Intl.DateTimeFormat('en-US').format(new Date(user.createdAt));
  const initials = user.name.split(' ').map(n => n[0]).join('').toUpperCase();
  const handleCardClick = (e: React.MouseEvent) => {
    e.preventDefault();
    onSelect(user.id);
  };

  return (
    <div className="flex flex-col items-center justify-between p-6 bg-white dark:bg-zinc-900 rounded-xl shadow-lg border border-slate-200 hover:shadow-2xl transition-all duration-300 w-full max-w-sm">
      <h2 className={clsx("text-lg font-bold", isHovered ? "text-emerald-500" : "text-zinc-500")}>
        {user.name} ({initials})
      </h2>
      <p className="text-sm text-zinc-400 mt-2 leading-relaxed">Member since: {formattedDate}</p>
      <button onClick={handleCardClick} className="mt-4 px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-lg shadow-sm focus:outline-none">
        Select Profile
      </button>
    </div>
  );
}
```

#### ✅ After Urai (Architecturally Intact, Token-Dense)
```markdown
### React Component Breakdown: `<UserProfileCard>` 
- **Props**:
  - `user` (type: `User`)
  - `onSelect` (type: `(id: string) => void`)
- **State Management**:
  - Manages state `isHovered` via setter `setIsHovered`.
- **Hooks**: Uses `useState, useEffect` (Total Side-Effects: 1).
- **Event Handlers**: Handlers attached: `onClick`.

```tsx
export function UserProfileCard({ user, onSelect }: ProfileCardProps) {
  useEffect(() => {
    trackImpression(user.id);
  }, [user.id]);

  return (
    <div>
      <h2 className={clsx("text-lg font-bold", isHovered ? "text-emerald-500" : "text-zinc-500")}>
        {user.name} ({initials})
      </h2>
      <p>Member since: {formattedDate}</p>
      <button onClick={handleCardClick}>
        Select Profile
      </button>
    </div>
  );
}
```
*(Dynamic conditional classes like `clsx(...)`, lifecycle hooks, and JSX hierarchy are preserved; static class noise and internal boilerplate are eliminated).*

---

## ✨ Key Features

| Feature | Description |
| :--- | :--- |
| 🌳 **AST Structural Stubbing** | Uses `is_structural_stub_stmt` to preserve inner functions, React hooks (`use*`), async timers (`setTimeout`), DOM listeners, and JSX layouts while stripping repetitive logic. |
| ✂️ **Smart Tailwind Pruner** | Four configurable modes (`remove`, `remove_aggr`, `summarize`, `preserve`) to strip static classes while preserving dynamic expressions. |
| ⚛️ **React Deep Introspection** | Parses component trees to extract props with TypeScript types, state names, setters, lifecycle side-effects, and rendered JSX elements. |
| 🛣️ **API Route Extractor** | Auto-discovers endpoints across **Express, Fastify, Next.js App Router, and NestJS** into Markdown tables. |
| 🦙 **Local Ollama AI Summaries** | Summarizes complex class methods and functions locally; caches summaries in `.urai-cache` to avoid duplicate API calls. |
| 🚀 **Rayon Multi-Threading** | Traverses, parses, and processes massive mono-repositories in parallel across all CPU cores. |
| 📁 **Git-Aware File Tree** | Powered by `ignore::WalkBuilder`, automatically ignoring `.gitignore`, hidden files, and build directories (`node_modules`, `dist`, `build`, `target`). |

---

## 🚀 Quick Start

### 1. Installation

#### Via Cargo (Recommended)
```bash
cargo install --git https://github.com/sanjaiyan-dev/urai-ecma.git
```

#### Build from Source
```bash
git clone https://github.com/sanjaiyan-dev/urai-ecma.git
cd urai-ecma
cargo build --release
cp target/release/urai-ecma /usr/local/bin/
```

Verify your installation:
```bash
urai-ecma --version
# urai-ecma 1.0
```

---

### 2. Scaffold Configuration

Run `create` in your project root to generate a commented configuration template:

```bash
urai-ecma create
```

This generates `urai.config.jsonc` in your current working directory.

---

### 3. Run Analysis

Generate an optimized prompt from your default config:
```bash
urai-ecma
```

Or run on-the-fly via command-line flags:
```bash
urai-ecma -i ./src -o ./prompt.md --tailwind-mode remove
```

---

## 🎨 Tailwind CSS Optimization Modes

Tailwind utility classes are among the biggest contributors to token bloat. Configure how `urai-ecma` processes them via `--tailwind-mode`:

| Mode | Behavior | Best Used For |
| :--- | :--- | :--- |
| **`remove`** *(Default)* | Strips static class strings exceeding `tailwind_threshold`. **Preserves dynamic JSX expressions** (`clsx`, `cn`, ternaries). | General refactoring, bug-fixing, business logic tasks. |
| **`remove_aggr`** | Aggressively eliminates all static class strings, regardless of length. | Backend migrations or architecture reviews where visual styling is irrelevant. |
| **`summarize`** | Prompts your local Ollama model to yield a 1-line style intent descriptor (e.g. `/* UI: Responsive flex card */`). | Design system audits and high-level UI component reviews. |
| **`preserve`** | Keeps all `className` and `style` attributes intact. | Pixel-perfect UI styling tasks or CSS debugging. |

---

## ⚙️ Configuration (`urai.config.jsonc`)

`urai-ecma` natively supports JSONC (with comments and trailing commas):

```jsonc
{
  // Root path to the project directory or single source file
  "input_project": "./src",

  // Destination path for the assembled Markdown prompt
  "output_file": "./output.md",

  // Ollama local endpoint URL (Optional, e.g., "http://localhost:11434")
  "ollama_endpoint": "http://localhost:11434",

  // Ollama model tag for offline code summaries (e.g., "llama3.2", "gemma4")
  "ollama_modelname": "gemma4",

  // Tailwind pruning strategy: "remove" | "remove_aggr" | "summarize" | "preserve"
  "tailwind_mode": "remove",

  // Character length threshold to trigger class pruning (default: 96 chars)
  "tailwind_threshold": 96,

  // Summarize function & method bodies using local LLM or JSDoc comments
  "summarize_functions": true,

  // Line count threshold to trigger function summarization (default: 5 lines)
  "summarize_functions_threshold": 5,

  // Extract and generate Express/Fastify/Next.js/NestJS API Route Tables
  "generate_route_table": true,

  // Introspect React / React Native components (props, state, hooks, tags)
  "analyze_react_components": true,

  // Generate ASCII File Structure & Module Dependency Graph
  "generate_file_graph": true
}
```

---

## 💻 CLI Reference

```text
AST-aware JS/TS code to prompt tool

Usage: urai-ecma [COMMAND] [OPTIONS]

Commands:
  create   Creates a default urai.config.jsonc file

Options:
  -i, --input-project <PROJECT_PATH>                 Path to source directory or file
  -o, --output-file <FILE_PATH>                      Destination Markdown output path
  -e, --ollama-endpoint <URL>                        Ollama endpoint [env: OLLAMA_ENDPOINT=]
  -m, --ollama-modelname <NAME>                      Ollama model name (e.g., gemma4)
      --tailwind-mode <MODE>                         Modes: remove | remove_aggr | summarize | preserve
      --tailwind-threshold <CHARS>                   Character pruning threshold (default: 96)
      --summarize-functions <BOOL>                   Summarize function bodies (default: true)
      --summarize-functions-threshold <LINES>        Line threshold for summarization (default: 5)
      --generate-route-table <BOOL>                  Generate backend route table (default: true)
      --analyze-react-components <BOOL>              Analyze React components (default: true)
      --generate-file-graph <BOOL>                   Generate ASCII file graph (default: true)
  -h, --help                                         Print help
  -V, --version                                      Print version
```

---

## 🏗️ Architecture & Pipeline

```text
┌────────────────────────────────────────────────────────┐
│                   JS / TS / TSX Codebase               │
└───────────────────────────┬────────────────────────────┘
                            │
              ignore::WalkBuilder (git-aware)
                            │
                            ▼
               rayon::par_iter() [Parallel]
                            │
               ┌────────────┴────────────┐
               ▼                         ▼
         [SWC AST Parser]      [SingleThreadedComments]
               │                         │
               ├─────────────────────────┤
               ▼                         ▼
      RouteVisitor              ReactComponentAnalyzer
  (Express/Next/Fastify)      (Props, State, Hooks, JSX)
               │                         │
               ├─────────────────────────┤
               ▼                         ▼
      ReactJsxPruner           FunctionSummarizerVisitor
  (Tailwind 4-mode pruner)    (JSDoc ──► Ollama ──► Stubs)
               │                         │
               └────────────┬────────────┘
                            ▼
                swc_ecma_codegen (Emitter)
                            │
                            ▼
              MarkdownContentBuilder + Graph
                            │
                            ▼
               tiktoken Telemetry Engine
            (llama3 & o200k_base benchmark)
                            │
                            ▼
               Clean, Dense prompt.md 🚀
```

---

## 🤝 Contributing

Contributions make the open-source community an inspiring place to learn, create, and build:

1. **Fork** the repository.
2. **Create** your feature branch (`git checkout -b feature/ast-svelte-support`).
3. **Commit** your changes (`git commit -m 'feat: add Svelte AST visitor'`).
4. **Push** to the branch (`git push origin feature/ast-svelte-support`).
5. **Open** a Pull Request.

---

## 📜 License

Distributed under the **MIT License**. See [`LICENSE`](LICENSE) for more information.

<div align="center">
<br />
Made with 🦀 and classical inspiration by <a href="https://github.com/sanjaiyan-dev"><b>sanjaiyan-dev</b></a>
</div>
