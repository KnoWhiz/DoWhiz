import { useState, useEffect } from 'react';

function App() {
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  return (
    <div className={`app-container ${mounted ? 'visible' : ''}`}>
      {/* Navigation */}
      <nav className="navbar">
        <div className="container nav-content">
          <div className="logo">DoWhiz</div>
          <div className="nav-links">
            <a href="#features">Features</a>
            <a href="#roles">Digital Workers</a>
            <a href="#security">Security</a>
          </div>
          <button className="btn-small">Contact Sales</button>
        </div>
      </nav>

      {/* Hero Section */}
      <section className="hero-section">
        <div className="halo-effect"></div>
        <div className="container hero-content">
          <h1 className="hero-title">
            Empower Every User with <br />
            <span className="text-gradient">A Digital Team via Email</span>
          </h1>
          <p className="hero-subtitle">
            Seamlessly collaborate with Oliver, Mini-Mouse, and your entire digital workforce—directly from your inbox.
          </p>
          <div className="hero-cta">
            <button className="btn btn-primary">
              Start Free Trial
            </button>
          </div>
        </div>
      </section>

      {/* Core Experience */}
      <section id="features" className="section features-section">
        <div className="container">
          <h2 className="section-title">Core Experience</h2>
          <div className="features-grid">
            {[
              { title: "Task via Email", desc: "Delegate tasks naturally by sending an email. No complex dashboards or new apps to master." },
              { title: "Autonomous Tools", desc: "Agents intelligently select and utilize the right tools for each request without manual configuration." },
              { title: "Direct Delivery", desc: "Completed work is delivered straight back to your inbox. Focus on results, not the process." }
            ].map((item, idx) => (
              <div key={idx} className="feature-card">
                <h3>{item.title}</h3>
                <p>{item.desc}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Roles & Scenarios */}
      <section id="roles" className="section roles-section">
        <div className="container">
          <h2 className="section-title">Meet Your Digital Team</h2>
          <div className="roles-grid">
            {/* Oliver */}
            <div className="role-card active-role">
              <div className="role-header">
                <div>
                  <h3>Oliver</h3>
                  <code className="email-tag">Oliver@DoWhiz.com</code>
                </div>
                <span className="status-badge status-active">Active</span>
              </div>
              <p className="role-desc">Specialist in data analysis, comprehensive reporting, and market intelligence.</p>
              <div className="role-example">
                <span className="example-label">Example Task</span>
                <p>"Analyze the attached sales report and summarize key Q3 trends."</p>
              </div>
            </div>

            {/* Mini-Mouse */}
            <div className="role-card coming-soon">
              <div className="role-header">
                <div>
                  <h3>Mini-Mouse</h3>
                  <code className="email-tag">Mini-Mouse@DoWhiz.com</code>
                </div>
                <span className="status-badge status-soon">Coming Soon</span>
              </div>
              <p className="role-desc">Expert in creative writing, PR communications, and content drafting.</p>
              <div className="role-example">
                <span className="example-label">Example Task</span>
                <p>"Draft a press release for our product launch based on these notes."</p>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Workflow */}
      <section className="section workflow-section">
        <div className="container">
          <h2 className="section-title">How It Works</h2>
          <div className="workflow-container">
            <div className="workflow-line"></div>
            {[
              { step: "1", label: "Email", icon: "✉️" },
              { step: "2", label: "Triage", icon: "🧠" },
              { step: "3", label: "Execution", icon: "⚙️" },
              { step: "4", label: "Return", icon: "✅" },
            ].map((s, i) => (
              <div key={i} className="workflow-step">
                <div className="step-icon">
                  {s.icon}
                </div>
                <h4>{s.label}</h4>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Security */}
      <section id="security" className="section security-section">
        <div className="container">
          <h2 className="section-title">Security First Design</h2>
          <div className="security-grid">
            <div className="security-card">
              <h3>Isolation</h3>
              <p>
                User data and agent memories are strictly isolated. Your data never leaks to other tenants.
              </p>
            </div>
            <div className="security-card">
              <h3>Auditability & Trust</h3>
              <p>
                All actions are logged and reversible. You maintain full control over your digital workforce.
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer>
        <p>&copy; {new Date().getFullYear()} DoWhiz. All rights reserved.</p>
      </footer>
    </div>
  );
}

export default App;
