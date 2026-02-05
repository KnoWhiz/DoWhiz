import { useState, useEffect } from 'react';
import oliverImg from './assets/Oliver.jpg';
import miniMouseImg from './assets/Mini-Mouse.jpg';
import workflowEmail from './assets/Workflow-Email.png';
import workflowTriage from './assets/Workflow-Triage.png';
import workflowExecution from './assets/Workflow-Execution.png';
import workflowReturn from './assets/Workflow-Return.png';

function App() {
  const [mounted, setMounted] = useState(false);
  const [theme, setTheme] = useState('dark');

  useEffect(() => {
    setMounted(true);
  }, []);

  useEffect(() => {
    document.documentElement.setAttribute('data-theme', theme);
  }, [theme]);

  const handleEmailClick = (email, subject, body) => {
    const encodedSubject = encodeURIComponent(subject);
    const encodedBody = encodeURIComponent(body);
    window.location.href = `mailto:${email}?subject=${encodedSubject}&body=${encodedBody}`;
  };

  return (
    <div className={`app-container ${mounted ? 'visible' : ''}`}>
      {/* Navigation */}
      <nav className="navbar">
        <div className="container nav-content">
          <a href="#" className="logo">Do<span className="text-gradient">Whiz</span></a>
          <div className="nav-links">
            <a href="#roles" className="nav-btn">Team</a>
            <a href="#features" className="nav-btn">Features</a>
            <a href="#security" className="nav-btn">Security</a>
          </div>
          <div className="nav-actions">
            <div className="social-links">
              <a href="https://github.com/KnoWhiz/DoWhiz" target="_blank" rel="noopener noreferrer" className="btn-small" aria-label="GitHub">
                <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round">
                  <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                </svg>
                <span>GitHub</span>
              </a>
              <a href="https://discord.gg/7ucnweCKk8" target="_blank" rel="noopener noreferrer" className="btn-small" aria-label="Discord">
                <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round">
                  <path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"></path>
                </svg>
                <span>Discord</span>
              </a>
              <a className="btn-small" href="mailto:admin@dowhiz.com" aria-label="Contact">
                <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round">
                  <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
                  <polyline points="22,6 12,13 2,6"></polyline>
                </svg>
                <span>Contact</span>
              </a>
            </div>
          </div>
        </div>
      </nav>

      {/* Hero Section */}
      <section className="hero-section">
        <div className="halo-effect"></div>
        <div className="container hero-content">
          <h1 className="hero-title">
            Empower Everyone<br />
            <span className="text-gradient">with A Digital Employee Team</span>
          </h1>
          <p className="hero-subtitle">
            Seamlessly collaborate with <a href="#roles" className="role-link">Oliver</a>, <a href="#roles" className="role-link">Mini-Mouse</a>, and your entire digital workforce—directly from your email inbox.
          </p>
          <div className="hero-cta">
            <a className="btn btn-primary" href="mailto:oliver@dowhiz.com">
              Start Free Trial
            </a>
          </div>
        </div>
      </section>

      {/* Roles & Scenarios */}
      <section id="roles" className="section roles-section">
        <div className="container">
          <h2 className="section-title">Meet Your Digital Employee Team</h2>
          <div className="roles-grid">
            {/* Oliver */}
            <div
              className="role-card active-role clickable-card"
              onClick={() => handleEmailClick('Oliver@DoWhiz.com', 'Task Request', 'Analyze the attached sales report and summarize key Q3 trends.')}
              title="Click to email Oliver"
            >
              <div className="role-header">
                <div className="role-profile">
                  <img src={oliverImg} alt="Oliver" className="role-avatar" />
                  <div>
                    <h3>Oliver</h3>
                    <code className="email-tag">Oliver@DoWhiz.com</code>
                  </div>
                </div>
                <span className="status-badge status-active">Active</span>
              </div>
              <p className="role-desc">Specialist in daily office tasks like scheduling, summaries, and reporting.</p>
              <div className="role-example">
                <span className="example-label">Example Task</span>
                <p>"Analyze the attached sales report and summarize key Q3 trends."</p>
              </div>
            </div>

            {/* Mini-Mouse */}
            <div
              className="role-card coming-soon clickable-card"
              onClick={() => handleEmailClick('Mini-Mouse@DoWhiz.com', 'Creative Request', 'Draft a press release for our product launch based on these notes.')}
              title="Click to email Mini-Mouse"
            >
              <div className="role-header">
                <div className="role-profile">
                  <img src={miniMouseImg} alt="Mini-Mouse" className="role-avatar" />
                  <div>
                    <h3>Mini-Mouse</h3>
                    <code className="email-tag">Mini-Mouse@DoWhiz.com</code>
                  </div>
                </div>
                <span className="status-badge status-soon">Coming</span>
              </div>
              <p className="role-desc">Expert in creativity like video editing, content drafts, and visual storytelling.</p>
              <div className="role-example">
                <span className="example-label">Example Task</span>
                <p>"Draft a press release for our product launch based on these notes."</p>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Core Experience */}
      <section id="features" className="section features-section">
        <div className="container">
          <h2 className="section-title">Core Features</h2>
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

      {/* Workflow */}
      <section className="section workflow-section">
        <div className="container">
          <h2 className="section-title">How It Works</h2>
          <div className="workflow-container">
            <div className="workflow-line"></div>
            {[
              { step: "1", label: "Email", img: workflowEmail },
              { step: "2", label: "Triage", img: workflowTriage },
              { step: "3", label: "Execution", img: workflowExecution },
              { step: "4", label: "Return", img: workflowReturn },
            ].map((s, i) => (
              <div key={i} className="workflow-step" style={{ animationDelay: `${i * 0.2}s` }}>
                <div className="step-icon-wrapper">
                  <div className="step-icon">
                    <img src={s.img} alt={s.label} />
                  </div>
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

      {/* Theme Toggle */}
      <button
        className="theme-toggle"
        onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
        aria-label="Toggle theme"
      >
        {theme === 'dark' ? (
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round">
            <circle cx="12" cy="12" r="5"></circle>
            <line x1="12" y1="1" x2="12" y2="3"></line>
            <line x1="12" y1="21" x2="12" y2="23"></line>
            <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
            <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
            <line x1="1" y1="12" x2="3" y2="12"></line>
            <line x1="21" y1="12" x2="23" y2="12"></line>
            <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
            <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
          </svg>
        ) : (
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round">
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
          </svg>
        )}
      </button>
    </div>
  );
}

export default App;
