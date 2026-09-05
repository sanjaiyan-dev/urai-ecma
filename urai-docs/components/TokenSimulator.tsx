import { useState } from 'react';

type Mode = 'remove' | 'remove_aggr' | 'summarize' | 'preserve';

const CODE_RAW = `export function UserProfileCard({ user, onSelect }: ProfileCardProps) {
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
}`;

const CODE_MODES: Record<
  Mode,
  { title: string; desc: string; output: string; tokens: number }
> = {
  remove: {
    title: 'remove (Default)',
    desc: 'Strips static strings exceeding threshold while preserving dynamic clsx(...) expressions and React hooks.',
    output: `### React Component Breakdown: <UserProfileCard>
- **Props**: \`user\` (type: \`User\`), \`onSelect\` (type: \`(id: string) => void\`)
- **State Management**: Manages state \`isHovered\` via setter \`setIsHovered\`
- **Hooks**: Uses \`useState, useEffect\` (Total Side-Effects: 1)
- **Event Handlers**: \`onClick\`

\`\`\`tsx
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
      <button onClick={handleCardClick}>Select Profile</button>
    </div>
  );
}
\`\`\``,
    tokens: 72,
  },
  summarize: {
    title: 'summarize (Ollama AI)',
    desc: 'Sends class strings to local Ollama (Gemma 2 / Llama 3) for 1-line style descriptions.',
    output: `\`\`\`tsx
export function UserProfileCard({ user, onSelect }: ProfileCardProps) {
  useEffect(() => { trackImpression(user.id); }, [user.id]);

  return (
    <div className="/* UI: Responsive dark/light frosted card with hover depth */">
      <h2 className={clsx("text-lg font-bold", isHovered ? "text-emerald-500" : "text-zinc-500")}>
        {user.name} ({initials})
      </h2>
      <p className="/* UI: Subtitle text with muted margin */">Member since: {formattedDate}</p>
      <button onClick={handleCardClick} className="/* UI: Indigo primary action button with shadow */">
        Select Profile
      </button>
    </div>
  );
}
\`\`\``,
    tokens: 94,
  },
  remove_aggr: {
    title: 'remove_aggr (Aggressive)',
    desc: 'Eliminates every static class name unconditionally. Maximum token compression.',
    output: `\`\`\`tsx
export function UserProfileCard({ user, onSelect }: ProfileCardProps) {
  useEffect(() => { trackImpression(user.id); }, [user.id]);

  return (
    <div>
      <h2 className={clsx(isHovered ? "text-emerald-500" : "text-zinc-500")}>
        {user.name} ({initials})
      </h2>
      <p>Member since: {formattedDate}</p>
      <button onClick={handleCardClick}>Select Profile</button>
    </div>
  );
}
\`\`\``,
    tokens: 58,
  },
  preserve: {
    title: 'preserve (Full CSS)',
    desc: 'Leaves all classes and styles untouched for pixel-perfect CSS debugging.',
    output: `\`\`\`tsx
export function UserProfileCard({ user, onSelect }: ProfileCardProps) {
  useEffect(() => { trackImpression(user.id); }, [user.id]);

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
\`\`\``,
    tokens: 184,
  },
};

