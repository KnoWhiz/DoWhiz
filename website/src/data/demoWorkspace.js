export const demoWorkspace = {
  name: 'Founder Workspace (Preview)',
  summary: 'This demo workspace shows the operating-system view before a founder intake is submitted.',
  blueprint: {
    schema_version: '2026-03-13',
    founder: {
      name: 'Demo Founder',
      email: 'founder@example.com'
    },
    venture: {
      name: 'Acme Launchpad',
      thesis: 'Build a vertical AI copilot for customer onboarding workflows.',
      stage: 'mvp'
    },
    plan_horizon_days: 60,
    goals_30_90_days: [
      'Launch MVP with 3 pilot customers',
      'Ship onboarding analytics dashboard',
      'Close first paid design partner'
    ],
    current_assets: ['Landing page', 'Figma mockups', 'Pilot interview notes'],
    preferred_channels: ['Email', 'GitHub', 'Slack', 'Google Docs'],
    stack: {
      has_existing_repo: true,
      primary_repo_provider: 'github',
      has_docs_workspace: true
    },
    requested_agents: [
      { role: 'Builder', owner: 'Demo Founder' },
      { role: 'Chief of Staff', owner: null },
      { role: 'GTM Strategist', owner: null }
    ]
  }
};
