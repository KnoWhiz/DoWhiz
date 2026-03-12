import { useEffect, useMemo, useState } from 'react';
import { createClient } from '@supabase/supabase-js';
import { getDoWhizApiBaseUrl } from './analytics';
import './dashboard.css';

const supabase = createClient(
  'https://resmseutzmwumflevfqw.supabase.co',
  'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InJlc21zZXV0em13dW1mbGV2ZnF3Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3NzAxNTQ1MjIsImV4cCI6MjA4NTczMDUyMn0.-QMndwi4m8nBtjMeS5WbDmrHZSe2l1UFY-UQJCl0Frc'
);

const RANGE_OPTIONS = [
  { label: 'Last 7 days', value: '7d' },
  { label: 'Last 30 days', value: '30d' },
  { label: 'Last 90 days', value: '90d' },
  { label: 'Last 180 days', value: '180d' }
];

const clampPercent = (value) => {
  if (!Number.isFinite(value)) return 0;
  return Math.max(0, Math.min(1, value));
};

const formatNumber = (value) => {
  if (!Number.isFinite(value)) return '0';
  return new Intl.NumberFormat('en-US').format(value);
};

const formatPercent = (value) => `${(clampPercent(value) * 100).toFixed(1)}%`;

const formatHours = (value) => {
  if (!Number.isFinite(value)) return 'N/A';
  return `${value.toFixed(1)}h`;
};

const formatCurrency = (value) => {
  if (!Number.isFinite(value)) return '$0';
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
    maximumFractionDigits: 0
  }).format(value);
};

function Section({ title, subtitle, children }) {
  return (
    <section className="dash-section">
      <div className="dash-section-head">
        <h2>{title}</h2>
        {subtitle ? <p>{subtitle}</p> : null}
      </div>
      {children}
    </section>
  );
}

function EmptyState({ label }) {
  return <div className="dash-empty">{label}</div>;
}

