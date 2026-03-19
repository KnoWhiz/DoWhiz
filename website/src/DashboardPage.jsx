import { useEffect, useMemo, useState } from 'react';
import { getDoWhizApiBaseUrl } from './analytics';
import { supabase } from './app/supabaseClient';
import './dashboard.css';

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

const METRIC_HELP = {
  unique_visitors: 'Distinct identities that triggered landing_page_view in the selected window.',
  signup_conversion: 'signup_completed identities / unique visitor identities.',
  activation_rate: 'Identities with first successful task / identities with signup_completed.',
  activation_to_paid: 'Paid identities (payment_succeeded or subscription_activated) / activated identities.',
  time_to_first_value: 'Median hours from signup_completed to first_task_succeeded.',
  d7_retention: 'Eligible signup_completed identities with a usage event 7-8 days later.',
  active_workspaces: 'Distinct workspace/account ids associated with tracked identities in the selected window.',
  paid_accounts_30d: 'Distinct accounts with at least one payment in the 30 days ending at window end.',
  revenue_range: 'Sum of successful payment amounts (USD) recorded in the selected window.',
  funnel_step_conversion: 'Current step identities / previous funnel step identities.',
  funnel_overall_conversion: 'Current step identities / first funnel step identities.',
  pricing_page_views: 'Count of pricing_page_view events.',
  upgrade_clicks: 'Count of upgrade_clicked events.',
  paywall_views: 'Count of upgrade_viewed_or_paywall_seen and paywall_seen events.',
  checkout_starts: 'Count of checkout_started events (includes fallback interpretation in funnel sequencing).',
  checkout_abandon_rate:
    'checkout_abandoned / checkout_started when available; otherwise max(checkout_started - payment_succeeded, 0) / checkout_started.',
  payment_succeeded: 'Count of payment_succeeded events.',
  subscription_activated: 'Count of subscription_activated events.',
  subscription_renewals: 'Count of subscription_renewed events.',
  subscription_canceled: 'Count of subscription_canceled events.',
  agent_or_workflow_creation_rate:
    'Signup_completed identities with first_agent_or_workflow_created / signup_completed identities.',
  multi_channel_connection_rate:
    'Signup_completed identities connected to 2+ channel/tool types / signup_completed identities.',
  d1_retention: 'Eligible signup_completed identities with a usage event 1-2 days later.',
  d30_retention: 'Eligible signup_completed identities with a usage event 30-31 days later.',
  repeat_successful_task_rate:
    'Identities with second_successful_task within 7 days of first_task_succeeded / identities with first_task_succeeded.',
  dau_wau: 'Distinct active users today / distinct active users in trailing 7 days.',
  dau_mau: 'Distinct active users today / distinct active users in trailing 30 days.',
  active_users_trend: 'Daily count of active identities based on usage events.',
  active_workspaces_trend: 'Daily count of active workspaces based on usage events.',
  task_success_rate: 'task_succeeded / (task_succeeded + task_failed).',
  api_error_rate: 'api_error / api_request, when api_request telemetry is present.',
  integration_failure_rate:
    '(channel_connect_failed + tool_connect_failed + integration_error) / connection attempts, when attempts telemetry is present.',
  checkout_failure_rate: 'checkout_error / checkout_started, when checkout_started > 0.',
  avg_latency_ms: 'Average latency in milliseconds across latency_metric_logged events for each endpoint/workflow.',
  p95_latency_ms: '95th percentile latency in milliseconds across latency_metric_logged events for each endpoint/workflow.',
  cohort_users: 'Number of signup_completed identities in the cohort week.',
  retention_rate_col: 'Retention rate for that cohort at the given day marker.',
  breakdown_count: 'Identity count for this segment in the selected window.',
  breakdown_rate: 'Segment count / segment denominator for this table.'
};

