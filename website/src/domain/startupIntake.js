import { REPO_PROVIDER_OPTIONS, createFounderIntakeDefaults } from './workspaceBlueprint';

export const DASHBOARD_PATH = '/auth/index.html?loggedIn=true#section-workspace';
export const DASHBOARD_POPUP_AUTH_PATH = '/auth/index.html?loggedIn=true&popupAuth=1#section-workspace';
export const AUTH_POPUP_SUCCESS_EVENT = 'dowhiz-auth-popup-success';
export const INTAKE_CHAT_API_PATH = '/api/startup-workspace/intake-chat';
export const EDIT_MODE_QUERY_VALUE = 'edit';

const DEFAULT_RESOURCE_SELECTIONS = {
  build_system: 'github',
  formal_docs: 'google_docs',
  coordination: 'slack',
  external_execution: 'email'
};

const INITIAL_ASSISTANT_PROMPT =
  'Describe the project you want to start. I will ask follow-ups and build the JSON blueprint draft with you.';

export function createConversationMessage(role, text) {
  return {
    id: `${role}-${Date.now()}-${Math.random().toString(36).slice(2, 10)}`,
    role,
    text
  };
}

export function createInitialConversationMessages() {
  return [createConversationMessage('assistant', INITIAL_ASSISTANT_PROMPT)];
}

function normalizeLaunchMode(mode) {
  const value = String(mode || '').trim().toLowerCase();
  if (!value) return null;
  if (value.includes('default') || value === 'auto') return 'default';
  if (value.includes('custom') || value === 'manual') return 'custom';
  return null;
}

export function normalizeToolValue(value, allowed) {
  const normalized = String(value || '').trim().toLowerCase();
  return allowed.includes(normalized) ? normalized : null;
}

function normalizeToolSelections(mode, resourceTools = {}) {
  if (mode === 'default') {
    return { ...DEFAULT_RESOURCE_SELECTIONS };
  }

  return {
    build_system: normalizeToolValue(resourceTools.build_system, ['github', 'gitlab', 'bitbucket']) || 'github',
    formal_docs: normalizeToolValue(resourceTools.formal_docs, ['google_docs', 'notion']) || 'google_docs',
    coordination: normalizeToolValue(resourceTools.coordination, ['slack', 'discord', 'email']) || 'slack',
    external_execution:
      normalizeToolValue(resourceTools.external_execution, ['email', 'slack', 'discord']) || 'email'
  };
}

export function clampPlanHorizonSelection(value) {
  const numeric = Number.parseInt(value, 10);
  if (!Number.isFinite(numeric) || Number.isNaN(numeric) || numeric <= 30) {
    return '30';
  }
  if (numeric <= 60) {
    return '60';
  }
  return '90';
}

export function normalizeStringList(values) {
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

export function mapDraftToIntake(draft) {
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
    plan_horizon_days: clampPlanHorizonSelection(draft?.plan_horizon_days),
    goals_text: goals.join('\n'),
    assets_text: assets.join('\n'),
    requested_agents_text: requestedAgentsText
  };
}

export function mapBlueprintToIntake(blueprint) {
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
    plan_horizon_days: clampPlanHorizonSelection(blueprint.plan_horizon_days),
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

export function summarizeToolSelections(draft) {
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

export async function requestStartupIntakeTurn(apiBaseUrl, messages, currentDraft) {
  const response = await fetch(`${apiBaseUrl}${INTAKE_CHAT_API_PATH}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      messages: serializeMessagesForApi(messages),
      current_draft: currentDraft
    })
  });

  if (!response.ok) {
    const errorPayload = await response.json().catch(() => ({}));
    throw new Error(errorPayload.error || 'Failed to get a model response for startup intake.');
  }

  return response.json();
}

export function openDashboardAuthPopup() {
  if (typeof window === 'undefined') {
    return null;
  }

  const width = 540;
  const height = 760;
  const left = Math.max(0, window.screenX + Math.round((window.outerWidth - width) / 2));
  const top = Math.max(0, window.screenY + Math.round((window.outerHeight - height) / 2));

  return window.open(
    DASHBOARD_POPUP_AUTH_PATH,
    'dowhiz_auth',
    `popup=yes,width=${width},height=${height},left=${left},top=${top},resizable=yes,scrollbars=yes`
  );
}
