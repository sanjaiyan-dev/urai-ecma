import { useState } from 'react';

interface ModelPricing {
  name: string;
  inputPerMillion: number;
  outputPerMillion: number;
}

const MODEL_PRICING: Record<string, ModelPricing> = {
  'gpt-4o': {
    name: 'GPT-4o (o200k)',
    inputPerMillion: 2.5,
    outputPerMillion: 10.0,
  },
  'claude-3-5-sonnet': {
    name: 'Claude 3.5 Sonnet',
    inputPerMillion: 3.0,
    outputPerMillion: 15.0,
  },
  'gemini-1-5-pro': {
    name: 'Gemini 1.5 Pro',
    inputPerMillion: 1.25,
    outputPerMillion: 5.0,
  },
};

export function TokenTelemetryHud() {
  const [rawKTokens, setRawKTokens] = useState<number>(148);
  const [selectedModel, setSelectedModel] = useState<string>('gpt-4o');
  const [runsPerMonth, setRunsPerMonth] = useState<number>(500);

  // URAI Empirical Compression Baseline (SWC AST Pruning + Tailwind Strip)
  const reductionRate = 0.8012; // 80.12% average reduction
  const rawTokens = rawKTokens * 1000;
  const optimizedTokens = Math.round(rawTokens * (1 - reductionRate));
  const savedTokensPerRun = rawTokens - optimizedTokens;

  // Financial & Latency Math
  const model = MODEL_PRICING[selectedModel];
  const monthlyRawCost =
    (rawTokens / 1_000_000) * model.inputPerMillion * runsPerMonth;
  const monthlyUraiCost =
    (optimizedTokens / 1_000_000) * model.inputPerMillion * runsPerMonth;
  const monthlySavings = monthlyRawCost - monthlyUraiCost;
  const estimatedPrefillLatencyRaw = ((rawTokens / 1000) * 0.018).toFixed(2);
  const estimatedPrefillLatencyUrai = (
    (optimizedTokens / 1000) *
    0.018
  ).toFixed(2);

  return (
    <div
      style={{
        background: 'var(--urai-glass-surface)',
        backdropFilter: 'var(--urai-glass-blur)',
        WebkitBackdropFilter: 'var(--urai-glass-blur)',
        border: '1px solid rgba(0, 242, 254, 0.3)',
        borderRadius: 'var(--rp-radius)',
        padding: '1.75rem',
        margin: '2rem 0',
        boxShadow:
          '0 20px 45px -10px rgba(0,0,0,0.45), 0 0 20px -2px var(--urai-cyan-glow)',
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
          background: 'linear-gradient(90deg, #d97736, #00f2fe, #10b981)',
        }}
      />

      {/* Top Controls HUD */}
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
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <span className="urai-badge-cyan">Interactive Token Telemetry</span>
          <span className="urai-badge-terracotta">BPE o200k Engine</span>
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '0.75rem' }}>
          <label
            style={{
              fontSize: '0.78rem',
              textTransform: 'uppercase',
              color: 'var(--rp-c-text-2)',
              fontWeight: 700,
            }}
          >
            Target Model:
          </label>
          <select
            value={selectedModel}
            onChange={(e) => setSelectedModel(e.target.value)}
            style={{
              background: 'var(--rp-c-bg-mute)',
              color: 'var(--rp-c-text-0)',
              border: '1px solid var(--rp-c-divider)',
              borderRadius: '6px',
              padding: '4px 10px',
              fontSize: '0.8rem',
              fontFamily: 'var(--rp-font-family-mono)',
              outline: 'none',
              cursor: 'pointer',
            }}
          >
            <option value="gpt-4o">OpenAI GPT-4o</option>
            <option value="claude-3-5-sonnet">
              Anthropic Claude 3.5 Sonnet
            </option>
            <option value="gemini-1-5-pro">Google Gemini 1.5 Pro</option>
          </select>
        </div>
      </div>

      {/* Slider Interactive Inputs */}
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))',
          gap: '1.5rem',
          marginBottom: '1.5rem',
        }}
      >
        <div>
          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              marginBottom: '0.5rem',
            }}
          >
            <span
              style={{
                fontSize: '0.82rem',
                fontWeight: 600,
                color: 'var(--rp-c-text-1)',
              }}
            >
              Input Codebase Size
            </span>
            <span
              style={{
                fontFamily: 'var(--rp-font-family-mono)',
                color: 'var(--urai-cyan)',
                fontWeight: 700,
              }}
            >
              {rawKTokens}k tokens ({rawTokens.toLocaleString()} tokens)
            </span>
          </div>
          <input
            type="range"
            min="20"
            max="400"
            step="5"
            value={rawKTokens}
            onChange={(e) => setRawKTokens(Number(e.target.value))}
            style={{
              width: '100%',
              accentColor: 'var(--urai-cyan)',
              cursor: 'pointer',
            }}
          />
          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              fontSize: '0.7rem',
              color: 'var(--rp-c-text-3)',
            }}
          >
            <span>Small Module (20k)</span>
            <span>Enterprise Monorepo (400k)</span>
          </div>
        </div>

        <div>
          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              marginBottom: '0.5rem',
            }}
          >
            <span
              style={{
                fontSize: '0.82rem',
                fontWeight: 600,
                color: 'var(--rp-c-text-1)',
              }}
            >
              Monthly Developer Pipeline Runs
            </span>
            <span
              style={{
                fontFamily: 'var(--rp-font-family-mono)',
                color: '#d97736',
                fontWeight: 700,
              }}
            >
              {runsPerMonth} runs/mo
            </span>
          </div>
          <input
            type="range"
            min="50"
            max="3000"
            step="50"
            value={runsPerMonth}
            onChange={(e) => setRunsPerMonth(Number(e.target.value))}
            style={{ width: '100%', accentColor: '#d97736', cursor: 'pointer' }}
          />
          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              fontSize: '0.7rem',
              color: 'var(--rp-c-text-3)',
            }}
          >
            <span>Solo Engineer (50)</span>
            <span>Engineering Org (3,000)</span>
          </div>
        </div>
      </div>

      {/* Visual Relative Token Bar */}
      <div style={{ marginBottom: '1.75rem' }}>
        <div
          style={{
            fontSize: '0.75rem',
            textTransform: 'uppercase',
            letterSpacing: '0.05em',
            color: 'var(--rp-c-text-3)',
            marginBottom: '6px',
          }}
        >
          Context Window Load Comparison
        </div>
        <div
          style={{
            position: 'relative',
            height: '24px',
            background: 'rgba(0,0,0,0.35)',
            borderRadius: '12px',
            overflow: 'hidden',
            border: '1px solid var(--rp-c-divider-light)',
          }}
        >
          {/* Raw Bar (Background red) */}
          <div
            style={{
              position: 'absolute',
              top: 0,
              left: 0,
              bottom: 0,
              width: '100%',
              background: 'rgba(239, 68, 68, 0.45)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'flex-end',
              paddingRight: '12px',
              fontSize: '0.7rem',
              fontFamily: 'var(--rp-font-family-mono)',
              color: '#fca5a5',
            }}
          >
            Raw Repo Dump (100%)
          </div>

          {/* URAI Optimized Bar (Foreground Cyan) */}
          <div
            style={{
              position: 'absolute',
              top: 0,
              left: 0,
              bottom: 0,
              width: `${((1 - reductionRate) * 100).toFixed(1)}%`,
              background: 'linear-gradient(90deg, #b85d20, #00f2fe)',
              boxShadow: '0 0 16px var(--urai-cyan-glow)',
              display: 'flex',
              alignItems: 'center',
              paddingLeft: '10px',
              fontSize: '0.7rem',
              fontFamily: 'var(--rp-font-family-mono)',
              color: '#090c13',
              fontWeight: 800,
            }}
          >
            URAI (19.16%)
          </div>
        </div>
      </div>

      {/* Metric Tiles HUD */}
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(160px, 1fr))',
          gap: '1rem',
        }}
      >
        <div
          style={{
            padding: '1rem',
            background: 'rgba(0,0,0,0.3)',
            borderRadius: '8px',
            border: '1px solid var(--rp-c-divider-light)',
          }}
        >
          <div
            style={{
              fontSize: '0.7rem',
              color: 'var(--rp-c-text-3)',
              textTransform: 'uppercase',
            }}
          >
            Optimized Prompt
          </div>
          <div
            style={{
              fontSize: '1.45rem',
              fontWeight: 800,
              fontFamily: 'var(--rp-font-family-mono)',
              color: 'var(--urai-cyan)',
              textShadow: '0 0 12px var(--urai-cyan-glow)',
            }}
          >
            {optimizedTokens.toLocaleString()}
          </div>
          <div
            style={{ fontSize: '0.72rem', color: '#10b981', fontWeight: 600 }}
          >
            -80.12% token drop
          </div>
        </div>

        <div
          style={{
            padding: '1rem',
            background: 'rgba(0,0,0,0.3)',
            borderRadius: '8px',
            border: '1px solid var(--rp-c-divider-light)',
          }}
        >
          <div
            style={{
              fontSize: '0.7rem',
              color: 'var(--rp-c-text-3)',
              textTransform: 'uppercase',
            }}
          >
            Monthly Cost Savings
          </div>
          <div
            style={{
              fontSize: '1.45rem',
              fontWeight: 800,
              fontFamily: 'var(--rp-font-family-mono)',
              color: '#10b981',
            }}
          >
            ${monthlySavings.toFixed(2)}
          </div>
          <div style={{ fontSize: '0.72rem', color: 'var(--rp-c-text-3)' }}>
            vs ${monthlyRawCost.toFixed(2)} raw cost
          </div>
        </div>

        <div
          style={{
            padding: '1rem',
            background: 'rgba(0,0,0,0.3)',
            borderRadius: '8px',
            border: '1px solid var(--rp-c-divider-light)',
          }}
        >
          <div
            style={{
              fontSize: '0.7rem',
              color: 'var(--rp-c-text-3)',
              textTransform: 'uppercase',
            }}
          >
            Tokens Saved / Run
          </div>
          <div
            style={{
              fontSize: '1.45rem',
              fontWeight: 800,
              fontFamily: 'var(--rp-font-family-mono)',
              color: '#d97736',
            }}
          >
            ~{(savedTokensPerRun / 1000).toFixed(0)}k
          </div>
          <div style={{ fontSize: '0.72rem', color: 'var(--rp-c-text-3)' }}>
            {savedTokensPerRun.toLocaleString()} tokens preserved
          </div>
        </div>

        <div
          style={{
            padding: '1rem',
            background: 'rgba(0,0,0,0.3)',
            borderRadius: '8px',
            border: '1px solid var(--rp-c-divider-light)',
          }}
        >
          <div
            style={{
              fontSize: '0.7rem',
              color: 'var(--rp-c-text-3)',
              textTransform: 'uppercase',
            }}
          >
            Prefill Latency (TTFT)
          </div>
          <div
            style={{
              fontSize: '1.45rem',
              fontWeight: 800,
              fontFamily: 'var(--rp-font-family-mono)',
              color: '#00f2fe',
            }}
          >
            {estimatedPrefillLatencyUrai}s
          </div>
          <div style={{ fontSize: '0.72rem', color: 'var(--rp-c-text-3)' }}>
            Down from {estimatedPrefillLatencyRaw}s raw
          </div>
        </div>
      </div>
    </div>
  );
}
