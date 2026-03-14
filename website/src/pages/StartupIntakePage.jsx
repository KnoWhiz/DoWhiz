import { Link } from 'react-router-dom';

function StartupIntakePage() {
  return (
    <main className="route-shell route-shell-intake">
      <div className="route-card">
        <p className="route-kicker">Phase 0 Seam</p>
        <h1>Startup Intake</h1>
        <p>
          Intake has moved to a dedicated route and will be replaced by the structured founder workflow in the next
          phase.
        </p>
        <div className="route-actions">
          <a className="btn btn-primary" href="/#deployment-intake">
            Open Current Intake Form
          </a>
          <Link className="btn btn-secondary" to="/">
            Back to Landing
          </Link>
        </div>
      </div>
    </main>
  );
}

export default StartupIntakePage;
