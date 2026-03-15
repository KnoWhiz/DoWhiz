export const WORKSPACE_BLUEPRINT_SCHEMA_VERSION = '2026-03-13';
export const STARTUP_BLUEPRINT_STORAGE_KEY = 'dowhiz_startup_workspace_blueprint_v1';
// Canonical schema mirrors scheduler_module::domain::workspace_blueprint::StartupWorkspaceBlueprint.

export const STAGE_OPTIONS = [
  { value: 'idea', label: 'Idea' },
  { value: 'prototype', label: 'Prototype' },
  { value: 'mvp', label: 'MVP' },
  { value: 'post_mvp', label: 'Post-MVP' },
  { value: 'growth', label: 'Growth' }
];

export const PLAN_HORIZON_OPTIONS = [30, 60, 90];

export const CHANNEL_OPTIONS = [
  'Email',
  'Slack',
  'Discord',
  'GitHub',
  'Google Docs',
  'Google Sheets',
  'Google Slides'
];

export const REPO_PROVIDER_OPTIONS = ['github', 'gitlab', 'bitbucket', 'other'];

export function createEmptyWorkspaceBlueprint() {
  return {
    schema_version: WORKSPACE_BLUEPRINT_SCHEMA_VERSION,
    founder: {
      name: '',
      email: ''
    },
    venture: {
      name: '',
      thesis: '',
      stage: null
    },
    plan_horizon_days: 30,
    goals_30_90_days: [],
    current_assets: [],
    preferred_channels: [],
    stack: {
      has_existing_repo: false,
      primary_repo_provider: null,
      has_docs_workspace: false
    },
    requested_agents: []
  };
}

export function createFounderIntakeDefaults() {
  return {
    founder_name: '',
    founder_email: '',
    venture_name: '',
    venture_thesis: '',
    venture_stage: 'idea',
    plan_horizon_days: '30',
    goals_text: '',
    assets_text: '',
    preferred_channels: ['Email', 'GitHub'],
    has_existing_repo: false,
    primary_repo_provider: 'github',
    has_docs_workspace: false,
    requested_agents_text: ''
  };
}

export function buildWorkspaceBlueprintFromIntake(intake) {
  const goals = parseListField(intake.goals_text);
  const assets = parseListField(intake.assets_text);
  const requestedAgents = parseRequestedAgents(intake.requested_agents_text);

  return {
    schema_version: WORKSPACE_BLUEPRINT_SCHEMA_VERSION,
    founder: {
      name: intake.founder_name,
      email: intake.founder_email
    },
    venture: {
      name: intake.venture_name,
      thesis: intake.venture_thesis,
      stage: intake.venture_stage || null
    },
    plan_horizon_days: Number.parseInt(intake.plan_horizon_days, 10),
    goals_30_90_days: goals,
    current_assets: assets,
    preferred_channels: intake.preferred_channels || [],
    stack: {
      has_existing_repo: Boolean(intake.has_existing_repo),
      primary_repo_provider: intake.has_existing_repo ? intake.primary_repo_provider || null : null,
      has_docs_workspace: Boolean(intake.has_docs_workspace)
    },
    requested_agents: requestedAgents
  };
}

export function normalizeWorkspaceBlueprint(blueprint) {
  const normalized = {
    ...createEmptyWorkspaceBlueprint(),
    ...blueprint,
    schema_version: WORKSPACE_BLUEPRINT_SCHEMA_VERSION,
    founder: {
      ...createEmptyWorkspaceBlueprint().founder,
      ...(blueprint?.founder || {})
    },
    venture: {
      ...createEmptyWorkspaceBlueprint().venture,
      ...(blueprint?.venture || {})
    },
    stack: {
      ...createEmptyWorkspaceBlueprint().stack,
      ...(blueprint?.stack || {})
    }
  };

  normalized.founder.name = normalizeString(normalized.founder.name);
  normalized.founder.email = normalizeString(normalized.founder.email);
  normalized.venture.name = normalizeString(normalized.venture.name);
  normalized.venture.thesis = normalizeString(normalized.venture.thesis);
  normalized.venture.stage = normalizeOptionalString(normalized.venture.stage);

  normalized.plan_horizon_days = clampPlanHorizon(normalized.plan_horizon_days);
  normalized.goals_30_90_days = normalizeUniqueList(normalized.goals_30_90_days);
  normalized.current_assets = normalizeUniqueList(normalized.current_assets);
  normalized.preferred_channels = normalizeUniqueList(normalized.preferred_channels);

  normalized.stack.has_existing_repo = Boolean(normalized.stack.has_existing_repo);
  normalized.stack.has_docs_workspace = Boolean(normalized.stack.has_docs_workspace);
  normalized.stack.primary_repo_provider = normalized.stack.has_existing_repo
    ? normalizeOptionalString(normalized.stack.primary_repo_provider)
    : null;

  normalized.requested_agents = normalizeAgents(normalized.requested_agents);

  return normalized;
}

