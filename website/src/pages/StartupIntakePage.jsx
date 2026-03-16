import { useMemo, useState } from 'react';
import { Link } from 'react-router-dom';
import {
  CHANNEL_OPTIONS,
  REPO_PROVIDER_OPTIONS,
  createFounderIntakeDefaults,
  createValidatedWorkspaceBlueprintFromIntake,
  saveWorkspaceBlueprint
} from '../domain/workspaceBlueprint';

function StartupIntakePage() {
  const [intake, setIntake] = useState(() => createFounderIntakeDefaults());
  const [errors, setErrors] = useState([]);
  const [blueprint, setBlueprint] = useState(null);

  const blueprintJson = useMemo(
    () => (blueprint ? JSON.stringify(blueprint, null, 2) : ''),
    [blueprint]
  );

  const updateField = (field) => (event) => {
    const value = event.target.type === 'checkbox' ? event.target.checked : event.target.value;

    setIntake((prev) => {
      if (field === 'has_existing_repo' && !value) {
        return {
          ...prev,
          has_existing_repo: false,
          primary_repo_provider: 'github'
        };
      }

      return {
        ...prev,
        [field]: value
      };
    });
  };

  const toggleChannel = (channel) => {
    setIntake((prev) => {
      const hasChannel = prev.preferred_channels.includes(channel);
      return {
        ...prev,
        preferred_channels: hasChannel
          ? prev.preferred_channels.filter((item) => item !== channel)
          : [...prev.preferred_channels, channel]
      };
    });
  };

  const handleSubmit = (event) => {
    event.preventDefault();

    const result = createValidatedWorkspaceBlueprintFromIntake(intake);
    if (!result.is_valid) {
      setErrors(result.errors);
      setBlueprint(null);
      return;
    }

    saveWorkspaceBlueprint(result.blueprint);
    setErrors([]);
    setBlueprint(result.blueprint);
  };

  return (
    <main className="route-shell route-shell-intake">
      <div className="route-card route-card-intake">
        <p className="route-kicker">Founder Intake</p>
        <h1>Create Your Agent Team</h1>
        <p>
          Share the core brief once. We will generate your workspace blueprint and use it in your unified dashboard.
        </p>

        <form className="intake-form" onSubmit={handleSubmit} noValidate>
          <section className="route-section intake-grid">
            <div className="intake-field">
              <label htmlFor="founder_name">Founder name</label>
              <input
                id="founder_name"
                type="text"
                value={intake.founder_name}
                onChange={updateField('founder_name')}
                placeholder="Jane Founder"
                required
              />
            </div>

            <div className="intake-field">
              <label htmlFor="venture_name">Company / project name</label>
              <input
                id="venture_name"
                type="text"
                value={intake.venture_name}
                onChange={updateField('venture_name')}
                placeholder="Acme AI"
              />
            </div>

            <div className="intake-field intake-field-full">
              <label htmlFor="venture_thesis">Company or project thesis</label>
              <textarea
                id="venture_thesis"
                value={intake.venture_thesis}
                onChange={updateField('venture_thesis')}
                placeholder="What company are you building and why now?"
                required
              />
            </div>

            <div className="intake-field intake-field-full">
              <label htmlFor="goals_text">Top goals (one per line)</label>
              <textarea
                id="goals_text"
                value={intake.goals_text}
                onChange={updateField('goals_text')}
                placeholder="Ship MVP\nClose 3 design partners"
                required
              />
            </div>
          </section>

          <section className="route-section">
            <h2>Preferred Channels</h2>
            <p>Select where your team should receive and execute requests.</p>
            <div className="intake-chip-grid">
              {CHANNEL_OPTIONS.map((channel) => {
                const checked = intake.preferred_channels.includes(channel);
                return (
                  <label key={channel} className={`intake-chip ${checked ? 'is-checked' : ''}`}>
                    <input
                      type="checkbox"
                      checked={checked}
                      onChange={() => toggleChannel(channel)}
                    />
                    <span>{channel}</span>
                  </label>
                );
              })}
            </div>
          </section>

          <section className="route-section intake-grid">
            <div className="intake-field intake-field-full">
              <label htmlFor="requested_agents_text">Agent roles (one per line, optional owner via role:owner)</label>
              <textarea
                id="requested_agents_text"
                value={intake.requested_agents_text}
                onChange={updateField('requested_agents_text')}
                placeholder="Builder\nGTM Strategist\nChief of Staff:Founder"
              />
            </div>

            <div className="intake-field intake-field-checkbox">
              <label>
                <input
                  type="checkbox"
                  checked={intake.has_existing_repo}
                  onChange={updateField('has_existing_repo')}
                />
                <span>Existing code repository</span>
              </label>
            </div>

            <div className="intake-field intake-field-checkbox">
              <label>
                <input
                  type="checkbox"
                  checked={intake.has_docs_workspace}
                  onChange={updateField('has_docs_workspace')}
                />
                <span>Existing docs workspace (Google Docs / Sheets / Slides / Notion)</span>
              </label>
            </div>

            {intake.has_existing_repo ? (
              <div className="intake-field">
                <label htmlFor="primary_repo_provider">Primary repo provider</label>
                <select
                  id="primary_repo_provider"
                  value={intake.primary_repo_provider}
                  onChange={updateField('primary_repo_provider')}
                >
                  {REPO_PROVIDER_OPTIONS.map((provider) => (
                    <option key={provider} value={provider}>
                      {provider}
                    </option>
                  ))}
                </select>
              </div>
            ) : null}
          </section>

          <section className="route-section">
            <details className="intake-advanced">
              <summary>Advanced details (optional)</summary>
              <div className="intake-grid intake-advanced-grid">
                <div className="intake-field">
                  <label htmlFor="founder_email">Founder email</label>
                  <input
                    id="founder_email"
                    type="email"
                    value={intake.founder_email}
                    onChange={updateField('founder_email')}
                    placeholder="jane@startup.com"
                  />
                </div>

                <div className="intake-field">
                  <label htmlFor="venture_stage">Stage</label>
                  <select id="venture_stage" value={intake.venture_stage} onChange={updateField('venture_stage')}>
                    <option value="idea">Idea</option>
                    <option value="prototype">Prototype</option>
                    <option value="mvp">MVP</option>
                    <option value="post_mvp">Post-MVP</option>
                    <option value="growth">Growth</option>
                  </select>
                </div>

                <div className="intake-field">
                  <label htmlFor="plan_horizon_days">Planning horizon</label>
                  <select
                    id="plan_horizon_days"
                    value={intake.plan_horizon_days}
                    onChange={updateField('plan_horizon_days')}
                  >
                    <option value="30">30 days</option>
                    <option value="60">60 days</option>
                    <option value="90">90 days</option>
                  </select>
                </div>

                <div className="intake-field intake-field-full">
                  <label htmlFor="assets_text">Current assets (one per line)</label>
                  <textarea
                    id="assets_text"
                    value={intake.assets_text}
                    onChange={updateField('assets_text')}
                    placeholder="Pitch deck\nCustomer interviews\nLanding page"
                  />
                </div>
              </div>
            </details>
          </section>

          {errors.length ? (
            <section className="route-section intake-errors" aria-live="polite">
              <h2>Blueprint validation issues</h2>
              <ul>
                {errors.map((error) => (
                  <li key={error}>{error}</li>
                ))}
              </ul>
            </section>
          ) : null}

          <div className="route-actions">
            <button type="submit" className="btn btn-primary">
              Save team blueprint
            </button>
            <a className="btn btn-secondary" href="/auth/index.html?loggedIn=true#section-workspace">
              Open dashboard
            </a>
            <Link className="btn btn-secondary" to="/">
              Back to landing
            </Link>
          </div>
        </form>

        {blueprint ? (
          <section className="route-section">
            <h2>Blueprint saved</h2>
            <p>
              Your team blueprint is saved locally and now appears in your dashboard workspace section.
            </p>
            <details className="intake-advanced">
              <summary>View blueprint JSON</summary>
              <pre className="intake-blueprint-preview">{blueprintJson}</pre>
            </details>
          </section>
        ) : null}
      </div>
    </main>
  );
}

export default StartupIntakePage;
