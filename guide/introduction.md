> For AI agents: the complete documentation index is available at https://sanjaiyan-dev.github.io/urai-ecma/llms.txt, the full documentation bundle is available at https://sanjaiyan-dev.github.io/urai-ecma/llms-full.txt.

# The Philosophy of URAI (உரை
)

Rust SWC Core
Attention Budget Optimizer
Rayon Multi-Core
Foyer Hybrid Cache
Tiktoken o200k BPE

> **“உரை” (_Urai_)** in classical Tamil translates to _reasoned exposition, critical discourse, structural commentary, or analytical narrative_.
>
> **`urai-ecma`** transforms massive JavaScript, TypeScript, and React codebases into an information-dense, noise-free Markdown prompt engineered specifically for Large Language Model context windows (GPT-4o, Claude 3.5 Sonnet, Gemini 1.5 Pro, and local Llama/Qwen models).

## 📊 Live Interactive BPE Telemetry Simulator

Explore how `urai-ecma` compresses real-world source trees down to their syntactic essence while preserving interfaces, routing tables, and component lifecycles. Drag the codebase size slider to model your project:


Interactive Token TelemetryBPE o200k Engine

Target Model:OpenAI GPT-4oAnthropic Claude 3.5 SonnetGoogle Gemini 1.5 Pro

Input Codebase Size148k tokens (148,000 tokens)

Small Module (20k)Enterprise Monorepo (400k)

Monthly Developer Pipeline Runs500 runs/mo

Solo Engineer (50)Engineering Org (3,000)

Context Window Load Comparison

Raw Repo Dump (100%)

URAI (19.16%)

Optimized Prompt

29,422

-80.12% token drop

Monthly Cost Savings

$148.22

vs $185.00 raw cost

Tokens Saved / Run

~119k

118,578 tokens preserved

Prefill Latency (TTFT)

0.53s

Down from 2.66s raw

***

## ⚡ Interactive Engine Deconstruction

Inspect how `urai-ecma` executes internally across its 5-stage compiler pipeline. Click between the stages below to see how code is crawled, parsed into SWC nodes, mutated, and cached via Foyer:


⚡ Rust Compiler Deconstruction HUDInteractive Pipeline

Click any stage below to inspect internal AST transformations, memory states, and Rust code.

Code DeltaAST OperationsRust Source

ignore::WalkBuilder

1. Ingestion & Filtering

swc_ecma_parser

2. Rayon & SWC Parse

is_structural_stub_stmt

3. AST Mutation & Pruning

Sha512_256 + Zstd

4. Foyer Hybrid Cache

tiktoken o200k_base

5. Synthesis & Telemetry

MODULE: visitor/function_summarizer.rs & react.rs

Selectively strips static Tailwind class noise and imperative function internals while preserving React hooks, listeners, and JSX layouts.

Execution

14.2ms

Stage Reduction

-74.1%

❌ Ingested State (Bloated / Raw)

````
return (
  <div className="flex flex-col items-center justify-between p-8 bg-white dark:bg-zinc-950 rounded-2xl shadow-xl border border-slate-200 hover:shadow-2xl transition-all duration-300 w-full max-w-4xl">
    <h2 className={clsx("text-lg", isHovered && "text-cyan-400")}>{user.name}</h2>
  </div>
);
````

✅ Synthesized State (AST-Pruned & Dense)

````
return (
  <div>
    <h2 className={clsx("text-lg", isHovered && "text-cyan-400")}>{user.name}</h2>
  </div>
);
````

***

## 🏛️ The Classical Inspiration: The Art of _"உரை எழுதுதல்"_ (Urai Ezhuthudhal)

In classical Tamil literary heritage, monumental masterworks—such as the **Thirukkuṛaḷ** (திருக்குறள்), **Tolkāppiyam** (தொல்காப்பியம்), and **Cilappatikāram** (சிலப்பதிகாரம்)—encompass vast volumes of dense, poetic, and multi-layered thought. To make these works understandable without sacrificing their architectural depth, classical scholars practiced **உரை எழுதுதல் (_Urai Ezhuthudhal_)**.

