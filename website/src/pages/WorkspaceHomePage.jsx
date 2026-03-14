import { Link } from 'react-router-dom';
import { demoWorkspace } from '../data/demoWorkspace';

function WorkspaceHomePage() {
  return (
    <main className="route-shell route-shell-workspace">
      <div className="route-card">
        <p className="route-kicker">Phase 0 Seam</p>
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

export default WorkspaceHomePage;
