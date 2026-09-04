import React, { useState } from 'react';

interface PipelineStage {
  id: string;
  name: string;
  badge: string;
  badgeColor: string;
  rustModule: string;
  description: string;
  rawSnippet: string;
  astAction: string;
  synthesizedSnippet: string;
  rustCode: string;
  telemetry: {
    latency: string;
    tokensBefore: number;
    tokensAfter: number;
    compression: string;
  };
}

const PIPELINE_STAGES: PipelineStage[] = [
  {
    id: 'ingest',
    name: '1. Ingestion & Filtering',
    badge: 'ignore::WalkBuilder',
    badgeColor: '#d97736',
    rustModule: 'main.rs :: collect_source_files()',
    description:
      'Traverses the repository with Git-aware work-stealing filters. Automatically ignores hidden directories, .gitignore files, node_modules, dist, and build targets.',
    rawSnippet: `// 48 Files Detected across monorepo:
apps/web/src/components/UserProfile.tsx
apps/web/src/app/api/checkout/route.ts
packages/ui/src/button.tsx
node_modules/react/... (SKIPPED)
dist/index.js           (SKIPPED)`,
    astAction: `[WalkBuilder Filter Active]
- GitIgnore rules applied
- Binary files pruned
- Supported: .js, .jsx, .ts, .tsx, .mjs, .cjs`,
    synthesizedSnippet: `📁 Project File Tree (48 Files Validated)
└── 📄 apps/web/src/components/UserProfile.tsx
└── 📄 apps/web/src/app/api/checkout/route.ts
└── 📄 packages/ui/src/button.tsx`,
    rustCode: `WalkBuilder::new(path)
    .hidden(true)
    .git_ignore(true)
    .filter_entry(|entry| {
        if let Some(name) = entry.file_name().to_str() {
            return name != "node_modules" && name != "dist" && name != "build";
        }
        true
    })
    .build()`,
    telemetry: {
      latency: '2.4ms',
      tokensBefore: 148290,
      tokensAfter: 148290,
      compression: '0.0%',
    },
  },
  {
    id: 'parse',
    name: '2. Rayon & SWC Parse',
    badge: 'swc_ecma_parser',
    badgeColor: '#00f2fe',
    rustModule: 'ast/analyze.rs :: run_project_analysis()',
    description:
      'Spawns parallel worker threads across all available CPU cores using Rayon. Converts raw TypeScript text into concrete SWC Abstract Syntax Trees.',
    rawSnippet: `export function UserCard({ user, onSelect }: Props) {
  const [isHovered, setIsHovered] = useState(false);
  useEffect(() => { trackEvent(user.id); }, [user.id]);
  const formatted = user.name.toUpperCase();
  return <div className="p-6 bg-white rounded-xl...">{formatted}</div>;
}`,
    astAction: `Module {
  body: [
    ExportDecl -> FnDecl("UserCard") {
      params: [Pat::Ident("Props")],
      body: BlockStmt [
        Call("useState"),
        Call("useEffect"),
        VarDecl("formatted"),
        ReturnStmt(JSXElement)
      ]
    }
  ]
}`,
    synthesizedSnippet: `[SWC In-Memory AST Representation Ready]
Nodes: 42 Syntax Elements
SourceMap: Character positions resolved
Comments: SingleThreadedComments borrowed`,
    rustCode: `files.par_iter().filter_map(|file_path| {
    let raw_content = fs::read_to_string(file_path).ok()?;
    let (mut module, comments, cm) = parse_file(&raw_content, file_path.to_str()?)?;
    // Parallel AST mutation pipelines executed here
    ...
})`,
    telemetry: {
      latency: '8.1ms',
      tokensBefore: 148290,
      tokensAfter: 148290,
      compression: '0.0%',
    },
  },
  {
    id: 'prune',
    name: '3. AST Mutation & Pruning',
    badge: 'is_structural_stub_stmt',
    badgeColor: '#10b981',
    rustModule: 'visitor/function_summarizer.rs & react.rs',
    description:
      'Selectively strips static Tailwind class noise and imperative function internals while preserving React hooks, listeners, and JSX layouts.',
    rawSnippet: `return (
  <div className="flex flex-col items-center justify-between p-8 bg-white dark:bg-zinc-950 rounded-2xl shadow-xl border border-slate-200 hover:shadow-2xl transition-all duration-300 w-full max-w-4xl">
    <h2 className={clsx("text-lg", isHovered && "text-cyan-400")}>{user.name}</h2>
  </div>
);`,
    astAction: `[ReactJsxPruner]:
- className string literal (164 chars) > threshold (96) -> PRUNED
- clsx dynamic expression -> PRESERVED

[FunctionSummarizerVisitor]:
- Retains: useState, useEffect, JSX Return
- Prunes: for loops, intermediate math`,
    synthesizedSnippet: `return (
  <div>
    <h2 className={clsx("text-lg", isHovered && "text-cyan-400")}>{user.name}</h2>
  </div>
);`,
    rustCode: `fn is_structural_stub_stmt(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Decl(Decl::Fn(_)) => true,
        Stmt::Expr(e) if is_hook_or_timer(e) => true,
        Stmt::Return(r) if is_jsx_return(r) => true,
        _ => false, // Prunes imperative loops & math
    }
}`,
    telemetry: {
      latency: '14.2ms',
      tokensBefore: 148290,
      tokensAfter: 38400,
      compression: '-74.1%',
    },
  },
  {
    id: 'cache',
    name: '4. Foyer Hybrid Cache',
    badge: 'Sha512_256 + Zstd',
    badgeColor: '#f59e0b',
    rustModule: 'cache.rs :: init_cache()',
    description:
      'Queries local Ollama models for 1-sentence function summaries. Results are cached into 64MB RAM and 128MB Zstd-compressed block disk storage.',
    rawSnippet: `function calculateTaxMatrix(user: User, invoices: Invoice[]) {
  // 45 lines of complex multi-tier regional tax calculation
  let base = 0;
  for (const inv of invoices) { base += inv.amount * 0.18; }
  return applyDeductions(base, user.taxCode);
}`,
    astAction: `1. Calculate Hash: Sha512_256(snippet)
2. Foyer Cache Lookup:
   - Memory Tier: MISS
   - Disk Tier: MISS
3. Ollama Query (gemma2): "Calculates regional tax matrix with deductions."
4. Insert Foyer: Cache Key -> Zstd Compressed Block`,
    synthesizedSnippet: `function calculateTaxMatrix(user: User, invoices: Invoice[]) {
  /* "Calculates regional tax matrix with applicable deductions." */
}`,
    rustCode: `let hybrid = HybridCacheBuilder::new()
    .memory(64 * 1024 * 1024)
    .storage()
    .with_engine_config(BlockEngineConfig::new(device))
    .with_compression(foyer::Compression::Zstd)
    .build()
    .await?;`,
    telemetry: {
      latency: '0.8ms (Cached)',
      tokensBefore: 38400,
      tokensAfter: 28410,
      compression: '-80.8%',
    },
  },
  {
    id: 'emit',
    name: '5. Synthesis & Telemetry',
    badge: 'tiktoken o200k_base',
    badgeColor: '#38bdf8',
    rustModule: 'markdown/markdown_content.rs',
    description:
      'Assembles the final Markdown prompt with Route Tables, React Architectures, ASCII Tree, and AST-stubs. Emits instant BPE token metrics.',
    rawSnippet: `[Raw Codebase Dump: 48 Files, 148,290 Tokens]`,
    astAction: `1. Generate ASCII PEG Tree
2. Build Mermaid Module Graph
3. Assemble Route Table (Next.js, Fastify, NestJS)
4. Append React Component Explanations
5. Emit Clean AST-Pruned Source Repositories
6. Execute tiktoken o200k_base encode`,
    synthesizedSnippet: `## Backend API Route Table
| Framework | Method | Path | Handler |
| Next.js | POST | /api/checkout | POST |

## React Component Breakdown: <UserCard>
- Props: user (User), onSelect ((id: string) => void)
- State: isHovered (boolean)

## AST-Pruned Source Code
\`\`\`tsx
export function UserCard({ user, onSelect }: Props) { ... }
\`\`\``,
    rustCode: `if let Some(bpe) = tiktoken::get_encoding("o200k_base") {
    let output_tokens = bpe.encode(&final_markdown).len();
    let raw_tokens = calculate_raw_project_tokens(&ctx.input_project, bpe);
    let saved_tokens = raw_tokens.saturating_sub(output_tokens);
    let reduction = (saved_tokens as f64 / raw_tokens as f64) * 100.0;
}`,
    telemetry: {
      latency: '82.0ms Total',
      tokensBefore: 148290,
      tokensAfter: 28410,
      compression: '-80.84%',
    },
  },
];