const FUNNEL_STEP_HELP = {
  landing_page_view: 'Visitor identity generated landing_page_view.',
  primary_cta_click:
    'Visitor clicked the landing-page primary CTA. This can be lower than later signup steps when users enter directly on auth pages.',
  signup_started:
    'Identities with signup_started. To prevent instrumentation undercount, signup_completed is also treated as a fallback signal for this step.',
  signup_completed: 'Account signup completed successfully (created or backfilled account creation within the window).',
  first_authenticated_session: 'First authenticated app session for the identity.',
  workspace_created: 'Initial account workspace provisioned.',
  first_channel_or_tool_connected: 'First successful external channel/tool connection.',
  first_agent_or_workflow_created: 'First agent or workflow creation event.',
  first_task_started:
    'First task start event. Success events are accepted as fallback to avoid impossible success-without-start ordering.',
  first_task_succeeded:
    'First successful task event. second_successful_task is accepted as fallback for ordering consistency.',
  second_successful_task: 'Second successful task event for the identity.',
  upgrade_viewed_or_paywall_seen:
    'Upgrade/paywall intent event. In-app upgrade surfaces can vary by route and entrypoint.',
  checkout_started:
    'Checkout initiated. payment_succeeded/subscription_activated are accepted as fallback signals for ordering consistency.',
  payment_succeeded: 'Successful payment event.',
  subscription_activated: 'Subscription or paid credit state activated.'
};

function MetricHeading({ label, help }) {
  if (!help) {
    return <>{label}</>;
  }
  return (
    <span className="dash-metric-heading">
      <span>{label}</span>
      <span className="dash-help-wrap">
        <button type="button" className="dash-help-btn" aria-label={`${label}. ${help}`}>
          ?
        </button>
        <span className="dash-help-tooltip" role="tooltip">
          {help}
        </span>
      </span>
    </span>
  );
}

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

