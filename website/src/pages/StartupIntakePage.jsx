import { useEffect, useMemo, useRef, useState } from 'react';
import { Link, useLocation } from 'react-router-dom';
import { supabase } from '../app/supabaseClient';
import { getDoWhizApiBaseUrl } from '../analytics';
import {
  CHANNEL_OPTIONS,
  PLAN_HORIZON_OPTIONS,
  REPO_PROVIDER_OPTIONS,
  STAGE_OPTIONS,
  createFounderIntakeDefaults,
  createValidatedWorkspaceBlueprintFromIntake,
  loadWorkspaceBlueprint,
  saveWorkspaceBlueprint
} from '../domain/workspaceBlueprint';

const DASHBOARD_PATH = '/auth/index.html?loggedIn=true#section-workspace';
const INTAKE_CHAT_API_PATH = '/api/startup-workspace/intake-chat';
const EDIT_MODE_QUERY_VALUE = 'edit';

const DEFAULT_RESOURCE_SELECTIONS = {
  build_system: 'github',
  formal_docs: 'google_docs',
  coordination: 'slack',
  external_execution: 'email'
};

const INITIAL_ASSISTANT_PROMPT =
  'Describe the project you want to start. I will ask follow-ups and build the JSON blueprint draft with you.';

function createMessage(role, text) {
  return {
    id: `${role}-${Date.now()}-${Math.random().toString(36).slice(2, 10)}`,
    role,
    text
  };
}

function createInitialMessages() {
  return [createMessage('assistant', INITIAL_ASSISTANT_PROMPT)];
}

function normalizeLaunchMode(mode) {
  const value = String(mode || '').trim().toLowerCase();
  if (!value) return null;
  if (value.includes('default') || value === 'auto') return 'default';
  if (value.includes('custom') || value === 'manual') return 'custom';
  return null;
}

function normalizeToolValue(value, allowed) {
  const normalized = String(value || '').trim().toLowerCase();
  return allowed.includes(normalized) ? normalized : null;
}

function normalizeToolSelections(mode, resourceTools = {}) {
  if (mode === 'default') {
    return { ...DEFAULT_RESOURCE_SELECTIONS };
  }

  return {
    build_system:
      normalizeToolValue(resourceTools.build_system, ['github', 'gitlab', 'bitbucket']) || 'github',
    formal_docs: normalizeToolValue(resourceTools.formal_docs, ['google_docs', 'notion']) || 'google_docs',
    coordination: normalizeToolValue(resourceTools.coordination, ['slack', 'discord', 'email']) || 'slack',
    external_execution:
      normalizeToolValue(resourceTools.external_execution, ['email', 'slack', 'discord']) || 'email'
  };
}

function clampPlanHorizon(value) {
  const numeric = Number.parseInt(value, 10);
  if (!Number.isFinite(numeric) || Number.isNaN(numeric) || numeric <= 30) {
    return '30';
  }
  if (numeric <= 60) {
    return '60';
  }
  return '90';
}

function normalizeStringList(values) {
  if (!Array.isArray(values)) {
    return [];
  }
  const seen = new Set();
  const result = [];
  for (const value of values) {
    const normalized = String(value || '').trim();
    if (!normalized) {
      continue;
    }
    const key = normalized.toLowerCase();
    if (!seen.has(key)) {
      seen.add(key);
      result.push(normalized);
    }
  }
  return result;
}

function normalizeRequestedAgents(values) {
  if (!Array.isArray(values)) {
    return [];
  }
  const seen = new Set();
  const result = [];
  for (const value of values) {
    const role = String(value?.role || '').trim();
    const owner = String(value?.owner || '').trim();
    if (!role) {
      continue;
    }
    const key = role.toLowerCase();
    if (seen.has(key)) {
      continue;
    }
    seen.add(key);
    result.push({
      role,
      owner
    });
  }
  return result;
}

