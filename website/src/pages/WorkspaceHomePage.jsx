import { Link } from 'react-router-dom';
import { demoWorkspace } from '../data/demoWorkspace';
import { loadWorkspaceBlueprint } from '../domain/workspaceBlueprint';

function WorkspaceHomePage() {
  const blueprint = loadWorkspaceBlueprint();

  if (!blueprint) {
    return (
      <main className="route-shell route-shell-workspace">
        <div className="route-card">
          <p className="route-kicker">Workspace Home</p>
          <h1>{demoWorkspace.name}</h1>
          <p>{demoWorkspace.summary}</p>
          <section className="route-section">
            <h2>Connected Surfaces</h2>
            <ul>
              {demoWorkspace.channels.map((channel) => (
                <li key={channel}>{channel}</li>
              ))}
            </ul>
          </section>
          <section className="route-section">
            <h2>Next Actions</h2>
            <ul>
              {demoWorkspace.nextActions.map((item) => (
                <li key={item}>{item}</li>
              ))}
            </ul>
          </section>
          <div className="route-actions">
            <Link className="btn btn-primary" to="/start">
              Continue to Intake
            </Link>
            <Link className="btn btn-secondary" to="/dashboard">
              Open Internal Dashboard
            </Link>
          </div>
        </div>
      </main>
    );
  }

  return (
    <main className="route-shell route-shell-workspace">
      <div className="route-card">
        <p className="route-kicker">Workspace Home</p>
        <h1>{blueprint.venture.name || 'Founder Workspace'}</h1>
        <p>{blueprint.venture.thesis}</p>

        <section className="route-section">
          <h2>Startup Brief</h2>
          <ul>
            <li>Founder: {blueprint.founder.name}</li>
            <li>Stage: {blueprint.venture.stage || 'Not set'}</li>
            <li>Planning horizon: {blueprint.plan_horizon_days} days</li>
          </ul>
        </section>

        <section className="route-section">
          <h2>Goals (30-90 Days)</h2>
          <ul>
            {blueprint.goals_30_90_days.map((goal) => (
              <li key={goal}>{goal}</li>
            ))}
          </ul>
        </section>

        <section className="route-section">
          <h2>Agent Roster Request</h2>
          <ul>
            {blueprint.requested_agents.length ? (
              blueprint.requested_agents.map((agent) => (
                <li key={`${agent.role}-${agent.owner || 'unassigned'}`}>
                  {agent.role}
                  {agent.owner ? ` (owner: ${agent.owner})` : ''}
                </li>
              ))
            ) : (
              <li>No specific agents requested yet.</li>
            )}
          </ul>
        </section>

        <section className="route-section">
          <h2>Preferred Channels</h2>
          <ul>
            {blueprint.preferred_channels.length ? (
              blueprint.preferred_channels.map((channel) => <li key={channel}>{channel}</li>)
            ) : (
              <li>No channel preferences selected yet.</li>
            )}
          </ul>
        </section>

        <section className="route-section">
          <h2>Stack Snapshot</h2>
          <ul>
            <li>Existing repo: {blueprint.stack.has_existing_repo ? 'Yes' : 'No'}</li>
            <li>Repo provider: {blueprint.stack.primary_repo_provider || 'Not connected'}</li>
            <li>Docs workspace: {blueprint.stack.has_docs_workspace ? 'Yes' : 'No'}</li>
          </ul>
        </section>

        <section className="route-section">
          <h2>Current Assets</h2>
          <ul>
            {blueprint.current_assets.length ? (
              blueprint.current_assets.map((asset) => <li key={asset}>{asset}</li>)
            ) : (
              <li>No assets listed yet.</li>
            )}
          </ul>
        </section>

        <div className="route-actions">
          <Link className="btn btn-primary" to="/start">
            Edit intake
          </Link>
          <Link className="btn btn-secondary" to="/dashboard">
            Open internal dashboard
          </Link>
        </div>
      </div>
    </main>
  );
}

export default WorkspaceHomePage;
