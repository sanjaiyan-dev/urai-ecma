import { useState, useMemo } from 'react';

type TailwindMode = 'remove' | 'remove_aggr' | 'summarize' | 'preserve';
type PresetKey =
  | 'nextjs'
  | 'design_system'
  | 'offline_ast'
  | 'backend_api'
  | 'max_compression';

interface PresetConfig {
  name: string;
  badge: string;
  inputProject: string;
  outputFile: string;
  enableOllama: boolean;
  ollamaEndpoint: string;
  ollamaModel: string;
  tailwindMode: TailwindMode;
  tailwindThreshold: number;
  summarizeFunctions: boolean;
  summarizeFunctionsThreshold: number;
  generateRouteTable: boolean;
  analyzeReactComponents: boolean;
  generateFileGraph: boolean;
}

const PRESETS: Record<PresetKey, PresetConfig> = {
  nextjs: {
    name: 'Next.js App Router',
    badge: 'Full-Stack',
    inputProject: './src',
    outputFile: './prompt.md',
    enableOllama: false,
    ollamaEndpoint: 'http://localhost:11434',
    ollamaModel: 'gemma4',
    tailwindMode: 'remove',
    tailwindThreshold: 96,
    summarizeFunctions: true,
    summarizeFunctionsThreshold: 6,
    generateRouteTable: true,
    analyzeReactComponents: true,
    generateFileGraph: true,
  },
  design_system: {
    name: 'UI & Design System',
    badge: 'AI Summarizer',
    inputProject: './packages/ui/src',
    outputFile: './ui-context.md',
    enableOllama: true,
    ollamaEndpoint: 'http://localhost:11434',
    ollamaModel: 'gemma2:2b',
    tailwindMode: 'summarize',
    tailwindThreshold: 64,
    summarizeFunctions: true,
    summarizeFunctionsThreshold: 4,
    generateRouteTable: false,
    analyzeReactComponents: true,
    generateFileGraph: true,
  },
  offline_ast: {
    name: 'Offline Zero-LLM',
    badge: 'Blazing Fast',
    inputProject: './src',
    outputFile: './output.md',
    enableOllama: false,
    ollamaEndpoint: 'http://localhost:11434',
    ollamaModel: 'gemma4',
    tailwindMode: 'remove',
    tailwindThreshold: 96,
    summarizeFunctions: false,
    summarizeFunctionsThreshold: 5,
    generateRouteTable: true,
    analyzeReactComponents: true,
    generateFileGraph: true,
  },
  backend_api: {
    name: 'NestJS & Express API',
    badge: 'Backend',
    inputProject: './apps/api/src',
    outputFile: './api-spec.md',
    enableOllama: true,
    ollamaEndpoint: 'http://localhost:11434',
    ollamaModel: 'llama3.2',
    tailwindMode: 'preserve',
    tailwindThreshold: 96,
    summarizeFunctions: true,
    summarizeFunctionsThreshold: 8,
    generateRouteTable: true,
    analyzeReactComponents: false,
    generateFileGraph: true,
  },
  max_compression: {
    name: 'Max Token Compression',
    badge: 'Aggressive',
    inputProject: './src',
    outputFile: './lean-prompt.md',
    enableOllama: false,
    ollamaEndpoint: 'http://localhost:11434',
    ollamaModel: 'gemma4',
    tailwindMode: 'remove_aggr',
    tailwindThreshold: 32,
    summarizeFunctions: true,
    summarizeFunctionsThreshold: 3,
    generateRouteTable: true,
    analyzeReactComponents: true,
    generateFileGraph: true,
  },
};

const POPULAR_MODELS = [
  'gemma4',
  'gemma2:2b',
  'llama3.2',
  'qwen2.5-coder:1.5b',
  'mistral',
];