function applyResourceSelectionsToIntake(intake, selections) {
  const next = {
    ...intake
  };

  const channels = [];
  const addChannel = (channel) => {
    if (!channels.includes(channel)) {
      channels.push(channel);
    }
  };

  if (selections.build_system === 'github') {
    addChannel('GitHub');
  }

  if (selections.formal_docs === 'google_docs') {
    addChannel('Google Docs');
  }

  if (selections.coordination === 'slack') {
    addChannel('Slack');
  }
  if (selections.coordination === 'discord') {
    addChannel('Discord');
  }
  if (selections.coordination === 'email') {
    addChannel('Email');
  }

  if (selections.external_execution === 'email') {
    addChannel('Email');
  }
  if (selections.external_execution === 'slack') {
    addChannel('Slack');
  }
  if (selections.external_execution === 'discord') {
    addChannel('Discord');
  }

  if (channels.length === 0) {
    addChannel('Email');
  }

  const hasExistingRepo =
    selections.build_system === 'github' ||
    selections.build_system === 'gitlab' ||
    selections.build_system === 'bitbucket';

  next.preferred_channels = channels;
  next.has_existing_repo = hasExistingRepo;
  next.primary_repo_provider = hasExistingRepo ? selections.build_system : 'github';
  next.has_docs_workspace = selections.formal_docs === 'google_docs' || selections.formal_docs === 'notion';

  return next;
}

function mapDraftToIntake(draft) {
  const base = createFounderIntakeDefaults();
  const launchMode = normalizeLaunchMode(draft?.resource_launch_mode);
  const normalizedTools = normalizeToolSelections(launchMode, draft?.resource_tools || {});
  const withResources = applyResourceSelectionsToIntake(base, normalizedTools);

  const goals = normalizeStringList(draft?.goals_30_90_days);
  const assets = normalizeStringList(draft?.current_assets);
  const requestedAgents = normalizeRequestedAgents(draft?.requested_agents);
  const requestedAgentsText = requestedAgents
    .map((agent) => (agent.owner ? `${agent.role}:${agent.owner}` : agent.role))
    .join('\n');

  return {
    ...withResources,
    founder_name: String(draft?.founder_name || '').trim(),
    founder_email: String(draft?.founder_email || '').trim(),
    venture_name: String(draft?.venture_name || '').trim(),
    venture_thesis: String(draft?.venture_thesis || '').trim(),
    venture_stage: String(draft?.venture_stage || '').trim() || 'idea',
    plan_horizon_days: clampPlanHorizon(draft?.plan_horizon_days),
    goals_text: goals.join('\n'),
    assets_text: assets.join('\n'),
    requested_agents_text: requestedAgentsText
  };
}

function mapBlueprintToIntake(blueprint) {
  const base = createFounderIntakeDefaults();
  if (!blueprint) {
    return base;
  }

  const goals = normalizeStringList(blueprint.goals_30_90_days);
  const assets = normalizeStringList(blueprint.current_assets);
  const channels = normalizeStringList(blueprint.preferred_channels);
  const requestedAgents = normalizeRequestedAgents(blueprint.requested_agents);
  const requestedAgentsText = requestedAgents
    .map((agent) => (agent.owner ? `${agent.role}:${agent.owner}` : agent.role))
    .join('\n');

  return {
    ...base,
    founder_name: String(blueprint.founder?.name || '').trim(),
    founder_email: String(blueprint.founder?.email || '').trim(),
    venture_name: String(blueprint.venture?.name || '').trim(),
    venture_thesis: String(blueprint.venture?.thesis || '').trim(),
    venture_stage: String(blueprint.venture?.stage || '').trim() || base.venture_stage,
    plan_horizon_days: clampPlanHorizon(blueprint.plan_horizon_days),
    goals_text: goals.join('\n'),
    assets_text: assets.join('\n'),
    preferred_channels: channels.length ? channels : base.preferred_channels,
    has_existing_repo: Boolean(blueprint.stack?.has_existing_repo),
    primary_repo_provider:
      normalizeToolValue(blueprint.stack?.primary_repo_provider, REPO_PROVIDER_OPTIONS) ||
      base.primary_repo_provider,
    has_docs_workspace: Boolean(blueprint.stack?.has_docs_workspace),
    requested_agents_text: requestedAgentsText
  };
}