export function validateWorkspaceBlueprint(blueprint) {
  const errors = [];

  if (!normalizeString(blueprint?.founder?.name)) {
    errors.push('Founder name is required.');
  }

  if (!normalizeString(blueprint?.venture?.thesis)) {
    errors.push('Startup thesis is required.');
  }

  const horizon = Number.parseInt(blueprint?.plan_horizon_days, 10);
  if (![30, 60, 90].includes(horizon)) {
    errors.push('Plan horizon must be 30, 60, or 90 days.');
  }

  if (!Array.isArray(blueprint?.goals_30_90_days) || blueprint.goals_30_90_days.length === 0) {
    errors.push('At least one 30-90 day goal is required.');
  }

  return {
    is_valid: errors.length === 0,
    errors
  };
}

export function createValidatedWorkspaceBlueprintFromIntake(intake) {
  const candidate = buildWorkspaceBlueprintFromIntake(intake);
  const normalized = normalizeWorkspaceBlueprint(candidate);
  const validation = validateWorkspaceBlueprint(normalized);

  return {
    blueprint: normalized,
    ...validation
  };
}

export function saveWorkspaceBlueprint(blueprint) {
  if (typeof window === 'undefined') {
    return;
  }

  window.localStorage.setItem(STARTUP_BLUEPRINT_STORAGE_KEY, JSON.stringify(blueprint));
}

export function loadWorkspaceBlueprint() {
  if (typeof window === 'undefined') {
    return null;
  }

  const raw = window.localStorage.getItem(STARTUP_BLUEPRINT_STORAGE_KEY);
  if (!raw) {
    return null;
  }

  try {
    const parsed = JSON.parse(raw);
    const normalized = normalizeWorkspaceBlueprint(parsed);
    const validation = validateWorkspaceBlueprint(normalized);
    return validation.is_valid ? normalized : null;
  } catch {
    return null;
  }
}

function normalizeString(value) {
  return String(value || '').trim();
}

function normalizeOptionalString(value) {
  const normalized = normalizeString(value);
  return normalized || null;
}

function normalizeUniqueList(values) {
  if (!Array.isArray(values)) {
    return [];
  }

  const output = [];
  for (const value of values) {
    const normalized = normalizeString(value);
    if (!normalized) {
      continue;
    }
    if (!output.some((item) => item.toLowerCase() === normalized.toLowerCase())) {
      output.push(normalized);
    }
  }

  return output;
}

function normalizeAgents(values) {
  if (!Array.isArray(values)) {
    return [];
  }

  const output = [];
  for (const value of values) {
    const role = normalizeString(value?.role);
    if (!role) {
      continue;
    }

    const owner = normalizeOptionalString(value?.owner);
    if (!output.some((item) => item.role.toLowerCase() === role.toLowerCase())) {
      output.push({ role, owner });
    }
  }

  return output;
}

function parseListField(value) {
  return normalizeUniqueList(String(value || '').split(/\n|,/));
}

function parseRequestedAgents(value) {
  const lines = String(value || '').split('\n');
  const parsed = lines.map((line) => {
    const [rolePart, ...ownerParts] = line.split(':');
    return {
      role: normalizeString(rolePart),
      owner: normalizeOptionalString(ownerParts.join(':'))
    };
  });

  return normalizeAgents(parsed);
}

function clampPlanHorizon(value) {
  const numeric = Number.parseInt(value, 10);

  if (numeric <= 30 || Number.isNaN(numeric)) {
    return 30;
  }
  if (numeric <= 60) {
    return 60;
  }
  return 90;
}