Celebrated _Uraiyāsiriyars_ (master commentators) like **Parimelazhagar** (பரிமேலழகர்) and **Ilampuranar** (இளம்பூரணர்) did not simply transcribe or mechanically summarize texts. They performed **structural distillation**:

1. Isolating the core semantic axioms of each stanza.
2. Stripping linguistic ornamentation that could obscure meaning.
3. Exposing grammar, intent, and relationships for reasoned debate.


The Modern Software Parallel

Today, enterprise JavaScript and TypeScript codebases are the **epic literatures of modern software engineering**. Spanning hundreds of directories across Next.js, React, Node.js, and Express, they are laden with repetitive utility classes, nested loops, boilerplate type casts, and mechanical handlers.
When feeding these systems to an LLM:
- **The model does not need raw syntactic exhaustion**—it requires the structural anatomy, API contracts, state flows, and component signatures.
- **`urai-ecma` acts as the modern _Uraiyāsiriyar_**: reading your codebase via its Abstract Syntax Tree (AST), pruning styling bloat and loop mechanics, and writing a structured, token-dense **"உரை" (Prompt Commentary)** engineered for AI reasoning.

***

## ⚠️ The Engineering Crisis: The "Infinite Context" Fallacy

Recent advances in Large Language Models introduced 1M+ and 2M+ token context windows. However, treating these windows as unbounded storage introduces four major architectural bottlenecks:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│               THE FOUR BOTTLENECKS OF NAIVE CODEBASE CONTEXT               │
├──────────────────────────────┬──────────────────────────────────────────────┤
│ 1. Attention Head Saturation │ Dense needle-in-a-haystack degradation; key  │
│    ("Lost-in-the-Middle")    │ architectural interfaces drown in loops.     │
├──────────────────────────────┼──────────────────────────────────────────────┤
│ 2. KV-Cache Prefill Latency  │ Time-To-First-Token (TTFT) scales with input │
│                              │ volume; 150k token inputs stall interactive  │
│                              │ developer feedback loops.                    │
├──────────────────────────────┼──────────────────────────────────────────────┤
│ 3. Escalating API Economics  │ Teams spend thousands of dollars ingesting   │
│                              │ static Tailwind strings and boilerplate code.│
├──────────────────────────────┼──────────────────────────────────────────────┤
│ 4. Rate-Limit Throttling     │ Large prompts trigger TPM (Tokens Per Minute)│
│                              │ exhaustion in automated CI/CD pipelines.     │
└──────────────────────────────┴──────────────────────────────────────────────┘
```

### What Actually Comprises an Enterprise Frontend Codebase?

Empirical token breakdown of an average Next.js + React + Tailwind repository:

```
┌───────────────────────────────────────────────────────────────────────┐
│              ANATOMY OF A RAW FRONTEND REPOSITORY (TOKENS)            │
└───────────────────────────────────────────────────────────────────────┘
  [███████████████████████████████████████████████] 58% Tailwind CSS Class Names
  [████████████████] 22% Imperative Function Bodies & Loops
  [███████] 12% Type Boilerplate & Import Directives
  [████] 8% Core Structural Anatomy (State, Props, Routes, Hooks)