function summarizeToolSelections(draft) {
  const mode = normalizeLaunchMode(draft?.resource_launch_mode);
  const tools = normalizeToolSelections(mode, draft?.resource_tools || {});
  return [
    `Mode: ${mode || 'not selected'}`,
    `Build System: ${tools.build_system}`,
    `Formal Docs: ${tools.formal_docs}`,
    `Coordination: ${tools.coordination}`,
    `External Execution: ${tools.external_execution}`
  ].join('\n');
}

function serializeMessagesForApi(messages) {
  return messages.map((message) => ({
    role: message.role,
    content: message.text
  }));
}

function openDashboardAuthPopup() {
  if (typeof window === 'undefined') {
    return null;
  }

  const width = 540;
  const height = 760;
  const left = Math.max(0, window.screenX + Math.round((window.outerWidth - width) / 2));
  const top = Math.max(0, window.screenY + Math.round((window.outerHeight - height) / 2));

  return window.open(
    DASHBOARD_PATH,
    'dowhiz_auth',
    `popup=yes,width=${width},height=${height},left=${left},top=${top},resizable=yes,scrollbars=yes`
  );
}

function StartupIntakePage() {
  const location = useLocation();
  const savedBlueprint = useMemo(() => loadWorkspaceBlueprint(), []);
  const hasSavedBlueprint = Boolean(savedBlueprint);
  const isEditModeRequested = useMemo(() => {
    const searchParams = new URLSearchParams(location.search);
    return searchParams.get('mode') === EDIT_MODE_QUERY_VALUE;
  }, [location.search]);
  const shouldShowQuestionnaire = isEditModeRequested && hasSavedBlueprint;

  const [messages, setMessages] = useState(() => createInitialMessages());
  const [inputValue, setInputValue] = useState('');
  const [isSending, setIsSending] = useState(false);
  const [errors, setErrors] = useState([]);
  const [blueprint, setBlueprint] = useState(null);
  const [intakeDraft, setIntakeDraft] = useState(null);
  const [questionnaireIntake, setQuestionnaireIntake] = useState(() =>
    mapBlueprintToIntake(savedBlueprint)
  );
  const [questionnaireNotice, setQuestionnaireNotice] = useState('');
  const [missingFields, setMissingFields] = useState([]);
  const [readyForBlueprint, setReadyForBlueprint] = useState(false);
  const [requestError, setRequestError] = useState('');
  const [hasActiveSession, setHasActiveSession] = useState(false);
  const chatFeedRef = useRef(null);

  const blueprintJson = useMemo(
    () => (blueprint ? JSON.stringify(blueprint, null, 2) : ''),
    [blueprint]
  );

  const draftJson = useMemo(
    () => (intakeDraft ? JSON.stringify(intakeDraft, null, 2) : ''),
    [intakeDraft]
  );

  const selectedChannels = useMemo(() => {
    return new Set(
      normalizeStringList(questionnaireIntake.preferred_channels).map((channel) => channel.toLowerCase())
    );
  }, [questionnaireIntake.preferred_channels]);

  useEffect(() => {
    const node = chatFeedRef.current;
    if (!node) {
      return;
    }
    node.scrollTop = node.scrollHeight;
  }, [messages]);

  useEffect(() => {
    let isMounted = true;

    supabase.auth.getSession().then(({ data }) => {
      if (!isMounted) {
        return;
      }
      setHasActiveSession(Boolean(data?.session));
    });

    const { data: authStateChange } = supabase.auth.onAuthStateChange((_event, session) => {
      if (!isMounted) {
        return;
      }
      setHasActiveSession(Boolean(session));
    });

    return () => {
      isMounted = false;
      authStateChange?.subscription?.unsubscribe();
    };
  }, []);

  useEffect(() => {
    if (shouldShowQuestionnaire) {
      setQuestionnaireIntake(mapBlueprintToIntake(savedBlueprint));
      setQuestionnaireNotice('');
      setErrors([]);
      setBlueprint(null);
      setRequestError('');
    }
  }, [savedBlueprint, shouldShowQuestionnaire]);

  const addAssistantMessage = (text) => {
    setMessages((prev) => [...prev, createMessage('assistant', text)]);
  };

  const resetConversation = () => {
    setMessages(createInitialMessages());
    setInputValue('');
    setIsSending(false);
    setErrors([]);
    setBlueprint(null);
    setIntakeDraft(null);
    setMissingFields([]);
    setReadyForBlueprint(false);
    setRequestError('');
  };

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
    setRequestError('');
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
      const existingIndex = channels.findIndex(
        (item) => item.toLowerCase() === channel.toLowerCase()
      );

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
    setRequestError('');
    setBlueprint(null);
    setQuestionnaireNotice('');

    const normalizedIntake = {
      ...questionnaireIntake,
      founder_name: String(questionnaireIntake.founder_name || '').trim(),
      founder_email: String(questionnaireIntake.founder_email || '').trim(),
      venture_name: String(questionnaireIntake.venture_name || '').trim(),
      venture_thesis: String(questionnaireIntake.venture_thesis || '').trim(),
      venture_stage: String(questionnaireIntake.venture_stage || '').trim() || 'idea',
      plan_horizon_days: clampPlanHorizon(questionnaireIntake.plan_horizon_days),
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

  const handleTextSubmit = async (event) => {
    event.preventDefault();
    if (isSending) {
      return;
    }

    const value = inputValue.trim();
    if (!value) {
      return;
    }

    const userMessage = createMessage('user', value);
    const nextMessages = [...messages, userMessage];
    setMessages(nextMessages);
    setInputValue('');
    setErrors([]);
    setBlueprint(null);
    setRequestError('');
    setIsSending(true);

    try {
      const response = await fetch(`${getDoWhizApiBaseUrl()}${INTAKE_CHAT_API_PATH}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          messages: serializeMessagesForApi(nextMessages),
          current_draft: intakeDraft
        })
      });

      if (!response.ok) {
        const errorPayload = await response.json().catch(() => ({}));
        throw new Error(errorPayload.error || 'Failed to get a model response for startup intake.');
      }

      const payload = await response.json();
      const assistantMessage = String(payload.assistant_message || '').trim();
      setIntakeDraft(payload.intake_draft || null);
      setMissingFields(Array.isArray(payload.missing_fields) ? payload.missing_fields : []);
      setReadyForBlueprint(Boolean(payload.ready_for_blueprint));

      if (assistantMessage) {
        addAssistantMessage(assistantMessage);
      } else {
        addAssistantMessage('I updated the intake JSON. Please continue with any missing details.');
      }
    } catch (error) {
      const message =
        error instanceof Error
          ? error.message
          : 'Startup intake conversation request failed.';
      setRequestError(message);
      addAssistantMessage(
        'I could not reach the startup intake model right now. Please try again in a few seconds.'
      );
    } finally {
      setIsSending(false);
    }
  };

  const handleCreateBlueprint = () => {
    setErrors([]);
    setRequestError('');

    if (!intakeDraft) {
      addAssistantMessage('I need more intake context first. Please describe your project to continue.');
      return;
    }

    if (!readyForBlueprint) {
      addAssistantMessage(
        `I still need a few fields before creating the blueprint:\n- ${
          missingFields.length ? missingFields.join('\n- ') : 'additional details'
        }`
      );
      return;
    }

    const intake = mapDraftToIntake(intakeDraft);
    const result = createValidatedWorkspaceBlueprintFromIntake(intake);

    if (!result.is_valid) {
      setErrors(result.errors);
      setBlueprint(null);
      addAssistantMessage(`Validation still failed:\n- ${result.errors.join('\n- ')}`);
      return;
    }

    saveWorkspaceBlueprint(result.blueprint);
    setBlueprint(null);
    setErrors([]);

    if (hasActiveSession) {
      addAssistantMessage('Blueprint saved. Opening Team Workspace now.');
      window.location.assign(DASHBOARD_PATH);
      return;
    }

    const authPopup = openDashboardAuthPopup();
    if (authPopup) {
      authPopup.focus();
      addAssistantMessage(
        'Blueprint saved. Sign in or sign up in the popup. After auth, Team Workspace will reflect your blueprint.'
      );
      return;
    }

    addAssistantMessage(
      'Blueprint saved. Popup was blocked, so redirecting to sign in. After auth, Team Workspace will reflect your blueprint.'
    );
    window.location.assign(DASHBOARD_PATH);
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
                        <label
                          key={channel}
                          className={`intake-chip${isChecked ? ' is-checked' : ''}`}
                        >
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

  return (
    <main className="route-shell route-shell-intake">
      <div className="route-card route-card-intake">
        <p className="route-kicker">Conversational Intake</p>
        <h1>Create Your Agent Team</h1>
        <p>
          This chat is powered by GPT-5.4. Describe your project and I will gather what is needed for blueprint JSON.
        </p>

        {isEditModeRequested && !hasSavedBlueprint ? (
          <p className="workspace-inline-note">
            No saved team brief was found, so you are in first-time conversational setup.
          </p>
        ) : null}

        <section className="route-section intake-chat-shell" aria-label="Conversational workspace intake">
          <div className="intake-chat-feed" ref={chatFeedRef}>
            {messages.map((message) => (
              <article key={message.id} className={`intake-chat-message is-${message.role}`}>
                <p>{message.text}</p>
              </article>
            ))}
          </div>

          <form className="intake-chat-composer" onSubmit={handleTextSubmit}>
            <input
              type="text"
              className="intake-chat-input"
              value={inputValue}
              onChange={(event) => setInputValue(event.target.value)}
              placeholder="Share project details or answer the latest question..."
              disabled={isSending}
            />
            <button type="submit" className="btn btn-primary intake-chat-send-btn" disabled={isSending}>
              {isSending ? 'Thinking...' : 'Send'}
            </button>
          </form>
        </section>

        {requestError ? (
          <section className="route-section intake-errors" aria-live="polite">
            <h2>Conversation API error</h2>
            <ul>
              <li>{requestError}</li>
            </ul>
          </section>
        ) : null}

        {intakeDraft ? (
          <section className="route-section">
            <h2>Current JSON Draft</h2>
            <p>The model updates this draft every turn.</p>
            <pre className="intake-conversation-summary">{summarizeToolSelections(intakeDraft)}</pre>
            <details className="intake-advanced">
              <summary>View full draft JSON</summary>
              <pre className="intake-blueprint-preview">{draftJson}</pre>
            </details>
            <p className="workspace-inline-note">
              {readyForBlueprint
                ? 'Ready to create blueprint.'
                : `Missing fields: ${missingFields.length ? missingFields.join(', ') : 'waiting for more details'}`}
            </p>
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

        <div className="route-actions">
          <button type="button" className="btn btn-primary" onClick={handleCreateBlueprint}>
            Create blueprint now
          </button>
          <button type="button" className="btn btn-secondary" onClick={resetConversation}>
            Restart chat
          </button>
          <a className="btn btn-secondary" href={DASHBOARD_PATH}>
            Open dashboard
          </a>
          <Link className="btn btn-secondary" to="/">
            Back to landing
          </Link>
        </div>

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