export default function TokenSavingSimulator() {
  const [activeMode, setActiveMode] = useState<Mode>('remove');
  const rawTokens = 312; // Raw UserProfileCard component token weight
  const current = CODE_MODES[activeMode];
  const savedTokens = rawTokens - current.tokens;
  const reduction = ((savedTokens / rawTokens) * 100).toFixed(1);
  const costSavings = (savedTokens * 0.000005 * 100000).toFixed(2);

  return (
    <div
      style={{
        background: 'var(--urai-glass-surface)',
        backdropFilter: 'var(--urai-glass-blur)',
        border: 'var(--urai-glass-border)',
        borderRadius: 'var(--rp-radius-large)',
        padding: '1.75rem',
        margin: '2rem 0',
        boxShadow: 'var(--urai-shadow-3d)',
      }}
    >
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          flexWrap: 'wrap',
          gap: '1rem',
        }}
      >
        <div>
          <span className="urai-badge-cyan">
            SWC AST Walk + tiktoken (o200k_base)
          </span>
          <h3 style={{ margin: '0.4rem 0 0 0', color: 'var(--rp-c-text-0)' }}>
            Live Codebase-to-Prompt Optimizer
          </h3>
        </div>

        {/* Mode selector buttons */}
        <div style={{ display: 'flex', gap: '0.4rem', flexWrap: 'wrap' }}>
          {(Object.keys(CODE_MODES) as Mode[]).map((mode) => (
            <button
              key={mode}
              onClick={() => setActiveMode(mode)}
              style={{
                background:
                  activeMode === mode
                    ? 'var(--rp-c-brand)'
                    : 'var(--rp-c-bg-mute)',
                color: activeMode === mode ? '#ffffff' : 'var(--rp-c-text-1)',
                border:
                  activeMode === mode
                    ? '1px solid #b85d20'
                    : '1px solid var(--rp-c-divider-light)',
                borderRadius: '6px',
                padding: '6px 14px',
                fontSize: '0.76rem',
                fontWeight: 700,
                cursor: 'pointer',
                transition: 'var(--urai-transition-spring)',
              }}
            >
              {mode}
            </button>
          ))}
        </div>
      </div>

      <p
        style={{
          fontSize: '0.85rem',
          color: 'var(--rp-c-text-2)',
          margin: '0.75rem 0 1.25rem 0',
        }}
      >
        {current.desc}
      </p>

      {/* Telemetry HUD metrics */}
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(140px, 1fr))',
          gap: '0.85rem',
          marginBottom: '1.5rem',
        }}
      >
        <div
          style={{
            background: 'rgba(255,255,255,0.03)',
            padding: '0.85rem',
            borderRadius: '8px',
            border: '1px solid var(--rp-c-divider-light)',
          }}
        >
          <div
            style={{
              fontSize: '0.72rem',
              color: 'var(--rp-c-text-2)',
              fontWeight: 600,
            }}
          >
            Raw Source (BPE)
          </div>
          <div
            style={{
              fontSize: '1.5rem',
              fontWeight: 800,
              color: 'var(--rp-c-text-0)',
              fontFamily: 'var(--rp-font-family-mono)',
            }}
          >
            {rawTokens}
          </div>
        </div>
        <div
          style={{
            background: 'rgba(217, 119, 54, 0.08)',
            padding: '0.85rem',
            borderRadius: '8px',
            border: '1px solid rgba(217, 119, 54, 0.25)',
          }}
        >
          <div
            style={{
              fontSize: '0.72rem',
              color: 'var(--rp-c-text-2)',
              fontWeight: 600,
            }}
          >
            urai-ecma Output
          </div>
          <div
            style={{
              fontSize: '1.5rem',
              fontWeight: 800,
              color: 'var(--rp-c-brand)',
              fontFamily: 'var(--rp-font-family-mono)',
            }}
          >
            {current.tokens}
          </div>
        </div>
        <div
          style={{
            background: 'rgba(16, 185, 129, 0.08)',
            padding: '0.85rem',
            borderRadius: '8px',
            border: '1px solid rgba(16, 185, 129, 0.25)',
          }}
        >
          <div
            style={{
              fontSize: '0.72rem',
              color: 'var(--rp-c-text-2)',
              fontWeight: 600,
            }}
          >
            Token Reduction
          </div>
          <div
            style={{
              fontSize: '1.5rem',
              fontWeight: 800,
              color: 'var(--urai-moss)',
              fontFamily: 'var(--rp-font-family-mono)',
            }}
          >
            -{reduction}%
          </div>
        </div>
        <div
          style={{
            background: 'rgba(0, 242, 254, 0.08)',
            padding: '0.85rem',
            borderRadius: '8px',
            border: '1px solid rgba(0, 242, 254, 0.3)',
          }}
        >
          <div
            style={{
              fontSize: '0.72rem',
              color: 'var(--rp-c-text-2)',
              fontWeight: 600,
            }}
          >
            Saved / 100k Calls
          </div>
          <div
            style={{
              fontSize: '1.5rem',
              fontWeight: 800,
              color: 'var(--urai-cyan)',
              fontFamily: 'var(--rp-font-family-mono)',
            }}
          >
            ${costSavings}
          </div>
        </div>
      </div>

      {/* Code comparison split */}
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
              fontSize: '0.75rem',
              fontWeight: 700,
              textTransform: 'uppercase',
              color: 'var(--rp-c-text-2)',
              marginBottom: '0.5rem',
            }}
          >
            ❌ Raw Exhaustive Code (React + Tailwind)
          </div>
          <pre
            style={{
              margin: 0,
              padding: '1rem',
              background: 'var(--rp-code-block-bg)',
              border: 'var(--rp-code-block-border)',
              borderRadius: '8px',
              fontSize: '0.75rem',
              overflowX: 'auto',
              maxHeight: '280px',
              lineHeight: 1.5,
            }}
          >
            <code>{CODE_RAW}</code>
          </pre>
        </div>
        <div>
          <div
            style={{
              fontSize: '0.75rem',
              fontWeight: 700,
              textTransform: 'uppercase',
              color: 'var(--urai-cyan)',
              marginBottom: '0.5rem',
            }}
          >
            ✅ URAI Optimized Prompt (Mode: {activeMode})
          </div>
          <pre
            style={{
              margin: 0,
              padding: '1rem',
              background: 'var(--rp-code-block-bg)',
              border: '1px solid var(--urai-cyan)',
              borderRadius: '8px',
              fontSize: '0.75rem',
              overflowX: 'auto',
              maxHeight: '280px',
              lineHeight: 1.5,
            }}
          >
            <code>{current.output}</code>
          </pre>
        </div>
      </div>
    </div>
  );
}
