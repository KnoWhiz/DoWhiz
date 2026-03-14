import { RESOURCE_CATEGORY, RESOURCE_PROVISIONING_STATE } from './resourceModel';

export function createWorkspaceHomeModel(blueprint, options = {}) {
  const demoMode = Boolean(options.demoMode);
  const startupName = blueprint.venture.name || 'Founder Workspace';
  const founderName = blueprint.founder.name || 'Founder';

  const resources = deriveResources(blueprint);
  const starterTasks = deriveStarterTasks(blueprint);
  const recentArtifacts = deriveRecentArtifacts(blueprint, demoMode);
  const approvalQueue = deriveApprovalQueue(blueprint);
  const nextActions = deriveNextActions(resources, approvalQueue, starterTasks);
  const agentRoster = deriveAgentRoster(blueprint, demoMode);

  return {
    title: startupName,
    subtitle: blueprint.venture.thesis,
    founderName,
    stage: blueprint.venture.stage || 'idea',
    planHorizonDays: blueprint.plan_horizon_days,
    goals: blueprint.goals_30_90_days,
    currentAssets: blueprint.current_assets,
    preferredChannels: blueprint.preferred_channels,
    stack: blueprint.stack,
    agentRoster,
    resources,
    starterTasks,
    recentArtifacts,
    approvalQueue,
    nextActions,
    workspaceHealth: summarizeHealth(resources)
  };
}