function BreakdownTable({
  rows,
  firstCol,
  firstColHelp,
  secondCol = 'Count',
  secondColHelp = METRIC_HELP.breakdown_count,
  rateCol = 'Rate',
  rateColHelp = METRIC_HELP.breakdown_rate
}) {
  if (!rows?.length) {
    return <EmptyState label="No data in selected range." />;
  }

  return (
    <div className="dash-table-wrap">
      <table className="dash-table">
        <thead>
          <tr>
            <th>
              <MetricHeading label={firstCol} help={firstColHelp} />
            </th>
            <th>
              <MetricHeading label={secondCol} help={secondColHelp} />
            </th>
            <th>
              <MetricHeading label={rateCol} help={rateColHelp} />
            </th>
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
      <h3>
        <MetricHeading
          label={title}
          help="Segmented acquisition funnel: visitors, signups, activated, and paid identities grouped by this dimension."
        />
      </h3>
      {!rows?.length ? (
        <EmptyState label="No data in selected range." />
      ) : (
        <div className="dash-table-wrap">
          <table className="dash-table">
            <thead>
              <tr>
                <th>Segment</th>
                <th>
                  <MetricHeading label="Visitors" help="Distinct identities with landing_page_view in this segment." />
                </th>
                <th>
                  <MetricHeading label="Signups" help="Distinct identities with signup_completed in this segment." />
                </th>
                <th>
                  <MetricHeading
                    label="Activated"
                    help="Distinct identities with first_task_succeeded or task_succeeded in this segment."
                  />
                </th>
                <th>
                  <MetricHeading
                    label="Paid"
                    help="Distinct identities with payment_succeeded or subscription_activated in this segment."
                  />
                </th>
                <th>
                  <MetricHeading label="Signup CVR" help="Signups / Visitors for this segment." />
                </th>
                <th>
                  <MetricHeading label="Activation" help="Activated / Signups for this segment." />
                </th>
                <th>
                  <MetricHeading label="Paid CVR" help="Paid / Signups for this segment." />
                </th>
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

function TrendBars({ points, title, titleHelp }) {
  if (!points?.length) {
    return (
      <div className="dash-card">
        <h3>
          <MetricHeading label={title} help={titleHelp} />
        </h3>
        <EmptyState label="No trend data in selected range." />
      </div>
    );
  }

  const trimmed = points.slice(-21);
  const max = Math.max(...trimmed.map((point) => point.count), 1);

  return (
    <div className="dash-card">
      <h3>
        <MetricHeading label={title} help={titleHelp} />
      </h3>
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
                  <h3>
                    <MetricHeading label="Unique visitors" help={METRIC_HELP.unique_visitors} />
                  </h3>
                  <p>{formatNumber(dashboard.kpis.unique_visitors)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Signup conversion" help={METRIC_HELP.signup_conversion} />
                  </h3>
                  <p>{formatPercent(dashboard.kpis.signup_conversion_rate)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Activation rate" help={METRIC_HELP.activation_rate} />
                  </h3>
                  <p>{formatPercent(dashboard.kpis.activation_rate)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Activation -> paid conversion" help={METRIC_HELP.activation_to_paid} />
                  </h3>
                  <p>{formatPercent(dashboard.kpis.activation_to_paid_rate)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Time to first value" help={METRIC_HELP.time_to_first_value} />
                  </h3>
                  <p>{formatHours(dashboard.kpis.median_time_to_first_value_hours)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="D7 retention" help={METRIC_HELP.d7_retention} />
                  </h3>
                  <p>{formatPercent(dashboard.kpis.d7_retention_rate)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Active workspaces" help={METRIC_HELP.active_workspaces} />
                  </h3>
                  <p>{formatNumber(dashboard.kpis.active_workspaces)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Paid accounts (30d)" help={METRIC_HELP.paid_accounts_30d} />
                  </h3>
                  <p>{formatNumber(dashboard.kpis.active_paid_accounts_30d)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Revenue (range)" help={METRIC_HELP.revenue_range} />
                  </h3>
                  <p>{formatCurrency(dashboard.kpis.revenue_usd)}</p>
                </article>
              </div>
            </Section>

            <Section title="Main Funnel" subtitle="Ordered funnel from first visit to active paid state.">
              {!dashboard.funnel?.steps?.length ? (
                <EmptyState label="No funnel events in selected range." />
              ) : (
                <div className="dash-funnel">
                  {dashboard.funnel.steps.map((step, stepIndex) => (
                    <article className="dash-funnel-step" key={step.event_name}>
                      <div className="dash-funnel-title-row">
                        <h3>
                          <MetricHeading
                            label={step.label}
                            help={FUNNEL_STEP_HELP[step.event_name] || 'Funnel step count for this event.'}
                          />
                        </h3>
                        <span>{formatNumber(step.identities)}</span>
                      </div>
                      <div className="dash-funnel-bar" aria-hidden="true">
                        <div
                          className="dash-funnel-bar-fill"
                          style={{ width: `${(clampPercent(step.overall_conversion_rate) * 100).toFixed(1)}%` }}
                        />
                      </div>
                      <div className="dash-funnel-metrics">
                        <span>
                          <MetricHeading
                            label={`Step conversion: ${formatPercent(step.step_conversion_rate)}`}
                            help={
                              stepIndex === 0
                                ? 'For the first funnel step this is fixed at 100%.'
                                : METRIC_HELP.funnel_step_conversion
                            }
                          />
                        </span>
                        <span>
                          <MetricHeading
                            label={`Overall conversion: ${formatPercent(step.overall_conversion_rate)}`}
                            help={METRIC_HELP.funnel_overall_conversion}
                          />
                        </span>
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
                  <h3>
                    <MetricHeading
                      label="Signup auth method"
                      help="Breakdown of signup_completed identities by auth_method captured at signup."
                    />
                  </h3>
                  <BreakdownTable
                    rows={dashboard.activation?.by_auth_method}
                    firstCol="Auth method"
                    firstColHelp="Authentication method recorded on signup_completed."
                    rateColHelp="Auth method identities / all signup_completed identities."
                  />
                </div>
                <div className="dash-card">
                  <h3>
                    <MetricHeading
                      label="Workspace type"
                      help="Breakdown of signup_completed identities by workspace_type from workspace_created."
                    />
                  </h3>
                  <BreakdownTable
                    rows={dashboard.activation?.by_workspace_type}
                    firstCol="Workspace"
                    firstColHelp="Workspace type captured on workspace_created."
                    rateColHelp="Workspace type identities / all signup_completed identities."
                  />
                </div>
                <div className="dash-card">
                  <h3>
                    <MetricHeading
                      label="Connected channel / tool type"
                      help="Breakdown of first connected channel/tool type among signup_completed identities."
                    />
                  </h3>
                  <BreakdownTable
                    rows={dashboard.activation?.by_connected_channel_type}
                    firstCol="Channel/Tool"
                    firstColHelp="Channel/tool type captured from connection events."
                    rateColHelp="Channel/tool identities / all signup_completed identities."
                  />
                </div>
                <div className="dash-card">
                  <h3>
                    <MetricHeading
                      label="First task type"
                      help="Breakdown of first task type observed among signup_completed identities."
                    />
                  </h3>
                  <BreakdownTable
                    rows={dashboard.activation?.by_first_task_type}
                    firstCol="Task type"
                    firstColHelp="Task type captured on first_task_started or task_started."
                    rateColHelp="Task-type identities / all signup_completed identities."
                  />
                </div>
              </div>
              <div className="dash-rate-row">
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading
                      label="Agent/workflow creation rate"
                      help={METRIC_HELP.agent_or_workflow_creation_rate}
                    />
                  </h3>
                  <p>{formatPercent(dashboard.activation?.agent_or_workflow_creation_rate || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading
                      label="Multi-channel connection rate"
                      help={METRIC_HELP.multi_channel_connection_rate}
                    />
                  </h3>
                  <p>{formatPercent(dashboard.activation?.multi_channel_connection_rate || 0)}</p>
                </article>
              </div>
            </Section>

            <Section title="Monetization" subtitle="Upgrade intent, checkout flow, successful payment, and paid state activation.">
              <div className="dash-kpi-grid">
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Pricing page views" help={METRIC_HELP.pricing_page_views} />
                  </h3>
                  <p>{formatNumber(dashboard.monetization?.pricing_page_views || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Upgrade clicks" help={METRIC_HELP.upgrade_clicks} />
                  </h3>
                  <p>{formatNumber(dashboard.monetization?.upgrade_clicks || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Paywall views" help={METRIC_HELP.paywall_views} />
                  </h3>
                  <p>{formatNumber(dashboard.monetization?.paywall_views || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Checkout starts" help={METRIC_HELP.checkout_starts} />
                  </h3>
                  <p>{formatNumber(dashboard.monetization?.checkout_starts || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Checkout abandon rate" help={METRIC_HELP.checkout_abandon_rate} />
                  </h3>
                  <p>{formatPercent(dashboard.monetization?.checkout_abandon_rate || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Payment succeeded" help={METRIC_HELP.payment_succeeded} />
                  </h3>
                  <p>{formatNumber(dashboard.monetization?.payment_succeeded || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Subscription activated" help={METRIC_HELP.subscription_activated} />
                  </h3>
                  <p>{formatNumber(dashboard.monetization?.subscription_activated || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Subscription renewals" help={METRIC_HELP.subscription_renewals} />
                  </h3>
                  <p>{formatNumber(dashboard.monetization?.subscription_renewed || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Subscription canceled" help={METRIC_HELP.subscription_canceled} />
                  </h3>
                  <p>{formatNumber(dashboard.monetization?.subscription_canceled || 0)}</p>
                </article>
              </div>

              <div className="dash-card">
                <h3>
                  <MetricHeading label="Plan mix" help="Distribution of payment/subscription events by plan type." />
                </h3>
                <BreakdownTable
                  rows={dashboard.monetization?.plan_mix}
                  firstCol="Plan"
                  firstColHelp="Plan type from payment/subscription events."
                  rateColHelp="Plan event count / max(payment_succeeded, subscription_activated) events."
                />
              </div>
            </Section>

            <Section title="Retention and Cohorts" subtitle="D1/D7/D30 retention, repeat-value behavior, stickiness, and weekly cohorts.">
              <div className="dash-kpi-grid">
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="D1 retention" help={METRIC_HELP.d1_retention} />
                  </h3>
                  <p>{formatPercent(dashboard.retention?.d1_retention_rate || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="D7 retention" help={METRIC_HELP.d7_retention} />
                  </h3>
                  <p>{formatPercent(dashboard.retention?.d7_retention_rate || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="D30 retention" help={METRIC_HELP.d30_retention} />
                  </h3>
                  <p>{formatPercent(dashboard.retention?.d30_retention_rate || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Repeat successful task rate" help={METRIC_HELP.repeat_successful_task_rate} />
                  </h3>
                  <p>{formatPercent(dashboard.retention?.repeat_successful_task_rate || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="DAU / WAU" help={METRIC_HELP.dau_wau} />
                  </h3>
                  <p>{formatPercent(dashboard.retention?.stickiness?.dau_wau_ratio || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="DAU / MAU" help={METRIC_HELP.dau_mau} />
                  </h3>
                  <p>{formatPercent(dashboard.retention?.stickiness?.dau_mau_ratio || 0)}</p>
                </article>
              </div>

              <div className="dash-grid-2">
                <TrendBars
                  points={dashboard.retention?.active_users_trend}
                  title="Active users trend"
                  titleHelp={METRIC_HELP.active_users_trend}
                />
                <TrendBars
                  points={dashboard.retention?.active_workspaces_trend}
                  title="Active workspaces trend"
                  titleHelp={METRIC_HELP.active_workspaces_trend}
                />
              </div>

              <div className="dash-card">
                <h3>
                  <MetricHeading
                    label="Weekly cohorts"
                    help="Each row groups users by signup week and shows retention rates at D1, D7, and D30."
                  />
                </h3>
                {!dashboard.retention?.cohorts?.length ? (
                  <EmptyState label="No cohorts in selected range." />
                ) : (
                  <div className="dash-table-wrap">
                    <table className="dash-table">
                      <thead>
                        <tr>
                          <th>Cohort week</th>
                          <th>
                            <MetricHeading label="Users" help={METRIC_HELP.cohort_users} />
                          </th>
                          <th>
                            <MetricHeading label="D1" help={METRIC_HELP.retention_rate_col} />
                          </th>
                          <th>
                            <MetricHeading label="D7" help={METRIC_HELP.retention_rate_col} />
                          </th>
                          <th>
                            <MetricHeading label="D30" help={METRIC_HELP.retention_rate_col} />
                          </th>
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
                  <h3>
                    <MetricHeading label="Task success rate" help={METRIC_HELP.task_success_rate} />
                  </h3>
                  <p>{formatPercent(dashboard.reliability?.task_success_rate || 0)}</p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="API error rate" help={METRIC_HELP.api_error_rate} />
                  </h3>
                  <p>
                    {dashboard.reliability?.api_error_rate === null || dashboard.reliability?.api_error_rate === undefined
                      ? 'N/A'
                      : formatPercent(dashboard.reliability.api_error_rate)}
                  </p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Integration failure rate" help={METRIC_HELP.integration_failure_rate} />
                  </h3>
                  <p>
                    {dashboard.reliability?.integration_failure_rate === null ||
                    dashboard.reliability?.integration_failure_rate === undefined
                      ? 'N/A'
                      : formatPercent(dashboard.reliability.integration_failure_rate)}
                  </p>
                </article>
                <article className="dash-kpi-card">
                  <h3>
                    <MetricHeading label="Checkout failure rate" help={METRIC_HELP.checkout_failure_rate} />
                  </h3>
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
                  <h3>
                    <MetricHeading
                      label="Slowest endpoints / workflows"
                      help="Latency summary for endpoints/workflows ranked by highest average latency."
                    />
                  </h3>
                  {!dashboard.reliability?.slowest_endpoints_or_workflows?.length ? (
                    <EmptyState label="No latency metrics recorded yet." />
                  ) : (
                    <div className="dash-table-wrap">
                      <table className="dash-table">
                        <thead>
                          <tr>
                            <th>Endpoint / workflow</th>
                            <th>
                              <MetricHeading label="Avg latency (ms)" help={METRIC_HELP.avg_latency_ms} />
                            </th>
                            <th>
                              <MetricHeading label="P95 latency (ms)" help={METRIC_HELP.p95_latency_ms} />
                            </th>
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
                  <h3>
                    <MetricHeading
                      label="Top failure reasons"
                      help="Most frequent error reasons aggregated across failure event types in the selected window."
                    />
                  </h3>
                  {!dashboard.reliability?.top_failure_reasons?.length ? (
                    <EmptyState label="No failure reasons in selected range." />
                  ) : (
                    <div className="dash-table-wrap">
                      <table className="dash-table">
                        <thead>
                          <tr>
                            <th>Reason</th>
                            <th>
                              <MetricHeading label="Count" help="Number of events mapped to this failure reason." />
                            </th>
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
