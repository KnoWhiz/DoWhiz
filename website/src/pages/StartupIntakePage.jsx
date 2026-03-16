import { useEffect, useMemo, useRef, useState } from 'react';
import { Link } from 'react-router-dom';
import { getDoWhizApiBaseUrl } from '../analytics';
import {
  createFounderIntakeDefaults,
  createValidatedWorkspaceBlueprintFromIntake,
  saveWorkspaceBlueprint
} from '../domain/workspaceBlueprint';

const DASHBOARD_PATH = '/auth/index.html?loggedIn=true#section-workspace';
const INTAKE_CHAT_API_PATH = '/api/startup-workspace/intake-chat';

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

function StartupIntakePage() {
  const [messages, setMessages] = useState(() => createInitialMessages());
  const [inputValue, setInputValue] = useState('');
  const [isSending, setIsSending] = useState(false);
  const [errors, setErrors] = useState([]);
  const [blueprint, setBlueprint] = useState(null);
  const [intakeDraft, setIntakeDraft] = useState(null);
  const [missingFields, setMissingFields] = useState([]);
  const [readyForBlueprint, setReadyForBlueprint] = useState(false);
  const [requestError, setRequestError] = useState('');
  const chatFeedRef = useRef(null);

  const blueprintJson = useMemo(
    () => (blueprint ? JSON.stringify(blueprint, null, 2) : ''),
    [blueprint]
  );

  const draftJson = useMemo(
    () => (intakeDraft ? JSON.stringify(intakeDraft, null, 2) : ''),
    [intakeDraft]
  );

  useEffect(() => {
    const node = chatFeedRef.current;
    if (!node) {
      return;
    }
    node.scrollTop = node.scrollHeight;
  }, [messages]);

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
    setBlueprint(result.blueprint);
    setErrors([]);
    addAssistantMessage(
      'Blueprint saved. Open your dashboard workspace section to continue setup.'
    );
  };

  return (
    <main className="route-shell route-shell-intake">
      <div className="route-card route-card-intake">
        <p className="route-kicker">Conversational Intake</p>
        <h1>Create Your Agent Team</h1>
        <p>
          This chat is powered by GPT-5.4. Describe your project and I will gather what is needed for blueprint JSON.
        </p>

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