export function getResourceCategoryLabel(category) {
  const labels = {
    [RESOURCE_CATEGORY.WORKSPACE_HOME]: 'Workspace Home',
    [RESOURCE_CATEGORY.KNOWLEDGE_HUB_STRUCTURED]: 'Knowledge Hub (Structured)',
    [RESOURCE_CATEGORY.FORMAL_DOCS]: 'Formal Docs',
    [RESOURCE_CATEGORY.BUILD_SYSTEM]: 'Build System',
    [RESOURCE_CATEGORY.EXTERNAL_EXECUTION]: 'External Execution',
    [RESOURCE_CATEGORY.COORDINATION_LAYER]: 'Coordination Layer',
    [RESOURCE_CATEGORY.PUBLISH_PRESENCE]: 'Publish Presence',
    [RESOURCE_CATEGORY.AGENT_ROSTER]: 'Agent Roster',
    [RESOURCE_CATEGORY.TASK_BOARD]: 'Task Board',
    [RESOURCE_CATEGORY.ARTIFACT_QUEUE]: 'Artifact Queue',
    [RESOURCE_CATEGORY.APPROVAL_POLICY]: 'Approval Policy'
  };

  return labels[category] || category;
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

function deriveAgentRoster(blueprint, demoMode) {
  const requested = blueprint.requested_agents || [];

  if (requested.length) {
    return requested.map((agent, index) => ({
      name: agent.owner || `${agent.role} Owner`,
      role: agent.role,
      status: index === 0 ? 'active' : 'planned',
      focus: focusForRole(agent.role)
    }));
  }

  return [
    {
      name: demoMode ? 'Oliver' : 'Generalist Owner',
      role: 'Generalist',
      status: 'active',
      focus: 'Coordinate early execution across tools and channels.'
    },
    {
      name: 'Builder Lead',
      role: 'Builder',
      status: 'planned',
      focus: 'Ship MVP milestones and repo workflows.'
    },
    {
      name: 'GTM Lead',
      role: 'GTM Strategist',
      status: 'planned',
      focus: 'Drive distribution loops and launch readiness.'
    }
  ];
}

function deriveResources(blueprint) {
  const resources = [
    {
      category: RESOURCE_CATEGORY.WORKSPACE_HOME,
      provider: 'dowhiz_workspace',
      state: RESOURCE_PROVISIONING_STATE.CONNECTED,
      note: 'Primary startup operating surface.'
    },
    {
      category: RESOURCE_CATEGORY.AGENT_ROSTER,
      provider: 'dowhiz_agents',
      state: RESOURCE_PROVISIONING_STATE.CONNECTED,
      note: 'Digital team assignments and ownership.'
    },
    {
      category: RESOURCE_CATEGORY.TASK_BOARD,
      provider: 'dowhiz_task_board',
      state: RESOURCE_PROVISIONING_STATE.AVAILABLE_NOT_CONFIGURED,
      note: 'Starter task board is generated from blueprint.'
    },
    {
      category: RESOURCE_CATEGORY.ARTIFACT_QUEUE,
      provider: 'dowhiz_artifacts',
      state: RESOURCE_PROVISIONING_STATE.AVAILABLE_NOT_CONFIGURED,
      note: 'Reviewable artifact queue before delivery.'
    },
    {
      category: RESOURCE_CATEGORY.APPROVAL_POLICY,
      provider: 'dowhiz_approvals',
      state: RESOURCE_PROVISIONING_STATE.AVAILABLE_NOT_CONFIGURED,
      note: 'Human approval stays explicit for sensitive actions.'
    },
    {
      category: RESOURCE_CATEGORY.KNOWLEDGE_HUB_STRUCTURED,
      provider: 'notion',
      state: RESOURCE_PROVISIONING_STATE.PLANNED_MANUAL,
      note: 'Modeled now; may require manual setup.'
    },
    {
      category: RESOURCE_CATEGORY.PUBLISH_PRESENCE,
      provider: 'distribution_channels',
      state: RESOURCE_PROVISIONING_STATE.PLANNED_MANUAL,
      note: 'Publishing integrations can be phased in.'
    }
  ];

  resources.push({
    category: RESOURCE_CATEGORY.BUILD_SYSTEM,
    provider: blueprint.stack.primary_repo_provider || 'github',
    state: blueprint.stack.has_existing_repo
      ? RESOURCE_PROVISIONING_STATE.CONNECTED
      : RESOURCE_PROVISIONING_STATE.AVAILABLE_NOT_CONFIGURED,
    note: blueprint.stack.has_existing_repo
      ? 'Repository workflows can run immediately.'
      : 'Connect repository to unlock build workflows.'
  });

  const hasGoogleWorkspace =
    blueprint.stack.has_docs_workspace ||
    blueprint.preferred_channels.some((channel) => channel.toLowerCase().includes('google'));

  resources.push({
    category: RESOURCE_CATEGORY.FORMAL_DOCS,
    provider: 'google_workspace',
    state: hasGoogleWorkspace
      ? RESOURCE_PROVISIONING_STATE.CONNECTED
      : RESOURCE_PROVISIONING_STATE.AVAILABLE_NOT_CONFIGURED,
    note: hasGoogleWorkspace
      ? 'Docs/sheets/slides are available for formal artifacts.'
      : 'Connect Google Workspace for formal document workflows.'
  });

  resources.push({
    category: RESOURCE_CATEGORY.EXTERNAL_EXECUTION,
    provider: 'email',
    state: blueprint.preferred_channels.some((channel) => channel.toLowerCase() === 'email')
      ? RESOURCE_PROVISIONING_STATE.CONNECTED
      : RESOURCE_PROVISIONING_STATE.AVAILABLE_NOT_CONFIGURED,
    note: 'Email remains a strong outbound execution channel.'
  });

  const coordinationProvider = blueprint.preferred_channels.find((channel) => {
    const lower = channel.toLowerCase();
    return lower === 'slack' || lower === 'discord';
  });

  resources.push({
    category: RESOURCE_CATEGORY.COORDINATION_LAYER,
    provider: coordinationProvider || 'slack',
    state: coordinationProvider
      ? RESOURCE_PROVISIONING_STATE.CONNECTED
      : RESOURCE_PROVISIONING_STATE.PLANNED_MANUAL,
    note: coordinationProvider
      ? 'Approvals and updates are routed in-channel.'
      : 'Connect Slack or Discord for coordination loops.'
  });

  return resources;
}

function deriveStarterTasks(blueprint) {
  const owner = blueprint.requested_agents[0]?.role || 'Generalist';
  const hasRepo = blueprint.stack.has_existing_repo;
  const hasCoordination = blueprint.preferred_channels.some((channel) => {
    const lower = channel.toLowerCase();
    return lower === 'slack' || lower === 'discord';
  });

  return [
    {
      id: 'task_workspace_brief',
      title: 'Finalize startup workspace brief',
      ownerRole: owner,
      status: 'planned',
      dependsOn: [],
      rationale: 'Turn founder thesis and goals into an operating brief.'
    },
    {
      id: 'task_30_day_plan',
      title: 'Generate 30-day execution plan',
      ownerRole: 'Chief of Staff',
      status: 'planned',
      dependsOn: ['task_workspace_brief'],
      rationale: 'Create milestones with ownership and review points.'
    },
    {
      id: hasRepo ? 'task_repo_bootstrap' : 'task_repo_connect',
      title: hasRepo
        ? 'Bootstrap delivery workflow in repository'
        : 'Connect repository for build system workflows',
      ownerRole: hasRepo ? 'Builder' : 'Founder',
      status: hasRepo ? 'planned' : 'pending_review',
      dependsOn: ['task_30_day_plan'],
      rationale: hasRepo
        ? 'Repo is available for delivery operations.'
        : 'Build workflows need repo connection first.'
    },
    {
      id: 'task_coordination_channel',
      title: hasCoordination
        ? 'Activate coordination channel status loop'
        : 'Connect Slack or Discord for coordination',
      ownerRole: hasCoordination ? 'Chief of Staff' : 'Founder',
      status: hasCoordination ? 'planned' : 'pending_review',
      dependsOn: ['task_workspace_brief'],
      rationale: 'Keep approvals and progress updates visible to humans.'
    }
  ];
}

function deriveRecentArtifacts(blueprint, demoMode) {
  const firstChannel = blueprint.preferred_channels[0] || 'Workspace';
  const goalArtifacts = blueprint.goals_30_90_days.slice(0, 2).map((goal, index) => ({
    id: `artifact_goal_${index}`,
    title: `Goal brief: ${goal}`,
    surface: firstChannel,
    status: 'draft',
    updatedAtLabel: index === 0 ? 'Just now' : '5 minutes ago'
  }));

  const baseline = [
    {
      id: 'artifact_founder_summary',
      title: 'Founder intake summary',
      surface: 'Workspace Home',
      status: 'active',
      updatedAtLabel: demoMode ? 'Demo data' : 'Just now'
    }
  ];

  return [...baseline, ...goalArtifacts];
}

function deriveApprovalQueue(blueprint) {
  const queue = [];

  if (!blueprint.stack.has_existing_repo) {
    queue.push({
      id: 'approval_repo_connection',
      title: 'Approve repository connection scope',
      owner: 'Founder',
      status: 'pending_review',
      reason: 'Build-system execution requires repo access approval.'
    });
  }

  if (
    !blueprint.preferred_channels.some((channel) => {
      const lower = channel.toLowerCase();
      return lower === 'slack' || lower === 'discord';
    })
  ) {
    queue.push({
      id: 'approval_coordination_channel',
      title: 'Select coordination channel',
      owner: 'Founder',
      status: 'pending_review',
      reason: 'Coordination surface is needed for approvals and updates.'
    });
  }

  queue.push({
    id: 'approval_external_delivery',
    title: 'Review outbound delivery policy',
    owner: 'Founder',
    status: 'pending_review',
    reason: 'Keep human approval explicit before external execution.'
  });

  return queue;
}

function deriveNextActions(resources, approvalQueue, starterTasks) {
  const pendingResources = resources
    .filter((resource) => resource.state !== RESOURCE_PROVISIONING_STATE.CONNECTED)
    .slice(0, 2)
    .map(
      (resource) =>
        `Configure ${getResourceCategoryLabel(resource.category)} (${resource.provider})`
    );

  const approvalActions = approvalQueue.slice(0, 2).map((item) => item.title);

  const starterTaskActions = starterTasks
    .filter((task) => task.status === 'planned')
    .slice(0, 1)
    .map((task) => `Start task: ${task.title}`);

  return [...pendingResources, ...approvalActions, ...starterTaskActions];
}

function summarizeHealth(resources) {
  const connected = resources.filter(
    (resource) => resource.state === RESOURCE_PROVISIONING_STATE.CONNECTED
  ).length;
  const nonConnected = resources.length - connected;

  return {
    connected,
    nonConnected,
    readinessLabel:
      nonConnected === 0
        ? 'Ready to execute'
        : connected >= 4
          ? 'Partially configured'
          : 'Needs setup'
  };
}

function focusForRole(role) {
  const lower = role.toLowerCase();
  if (lower.includes('builder') || lower.includes('coder') || lower.includes('engineer')) {
    return 'Ship product increments with test-ready outputs.';
  }
  if (lower.includes('gtm') || lower.includes('growth') || lower.includes('marketing')) {
    return 'Drive distribution and launch loops across channels.';
  }
  if (lower.includes('chief') || lower.includes('staff') || lower.includes('ops')) {
    return 'Orchestrate planning, follow-ups, and approvals.';
  }

  return 'Coordinate multi-channel execution with shared memory continuity.';
}
