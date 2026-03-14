import {
  RESOURCE_PROVISIONING_STATE,
  buildStarterResourceObjects
} from './resourceModel';

export function createWorkspaceHomeModel(blueprint, options = {}) {
  const demoMode = Boolean(options.demoMode);
  const startupName = blueprint.venture.name || 'Founder Workspace';
  const founderName = blueprint.founder.name || 'Founder';

  const resources = buildStarterResourceObjects(blueprint);
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
    .map((resource) => {
      const providerName = resource.provider?.display_name || 'Provider';
      const baseAction = `Configure ${resource.object_name} (${providerName})`;
      return resource.manual_next_step ? `${baseAction}: ${resource.manual_next_step}` : baseAction;
    });

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