```

**Over 80% of total tokens provide zero signal to an LLM reasoning about architecture.**

Tools like `repomix`, `gitingest`, or `code2prompt` act as simple file concatenators. They blindly wrap raw files in XML tags, forcing models to parse thousands of characters of static styling strings (`"flex items-center justify-between p-6 bg-white dark:bg-zinc-900..."`).

***

## ⚡ The Compiler Difference: Synthesis vs. Concatenation

`urai-ecma` is an **AST-aware compiler engine** written in Rust, powered by ByteDance/Vercel's `swc_ecma` suite. Rather than treating code as raw strings, it parses JavaScript and TypeScript into concrete syntax trees and applies deterministic, semantic transformations.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                     URAI COMPILER SYNTHESIS PIPELINE                    │
└─────────────────────────────────────────────────────────────────────────┘
   Enterprise Monorepo (.ts, .tsx, .js, .mjs, .json)
                          │
                          ▼
            [ignore::WalkBuilder (Rust)]
         Honor .gitignore, prune node_modules & dist
                          │
                          ▼
             [Rayon Parallel Work-Stealing]
        Multi-threaded AST parsing across all CPU cores
                          │
          ┌───────────────┴───────────────┐
          ▼                               ▼
 [swc_ecma_parser]               [swc_ecma_parser]
  Worker Thread A                 Worker Thread B
          │                               │
          ├─► [RouteVisitor]              ├─► [RouteVisitor]
          │   Next.js/Express/NestJS      │   Next.js/Express/NestJS
          │                               │
          ├─► [ReactComponentAnalyzer]    ├─► [ReactComponentAnalyzer]
          │   Props, State, Hooks, JSX    │   Props, State, Hooks, JSX
          │                               │
          ├─► [ReactJsxPruner]            ├─► [ReactJsxPruner]
          │   Tailwind static class strip │   Tailwind static class strip
          │                               │
          └─► [FunctionSummarizerVisitor] └─► [FunctionSummarizerVisitor]
              Preserve structural stubs       Preserve structural stubs
                          │
                          ▼
         [Foyer Hybrid Cache (Disk + RAM)]
              Sha512_256 + Zstd compression
                          │
                          ▼
         [swc_ecma_codegen + Tiktoken Engine]
   Emits high-density Markdown prompt + BPE o200k report
```

***

## 🎯 The Three Pillars of AST Token Optimization

### 1. AST Structural Stubbing (`is_structural_stub_stmt`)

Traditional approaches present a false choice: either include complete function bodies (wasting tokens) or strip bodies completely down to empty signatures (which removes crucial React hooks, event listeners, and JSX hierarchies).

`urai-ecma` solves this through **structural stubbing**:

```rust
// Only statements defining component anatomy are preserved:
fn is_structural_stub_stmt(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Decl(Decl::Fn(_)) => true, // Closures & inner functions
        Stmt::Expr(expr_stmt) => {       // React Hooks, listeners, timers
            // Retains: use*, setTimeout, setInterval, addEventListener
        },
        Stmt::Return(ret_stmt) => {      // JSX layout hierarchies
            // Retains: <JSXElement />, <JSXFragment />, paren expressions
        },
        _ => false,                      // Prunes imperative loops & math
    }
}
```

### 2. Dynamic-Aware Tailwind Pruning

Static styling strings are pruned while dynamic expressions remain untouched:

- **Preserves dynamic code**: Functions using `clsx(...)`, `cva(...)`, or conditional ternaries (`isActive ? "text-cyan-400" : "text-zinc-500"`) are retained.
- **Prunes static strings**: Long static utility classes exceeding the configurable threshold (default: 96 characters) are cleanly stripped or summarized into short UI descriptors.

### 3. Zero-Leakage Offline AI Intelligence

When functions exceed threshold lines (default: 5 lines):

- `urai-ecma` checks for existing JSDoc annotations (`@description`, `@param`, `@return`).
- If missing, it queries a **local Ollama instance** (`gemma4`, `llama3.2`). No proprietary source code leaves your local machine.
- Results are cached using **Foyer** (a hybrid RAM + disk block cache with Zstd compression), yielding sub-millisecond lookups on subsequent runs.

***

## 🔬 Before & After: Real-World Code Transformation

Compare a raw React component against the output synthesized by `urai-ecma`:


**❌ Raw Input (Token Exhaustive)**

