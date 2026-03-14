export const RESOURCE_PROVISIONING_STATE = {
  CONNECTED: 'connected',
  AVAILABLE_NOT_CONFIGURED: 'available_not_configured',
  PLANNED_MANUAL: 'planned_manual',
  BLOCKED: 'blocked'
};

export const RESOURCE_CATEGORY = {
  WORKSPACE_HOME: 'workspace_home',
  KNOWLEDGE_HUB_STRUCTURED: 'knowledge_hub_structured',
  FORMAL_DOCS: 'formal_docs',
  BUILD_SYSTEM: 'build_system',
  EXTERNAL_EXECUTION: 'external_execution',
  COORDINATION_LAYER: 'coordination_layer',
  PUBLISH_PRESENCE: 'publish_presence',
  AGENT_ROSTER: 'agent_roster',
  TASK_BOARD: 'task_board',
  ARTIFACT_QUEUE: 'artifact_queue',
  APPROVAL_POLICY: 'approval_policy'
};

const RESOURCE_OBJECT_DEFINITIONS = {
  [RESOURCE_CATEGORY.WORKSPACE_HOME]: {
    object_name: 'Workspace Home',
    object_purpose: 'Primary startup operating surface for context, tasks, artifacts, and approvals.'
  },
  [RESOURCE_CATEGORY.KNOWLEDGE_HUB_STRUCTURED]: {
    object_name: 'Knowledge Hub (Structured)',
    object_purpose: 'Structured operating hub for captured knowledge and decision records.'
  },
  [RESOURCE_CATEGORY.FORMAL_DOCS]: {
    object_name: 'Formal Docs',
    object_purpose: 'Formal document artifact layer for specs, plans, and stakeholder-ready outputs.'
  },
  [RESOURCE_CATEGORY.BUILD_SYSTEM]: {
    object_name: 'Build System',
    object_purpose: 'Code execution and delivery workflows through repository-connected tooling.'
  },
  [RESOURCE_CATEGORY.EXTERNAL_EXECUTION]: {
    object_name: 'External Execution',
    object_purpose: 'Outbound execution surface for external stakeholders and operating communication.'
  },
  [RESOURCE_CATEGORY.COORDINATION_LAYER]: {
    object_name: 'Coordination Layer',
    object_purpose: 'Internal coordination loop for status updates, approvals, and handoffs.'
  },
  [RESOURCE_CATEGORY.PUBLISH_PRESENCE]: {
    object_name: 'Publish Presence',
    object_purpose: 'Publishing and distribution surfaces for launch and ongoing presence.'
  },
  [RESOURCE_CATEGORY.AGENT_ROSTER]: {
    object_name: 'Agent Roster',
    object_purpose: 'Ownership map for digital founding-team roles and responsibilities.'
  },
  [RESOURCE_CATEGORY.TASK_BOARD]: {
    object_name: 'Task Board',
    object_purpose: 'Execution board for startup milestones and active work.'
  },
  [RESOURCE_CATEGORY.ARTIFACT_QUEUE]: {
    object_name: 'Artifact Queue',
    object_purpose: 'Queue of generated artifacts for reviewable, auditable delivery.'
  },
  [RESOURCE_CATEGORY.APPROVAL_POLICY]: {
    object_name: 'Approval Policy',
    object_purpose: 'Human-review policy layer for sensitive or external actions.'
  }
};

const PROVIDER_METADATA = {
  dowhiz_workspace: { key: 'dowhiz_workspace', display_name: 'DoWhiz Workspace' },
  dowhiz_agents: { key: 'dowhiz_agents', display_name: 'DoWhiz Agents' },
  dowhiz_task_board: { key: 'dowhiz_task_board', display_name: 'DoWhiz Task Board' },
  dowhiz_artifacts: { key: 'dowhiz_artifacts', display_name: 'DoWhiz Artifacts' },
  github: { key: 'github', display_name: 'GitHub' },
  gitlab: { key: 'gitlab', display_name: 'GitLab' },
  bitbucket: { key: 'bitbucket', display_name: 'Bitbucket' },
  google_docs: { key: 'google_docs', display_name: 'Google Docs' },
  notion: { key: 'notion', display_name: 'Notion' },
  email: { key: 'email', display_name: 'Email' },
  slack: { key: 'slack', display_name: 'Slack' },
  discord: { key: 'discord', display_name: 'Discord' },
  publish_channels: { key: 'publish_channels', display_name: 'Publishing Channels' }
};

