import { Link } from 'react-router-dom';
import WorkspaceSectionCard from '../components/workspace/WorkspaceSectionCard';
import WorkspaceStatusPill from '../components/workspace/WorkspaceStatusPill';
import { demoWorkspace } from '../data/demoWorkspace';
import { getProvisioningLabel } from '../domain/resourceModel';
import { loadWorkspaceBlueprint } from '../domain/workspaceBlueprint';
import { createWorkspaceHomeModel } from '../domain/workspaceHomeModel';

function WorkspaceHomePage() {
  const savedBlueprint = loadWorkspaceBlueprint();
  const isUsingDemo = !savedBlueprint;
  const blueprint = savedBlueprint || demoWorkspace.blueprint;
  const model = createWorkspaceHomeModel(blueprint, { demoMode: isUsingDemo });

  return (
    <main className="route-shell route-shell-workspace">
      <div className="route-card route-card-workspace">
        <p className="route-kicker">Workspace Home</p>
        <h1>{model.title}</h1>
        <p>{model.subtitle}</p>

        {isUsingDemo ? (
          <p className="workspace-inline-note">
            Showing demo workspace data. Complete founder intake to generate your own canonical workspace blueprint.
          </p>
        ) : null}

        <section className="workspace-health-row" aria-label="Workspace readiness">
          <article className="workspace-health-item">
            <span>Connected resources</span>
            <strong>{model.workspaceHealth.connected}</strong>
          </article>
          <article className="workspace-health-item">
            <span>Pending setup</span>
            <strong>{model.workspaceHealth.nonConnected}</strong>
          </article>
          <article className="workspace-health-item">
            <span>Readiness</span>
            <strong>{model.workspaceHealth.readinessLabel}</strong>
          </article>
        </section>

        <section className="workspace-grid" aria-label="Workspace sections">
          <WorkspaceSectionCard
            title="Startup Brief"
            subtitle="Founder context and execution window"
          >
            <ul className="workspace-list">
              <li>
                <strong>Founder:</strong> {model.founderName}
              </li>
              <li>
                <strong>Stage:</strong> {model.stage}
              </li>
              <li>
                <strong>Planning horizon:</strong> {model.planHorizonDays} days
              </li>
            </ul>
            <h3>Goals (30-90 days)</h3>
            <ul className="workspace-list">
              {model.goals.map((goal) => (
                <li key={goal}>{goal}</li>
              ))}
            </ul>
          </WorkspaceSectionCard>

          <WorkspaceSectionCard title="Agent Roster" subtitle="Digital founding team ownership">
            <ul className="workspace-list">
              {model.agentRoster.map((agent) => (
                <li key={`${agent.role}-${agent.name}`} className="workspace-list-row">
                  <div>
                    <strong>{agent.role}</strong>
                    <p>{agent.focus}</p>
                  </div>
                  <div className="workspace-row-right">
                    <span>{agent.name}</span>
                    <WorkspaceStatusPill status={agent.status} label={agent.status} />
                  </div>
                </li>
              ))}
            </ul>
          </WorkspaceSectionCard>

          <WorkspaceSectionCard title="Resource Status" subtitle="Product objects mapped to providers">
            <ul className="workspace-list">
              {model.resources.map((resource) => (
                <li
                  key={`${resource.category}-${resource.provider.key}`}
                  className="workspace-list-row"
                >
                  <div>
                    <strong>{resource.object_name}</strong>
                    <p>
                      {resource.object_purpose}
                    </p>
                    <p>
                      Provider: {resource.provider.display_name}
                      {resource.note ? ` | ${resource.note}` : ''}
                    </p>
                    {resource.manual_next_step ? (
                      <p>Manual next step: {resource.manual_next_step}</p>
                    ) : null}
                  </div>
                  <WorkspaceStatusPill
                    status={resource.state}
                    label={getProvisioningLabel(resource.state)}
                  />
                </li>
              ))}
            </ul>
          </WorkspaceSectionCard>

          <WorkspaceSectionCard title="Starter Task Board" subtitle="Blueprint-derived execution graph">
            <ul className="workspace-list">
              {model.starterTasks.map((task) => (
                <li key={task.id} className="workspace-list-row">
                  <div>
                    <strong>{task.title}</strong>
                    <p>
                      Owner: {task.ownerRole}
                      {task.dependsOn.length ? ` | Depends on: ${task.dependsOn.join(', ')}` : ''}
                    </p>
                    <p>{task.rationale}</p>
                  </div>
                  <WorkspaceStatusPill status={task.status} label={task.status.replace('_', ' ')} />
                </li>
              ))}
            </ul>
          </WorkspaceSectionCard>

          <WorkspaceSectionCard title="Recent Artifacts" subtitle="Reviewable outputs">
            <ul className="workspace-list">
              {model.recentArtifacts.map((artifact) => (
                <li key={artifact.id} className="workspace-list-row">
                  <div>
                    <strong>{artifact.title}</strong>
                    <p>
                      Surface: {artifact.surface} | Updated: {artifact.updatedAtLabel}
                    </p>
                  </div>
                  <WorkspaceStatusPill status={artifact.status} label={artifact.status} />
                </li>
              ))}
            </ul>
          </WorkspaceSectionCard>

          <WorkspaceSectionCard title="Approval Queue" subtitle="Human decisions required before sensitive actions">
            <ul className="workspace-list">
              {model.approvalQueue.map((approval) => (
                <li key={approval.id} className="workspace-list-row">
                  <div>
                    <strong>{approval.title}</strong>
                    <p>
                      Owner: {approval.owner} | {approval.reason}
                    </p>
                  </div>
                  <WorkspaceStatusPill
                    status={approval.status}
                    label={approval.status.replace('_', ' ')}
                  />
                </li>
              ))}
            </ul>
          </WorkspaceSectionCard>

          <WorkspaceSectionCard title="Next Recommended Actions" subtitle="What to do next">
            <ol className="workspace-list workspace-list-ordered">
              {model.nextActions.map((action) => (
                <li key={action}>{action}</li>
              ))}
            </ol>
            <h3>Current assets</h3>
            <ul className="workspace-list">
              {model.currentAssets.length ? (
                model.currentAssets.map((asset) => <li key={asset}>{asset}</li>)
              ) : (
                <li>No assets listed yet.</li>
              )}
            </ul>
          </WorkspaceSectionCard>
        </section>

        <div className="route-actions">
          <Link className="btn btn-primary" to="/start">
            {isUsingDemo ? 'Create your blueprint' : 'Edit founder intake'}
          </Link>
          <Link className="btn btn-secondary" to="/">
            Back to landing
          </Link>
          <Link className="btn btn-secondary" to="/dashboard">
            Internal analytics dashboard
          </Link>
        </div>
      </div>
    </main>
  );
}

export default WorkspaceHomePage;
