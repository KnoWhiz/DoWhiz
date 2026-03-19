import test from 'node:test';
import assert from 'node:assert/strict';

import {
  applyProviderRuntimeState,
  buildStarterResourceObjects,
  RESOURCE_CATEGORY,
  RESOURCE_PROVISIONING_STATE
} from './resourceModel.js';

function buildBlueprint(overrides = {}) {
  return {
    venture: {
      name: 'Acme',
      ...(overrides.venture || {})
    },
    stack: {
      has_existing_repo: false,
      primary_repo_provider: null,
      has_docs_workspace: false,
      ...(overrides.stack || {})
    },
    preferred_channels: overrides.preferred_channels || []
  };
}

function findResource(resources, category) {
  const resource = resources.find((item) => item.category === category);

  assert.ok(resource, `expected resource for category ${category}`);
  return resource;
}

test('build system overlay preserves a selected GitLab provider when only GitHub runtime exists', () => {
  const starterResources = buildStarterResourceObjects(
    buildBlueprint({
      stack: {
        has_existing_repo: true,
        primary_repo_provider: 'gitlab'
      }
    })
  );

  const resources = applyProviderRuntimeState(starterResources, {
    runtime: {
      capabilities: { github: true },
      connected: { github: true }
    },
    identifiers: []
  });

  const buildSystem = findResource(resources, RESOURCE_CATEGORY.BUILD_SYSTEM);
  assert.equal(buildSystem.provider.key, 'gitlab');
  assert.equal(buildSystem.state, RESOURCE_PROVISIONING_STATE.PLANNED_MANUAL);
  assert.match(buildSystem.note, /GitLab remains the selected repository provider/i);
});

test('coordination overlay preserves Discord when both Slack and Discord runtimes are available', () => {
  const starterResources = buildStarterResourceObjects(
    buildBlueprint({
      preferred_channels: ['discord']
    })
  );

  const resources = applyProviderRuntimeState(starterResources, {
    runtime: {
      capabilities: { slack: true, discord: true },
      connected: {}
    },
    identifiers: []
  });

  const coordination = findResource(resources, RESOURCE_CATEGORY.COORDINATION_LAYER);
  const approval = findResource(resources, RESOURCE_CATEGORY.APPROVAL_POLICY);

  assert.equal(coordination.provider.key, 'discord');
  assert.equal(coordination.state, RESOURCE_PROVISIONING_STATE.AVAILABLE_NOT_CONFIGURED);
  assert.match(coordination.note, /Discord coordination runtime is available/i);

  assert.equal(approval.provider.key, 'discord');
  assert.equal(approval.state, RESOURCE_PROVISIONING_STATE.AVAILABLE_NOT_CONFIGURED);
  assert.match(approval.note, /Discord approval routing is available/i);
});

test('coordination overlay does not replace Discord with a connected Slack runtime', () => {
  const starterResources = buildStarterResourceObjects(
    buildBlueprint({
      preferred_channels: ['discord']
    })
  );

  const resources = applyProviderRuntimeState(starterResources, {
    runtime: {
      capabilities: { slack: true },
      connected: { slack: true }
    },
    identifiers: [{ identifier_type: 'slack', identifier: 'team-does-not-matter', verified: true }]
  });

  const coordination = findResource(resources, RESOURCE_CATEGORY.COORDINATION_LAYER);
  const approval = findResource(resources, RESOURCE_CATEGORY.APPROVAL_POLICY);

  assert.equal(coordination.provider.key, 'discord');
  assert.equal(coordination.state, RESOURCE_PROVISIONING_STATE.PLANNED_MANUAL);
  assert.match(coordination.note, /Discord remains the selected coordination channel/i);

  assert.equal(approval.provider.key, 'discord');
  assert.equal(approval.state, RESOURCE_PROVISIONING_STATE.PLANNED_MANUAL);
  assert.match(approval.note, /Discord remains the selected approval channel/i);
});