export function getResourceObjectDefinition(category) {
  return (
    RESOURCE_OBJECT_DEFINITIONS[category] || {
      object_name: category,
      object_purpose: 'Workspace resource object'
    }
  );
}

export function getProviderMetadata(providerKey) {
  return (
    PROVIDER_METADATA[providerKey] || {
      key: providerKey,
      display_name: 'Provider'
    }
  );
}

export function getProvisioningLabel(state) {
  const labels = {
    [RESOURCE_PROVISIONING_STATE.CONNECTED]: 'Connected',
    [RESOURCE_PROVISIONING_STATE.AVAILABLE_NOT_CONFIGURED]: 'Available, not configured',
    [RESOURCE_PROVISIONING_STATE.PLANNED_MANUAL]: 'Planned / manual',
    [RESOURCE_PROVISIONING_STATE.BLOCKED]: 'Blocked'
  };

  return labels[state] || state;
}

export function buildStarterResourceObjects(blueprint) {
  const resources = [];

  addResource(resources, RESOURCE_CATEGORY.WORKSPACE_HOME, 'dowhiz_workspace', {
    state: RESOURCE_PROVISIONING_STATE.CONNECTED,
    note: 'Workspace shell becomes the primary operating surface.'
  });
  addResource(resources, RESOURCE_CATEGORY.AGENT_ROSTER, 'dowhiz_agents', {
    state: RESOURCE_PROVISIONING_STATE.CONNECTED,
    note: 'Founding-team ownership map is ready.'
  });
  addResource(resources, RESOURCE_CATEGORY.TASK_BOARD, 'dowhiz_task_board', {
    state: RESOURCE_PROVISIONING_STATE.AVAILABLE_NOT_CONFIGURED,
    note: 'Starter task graph lands here once bootstrap is wired to runtime.',
    manual_next_step: 'Confirm default board lanes and SLA expectations.'
  });
  addResource(resources, RESOURCE_CATEGORY.ARTIFACT_QUEUE, 'dowhiz_artifacts', {
    state: RESOURCE_PROVISIONING_STATE.AVAILABLE_NOT_CONFIGURED,
    note: 'Artifact queue is reviewable before broad automation.',
    manual_next_step: 'Choose artifact retention and review policy.'
  });

  if (blueprint.stack.has_existing_repo || channelRequested(blueprint, 'github')) {
    const providerKey = blueprint.stack.primary_repo_provider || 'github';
    addResource(resources, RESOURCE_CATEGORY.BUILD_SYSTEM, normalizeRepoProvider(providerKey), {
      state: RESOURCE_PROVISIONING_STATE.CONNECTED,
      note: 'Repository execution workflows can run immediately.'
    });
  } else {
    addResource(resources, RESOURCE_CATEGORY.BUILD_SYSTEM, 'github', {
      state: RESOURCE_PROVISIONING_STATE.AVAILABLE_NOT_CONFIGURED,
      note: 'Connect a repository to unlock build-system execution.',
      manual_next_step: 'Connect GitHub (or another repo provider) and approve access scope.'
    });
  }

  if (
    blueprint.stack.has_docs_workspace ||
    channelRequested(blueprint, 'google docs') ||
    channelRequested(blueprint, 'google sheets') ||
    channelRequested(blueprint, 'google slides')
  ) {
    addResource(resources, RESOURCE_CATEGORY.FORMAL_DOCS, 'google_docs', {
      state: RESOURCE_PROVISIONING_STATE.CONNECTED,
      note: 'Formal document artifact layer is connected.'
    });
  } else {
    addResource(resources, RESOURCE_CATEGORY.FORMAL_DOCS, 'google_docs', {
      state: RESOURCE_PROVISIONING_STATE.AVAILABLE_NOT_CONFIGURED,
      note: 'Formal document artifacts are available once Google Docs is connected.',
      manual_next_step: 'Connect Google Docs for specification and execution artifacts.'
    });
  }

  if (channelRequested(blueprint, 'email')) {
    addResource(resources, RESOURCE_CATEGORY.EXTERNAL_EXECUTION, 'email', {
      state: RESOURCE_PROVISIONING_STATE.CONNECTED,
      note: 'External execution channel is active through email.'
    });
  } else {
    addResource(resources, RESOURCE_CATEGORY.EXTERNAL_EXECUTION, 'email', {
      state: RESOURCE_PROVISIONING_STATE.AVAILABLE_NOT_CONFIGURED,
      note: 'Email is the default external execution surface.',
      manual_next_step: 'Connect or approve outbound email routing.'
    });
  }

  const wantsCoordination = channelRequested(blueprint, 'slack') || channelRequested(blueprint, 'discord');
  const coordinationProvider = channelRequested(blueprint, 'slack') ? 'slack' : 'discord';

  if (wantsCoordination) {
    addResource(resources, RESOURCE_CATEGORY.COORDINATION_LAYER, coordinationProvider, {
      state: RESOURCE_PROVISIONING_STATE.CONNECTED,
      note: 'Coordination channel is active for status updates and approvals.'
    });
    addResource(resources, RESOURCE_CATEGORY.APPROVAL_POLICY, coordinationProvider, {
      state: RESOURCE_PROVISIONING_STATE.CONNECTED,
      note: 'Approval policy is enforced in the active coordination channel.'
    });
  } else {
    addResource(resources, RESOURCE_CATEGORY.COORDINATION_LAYER, 'slack', {
      state: RESOURCE_PROVISIONING_STATE.PLANNED_MANUAL,
      note: 'Slack/Discord can be connected later for team coordination.',
      manual_next_step: 'Connect Slack or Discord and assign the coordination channel.'
    });
    addResource(resources, RESOURCE_CATEGORY.APPROVAL_POLICY, 'slack', {
      state: RESOURCE_PROVISIONING_STATE.PLANNED_MANUAL,
      note: 'Approval routing depends on coordination channel setup.',
      manual_next_step: 'Configure approval path after Slack/Discord connection.'
    });
  }

  addResource(resources, RESOURCE_CATEGORY.KNOWLEDGE_HUB_STRUCTURED, 'notion', {
    state: RESOURCE_PROVISIONING_STATE.PLANNED_MANUAL,
    note: 'Structured operating hub can be modeled in Notion.',
    manual_next_step: 'Create Notion workspace + templates for recurring operating reviews.'
  });

  const hasPublishIdentity = String(blueprint.venture.name || '').trim().length > 0;
  if (hasPublishIdentity) {
    addResource(resources, RESOURCE_CATEGORY.PUBLISH_PRESENCE, 'publish_channels', {
      state: RESOURCE_PROVISIONING_STATE.PLANNED_MANUAL,
      note: 'Publishing/distribution presence can be connected incrementally.',
      manual_next_step: 'Select launch channels (for example LinkedIn/X/Product Hunt).'
    });
  } else {
    addResource(resources, RESOURCE_CATEGORY.PUBLISH_PRESENCE, 'publish_channels', {
      state: RESOURCE_PROVISIONING_STATE.BLOCKED,
      note: 'Publishing presence is blocked until startup identity is defined.',
      manual_next_step: 'Set a startup/project name in intake to unblock publish presence setup.'
    });
  }

  return resources;
}

function addResource(resources, category, providerKey, partial) {
  if (
    resources.some(
      (item) => item.category === category && item.provider.key === getProviderMetadata(providerKey).key
    )
  ) {
    return;
  }

  const objectDefinition = getResourceObjectDefinition(category);
  const provider = getProviderMetadata(providerKey);

  resources.push({
    category,
    object_name: objectDefinition.object_name,
    object_purpose: objectDefinition.object_purpose,
    provider,
    state: partial.state,
    note: partial.note || null,
    manual_next_step: partial.manual_next_step || null
  });
}

function normalizeRepoProvider(providerKey) {
  const lower = String(providerKey || '').toLowerCase();

  if (lower.includes('gitlab')) {
    return 'gitlab';
  }
  if (lower.includes('bitbucket')) {
    return 'bitbucket';
  }
  return 'github';
}

function channelRequested(blueprint, needle) {
  const requested = blueprint.preferred_channels || [];
  const needleLower = String(needle).toLowerCase();

  return requested.some((channel) => String(channel || '').toLowerCase().includes(needleLower));
}
