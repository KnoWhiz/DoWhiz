import { useEffect } from 'react';
import { Link } from 'react-router-dom';
import { demoWorkspace } from '../data/demoWorkspace';
import { loadWorkspaceBlueprint } from '../domain/workspaceBlueprint';
import { createWorkspaceHomeModel } from '../domain/workspaceHomeModel';

const DASHBOARD_WORKSPACE_ANCHOR = '/auth/index.html?loggedIn=true#section-workspace';

function WorkspaceHomePage() {
  const savedBlueprint = loadWorkspaceBlueprint();
  const isUsingDemo = !savedBlueprint;
  const blueprint = savedBlueprint || demoWorkspace.blueprint;
  const model = createWorkspaceHomeModel(blueprint, { demoMode: isUsingDemo });

  useEffect(() => {
    if (typeof window === 'undefined') {
      return undefined;
    }

    const timeoutId = window.setTimeout(() => {
      window.location.replace(DASHBOARD_WORKSPACE_ANCHOR);
    }, 250);

    return () => window.clearTimeout(timeoutId);
  }, []);

  return (
    <main className="route-shell route-shell-workspace">
      <div className="route-card route-card-workspace-compact">
        <p className="route-kicker">Workspace</p>
        <h1>Redirecting To Your Unified Dashboard</h1>
        <p>
          Workspace preview is now merged into your dashboard. You will be redirected automatically.
        </p>

        {isUsingDemo ? (
          <p className="workspace-inline-note">
            Showing demo values because you have not saved a founder intake yet.
          </p>
        ) : (
          <p className="workspace-inline-note">
            Blueprint loaded from your latest saved founder intake.
          </p>
        )}

        <section className="workspace-health-row" aria-label="Workspace readiness">
          <article className="workspace-health-item">
            <span>Workspace</span>
            <strong>{model.title}</strong>
          </article>
          <article className="workspace-health-item">
            <span>Connected resources</span>
            <strong>{model.workspaceHealth.connected}</strong>
          </article>
          <article className="workspace-health-item">
            <span>Readiness</span>
            <strong>{model.workspaceHealth.readinessLabel}</strong>
          </article>
        </section>

        <section className="workspace-quick-grid" aria-label="Workspace summary">
          <article className="workspace-quick-card">
            <h2>Team</h2>
            <ul className="workspace-list">
              <li>Founder: {model.founderName}</li>
              {model.agentRoster.slice(0, 3).map((agent) => (
                <li key={`${agent.role}-${agent.name}`}>
                  {agent.role}: {agent.name}
                </li>
              ))}
            </ul>
          </article>

          <article className="workspace-quick-card">
            <h2>Goals</h2>
            <ul className="workspace-list">
              {model.goals.slice(0, 3).map((goal) => (
                <li key={goal}>{goal}</li>
              ))}
            </ul>
          </article>

          <article className="workspace-quick-card">
            <h2>Channels</h2>
            <ul className="workspace-list">
              {model.preferredChannels.map((channel) => (
                <li key={channel}>{channel}</li>
              ))}
            </ul>
          </article>
        </section>

        <div className="route-actions">
          <a className="btn btn-primary" href={DASHBOARD_WORKSPACE_ANCHOR}>
            Open unified dashboard
          </a>
          <Link className="btn btn-secondary" to="/start">
            Edit team brief
          </Link>
          <Link className="btn btn-secondary" to="/">
            Back to landing
          </Link>
        </div>
      </div>
    </main>
  );
}

export default WorkspaceHomePage;