```tsx
// src/components/BillingDashboard.tsx (Raw: 412 tokens)
import React, { useState, useEffect, useMemo } from 'react';
import { loadStripe } from '@stripe/stripe-js';
import clsx from 'clsx';

export function BillingDashboard({ accountId, onInvoicePaid }: BillingProps) {
  const [invoices, setInvoices] = useState<Invoice[]>([]);
  const [isProcessing, setIsProcessing] = useState(false);

  useEffect(() => {
    fetchInvoices(accountId).then(data => setInvoices(data));
  }, [accountId]);

  // 35 lines of mechanical calculation and validation logic
  const totalOutstanding = useMemo(() => {
    return invoices.reduce((acc, inv) => {
      if (inv.status === 'unpaid' && !inv.isDisputed) {
        return acc + inv.amountCents / 100;
      }
      return acc;
    }, 0);
  }, [invoices]);

  const handlePayNow = async (invoiceId: string) => {
    setIsProcessing(true);
    const stripe = await loadStripe(process.env.NEXT_PUBLIC_STRIPE_KEY!);
    await triggerPaymentGateway(invoiceId);
    onInvoicePaid(invoiceId);
    setIsProcessing(false);
  };

  return (
    <div className="flex flex-col items-center justify-between p-8 bg-white dark:bg-zinc-950 rounded-2xl shadow-xl border border-slate-200 dark:border-zinc-800 hover:shadow-2xl transition-all duration-300 w-full max-w-4xl mx-auto my-6">
      <h2 className={clsx("text-2xl font-bold tracking-tight", totalOutstanding > 0 ? "text-amber-500" : "text-emerald-500")}>
        Outstanding: ${totalOutstanding}
      </h2>
      <button 
        onClick={() => handlePayNow(invoices[0]?.id)}
        className="mt-6 px-6 py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-semibold rounded-xl shadow-md hover:scale-105 active:scale-95 transition-transform duration-150"
      >
        Pay Balance
      </button>
    </div>
  );
}
```


**✅ URAI Synthesized Prompt (Architectural Signal)**

````markdown
### React Component Breakdown: `<BillingDashboard>` 
- **Props**:
  - `accountId` (type: `string`)
  - `onInvoicePaid` (type: `(invoiceId: string) => void`)
- **State Management**:
  - Manages state `invoices` via setter `setInvoices`.
  - Manages state `isProcessing` via setter `setIsProcessing`.
- **Hooks**: Uses `useState, useEffect, useMemo` (Total Side-Effects: 1).
- **Event Handlers**: Handlers attached: `onClick`.
- **Rendered JSX Tree**: `<div>, <h2>, <button>`

```tsx
export function BillingDashboard({ accountId, onInvoicePaid }: BillingProps) {
  const [invoices, setInvoices] = useState<Invoice[]>([]);
  const [isProcessing, setIsProcessing] = useState(false);

  useEffect(() => {
    /* "Fetches and populates invoices for given account ID." */
  }, [accountId]);

  return (
    <div>
      <h2 className={clsx("text-2xl font-bold tracking-tight", totalOutstanding > 0 ? "text-amber-500" : "text-emerald-500")}>
        Outstanding: ${totalOutstanding}
      </h2>
      <button onClick={() => handlePayNow(invoices[0]?.id)}>
        Pay Balance
      </button>
    </div>
  );
}

````
_Token count drops from **412 tokens down to 78 tokens** (-81.06% reduction). Dynamic styling, typed props, state variables, and hook lifecycles remain completely intact._


***

## 🚀 Getting Started

Explore the operational guides to integrate `urai-ecma` into your engineering workflow:


[⚡ Installation & Quick Start

Install prebuilt binaries via Shell, PowerShell, npm, or Cargo in seconds.


](/guide/quick-start)[🌳 Compiler Pipeline

Deep dive into the SWC visitor pattern, Foyer hybrid caching, and Rayon threading.


](/guide/architecture)[🛠️ CLI & Config Reference

Complete reference for all CLI arguments, environment variables, and config options.


](/reference/cli)[📈 Empirical Benchmarks

Performance and token comparisons against Repomix, GitIngest, and raw code dumps.


](/reference/benchmarks)