function BreakdownTable({ rows, firstCol, secondCol = 'Count', rateCol = 'Rate' }) {
  if (!rows?.length) {
    return <EmptyState label="No data in selected range." />;
  }

  return (
    <div className="dash-table-wrap">
      <table className="dash-table">
        <thead>
          <tr>
            <th>{firstCol}</th>
            <th>{secondCol}</th>
            <th>{rateCol}</th>
          </tr>
        </thead>
        <tbody>
          {rows.map((row) => (
            <tr key={`${firstCol}-${row.key}`}>
              <td>{row.key}</td>
              <td>{formatNumber(row.count ?? row.visitors ?? 0)}</td>
              <td>{formatPercent(row.rate ?? row.signup_conversion_rate ?? 0)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function AcquisitionTable({ rows, title }) {
  return (
    <div className="dash-card">
      <h3>{title}</h3>
      {!rows?.length ? (
        <EmptyState label="No data in selected range." />
      ) : (
        <div className="dash-table-wrap">
          <table className="dash-table">
            <thead>
              <tr>
                <th>Segment</th>
                <th>Visitors</th>
                <th>Signups</th>
                <th>Activated</th>
                <th>Paid</th>
                <th>Signup CVR</th>
                <th>Activation</th>
                <th>Paid CVR</th>
              </tr>
            </thead>
            <tbody>
              {rows.map((row) => (
                <tr key={`${title}-${row.key}`}>
                  <td>{row.key}</td>
                  <td>{formatNumber(row.visitors)}</td>
                  <td>{formatNumber(row.signups)}</td>
                  <td>{formatNumber(row.activated)}</td>
                  <td>{formatNumber(row.paid)}</td>
                  <td>{formatPercent(row.signup_conversion_rate)}</td>
                  <td>{formatPercent(row.activation_rate)}</td>
                  <td>{formatPercent(row.paid_conversion_rate)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}

function TrendBars({ points, title }) {
  if (!points?.length) {
    return (
      <div className="dash-card">
        <h3>{title}</h3>
        <EmptyState label="No trend data in selected range." />
      </div>
    );
  }

  const trimmed = points.slice(-21);
  const max = Math.max(...trimmed.map((point) => point.count), 1);

  return (
    <div className="dash-card">
      <h3>{title}</h3>
      <div className="dash-bars" role="img" aria-label={title}>
        {trimmed.map((point) => {
          const height = Math.max((point.count / max) * 100, point.count > 0 ? 6 : 2);
          return (
            <div key={`${title}-${point.day}`} className="dash-bar-group" title={`${point.day}: ${point.count}`}>
              <div className="dash-bar" style={{ height: `${height}%` }} />
              <span>{point.day.slice(5)}</span>
            </div>
          );
        })}
      </div>
    </div>
  );
}

function DashboardPage() {
  const [range, setRange] = useState('30d');
  const [loading, setLoading] = useState(true);
  const [refreshTick, setRefreshTick] = useState(0);
  const [session, setSession] = useState(null);
  const [error, setError] = useState('');
  const [dashboard, setDashboard] = useState(null);

  useEffect(() => {
    document.title = 'DoWhiz Internal Funnel Dashboard';

    let robots = document.querySelector('meta[name="robots"]');
    if (!robots) {
      robots = document.createElement('meta');
      robots.setAttribute('name', 'robots');
      document.head.appendChild(robots);
    }
    const previousRobots = robots.getAttribute('content');
    robots.setAttribute('content', 'noindex, nofollow');

    return () => {
      if (previousRobots) {
        robots.setAttribute('content', previousRobots);
      } else {
        robots.removeAttribute('content');
      }
    };
  }, []);

  useEffect(() => {
    let cancelled = false;

    const loadDashboard = async () => {
      setLoading(true);
      setError('');

      const {
        data: { session: currentSession }
      } = await supabase.auth.getSession();

      if (cancelled) return;

      setSession(currentSession ?? null);
      if (!currentSession) {
        setDashboard(null);
        setLoading(false);
        return;
      }

      try {
        const res = await fetch(`${getDoWhizApiBaseUrl()}/analytics/dashboard?range=${encodeURIComponent(range)}`, {
          headers: {
            Authorization: `Bearer ${currentSession.access_token}`
          }
        });

        if (!res.ok) {
          const body = await res.json().catch(() => ({}));
          const message =
            body.error ||
            (res.status === 403
              ? 'This dashboard is admin-only. Your authenticated user is not allowlisted.'
              : 'Failed to load dashboard data.');
          throw new Error(message);
        }

        const payload = await res.json();
        if (!cancelled) {
          setDashboard(payload);
        }
      } catch (fetchError) {
        if (!cancelled) {
          setDashboard(null);
          setError(fetchError instanceof Error ? fetchError.message : 'Failed to load dashboard data.');
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    };

    loadDashboard();

    return () => {
      cancelled = true;
    };
  }, [range, refreshTick]);

  const generatedAtLabel = useMemo(() => {
    if (!dashboard?.generated_at) {
      return null;
    }
    return new Date(dashboard.generated_at).toLocaleString();
  }, [dashboard]);

  if (!loading && !session) {
    return (
      <div className="dash-shell">
        <div className="dash-panel dash-auth-required">
          <h1>Internal Analytics Dashboard</h1>
          <p>You need a signed-in DoWhiz account to access this page.</p>
          <a className="dash-btn" href="/auth/index.html?loggedIn=true">
            Sign in to continue
          </a>
        </div>
      </div>
    );
  }

  return (
    <div className="dash-shell">
      <div className="dash-panel">
        <header className="dash-header">
          <div>
            <h1>DoWhiz Internal Funnel Dashboard</h1>
            <p>
              End-to-end funnel visibility from first touch to paid conversion. Revenue here reflects purchased
              credits in the selected date range.
            </p>
          </div>
          <div className="dash-header-controls">
            <label htmlFor="range-select">Date range</label>
            <select id="range-select" value={range} onChange={(event) => setRange(event.target.value)}>
              {RANGE_OPTIONS.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
            <button type="button" className="dash-btn" onClick={() => setRefreshTick((tick) => tick + 1)}>
              Refresh
            </button>
          </div>
        </header>

        <div className="dash-meta-row">
          <span>
            Signed in as <strong>{session?.user?.email || 'unknown'}</strong>
          </span>
          {dashboard?.range ? (
            <span>
              Window: {new Date(dashboard.range.start).toLocaleDateString()} to{' '}
              {new Date(dashboard.range.end).toLocaleDateString()} ({dashboard.range.days}d)
            </span>
          ) : null}
          {generatedAtLabel ? <span>Generated: {generatedAtLabel}</span> : null}
        </div>

        {loading ? <EmptyState label="Loading analytics data..." /> : null}
        {error ? <div className="dash-error">{error}</div> : null}

        {!loading && !error && dashboard ? (
          <>
            <Section title="Executive KPI Row" subtitle="Top-line conversion, activation, paid, and retention indicators.">
              <div className="dash-kpi-grid">
                <article className="dash-kpi-card">
                  <h3>Unique visitors</h3>
                  <p>{formatNumber(dashboard.kpis.unique_visitors)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Signup conversion</h3>
                  <p>{formatPercent(dashboard.kpis.signup_conversion_rate)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Activation rate</h3>
                  <p>{formatPercent(dashboard.kpis.activation_rate)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Paid conversion</h3>
                  <p>{formatPercent(dashboard.kpis.activation_to_paid_rate)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Time to first value</h3>
                  <p>{formatHours(dashboard.kpis.median_time_to_first_value_hours)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>D7 retention</h3>
                  <p>{formatPercent(dashboard.kpis.d7_retention_rate)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Active workspaces</h3>
                  <p>{formatNumber(dashboard.kpis.active_workspaces)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Paid accounts (30d)</h3>
                  <p>{formatNumber(dashboard.kpis.active_paid_accounts_30d)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Revenue (range)</h3>
                  <p>{formatCurrency(dashboard.kpis.revenue_usd)}</p>
                </article>
              </div>
            </Section>

            <Section title="Main Funnel" subtitle="Ordered funnel from first visit to active paid state.">
              {!dashboard.funnel?.steps?.length ? (
                <EmptyState label="No funnel events in selected range." />
              ) : (
                <div className="dash-funnel">
                  {dashboard.funnel.steps.map((step) => (
                    <article className="dash-funnel-step" key={step.event_name}>
                      <div className="dash-funnel-title-row">
                        <h3>{step.label}</h3>
                        <span>{formatNumber(step.identities)}</span>
                      </div>
                      <div className="dash-funnel-bar" aria-hidden="true">
                        <div
                          className="dash-funnel-bar-fill"
                          style={{ width: `${(clampPercent(step.overall_conversion_rate) * 100).toFixed(1)}%` }}
                        />
                      </div>
                      <div className="dash-funnel-metrics">
                        <span>Step conversion: {formatPercent(step.step_conversion_rate)}</span>
                        <span>Overall conversion: {formatPercent(step.overall_conversion_rate)}</span>
                      </div>
                    </article>
                  ))}
                </div>
              )}
            </Section>

            <Section
              title="Acquisition Breakdown"
              subtitle="Compare source/campaign, referrer, and device segments through signup, activation, and paid conversion."
            >
              <div className="dash-grid-2">
                <AcquisitionTable rows={dashboard.acquisition?.by_source_campaign} title="UTM source / medium / campaign" />
                <AcquisitionTable rows={dashboard.acquisition?.by_referrer} title="Referrer" />
                <AcquisitionTable rows={dashboard.acquisition?.by_device_type} title="Device type" />
                <AcquisitionTable rows={dashboard.acquisition?.by_landing_variant} title="Landing page variant" />
              </div>
            </Section>

            <Section
              title="Activation Breakdown"
              subtitle="Onboarding behavior that correlates with first task success and deeper usage."
            >
              <div className="dash-grid-2">
                <div className="dash-card">
                  <h3>Signup auth method</h3>
                  <BreakdownTable rows={dashboard.activation?.by_auth_method} firstCol="Auth method" />
                </div>
                <div className="dash-card">
                  <h3>Workspace type</h3>
                  <BreakdownTable rows={dashboard.activation?.by_workspace_type} firstCol="Workspace" />
                </div>
                <div className="dash-card">
                  <h3>Connected channel / tool type</h3>
                  <BreakdownTable rows={dashboard.activation?.by_connected_channel_type} firstCol="Channel/Tool" />
                </div>
                <div className="dash-card">
                  <h3>First task type</h3>
                  <BreakdownTable rows={dashboard.activation?.by_first_task_type} firstCol="Task type" />
                </div>
              </div>
              <div className="dash-rate-row">
                <article className="dash-kpi-card">
                  <h3>Agent/workflow creation rate</h3>
                  <p>{formatPercent(dashboard.activation?.agent_or_workflow_creation_rate || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Multi-channel connection rate</h3>
                  <p>{formatPercent(dashboard.activation?.multi_channel_connection_rate || 0)}</p>
                </article>
              </div>
            </Section>

            <Section title="Monetization" subtitle="Upgrade intent, checkout flow, successful payment, and paid state activation.">
              <div className="dash-kpi-grid">
                <article className="dash-kpi-card">
                  <h3>Pricing page views</h3>
                  <p>{formatNumber(dashboard.monetization?.pricing_page_views || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Upgrade clicks</h3>
                  <p>{formatNumber(dashboard.monetization?.upgrade_clicks || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Paywall views</h3>
                  <p>{formatNumber(dashboard.monetization?.paywall_views || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Checkout starts</h3>
                  <p>{formatNumber(dashboard.monetization?.checkout_starts || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Checkout abandon rate</h3>
                  <p>{formatPercent(dashboard.monetization?.checkout_abandon_rate || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Payment succeeded</h3>
                  <p>{formatNumber(dashboard.monetization?.payment_succeeded || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Subscription activated</h3>
                  <p>{formatNumber(dashboard.monetization?.subscription_activated || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Subscription renewals</h3>
                  <p>{formatNumber(dashboard.monetization?.subscription_renewed || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Subscription canceled</h3>
                  <p>{formatNumber(dashboard.monetization?.subscription_canceled || 0)}</p>
                </article>
              </div>

              <div className="dash-card">
                <h3>Plan mix</h3>
                <BreakdownTable rows={dashboard.monetization?.plan_mix} firstCol="Plan" />
              </div>
            </Section>

            <Section title="Retention and Cohorts" subtitle="D1/D7/D30 retention, repeat-value behavior, stickiness, and weekly cohorts.">
              <div className="dash-kpi-grid">
                <article className="dash-kpi-card">
                  <h3>D1 retention</h3>
                  <p>{formatPercent(dashboard.retention?.d1_retention_rate || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>D7 retention</h3>
                  <p>{formatPercent(dashboard.retention?.d7_retention_rate || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>D30 retention</h3>
                  <p>{formatPercent(dashboard.retention?.d30_retention_rate || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Repeat successful task rate</h3>
                  <p>{formatPercent(dashboard.retention?.repeat_successful_task_rate || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>DAU / WAU</h3>
                  <p>{formatPercent(dashboard.retention?.stickiness?.dau_wau_ratio || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>DAU / MAU</h3>
                  <p>{formatPercent(dashboard.retention?.stickiness?.dau_mau_ratio || 0)}</p>
                </article>
              </div>

              <div className="dash-grid-2">
                <TrendBars points={dashboard.retention?.active_users_trend} title="Active users trend" />
                <TrendBars points={dashboard.retention?.active_workspaces_trend} title="Active workspaces trend" />
              </div>

              <div className="dash-card">
                <h3>Weekly cohorts</h3>
                {!dashboard.retention?.cohorts?.length ? (
                  <EmptyState label="No cohorts in selected range." />
                ) : (
                  <div className="dash-table-wrap">
                    <table className="dash-table">
                      <thead>
                        <tr>
                          <th>Cohort week</th>
                          <th>Users</th>
                          <th>D1</th>
                          <th>D7</th>
                          <th>D30</th>
                        </tr>
                      </thead>
                      <tbody>
                        {dashboard.retention.cohorts.map((cohort) => (
                          <tr key={cohort.cohort_week}>
                            <td>{cohort.cohort_week}</td>
                            <td>{formatNumber(cohort.users)}</td>
                            <td>{formatPercent(cohort.d1_retention_rate)}</td>
                            <td>{formatPercent(cohort.d7_retention_rate)}</td>
                            <td>{formatPercent(cohort.d30_retention_rate)}</td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                )}
              </div>
            </Section>

            <Section title="Reliability" subtitle="Task delivery quality, error rates, latency hotspots, and top failure reasons.">
              <div className="dash-kpi-grid">
                <article className="dash-kpi-card">
                  <h3>Task success rate</h3>
                  <p>{formatPercent(dashboard.reliability?.task_success_rate || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>API error rate</h3>
                  <p>
                    {dashboard.reliability?.api_error_rate === null || dashboard.reliability?.api_error_rate === undefined
                      ? 'N/A'
                      : formatPercent(dashboard.reliability.api_error_rate)}
                  </p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Integration failure rate</h3>
                  <p>
                    {dashboard.reliability?.integration_failure_rate === null ||
                    dashboard.reliability?.integration_failure_rate === undefined
                      ? 'N/A'
                      : formatPercent(dashboard.reliability.integration_failure_rate)}
                  </p>
                </article>
                <article className="dash-kpi-card">
                  <h3>Checkout failure rate</h3>
                  <p>
                    {dashboard.reliability?.checkout_failure_rate === null ||
                    dashboard.reliability?.checkout_failure_rate === undefined
                      ? 'N/A'
                      : formatPercent(dashboard.reliability.checkout_failure_rate)}
                  </p>
                </article>
              </div>

              <div className="dash-grid-2">
                <div className="dash-card">
                  <h3>Slowest endpoints / workflows</h3>
                  {!dashboard.reliability?.slowest_endpoints_or_workflows?.length ? (
                    <EmptyState label="No latency metrics recorded yet." />
                  ) : (
                    <div className="dash-table-wrap">
                      <table className="dash-table">
                        <thead>
                          <tr>
                            <th>Endpoint / workflow</th>
                            <th>Avg latency (ms)</th>
                            <th>P95 latency (ms)</th>
                          </tr>
                        </thead>
                        <tbody>
                          {dashboard.reliability.slowest_endpoints_or_workflows.map((item) => (
                            <tr key={item.key}>
                              <td>{item.key}</td>
                              <td>{Math.round(item.avg_latency_ms)}</td>
                              <td>{Math.round(item.p95_latency_ms)}</td>
                            </tr>
                          ))}
                        </tbody>
                      </table>
                    </div>
                  )}
                </div>

                <div className="dash-card">
                  <h3>Top failure reasons</h3>
                  {!dashboard.reliability?.top_failure_reasons?.length ? (
                    <EmptyState label="No failure reasons in selected range." />
                  ) : (
                    <div className="dash-table-wrap">
                      <table className="dash-table">
                        <thead>
                          <tr>
                            <th>Reason</th>
                            <th>Count</th>
                          </tr>
                        </thead>
                        <tbody>
                          {dashboard.reliability.top_failure_reasons.map((item) => (
                            <tr key={item.reason}>
                              <td>{item.reason}</td>
                              <td>{formatNumber(item.count)}</td>
                            </tr>
                          ))}
                        </tbody>
                      </table>
                    </div>
                  )}
                </div>
              </div>
            </Section>

            <Section title="Metric Definitions" subtitle="Formulas used by this dashboard for trust and consistency.">
              {!dashboard.metric_definitions?.length ? (
                <EmptyState label="Metric definitions unavailable." />
              ) : (
                <div className="dash-table-wrap">
                  <table className="dash-table">
                    <thead>
                      <tr>
                        <th>Metric</th>
                        <th>Formula</th>
                      </tr>
                    </thead>
                    <tbody>
                      {dashboard.metric_definitions.map((row) => (
                        <tr key={row.metric}>
                          <td>{row.metric}</td>
                          <td>{row.formula}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              )}
            </Section>

            <Section title="Event Taxonomy" subtitle="Implemented and deferred events with trigger, properties, source, and status.">
              {!dashboard.taxonomy?.length ? (
                <EmptyState label="Taxonomy unavailable." />
              ) : (
                <div className="dash-table-wrap">
                  <table className="dash-table">
                    <thead>
                      <tr>
                        <th>Category</th>
                        <th>Event</th>
                        <th>Trigger</th>
                        <th>Required properties</th>
                        <th>Optional properties</th>
                        <th>Source</th>
                        <th>Status</th>
                      </tr>
                    </thead>
                    <tbody>
                      {dashboard.taxonomy.map((row) => (
                        <tr key={`${row.category}-${row.event_name}`}>
                          <td>{row.category}</td>
                          <td>{row.event_name}</td>
                          <td>{row.trigger}</td>
                          <td>{(row.required_properties || []).join(', ')}</td>
                          <td>{(row.optional_properties || []).join(', ')}</td>
                          <td>{row.emitted_from}</td>
                          <td>{row.status}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              )}
            </Section>
          </>
        ) : null}
      </div>
    </div>
  );
}

export default DashboardPage;
