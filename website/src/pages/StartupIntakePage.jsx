import { useMemo, useState } from 'react';
import { Link, useLocation } from 'react-router-dom';
import StartupIntakeConversation from '../components/intake/StartupIntakeConversation';
import {
  DASHBOARD_PATH,
  EDIT_MODE_QUERY_VALUE,
  clampPlanHorizonSelection,
  mapBlueprintToIntake,
  normalizeStringList,
  normalizeToolValue
} from '../domain/startupIntake';
import {
  CHANNEL_OPTIONS,
  PLAN_HORIZON_OPTIONS,
  REPO_PROVIDER_OPTIONS,
  STAGE_OPTIONS,
  createValidatedWorkspaceBlueprintFromIntake,
  loadWorkspaceBlueprint,
  saveWorkspaceBlueprint
} from '../domain/workspaceBlueprint';

function StartupIntakePage() {
  const location = useLocation();
  const savedBlueprint = useMemo(() => loadWorkspaceBlueprint(), []);
  const hasSavedBlueprint = Boolean(savedBlueprint);
  const isEditModeRequested = useMemo(() => {
    const searchParams = new URLSearchParams(location.search);
    return searchParams.get('mode') === EDIT_MODE_QUERY_VALUE;
  }, [location.search]);
  const shouldShowQuestionnaire = isEditModeRequested && hasSavedBlueprint;

  const [questionnaireIntake, setQuestionnaireIntake] = useState(() => mapBlueprintToIntake(savedBlueprint));
  const [questionnaireNotice, setQuestionnaireNotice] = useState('');
  const [errors, setErrors] = useState([]);
  const [blueprint, setBlueprint] = useState(null);

  const blueprintJson = useMemo(() => (blueprint ? JSON.stringify(blueprint, null, 2) : ''), [blueprint]);

  const selectedChannels = useMemo(() => {
    return new Set(
      normalizeStringList(questionnaireIntake.preferred_channels).map((channel) => channel.toLowerCase())
    );
  }, [questionnaireIntake.preferred_channels]);

  const updateQuestionnaireIntake = (updater) => {
    setQuestionnaireIntake((prev) => {
      const next = typeof updater === 'function' ? updater(prev) : updater;
      return {
        ...prev,
        ...next
      };
    });
    setErrors([]);
    setBlueprint(null);
    setQuestionnaireNotice('');
  };

  const handleQuestionnaireFieldChange = (event) => {
    const { name, type, checked, value } = event.target;
    updateQuestionnaireIntake({
      [name]: type === 'checkbox' ? checked : value
    });
  };

  const handlePreferredChannelToggle = (channel) => {
    updateQuestionnaireIntake((prev) => {
      const channels = normalizeStringList(prev.preferred_channels);
      const existingIndex = channels.findIndex((item) => item.toLowerCase() === channel.toLowerCase());

      if (existingIndex >= 0) {
        channels.splice(existingIndex, 1);
      } else {
        channels.push(channel);
      }

      return {
        preferred_channels: channels
      };
    });
  };

  const handleQuestionnaireSubmit = (event) => {
    event.preventDefault();
    setErrors([]);
    setBlueprint(null);
    setQuestionnaireNotice('');

    const normalizedIntake = {
      ...questionnaireIntake,
      founder_name: String(questionnaireIntake.founder_name || '').trim(),
      founder_email: String(questionnaireIntake.founder_email || '').trim(),
      venture_name: String(questionnaireIntake.venture_name || '').trim(),
      venture_thesis: String(questionnaireIntake.venture_thesis || '').trim(),
      venture_stage: String(questionnaireIntake.venture_stage || '').trim() || 'idea',
      plan_horizon_days: clampPlanHorizonSelection(questionnaireIntake.plan_horizon_days),
      goals_text: String(questionnaireIntake.goals_text || '').trim(),
      assets_text: String(questionnaireIntake.assets_text || '').trim(),
      preferred_channels: normalizeStringList(questionnaireIntake.preferred_channels),
      has_existing_repo: Boolean(questionnaireIntake.has_existing_repo),
      primary_repo_provider:
        normalizeToolValue(questionnaireIntake.primary_repo_provider, REPO_PROVIDER_OPTIONS) || 'github',
      has_docs_workspace: Boolean(questionnaireIntake.has_docs_workspace),
      requested_agents_text: String(questionnaireIntake.requested_agents_text || '').trim()
    };

    const result = createValidatedWorkspaceBlueprintFromIntake(normalizedIntake);
    if (!result.is_valid) {
      setErrors(result.errors);
      return;
    }

    saveWorkspaceBlueprint(result.blueprint);
    setBlueprint(result.blueprint);
    setQuestionnaireNotice('Team brief saved. Open your dashboard workspace section to continue setup.');
  };

  if (shouldShowQuestionnaire) {
    return (
      <main className="route-shell route-shell-intake">
        <div className="route-card route-card-intake">
          <p className="route-kicker">Team Brief Questionnaire</p>
          <h1>Update Your Team Brief</h1>
          <p>
            Review and edit your saved team context. Changes are validated and saved to your workspace blueprint.
          </p>

          <section className="route-section" aria-label="Team brief questionnaire">
            <form className="intake-form" onSubmit={handleQuestionnaireSubmit}>
              <div className="intake-grid">
                <div className="intake-field">
                  <label htmlFor="founder_name">Founder name</label>
                  <input
                    id="founder_name"
                    name="founder_name"
                    type="text"
                    value={questionnaireIntake.founder_name}
                    onChange={handleQuestionnaireFieldChange}
                    placeholder="Your name"
                  />
                </div>

                <div className="intake-field">
                  <label htmlFor="founder_email">Founder email</label>
                  <input
                    id="founder_email"
                    name="founder_email"
                    type="email"
                    value={questionnaireIntake.founder_email}
                    onChange={handleQuestionnaireFieldChange}
                    placeholder="you@example.com"
                  />
                </div>

                <div className="intake-field">
                  <label htmlFor="venture_name">Project / startup name</label>
                  <input
                    id="venture_name"
                    name="venture_name"
                    type="text"
                    value={questionnaireIntake.venture_name}
                    onChange={handleQuestionnaireFieldChange}
                    placeholder="Project name"
                  />
                </div>

                <div className="intake-field">
                  <label htmlFor="venture_stage">Stage</label>
                  <select
                    id="venture_stage"
                    name="venture_stage"
                    value={questionnaireIntake.venture_stage}
                    onChange={handleQuestionnaireFieldChange}
                  >
                    {STAGE_OPTIONS.map((option) => (
                      <option key={option.value} value={option.value}>
                        {option.label}
                      </option>
                    ))}
                  </select>
                </div>

                <div className="intake-field intake-field-full">
                  <label htmlFor="venture_thesis">Venture thesis</label>
                  <textarea
                    id="venture_thesis"
                    name="venture_thesis"
                    value={questionnaireIntake.venture_thesis}
                    onChange={handleQuestionnaireFieldChange}
                    placeholder="What are you building and why now?"
                  />
                </div>

                <div className="intake-field">
                  <label htmlFor="plan_horizon_days">Planning horizon</label>
                  <select
                    id="plan_horizon_days"
                    name="plan_horizon_days"
                    value={questionnaireIntake.plan_horizon_days}
                    onChange={handleQuestionnaireFieldChange}
                  >
                    {PLAN_HORIZON_OPTIONS.map((days) => (
                      <option key={days} value={String(days)}>
                        {days} days
                      </option>
                    ))}
                  </select>
                </div>

                <div className="intake-field intake-field-checkbox">
                  <label htmlFor="has_docs_workspace">
                    <input
                      id="has_docs_workspace"
                      name="has_docs_workspace"
                      type="checkbox"
                      checked={Boolean(questionnaireIntake.has_docs_workspace)}
                      onChange={handleQuestionnaireFieldChange}
                    />
                    Docs workspace is available
                  </label>
                </div>

                <div className="intake-field intake-field-full">
                  <label htmlFor="goals_text">30-90 day goals</label>
                  <textarea
                    id="goals_text"
                    name="goals_text"
                    value={questionnaireIntake.goals_text}
                    onChange={handleQuestionnaireFieldChange}
                    placeholder="One goal per line"
                  />
                </div>

                <div className="intake-field intake-field-full">
                  <label htmlFor="assets_text">Current assets</label>
                  <textarea
                    id="assets_text"
                    name="assets_text"
                    value={questionnaireIntake.assets_text}
                    onChange={handleQuestionnaireFieldChange}
                    placeholder="Team, code, channels, docs, data sources..."
                  />
                </div>

                <div className="intake-field intake-field-full">
                  <label>Preferred channels</label>
                  <div className="intake-chip-grid" role="group" aria-label="Preferred channels">
                    {CHANNEL_OPTIONS.map((channel) => {
                      const isChecked = selectedChannels.has(channel.toLowerCase());
                      return (
                        <label key={channel} className={`intake-chip${isChecked ? ' is-checked' : ''}`}>
                          <input
                            type="checkbox"
                            checked={isChecked}
                            onChange={() => handlePreferredChannelToggle(channel)}
                          />
                          <span>{channel}</span>
                        </label>
                      );
                    })}
                  </div>
                </div>

                <div className="intake-field intake-field-checkbox">
                  <label htmlFor="has_existing_repo">
                    <input
                      id="has_existing_repo"
                      name="has_existing_repo"
                      type="checkbox"
                      checked={Boolean(questionnaireIntake.has_existing_repo)}
                      onChange={handleQuestionnaireFieldChange}
                    />
                    Existing repository available
                  </label>
                </div>

                <div className="intake-field">
                  <label htmlFor="primary_repo_provider">Repository provider</label>
                  <select
                    id="primary_repo_provider"
                    name="primary_repo_provider"
                    value={questionnaireIntake.primary_repo_provider}
                    onChange={handleQuestionnaireFieldChange}
                    disabled={!questionnaireIntake.has_existing_repo}
                  >
                    {REPO_PROVIDER_OPTIONS.map((provider) => (
                      <option key={provider} value={provider}>
                        {provider}
                      </option>
                    ))}
                  </select>
                </div>

                <div className="intake-field intake-field-full">
                  <label htmlFor="requested_agents_text">Requested agents</label>
                  <textarea
                    id="requested_agents_text"
                    name="requested_agents_text"
                    value={questionnaireIntake.requested_agents_text}
                    onChange={handleQuestionnaireFieldChange}
                    placeholder="Role:Owner (one per line), for example: Builder:Alice"
                  />
                </div>
              </div>

              <div className="route-actions">
                <button type="submit" className="btn btn-primary">
                  Save team brief
                </button>
                <Link className="btn btn-secondary" to="/start">
                  Use conversational intake
                </Link>
                <a className="btn btn-secondary" href={DASHBOARD_PATH}>
                  Open dashboard
                </a>
                <Link className="btn btn-secondary" to="/">
                  Back to landing
                </Link>
              </div>
            </form>
          </section>

          {questionnaireNotice ? (
            <section className="route-section" aria-live="polite">
              <p className="workspace-inline-note">{questionnaireNotice}</p>
            </section>
          ) : null}

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

          {blueprint ? (
            <section className="route-section">
              <h2>Blueprint saved</h2>
              <p>Your team blueprint is saved locally and now appears in your dashboard workspace section.</p>
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

  return (
    <main className="route-shell route-shell-intake">
      <div className="route-card route-card-intake">
        <p className="route-kicker">Conversational Intake</p>
        <h1>Create Your Agent Team</h1>
        <p>
          This chat is powered by GPT-5.4. Describe your project and I will gather what is needed for blueprint
          JSON.
        </p>

        {isEditModeRequested && !hasSavedBlueprint ? (
          <p className="workspace-inline-note">
            No saved team brief was found, so you are in first-time conversational setup.
          </p>
        ) : null}

        <StartupIntakeConversation
          variant="page"
          footerActions={
            <>
              <a className="btn btn-secondary" href={DASHBOARD_PATH}>
                Open dashboard
              </a>
              <Link className="btn btn-secondary" to="/">
                Back to landing
              </Link>
            </>
          }
        />
      </div>
    </main>
  );
}

export default StartupIntakePage;
