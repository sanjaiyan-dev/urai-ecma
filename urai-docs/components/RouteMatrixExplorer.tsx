import { useState } from 'react';

interface MockRoute {
  framework: 'Next.js' | 'NestJS' | 'Express' | 'Fastify';
  method: 'GET' | 'POST' | 'PUT' | 'DELETE';
  path: string;
  handler: string;
  file: string;
}

const SAMPLE_ROUTES: MockRoute[] = [
  {
    framework: 'Next.js',
    method: 'GET',
    path: '/api/v1/users',
    handler: 'GET',
    file: 'app/api/v1/users/route.ts:5',
  },
  {
    framework: 'Next.js',
    method: 'POST',
    path: '/api/v1/users',
    handler: 'POST',
    file: 'app/api/v1/users/route.ts:18',
  },
  {
    framework: 'NestJS',
    method: 'POST',
    path: '/v1/billing/checkout',
    handler: 'BillingController::checkout',
    file: 'billing.controller.ts:14',
  },
  {
    framework: 'NestJS',
    method: 'GET',
    path: '/v1/billing/invoices',
    handler: 'BillingController::getInvoices',
    file: 'billing.controller.ts:28',
  },
  {
    framework: 'Express',
    method: 'GET',
    path: '/health',
    handler: 'anonymous_handler',
    file: 'server.ts:42',
  },
  {
    framework: 'Fastify',
    method: 'DELETE',
    path: '/session/:id',
    handler: 'anonymous_handler',
    file: 'routes/auth.ts:19',
  },
];

export default function RouteMatrixExplorer() {
  const [filter, setFilter] = useState<string>('ALL');

  const filtered =
    filter === 'ALL'
      ? SAMPLE_ROUTES
      : SAMPLE_ROUTES.filter((r) => r.framework === filter);

  return (
    <div
      style={{
        background: 'var(--urai-glass-surface)',
        backdropFilter: 'var(--urai-glass-blur)',
        border: 'var(--urai-glass-border)',
        borderRadius: 'var(--rp-radius)',
        padding: '1.25rem',
        margin: '2rem 0',
      }}
    >
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          marginBottom: '1rem',
          flexWrap: 'wrap',
          gap: '0.5rem',
        }}
      >
        <h4 style={{ margin: 0, color: 'var(--rp-c-text-0)' }}>
          🛣️ Auto-Discovered Route Table Matrix
        </h4>
        <div style={{ display: 'flex', gap: '0.4rem' }}>
          {['ALL', 'Next.js', 'NestJS', 'Express', 'Fastify'].map((fw) => (
            <button
              key={fw}
              onClick={() => setFilter(fw)}
              style={{
                background: filter === fw ? 'var(--rp-c-brand)' : 'transparent',
                color: filter === fw ? '#fff' : 'var(--rp-c-text-2)',
                border: '1px solid var(--rp-c-divider-light)',
                borderRadius: '6px',
                padding: '4px 10px',
                fontSize: '0.72rem',
                fontWeight: 600,
                cursor: 'pointer',
              }}
            >
              {fw}
            </button>
          ))}
        </div>
      </div>

      <div style={{ overflowX: 'auto' }}>
        <table style={{ margin: 0 }}>
          <thead>
            <tr>
              <th>Framework</th>
              <th>Method</th>
              <th>Path</th>
              <th>Handler</th>
              <th>Location</th>
            </tr>
          </thead>
          <tbody>
            {filtered.map((route, i) => (
              <tr key={i}>
                <td>
                  <strong>{route.framework}</strong>
                </td>
                <td>
                  <span
                    className={
                      route.method === 'GET'
                        ? 'urai-badge-cyan'
                        : 'urai-badge-terracotta'
                    }
                  >
                    {route.method}
                  </span>
                </td>
                <td>
                  <code>{route.path}</code>
                </td>
                <td>
                  <code>{route.handler}</code>
                </td>
                <td>
                  <small>{route.file}</small>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
