export const WORKSPACE_BLUEPRINT_VERSION = '0.1.0';

export function createEmptyWorkspaceBlueprint() {
  return {
    version: WORKSPACE_BLUEPRINT_VERSION,
    founder: {
      name: '',
      company: '',
      thesis: ''
    },
    goals: [],
    preferredChannels: [],
    starterAgents: [],
    resources: []
  };
}