export default function ConfigGenerator() {
  // State variables for ALL config options
  const [inputProject, setInputProject] = useState('./src');
  const [outputFile, setOutputFile] = useState('./output.md');
  const [enableOllama, setEnableOllama] = useState(true);
  const [ollamaEndpoint, setOllamaEndpoint] = useState(
    'http://localhost:11434',
  );
  const [ollamaModel, setOllamaModel] = useState('gemma4');
  const [tailwindMode, setTailwindMode] = useState<TailwindMode>('remove');
  const [tailwindThreshold, setTailwindThreshold] = useState(96);
  const [summarizeFunctions, setSummarizeFunctions] = useState(true);
  const [summarizeThreshold, setSummarizeThreshold] = useState(5);
  const [generateRouteTable, setGenerateRouteTable] = useState(true);
  const [analyzeReact, setAnalyzeReact] = useState(true);
  const [generateFileGraph, setGenerateFileGraph] = useState(true);

  // UI view state
  const [viewMode, setViewMode] = useState<'jsonc' | 'cli'>('jsonc');
  const [copied, setCopied] = useState(false);

  // Apply Preset
  const applyPreset = (key: PresetKey) => {
    const p = PRESETS[key];
    setInputProject(p.inputProject);
    setOutputFile(p.outputFile);
    setEnableOllama(p.enableOllama);
    setOllamaEndpoint(p.ollamaEndpoint);
    setOllamaModel(p.ollamaModel);
    setTailwindMode(p.tailwindMode);
    setTailwindThreshold(p.tailwindThreshold);
    setSummarizeFunctions(p.summarizeFunctions);
    setSummarizeThreshold(p.summarizeFunctionsThreshold);
    setGenerateRouteTable(p.generateRouteTable);
    setAnalyzeReact(p.analyzeReactComponents);
    setGenerateFileGraph(p.generateFileGraph);
  };

  // Build JSONC string
  const jsoncOutput = useMemo(() => {
    const lines = [
      '{',
      '  "$schema": "https://sanjaiyan-dev.github.io/urai-ecma/json-schema/v0/config.schema.json",',
      '',
      '  // Path to the project directory or single source file',
      `  "input_project": "${inputProject}",`,
      '',
      '  // Target output Markdown file path',
      `  "output_file": "${outputFile}",`,
    ];

    if (enableOllama) {
      lines.push(
        '',
        '  // Ollama local endpoint URL (Optional, e.g., "http://localhost:11434")',
        `  "ollama_endpoint": "${ollamaEndpoint}",`,
        '',
        '  // Ollama Model Name (e.g., "gemma4", "llama3.2")',
        `  "ollama_modelname": "${ollamaModel}",`,
      );
    }

    lines.push(
      '',
      '  // Tailwind CSS pruning mode: "remove" | "remove_aggr" | "summarize" | "preserve"',
      `  "tailwind_mode": "${tailwindMode}",`,
      '',
      '  // Character length threshold for Tailwind pruning (default: 96 characters)',
      `  "tailwind_threshold": ${tailwindThreshold},`,
      '',
      '  // Summarize function block bodies using local Ollama or fallback to JSDoc comments',
      `  "summarize_functions": ${summarizeFunctions},`,
      '',
      '  // Line count threshold to trigger function summarization (default: 5 lines)',
      `  "summarize_functions_threshold": ${summarizeThreshold},`,
      '',
      '  // Extract and generate Express/Fastify/Next.js/NestJS API Route Table',
      `  "generate_route_table": ${generateRouteTable},`,
      '',
      '  // Analyze React / React Native components and output detailed explanations',
      `  "analyze_react_components": ${analyzeReact},`,
      '',
      '  // Generate ASCII File Structure & Module Dependency Graph',
      `  "generate_file_graph": ${generateFileGraph}`,
      '}',
    );

    return lines.join('\n');
  }, [
    inputProject,
    outputFile,
    enableOllama,
    ollamaEndpoint,
    ollamaModel,
    tailwindMode,
    tailwindThreshold,
    summarizeFunctions,
    summarizeThreshold,
    generateRouteTable,
    analyzeReact,
    generateFileGraph,
  ]);

  // Build equivalent CLI Command
  const cliOutput = useMemo(() => {
    const flags = ['urai-ecma'];
    flags.push(`-i "${inputProject}"`);
    flags.push(`-o "${outputFile}"`);

    if (enableOllama) {
      flags.push(`-e "${ollamaEndpoint}"`);
      flags.push(`-m "${ollamaModel}"`);
    }

    flags.push(`--tailwind-mode ${tailwindMode}`);
    flags.push(`--tailwind-threshold ${tailwindThreshold}`);
    flags.push(`--summarize-functions ${summarizeFunctions}`);
    flags.push(`--summarize-functions-threshold ${summarizeThreshold}`);
    flags.push(`--generate-route-table ${generateRouteTable}`);
    flags.push(`--analyze-react-components ${analyzeReact}`);
    flags.push(`--generate-file-graph ${generateFileGraph}`);

    return flags.join(' \\\n  ');
  }, [
    inputProject,
    outputFile,
    enableOllama,
    ollamaEndpoint,
    ollamaModel,
    tailwindMode,
    tailwindThreshold,
    summarizeFunctions,
    summarizeThreshold,
    generateRouteTable,
    analyzeReact,
    generateFileGraph,
  ]);

  // Estimated Impact Metrics
  const estimatedImpact = useMemo(() => {
    let reduction = 72;
    if (tailwindMode === 'remove_aggr') reduction += 12;
    if (tailwindMode === 'preserve') reduction -= 30;
    if (tailwindMode === 'summarize') reduction -= 8;
    if (!summarizeFunctions) reduction -= 15;

    const speed = enableOllama
      ? tailwindMode === 'summarize'
        ? '~3.2s (AI style calls)'
        : '~1.1s (Hybrid cached)'
      : '~0.08s (Pure Rust AST)';

    return {
      reduction: Math.min(Math.max(reduction, 35), 89),
      speed,
      offline: !enableOllama,
    };
  }, [tailwindMode, summarizeFunctions, enableOllama]);

  // Copy Action
  const handleCopy = () => {
    const textToCopy = viewMode === 'jsonc' ? jsoncOutput : cliOutput;
    navigator.clipboard.writeText(textToCopy);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  // Download Action
  const handleDownload = () => {
    const blob = new Blob([jsoncOutput], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'urai.config.jsonc';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  return (
    <div
      style={{
        background: 'var(--urai-glass-surface)',
        backdropFilter: 'var(--urai-glass-blur)',
        WebkitBackdropFilter: 'var(--urai-glass-blur)',
        border: 'var(--urai-glass-border)',
        borderRadius: 'var(--rp-radius-large)',
        padding: '1.75rem',
        margin: '2rem 0',
        boxShadow: 'var(--urai-shadow-3d)',
        position: 'relative',
      }}
    >
      {/* Header & Quick Action Row */}
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          flexWrap: 'wrap',
          gap: '1rem',
          borderBottom: '1px solid var(--rp-c-divider-light)',
          paddingBottom: '1.25rem',
          marginBottom: '1.5rem',
        }}
      >
        <div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <span className="urai-badge-terracotta">Interactive Studio</span>
            <span className="urai-badge-cyan">v0 Schema Compliant</span>
          </div>
          <h3 style={{ margin: '0.4rem 0 0 0', color: 'var(--rp-c-text-0)' }}>
            <code>urai.config.jsonc</code> Visual Architect
          </h3>
        </div>

        <div
          style={{
            display: 'flex',
            gap: '0.5rem',
            alignItems: 'center',
            flexWrap: 'wrap',
          }}
        >
          {/* View Mode Toggle */}
          <div
            style={{
              display: 'inline-flex',
              background: 'var(--rp-c-bg)',
              borderRadius: '8px',
              padding: '2px',
              border: '1px solid var(--rp-c-divider)',
            }}
          >
            <button
              onClick={() => setViewMode('jsonc')}
              style={{
                background:
                  viewMode === 'jsonc' ? 'var(--rp-c-brand)' : 'transparent',
                color: viewMode === 'jsonc' ? '#fff' : 'var(--rp-c-text-2)',
                border: 'none',
                borderRadius: '6px',
                padding: '4px 10px',
                fontSize: '0.74rem',
                fontWeight: 700,
                cursor: 'pointer',
              }}
            >
              JSONC
            </button>
            <button
              onClick={() => setViewMode('cli')}
              style={{
                background:
                  viewMode === 'cli' ? 'var(--rp-c-brand)' : 'transparent',
                color: viewMode === 'cli' ? '#fff' : 'var(--rp-c-text-2)',
                border: 'none',
                borderRadius: '6px',
                padding: '4px 10px',
                fontSize: '0.74rem',
                fontWeight: 700,
                cursor: 'pointer',
              }}
            >
              CLI Equivalent
            </button>
          </div>

          {/* Copy Button */}
          <button
            onClick={handleCopy}
            style={{
              background: copied ? 'var(--urai-moss)' : 'var(--rp-c-brand)',
              color: '#fff',
              border: 'none',
              borderRadius: '7px',
              padding: '6px 14px',
              fontSize: '0.78rem',
              fontWeight: 700,
              cursor: 'pointer',
              transition: 'var(--urai-transition-spring)',
              boxShadow: '0 2px 8px var(--urai-terracotta-glow)',
            }}
          >
            {copied ? '✓ Copied!' : '📋 Copy'}
          </button>

          {/* Download Button */}
          <button
            onClick={handleDownload}
            style={{
              background: 'var(--urai-glass-surface)',
              color: 'var(--urai-cyan)',
              border: '1px solid var(--urai-cyan)',
              borderRadius: '7px',
              padding: '6px 12px',
              fontSize: '0.78rem',
              fontWeight: 700,
              cursor: 'pointer',
              transition: 'var(--urai-transition-spring)',
            }}
          >
            💾 Download .jsonc
          </button>
        </div>
      </div>

      {/* 1-Click Preset Bar */}
      <div style={{ marginBottom: '1.5rem' }}>
        <div
          style={{
            fontSize: '0.72rem',
            fontWeight: 800,
            textTransform: 'uppercase',
            letterSpacing: '0.06em',
            color: 'var(--rp-c-text-2)',
            marginBottom: '0.5rem',
          }}
        >
          ⚡ Quick Presets (Click to load)
        </div>
        <div style={{ display: 'flex', gap: '0.5rem', flexWrap: 'wrap' }}>
          {(Object.keys(PRESETS) as PresetKey[]).map((key) => {
            const p = PRESETS[key];
            return (
              <button
                key={key}
                onClick={() => applyPreset(key)}
                style={{
                  background: 'var(--rp-c-bg)',
                  border: '1px solid var(--rp-c-divider-light)',
                  borderRadius: '6px',
                  padding: '5px 12px',
                  fontSize: '0.75rem',
                  fontWeight: 600,
                  color: 'var(--rp-c-text-1)',
                  cursor: 'pointer',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  transition: 'all 0.2s ease',
                }}
              >
                <span>{p.name}</span>
                <span
                  style={{
                    fontSize: '0.65rem',
                    padding: '1px 5px',
                    borderRadius: '4px',
                    background: 'rgba(0, 242, 254, 0.12)',
                    color: 'var(--urai-cyan)',
                    fontWeight: 700,
                  }}
                >
                  {p.badge}
                </span>
              </button>
            );
          })}
        </div>
      </div>

      {/* Live Impact Telemetry Bar */}
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(160px, 1fr))',
          gap: '0.85rem',
          marginBottom: '1.75rem',
          padding: '1rem',
          background: 'rgba(0, 0, 0, 0.15)',
          borderRadius: 'var(--rp-radius)',
          border: '1px solid var(--rp-c-divider-light)',
        }}
      >
        <div>
          <div style={{ fontSize: '0.72rem', color: 'var(--rp-c-text-2)' }}>
            Est. Token Reduction
          </div>
          <div
            style={{
              fontSize: '1.35rem',
              fontWeight: 800,
              color: 'var(--urai-moss)',
              fontFamily: 'var(--rp-font-family-mono)',
            }}
          >
            ~{estimatedImpact.reduction}%
          </div>
        </div>
        <div>
          <div style={{ fontSize: '0.72rem', color: 'var(--rp-c-text-2)' }}>
            SWC Latency / 50 Files
          </div>
          <div
            style={{
              fontSize: '1.35rem',
              fontWeight: 800,
              color: 'var(--urai-cyan)',
              fontFamily: 'var(--rp-font-family-mono)',
            }}
          >
            {estimatedImpact.speed}
          </div>
        </div>
        <div>
          <div style={{ fontSize: '0.72rem', color: 'var(--rp-c-text-2)' }}>
            Offline Air-Gap Safety
          </div>
          <div
            style={{
              fontSize: '1.35rem',
              fontWeight: 800,
              color: estimatedImpact.offline
                ? 'var(--urai-cyan)'
                : 'var(--rp-c-brand)',
              fontFamily: 'var(--rp-font-family-mono)',
            }}
          >
            {estimatedImpact.offline ? '100% Offline' : 'Local Ollama'}
          </div>
        </div>
      </div>

      {/* Interactive Controls Grid */}
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(260px, 1fr))',
          gap: '1.25rem',
          marginBottom: '1.5rem',
        }}
      >
        {/* Section 1: Paths */}
        <div
          style={{
            background: 'var(--rp-c-bg)',
            padding: '1rem',
            borderRadius: 'var(--rp-radius)',
            border: '1px solid var(--rp-c-divider-light)',
          }}
        >
          <div
            style={{
              fontSize: '0.75rem',
              fontWeight: 800,
              textTransform: 'uppercase',
              letterSpacing: '0.05em',
              color: 'var(--rp-c-brand)',
              marginBottom: '0.85rem',
            }}
          >
            📁 I/O Path Mapping
          </div>

          <div style={{ marginBottom: '0.75rem' }}>
            <label
              style={{
                fontSize: '0.75rem',
                fontWeight: 600,
                color: 'var(--rp-c-text-1)',
              }}
            >
              <code>input_project</code>
            </label>
            <input
              type="text"
              value={inputProject}
              onChange={(e) => setInputProject(e.target.value)}
              style={{
                width: '100%',
                padding: '6px 10px',
                marginTop: '4px',
                borderRadius: '6px',
                background: 'var(--rp-c-bg-mute)',
                color: 'var(--rp-c-text-0)',
                border: '1px solid var(--rp-c-divider)',
                fontFamily: 'var(--rp-font-family-mono)',
                fontSize: '0.8rem',
              }}
            />
          </div>

          <div>
            <label
              style={{
                fontSize: '0.75rem',
                fontWeight: 600,
                color: 'var(--rp-c-text-1)',
              }}
            >
              <code>output_file</code>
            </label>
            <input
              type="text"
              value={outputFile}
              onChange={(e) => setOutputFile(e.target.value)}
              style={{
                width: '100%',
                padding: '6px 10px',
                marginTop: '4px',
                borderRadius: '6px',
                background: 'var(--rp-c-bg-mute)',
                color: 'var(--rp-c-text-0)',
                border: '1px solid var(--rp-c-divider)',
                fontFamily: 'var(--rp-font-family-mono)',
                fontSize: '0.8rem',
              }}
            />
          </div>
        </div>

        {/* Section 2: Tailwind Pruner */}
        <div
          style={{
            background: 'var(--rp-c-bg)',
            padding: '1rem',
            borderRadius: 'var(--rp-radius)',
            border: '1px solid var(--rp-c-divider-light)',
          }}
        >
          <div
            style={{
              fontSize: '0.75rem',
              fontWeight: 800,
              textTransform: 'uppercase',
              letterSpacing: '0.05em',
              color: 'var(--rp-c-brand)',
              marginBottom: '0.85rem',
            }}
          >
            🎨 Tailwind CSS Pruner
          </div>

          <div style={{ marginBottom: '0.75rem' }}>
            <label
              style={{
                fontSize: '0.75rem',
                fontWeight: 600,
                color: 'var(--rp-c-text-1)',
              }}
            >
              <code>tailwind_mode</code>
            </label>
            <select
              value={tailwindMode}
              onChange={(e) => setTailwindMode(e.target.value as TailwindMode)}
              style={{
                width: '100%',
                padding: '6px 10px',
                marginTop: '4px',
                borderRadius: '6px',
                background: 'var(--rp-c-bg-mute)',
                color: 'var(--rp-c-text-0)',
                border: '1px solid var(--rp-c-divider)',
                fontSize: '0.8rem',
                fontWeight: 600,
              }}
            >
              <option value="remove">
                remove (Strip static classes &gt; threshold; keep clsx)
              </option>
              <option value="remove_aggr">
                remove_aggr (Aggressive purge of all static classes)
              </option>
              <option value="summarize">
                summarize (Ollama 1-line natural style intent)
              </option>
              <option value="preserve">
                preserve (Keep untouched for CSS debugging)
              </option>
            </select>
          </div>

          <div>
            <div
              style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
              }}
            >
              <label
                style={{
                  fontSize: '0.75rem',
                  fontWeight: 600,
                  color: 'var(--rp-c-text-1)',
                }}
              >
                <code>tailwind_threshold</code>
              </label>
              <span
                style={{
                  fontSize: '0.75rem',
                  fontFamily: 'var(--rp-font-family-mono)',
                  color: 'var(--urai-cyan)',
                  fontWeight: 700,
                }}
              >
                {tailwindThreshold} chars
              </span>
            </div>
            <input
              type="range"
              min="20"
              max="256"
              value={tailwindThreshold}
              onChange={(e) => setTailwindThreshold(Number(e.target.value))}
              style={{ width: '100%', marginTop: '8px' }}
            />
          </div>
        </div>

        {/* Section 3: Semantic Function Summarization */}
        <div
          style={{
            background: 'var(--rp-c-bg)',
            padding: '1rem',
            borderRadius: 'var(--rp-radius)',
            border: '1px solid var(--rp-c-divider-light)',
          }}
        >
          <div
            style={{
              fontSize: '0.75rem',
              fontWeight: 800,
              textTransform: 'uppercase',
              letterSpacing: '0.05em',
              color: 'var(--rp-c-brand)',
              marginBottom: '0.85rem',
            }}
          >
            🧠 Function & Method Stubs
          </div>

          <div style={{ marginBottom: '0.75rem' }}>
            <label
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
                cursor: 'pointer',
                fontSize: '0.78rem',
                fontWeight: 600,
                color: 'var(--rp-c-text-1)',
              }}
            >
              <input
                type="checkbox"
                checked={summarizeFunctions}
                onChange={(e) => setSummarizeFunctions(e.target.checked)}
                style={{ width: '16px', height: '16px' }}
              />
              <span>
                Enable <code>summarize_functions</code>
              </span>
            </label>
          </div>

          <div
            style={{
              opacity: summarizeFunctions ? 1 : 0.4,
              pointerEvents: summarizeFunctions ? 'auto' : 'none',
            }}
          >
            <div
              style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
              }}
            >
              <label
                style={{
                  fontSize: '0.75rem',
                  fontWeight: 600,
                  color: 'var(--rp-c-text-1)',
                }}
              >
                <code>summarize_functions_threshold</code>
              </label>
              <span
                style={{
                  fontSize: '0.75rem',
                  fontFamily: 'var(--rp-font-family-mono)',
                  color: 'var(--urai-cyan)',
                  fontWeight: 700,
                }}
              >
                {summarizeThreshold} lines
              </span>
            </div>
            <input
              type="range"
              min="2"
              max="30"
              value={summarizeThreshold}
              onChange={(e) => setSummarizeThreshold(Number(e.target.value))}
              style={{ width: '100%', marginTop: '8px' }}
            />
          </div>
        </div>

        {/* Section 4: Ollama LLM Connection */}
        <div
          style={{
            background: 'var(--rp-c-bg)',
            padding: '1rem',
            borderRadius: 'var(--rp-radius)',
            border: '1px solid var(--rp-c-divider-light)',
          }}
        >
          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
              marginBottom: '0.85rem',
            }}
          >
            <div
              style={{
                fontSize: '0.75rem',
                fontWeight: 800,
                textTransform: 'uppercase',
                letterSpacing: '0.05em',
                color: 'var(--rp-c-brand)',
              }}
            >
              🦙 Ollama Local AI
            </div>
            <label
              style={{
                fontSize: '0.72rem',
                display: 'flex',
                alignItems: 'center',
                gap: '5px',
                cursor: 'pointer',
              }}
            >
              <input
                type="checkbox"
                checked={enableOllama}
                onChange={(e) => setEnableOllama(e.target.checked)}
              />
              <span>Enabled</span>
            </label>
          </div>

          <div
            style={{
              opacity: enableOllama ? 1 : 0.4,
              pointerEvents: enableOllama ? 'auto' : 'none',
            }}
          >
            <div style={{ marginBottom: '0.65rem' }}>
              <label
                style={{
                  fontSize: '0.75rem',
                  fontWeight: 600,
                  color: 'var(--rp-c-text-1)',
                }}
              >
                <code>ollama_endpoint</code>
              </label>
              <input
                type="text"
                value={ollamaEndpoint}
                onChange={(e) => setOllamaEndpoint(e.target.value)}
                style={{
                  width: '100%',
                  padding: '6px 10px',
                  marginTop: '4px',
                  borderRadius: '6px',
                  background: 'var(--rp-c-bg-mute)',
                  color: 'var(--rp-c-text-0)',
                  border: '1px solid var(--rp-c-divider)',
                  fontFamily: 'var(--rp-font-family-mono)',
                  fontSize: '0.78rem',
                }}
              />
            </div>

            <div>
              <label
                style={{
                  fontSize: '0.75rem',
                  fontWeight: 600,
                  color: 'var(--rp-c-text-1)',
                }}
              >
                <code>ollama_modelname</code>
              </label>
              <input
                type="text"
                value={ollamaModel}
                onChange={(e) => setOllamaModel(e.target.value)}
                style={{
                  width: '100%',
                  padding: '6px 10px',
                  marginTop: '4px',
                  borderRadius: '6px',
                  background: 'var(--rp-c-bg-mute)',
                  color: 'var(--rp-c-text-0)',
                  border: '1px solid var(--rp-c-divider)',
                  fontFamily: 'var(--rp-font-family-mono)',
                  fontSize: '0.78rem',
                }}
              />
              <div
                style={{
                  display: 'flex',
                  gap: '4px',
                  marginTop: '6px',
                  flexWrap: 'wrap',
                }}
              >
                {POPULAR_MODELS.map((m) => (
                  <button
                    key={m}
                    onClick={() => setOllamaModel(m)}
                    style={{
                      background:
                        ollamaModel === m
                          ? 'var(--urai-cyan)'
                          : 'var(--rp-c-bg-mute)',
                      color: ollamaModel === m ? '#000' : 'var(--rp-c-text-2)',
                      border: 'none',
                      borderRadius: '4px',
                      padding: '2px 6px',
                      fontSize: '0.68rem',
                      fontWeight: 700,
                      cursor: 'pointer',
                    }}
                  >
                    {m}
                  </button>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Section 5: Feature Flags Row */}
      <div
        style={{
          background: 'var(--rp-c-bg)',
          padding: '0.85rem 1.25rem',
          borderRadius: 'var(--rp-radius)',
          border: '1px solid var(--rp-c-divider-light)',
          display: 'flex',
          justifyContent: 'space-around',
          flexWrap: 'wrap',
          gap: '1rem',
          marginBottom: '1.5rem',
        }}
      >
        <label
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            cursor: 'pointer',
            fontSize: '0.78rem',
            fontWeight: 600,
          }}
        >
          <input
            type="checkbox"
            checked={generateRouteTable}
            onChange={(e) => setGenerateRouteTable(e.target.checked)}
          />
          <span>
            🛣️ <code>generate_route_table</code>
          </span>
        </label>

        <label
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            cursor: 'pointer',
            fontSize: '0.78rem',
            fontWeight: 600,
          }}
        >
          <input
            type="checkbox"
            checked={analyzeReact}
            onChange={(e) => setAnalyzeReact(e.target.checked)}
          />
          <span>
            ⚛️ <code>analyze_react_components</code>
          </span>
        </label>

        <label
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            cursor: 'pointer',
            fontSize: '0.78rem',
            fontWeight: 600,
          }}
        >
          <input
            type="checkbox"
            checked={generateFileGraph}
            onChange={(e) => setGenerateFileGraph(e.target.checked)}
          />
          <span>
            📁 <code>generate_file_graph</code>
          </span>
        </label>
      </div>

      {/* Real-Time Live Code Output */}
      <div>
        <div
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            marginBottom: '0.5rem',
          }}
        >
          <span
            style={{
              fontSize: '0.75rem',
              fontWeight: 800,
              textTransform: 'uppercase',
              letterSpacing: '0.06em',
              color: 'var(--urai-cyan)',
            }}
          >
            {viewMode === 'jsonc'
              ? 'Generated urai.config.jsonc'
              : 'Equivalent CLI Terminal Command'}
          </span>
          <span style={{ fontSize: '0.72rem', color: 'var(--rp-c-text-2)' }}>
            {viewMode === 'jsonc'
              ? 'Directly recognized by urai-ecma'
              : 'Run without creating a config file'}
          </span>
        </div>

        <pre
          style={{
            margin: 0,
            padding: '1.25rem',
            background: 'var(--rp-code-block-bg)',
            border: '1px solid var(--urai-cyan)',
            borderRadius: 'var(--rp-radius)',
            fontSize: '0.78rem',
            overflowX: 'auto',
            lineHeight: 1.5,
            boxShadow: '0 4px 20px -2px rgba(0, 242, 254, 0.15)',
          }}
        >
          <code>{viewMode === 'jsonc' ? jsoncOutput : cliOutput}</code>
        </pre>
      </div>
    </div>
  );
}