export function CompilerPipelineCard() {
  const [activeStageId, setActiveStageId] = useState<string>('prune');
  const [viewMode, setViewMode] = useState<'code' | 'ast' | 'rust'>('code');

  const activeStage =
    PIPELINE_STAGES.find((s) => s.id === activeStageId) || PIPELINE_STAGES[2];

  return (
    <div
      style={{
        background: 'var(--urai-glass-surface)',
        backdropFilter: 'var(--urai-glass-blur)',
        WebkitBackdropFilter: 'var(--urai-glass-blur)',
        border: '1px solid rgba(0, 242, 254, 0.35)',
        borderRadius: 'var(--rp-radius-large)',
        padding: '1.75rem',
        margin: '2.5rem 0',
        boxShadow:
          '0 25px 60px -10px rgba(0, 0, 0, 0.55), 0 0 30px -4px var(--urai-cyan-glow)',
        position: 'relative',
        overflow: 'hidden',
      }}
    >
      {/* Laser Header Accent */}
      <div
        style={{
          position: 'absolute',
          top: 0,
          left: 0,
          right: 0,
          height: '3px',
          background:
            'linear-gradient(90deg, #d97736, #00f2fe, #10b981, #f59e0b, #38bdf8)',
        }}
      />

      {/* Title Bar */}
      <div
        style={{
          display: 'flex',
          flexWrap: 'wrap',
          justifyContent: 'space-between',
          alignItems: 'center',
          gap: '1rem',
          marginBottom: '1.5rem',
          borderBottom: '1px solid var(--rp-c-divider-light)',
          paddingBottom: '1rem',
        }}
      >
        <div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <span
              style={{
                fontSize: '1.25rem',
                fontWeight: 800,
                color: 'var(--rp-c-text-0)',
              }}
            >
              ⚡ Rust Compiler Deconstruction HUD
            </span>
            <span className="urai-badge-cyan">Interactive Pipeline</span>
          </div>
          <div
            style={{
              fontSize: '0.8rem',
              color: 'var(--rp-c-text-2)',
              marginTop: '4px',
            }}
          >
            Click any stage below to inspect internal AST transformations,
            memory states, and Rust code.
          </div>
        </div>

        <div
          style={{
            display: 'flex',
            gap: '6px',
            background: 'rgba(0,0,0,0.3)',
            padding: '4px',
            borderRadius: '8px',
          }}
        >
          <button
            onClick={() => setViewMode('code')}
            style={{
              background:
                viewMode === 'code' ? 'var(--urai-cyan)' : 'transparent',
              color: viewMode === 'code' ? '#090c13' : 'var(--rp-c-text-2)',
              border: 'none',
              borderRadius: '6px',
              padding: '5px 12px',
              fontSize: '0.75rem',
              fontWeight: 700,
              cursor: 'pointer',
              transition: 'all 0.2s ease',
            }}
          >
            Code Delta
          </button>
          <button
            onClick={() => setViewMode('ast')}
            style={{
              background:
                viewMode === 'ast' ? 'var(--urai-cyan)' : 'transparent',
              color: viewMode === 'ast' ? '#090c13' : 'var(--rp-c-text-2)',
              border: 'none',
              borderRadius: '6px',
              padding: '5px 12px',
              fontSize: '0.75rem',
              fontWeight: 700,
              cursor: 'pointer',
              transition: 'all 0.2s ease',
            }}
          >
            AST Operations
          </button>
          <button
            onClick={() => setViewMode('rust')}
            style={{
              background: viewMode === 'rust' ? '#d97736' : 'transparent',
              color: viewMode === 'rust' ? '#ffffff' : 'var(--rp-c-text-2)',
              border: 'none',
              borderRadius: '6px',
              padding: '5px 12px',
              fontSize: '0.75rem',
              fontWeight: 700,
              cursor: 'pointer',
              transition: 'all 0.2s ease',
            }}
          >
            Rust Source
          </button>
        </div>
      </div>

      {/* Interactive Horizontal Pipeline Stepper */}
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(180px, 1fr))',
          gap: '0.75rem',
          marginBottom: '1.75rem',
        }}
      >
        {PIPELINE_STAGES.map((stage) => {
          const isActive = stage.id === activeStageId;
          return (
            <div
              key={stage.id}
              onClick={() => setActiveStageId(stage.id)}
              style={{
                background: isActive
                  ? 'rgba(0, 242, 254, 0.12)'
                  : 'rgba(0, 0, 0, 0.25)',
                border: isActive
                  ? `1.5px solid ${stage.badgeColor}`
                  : '1px solid var(--rp-c-divider-light)',
                borderRadius: 'var(--rp-radius-small)',
                padding: '0.85rem',
                cursor: 'pointer',
                transition: 'all 0.25s cubic-bezier(0.16, 1, 0.3, 1)',
                boxShadow: isActive
                  ? `0 0 18px -4px ${stage.badgeColor}`
                  : 'none',
                position: 'relative',
              }}
            >
              <div
                style={{
                  fontSize: '0.7rem',
                  fontWeight: 700,
                  textTransform: 'uppercase',
                  color: isActive ? stage.badgeColor : 'var(--rp-c-text-3)',
                  letterSpacing: '0.04em',
                }}
              >
                {stage.badge}
              </div>
              <div
                style={{
                  fontSize: '0.85rem',
                  fontWeight: 700,
                  color: isActive ? 'var(--rp-c-text-0)' : 'var(--rp-c-text-1)',
                  marginTop: '4px',
                }}
              >
                {stage.name}
              </div>
              {isActive && (
                <div
                  style={{
                    position: 'absolute',
                    bottom: '-5px',
                    left: '50%',
                    transform: 'translateX(-50%)',
                    width: '8px',
                    height: '8px',
                    borderRadius: '50%',
                    background: stage.badgeColor,
                    boxShadow: `0 0 10px ${stage.badgeColor}`,
                  }}
                />
              )}
            </div>
          );
        })}
      </div>

      {/* Stage Technical Detail Banner */}
      <div
        style={{
          background: 'rgba(0, 0, 0, 0.35)',
          border: '1px solid var(--rp-c-divider-light)',
          borderRadius: '8px',
          padding: '1rem',
          marginBottom: '1.25rem',
          display: 'flex',
          flexWrap: 'wrap',
          justifyContent: 'space-between',
          alignItems: 'center',
          gap: '1rem',
        }}
      >
        <div style={{ maxWidth: '650px' }}>
          <div
            style={{
              fontSize: '0.75rem',
              fontFamily: 'var(--rp-font-family-mono)',
              color: activeStage.badgeColor,
            }}
          >
            MODULE: {activeStage.rustModule}
          </div>
          <div
            style={{
              fontSize: '0.85rem',
              color: 'var(--rp-c-text-1)',
              marginTop: '4px',
              lineHeight: 1.5,
            }}
          >
            {activeStage.description}
          </div>
        </div>

        {/* Telemetry Micro-HUD */}
        <div style={{ display: 'flex', gap: '1rem' }}>
          <div>
            <div
              style={{
                fontSize: '0.68rem',
                color: 'var(--rp-c-text-3)',
                textTransform: 'uppercase',
              }}
            >
              Execution
            </div>
            <div
              style={{
                fontSize: '1rem',
                fontWeight: 800,
                fontFamily: 'var(--rp-font-family-mono)',
                color: 'var(--urai-cyan)',
              }}
            >
              {activeStage.telemetry.latency}
            </div>
          </div>
          <div>
            <div
              style={{
                fontSize: '0.68rem',
                color: 'var(--rp-c-text-3)',
                textTransform: 'uppercase',
              }}
            >
              Stage Reduction
            </div>
            <div
              style={{
                fontSize: '1rem',
                fontWeight: 800,
                fontFamily: 'var(--rp-font-family-mono)',
                color: '#10b981',
              }}
            >
              {activeStage.telemetry.compression}
            </div>
          </div>
        </div>
      </div>

      {/* Interactive Code / AST Viewport */}
      {viewMode === 'code' && (
        <div
          style={{
            display: 'grid',
            gridTemplateColumns: 'repeat(auto-fit, minmax(320px, 1fr))',
            gap: '1rem',
          }}
        >
          <div>
            <div
              style={{
                fontSize: '0.72rem',
                textTransform: 'uppercase',
                color: '#ff6b6b',
                fontWeight: 700,
                marginBottom: '6px',
              }}
            >
              ❌ Ingested State (Bloated / Raw)
            </div>
            <pre
              style={{
                background: '#0d121c',
                border: '1px solid rgba(255, 107, 107, 0.2)',
                borderRadius: '8px',
                padding: '1rem',
                fontSize: '0.78rem',
                fontFamily: 'var(--rp-font-family-mono)',
                color: '#e2d9cc',
                height: '240px',
                overflowY: 'auto',
                margin: 0,
              }}
            >
              <code>{activeStage.rawSnippet}</code>
            </pre>
          </div>

          <div>
            <div
              style={{
                fontSize: '0.72rem',
                textTransform: 'uppercase',
                color: 'var(--urai-cyan)',
                fontWeight: 700,
                marginBottom: '6px',
              }}
            >
              ✅ Synthesized State (AST-Pruned & Dense)
            </div>
            <pre
              style={{
                background: '#0d121c',
                border: '1px solid rgba(0, 242, 254, 0.25)',
                borderRadius: '8px',
                padding: '1rem',
                fontSize: '0.78rem',
                fontFamily: 'var(--rp-font-family-mono)',
                color: '#4ade80',
                height: '240px',
                overflowY: 'auto',
                margin: 0,
              }}
            >
              <code>{activeStage.synthesizedSnippet}</code>
            </pre>
          </div>
        </div>
      )}

      {viewMode === 'ast' && (
        <div>
          <div
            style={{
              fontSize: '0.72rem',
              textTransform: 'uppercase',
              color: 'var(--urai-cyan)',
              fontWeight: 700,
              marginBottom: '6px',
            }}
          >
            🔍 Concrete Syntax Tree Node Transformations
          </div>
          <pre
            style={{
              background: '#0a0e17',
              border: '1px solid rgba(0, 242, 254, 0.3)',
              borderRadius: '8px',
              padding: '1.2rem',
              fontSize: '0.8rem',
              fontFamily: 'var(--rp-font-family-mono)',
              color: '#38bdf8',
              height: '240px',
              overflowY: 'auto',
              margin: 0,
            }}
          >
            <code>{activeStage.astAction}</code>
          </pre>
        </div>
      )}

      {viewMode === 'rust' && (
        <div>
          <div
            style={{
              fontSize: '0.72rem',
              textTransform: 'uppercase',
              color: '#d97736',
              fontWeight: 700,
              marginBottom: '6px',
            }}
          >
            🦀 Active Rust Engine Logic ({activeStage.rustModule})
          </div>
          <pre
            style={{
              background: '#0d121c',
              border: '1px solid rgba(217, 119, 54, 0.3)',
              borderRadius: '8px',
              padding: '1.2rem',
              fontSize: '0.8rem',
              fontFamily: 'var(--rp-font-family-mono)',
              color: '#fdba74',
              height: '240px',
              overflowY: 'auto',
              margin: 0,
            }}
          >
            <code>{activeStage.rustCode}</code>
          </pre>
        </div>
      )}
    </div>
  );
}
